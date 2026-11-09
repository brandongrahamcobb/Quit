/* app/src/system/product/activate/add_edit.rs
 * The purpose of this module is to provide a add and edit observers for the product system.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use base;
use bevy::prelude::*;
use bevy::text::{EditableText, TextEdit};
use bevy::ui_widgets::Activate;
use bigdecimal::BigDecimal;
use db::product::getters::get_all_product_models;
use db::product::model::{NewProductModel, ProductCompoundInput, UpdateProductModel};
use db::product::setters::{insert_product_with_compounds, update_product_with_compounds};
use crate::component::product::{ProductBrandText, ProductCompositionRemoveButton, ProductCompositionSearchText, ProductCompositionUnitButton, ProductCompoundToggleButton, ProductDescriptionText, ProductEditButton, ProductNameText, ProductRouteOptionButton, ProductServingDescriptionText, ProductServingSizeText, ProductServingUnitOptionButton};
use crate::resource::custom_resource::Database;
use crate::resource::product_resource::{ProductCompositionEntry, ProductEntry, ProductImageUploadReceiver, ProductOverlay, ProductPageState, ProductSaveResults};
use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Builder;

pub fn cancel_save(
    _activate: On<Activate>,
    mut state: ResMut<ProductPageState>,
) {
    if state.loading {
        return;
    }
    state.overlay = None;
}

pub fn save_product(
    _activate: On<Activate>,
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductSaveResults>,
) {
    if state.loading {
        return;
    }
    let editing_id = match state.overlay {
        Some(ProductOverlay::Form { editing_id }) => editing_id,
        _ => return,
    };
    let name = state.name.trim().to_string();
    let route = state.route.clone();
    if name.is_empty() {
        state.error = Some("Product name is required.".to_string());
        return;
    }
    if !base::product::PRODUCT_ROUTES.contains(&route.as_str()) {
        state.error = Some("Select a valid route.".to_string());
        return;
    }
    let Ok(serving_size) = state.serving_size.trim().parse::<BigDecimal>() else {
        state.error = Some("Enter a valid serving size.".to_string());
        return;
    };
    if serving_size <= BigDecimal::from(0) {
        state.error = Some("Serving size must be greater than zero.".to_string());
        return;
    }
    if !["mg", "mL", "unit"].contains(&state.serving_unit.as_str()) {
        state.error = Some("Select a valid serving unit.".to_string());
        return;
    }
    let mut compounds = Vec::new();
    for compound in &state.selected_compounds {
        let Ok(amount) = compound.compound_amount.parse::<BigDecimal>() else {
            state.error = Some(format!("Enter a valid amount for {}.", compound.compound_name));
            return;
        };
        if amount <= BigDecimal::from(0) {
            state.error = Some(format!("Amount for {} must be greater than zero.", compound.compound_name));
            return;
        }
        if !["mg", "mL"].contains(&compound.compound_units.as_str()) {
            state.error = Some(format!("Select a valid unit for {}.", compound.compound_name));
            return;
        }
        compounds.push(ProductCompoundInput {
            compound_id: compound.compound_id,
            compound_amount: amount,
            compound_units: compound.compound_units.clone(),
        });
    }
    let new_product = NewProductModel {
        name,
        route,
        brand: {
            let brand = state.brand.trim();
            (!brand.is_empty()).then(|| brand.to_string())
        },
        description: {
            let description = state.description.trim();
            (!description.is_empty()).then(|| description.to_string())
        },
        image_path: state.image_path.clone(),
        serving_size,
        serving_unit: state.serving_unit.clone(),
        serving_description: {
            let description = state.serving_description.trim();
            (!description.is_empty()).then(|| description.to_string())
        },
    };
    let updated_product = UpdateProductModel {
        name: new_product.name.clone(),
        route: new_product.route.clone(),
        brand: new_product.brand.clone(),
        description: new_product.description.clone(),
        image_path: new_product.image_path.clone(),
        serving_size: new_product.serving_size.clone(),
        serving_unit: new_product.serving_unit.clone(),
        serving_description: new_product.serving_description.clone(),
    };
    let pool = database.0.clone();
    let sender = results.sender.clone();
    state.loading = true;
    state.error = None;
    thread::spawn(move || {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();
        let result = runtime.block_on(async move {
            let result = match editing_id {
                Some(product_id) => update_product_with_compounds(&pool, product_id, updated_product, compounds).await,
                None => insert_product_with_compounds(&pool, new_product, compounds).await,
            };
            match result {
                Ok(_) => get_all_product_models(&pool).await.map(|products| {
                    products.into_iter().map(|product| ProductEntry {
                        id: product.id,
                        name: product.name,
                        route: product.route,
                        brand: product.brand,
                        description: product.description,
                        image_path: product.image_path,
                        serving_size: product.serving_size,
                        serving_unit: product.serving_unit,
                        serving_description: product.serving_description,
                    }).collect()
                }),
                Err(error) => Err(error),
            }
        });
        let _ = sender.send(result.map_err(|error| error.to_string()));
    });
    state.overlay = None;
}

pub fn upload_product_image(
    _activate: On<Activate>,
    receiver: ResMut<ProductImageUploadReceiver>,
    mut state: ResMut<ProductPageState>,
) {
    if state.image_loading {
        return;
    }
    let (sender, channel_receiver) = channel();
    *receiver.0.lock().unwrap() = Some(channel_receiver);
    state.image_loading = true;
    thread::spawn(move || {
        let result = (|| {
            let Some(source) = rfd::FileDialog::new()
                .add_filter("Product images", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            else {
                return Ok(None);
            };
            let extension = source.extension()
                .and_then(|extension| extension.to_str())
                .map(str::to_ascii_lowercase)
                .filter(|extension| ["png", "jpg", "jpeg", "webp"].contains(&extension.as_str()))
                .ok_or_else(|| "Unsupported image format.".to_owned())?;
            let directory = std::path::Path::new("crates/app/assets/product_images");
            std::fs::create_dir_all(directory)
                .map_err(|error| format!("Could not create image directory: {error}"))?;
            let filename = format!(
                "product_{}.{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|error| error.to_string())?
                    .as_nanos(),
                extension
            );
            let asset_path = format!("product_images/{filename}");
            let destination = directory.join(&filename);
            std::fs::copy(&source, &destination)
                .map_err(|error| format!("Could not copy image: {error}"))?;
            Ok(Some(asset_path))
        })();
        let _ = sender.send(result);
    });
}

pub fn remove_product_image(
    _activate: On<Activate>,
    mut state: ResMut<ProductPageState>,
) {
    state.image_path = None;
    state.error = None;
}

pub fn edit_product(
    activate: On<Activate>,
    edit_buttons: Query<&ProductEditButton>,
    inputs: ParamSet<(
        Query<&mut EditableText, With<ProductBrandText>>,
        Query<&mut EditableText, With<ProductDescriptionText>>,
        Query<&mut EditableText, With<ProductNameText>>,
        Query<&mut EditableText, With<ProductServingDescriptionText>>,
        Query<&mut EditableText, With<ProductServingSizeText>>,
        Query<&mut EditableText, With<ProductCompositionSearchText>>,
    )>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = edit_buttons.get(activate.entity) else {
        return;
    };
    let Some(product) = state.products.iter().find(|product| product.id == button.0).cloned() else {
        return;
    };
    state.composition_load_generation = state.composition_load_generation.wrapping_add(1);
    state.composition_loading = false;
    state.composition_load_requested_id = None;
    state.composition_search_query.clear();
    state.composition_search_query_open = false;
    state.name = product.name;
    state.route = product.route;
    state.brand = product.brand.unwrap_or_default();
    state.description = product.description.unwrap_or_default();
    state.selected_compounds.clear();
    state.image_path = product.image_path;
    state.serving_unit_selector_open = false;
    state.serving_size = product.serving_size.to_string();
    state.serving_unit = product.serving_unit.clone();
    state.serving_description = product.serving_description.unwrap_or_default();
    state.error = None;
    state.overlay = Some(ProductOverlay::Form { editing_id: Some(product.id) });
    fill_form(inputs, state);
}

pub fn fill_form(
    mut inputs: ParamSet<(
        Query<&mut EditableText, With<ProductBrandText>>,
        Query<&mut EditableText, With<ProductDescriptionText>>,
        Query<&mut EditableText, With<ProductNameText>>,
        Query<&mut EditableText, With<ProductServingDescriptionText>>,
        Query<&mut EditableText, With<ProductServingSizeText>>,
        Query<&mut EditableText, With<ProductCompositionSearchText>>,
    )>,
    state: ResMut<ProductPageState>,
) {
    fn set_text(input: &mut EditableText, value: &str) {
        input.queue_edit(TextEdit::SelectAll);
        if value.is_empty() {
            input.queue_edit(TextEdit::Delete);
        } else {
            input.queue_edit(TextEdit::Insert(value.into()));
        }
    }
    for mut i in &mut inputs.p2() { set_text(&mut i, &state.name); }
    for mut i in &mut inputs.p0() { set_text(&mut i, &state.brand); }
    for mut i in &mut inputs.p1() { set_text(&mut i, &state.description); }
    for mut i in &mut inputs.p3() { set_text(&mut i, &state.serving_description); }
    for mut i in &mut inputs.p4() { set_text(&mut i, &state.serving_size); }
    for mut i in &mut inputs.p5() { set_text(&mut i, ""); }
}

pub fn add_product(
    _activate: On<Activate>,
    inputs: ParamSet<(
        Query<&mut EditableText, With<ProductBrandText>>,
        Query<&mut EditableText, With<ProductDescriptionText>>,
        Query<&mut EditableText, With<ProductNameText>>,
        Query<&mut EditableText, With<ProductServingDescriptionText>>,
        Query<&mut EditableText, With<ProductServingSizeText>>,
        Query<&mut EditableText, With<ProductCompositionSearchText>>,
    )>,
    mut state: ResMut<ProductPageState>,
) {
    state.composition_load_generation = state.composition_load_generation.wrapping_add(1);
    state.composition_loading = false;
    state.composition_load_requested_id = None;
    state.composition_search_query.clear();
    state.composition_search_query_open = false;
    state.name.clear();
    state.route.clear();
    state.brand.clear();
    state.description.clear();
    state.selected_compounds.clear();
    state.image_path = None;
    state.serving_size = "1".to_string();
    state.serving_unit = "unit".to_string();
    state.serving_description.clear();
    state.serving_unit_selector_open = false;
    state.error = None;
    state.overlay = Some(ProductOverlay::Form { editing_id: None });
    fill_form(inputs, state);
}


pub fn toggle_product_composition(
    activate: On<Activate>,
    buttons: Query<&ProductCompoundToggleButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = buttons.get(activate.entity) else {
        return;
    };
    let id = button.0;
    if state.selected_compounds.iter().any(|compound| compound.compound_id == id) {
        state.selected_compounds.retain(|compound| compound.compound_id != id);
    } else if let Some(compound_name) = state.compounds.iter().find(|compound| compound.id == id).map(|compound| compound.common_name.clone()) {
        state.selected_compounds.push(ProductCompositionEntry {
            compound_id: id,
            compound_name,
            compound_amount: String::new(),
            compound_units: "mg".to_string(),
        });
    }
    state.composition_search_query_open = false;
    state.error = None;
}

pub fn cycle_product_compound_units(
    activate: On<Activate>,
    buttons: Query<&ProductCompositionUnitButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = buttons.get(activate.entity) else {
        return;
    };
    if let Some(compound) = state.selected_compounds.iter_mut().find(|compound| compound.compound_id == button.0) {
        compound.compound_units = match compound.compound_units.as_str() {
            "mg" => "mL".to_string(),
            _ => "mg".to_string(),
        };
    }
}

pub fn select_product_route(
    activate: On<Activate>,
    buttons: Query<&ProductRouteOptionButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = buttons.get(activate.entity) else {
        return;
    };
    state.route = button.0.clone();
    state.route_selector_open = false;
    state.error = None;
}

pub fn remove_product_compound(
    activate: On<Activate>,
    buttons: Query<&ProductCompositionRemoveButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = buttons.get(activate.entity) else {
        return;
    };
    state.selected_compounds.retain(|compound| compound.compound_id != button.0);
}

pub fn select_product_serving_unit(
    activate: On<Activate>,
    buttons: Query<&ProductServingUnitOptionButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = buttons.get(activate.entity) else {
        return;
    };
    state.serving_unit = button.0.clone();
    state.serving_unit_selector_open = false;
    state.error = None;
}

pub fn toggle_product_route_dropdown(
    _: On<Activate>,
    mut state: ResMut<ProductPageState>,
) {
    state.route_selector_open = !state.route_selector_open;
}

pub fn toggle_product_serving_unit_dropdown(
    _: On<Activate>,
    mut state: ResMut<ProductPageState>,
) {
    state.serving_unit_selector_open = !state.serving_unit_selector_open;
}
