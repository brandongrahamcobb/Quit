/* app/src/system/product.rs
 * The purpose of this module is to provide a product system.
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


use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use base;
use bigdecimal::BigDecimal;
use db::compound::getters::get_all_compound_models;
use db::product::getters::{get_all_product_models, get_product_compounds_by_id};
use db::product::model::{NewProductModel, ProductCompoundInput, UpdateProductModel};
use db::product::setters::{delete_product, insert_product_with_compounds, update_product_with_compounds};
use crate::component::product::{OpenProductsButton, ProductAddButton, ProductBrandButton, ProductBrandText, ProductCancelButton, ProductCompositionAmountButton, ProductCompositionAmountText, ProductCompositionItem, ProductCompositionListContainer, ProductCompositionRemoveButton, ProductCompositionUnitButton, ProductCompoundChoiceText, ProductCompoundListContainer, ProductCompoundToggleButton, ProductDashboardButton, ProductDeleteButton, ProductDeleteCancelButton, ProductDeleteConfirmButton, ProductDeleteDialog, ProductDeleteErrorText, ProductDeleteNameText, ProductDescriptionButton, ProductDescriptionText, ProductEditButton, ProductErrorText, ProductFormOverlay, ProductFormRoot, ProductImagePreview, ProductImagePreviewHandle, ProductImageRemoveButton, ProductImageUploadButton, ProductListContainer, ProductListItem, ProductListText, ProductNameButton, ProductNameText, ProductNavigationRoot, ProductPreviewButton, ProductPreviewImage, ProductRouteButton, ProductRouteOptionButton, ProductRouteOptionList, ProductRouteText, ProductSaveButton, ProductSaveButtonText, ProductSearchButton, ProductSearchText, ProductServingDescriptionButton, ProductServingDescriptionText, ProductServingSizeButton, ProductServingSizeText, ProductServingUnitButton, ProductServingUnitOptionButton, ProductServingUnitOptionList, ProductServingUnitText};
use crate::page::AppPage;
use crate::resource::compound_resource::CompoundEntry;
use crate::resource::custom_resource::Database;
// use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::product_resource::{ProductCompositionEntry, ProductCompositionLoadResults, ProductCompoundLoadResults, ProductDeleteResults, ProductEntry, ProductFormField, ProductImageUploadReceiver, ProductLoadReceiver, ProductPageState, ProductSaveResults};
use std::sync::mpsc::{TryRecvError, channel};
use std::thread;
use tokio::runtime::Builder;


pub fn open_products(buttons: Query<&Interaction, (Changed<Interaction>, With<OpenProductsButton>)>, mut next_page: ResMut<NextState<AppPage>>) {
    if buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        next_page.set(AppPage::Products);
    }
}

pub fn begin_product_load(database: Res<Database>, mut state: ResMut<ProductPageState>, receiver: Res<ProductLoadReceiver>) {
    if state.loading {
        return;
    }
    state.loading = true;
    state.error = None;
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        state.loading = false;
        state.error = Some("Could not initialize product loading.".to_string());
        return;
    }
    thread::spawn(move || {
        let result = match Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime
                .block_on(get_all_product_models(&pool))
                .map(|models| {
                    models
                        .into_iter()
                        .map(|model| ProductEntry {
                            id: model.id,
                            name: model.name,
                            route: model.route,
                            brand: model.brand,
                            description: model.description,
                            image_path: model.image_path,
                            serving_size: model.serving_size,
                            serving_unit: model.serving_unit,
                            serving_description: model.serving_description,
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|error| format!("{error:?}")),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn poll_product_load(receiver: Res<ProductLoadReceiver>, mut state: ResMut<ProductPageState>) {
    let result = {
        let Ok(mut slot) = receiver.0.lock() else {
            return;
        };
        let Some(result_receiver) = slot.as_ref() else {
            return;
        };
        match result_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Product loading worker disconnected.".to_string()))
            }
        }
    };
    let Some(result) = result else {
        return;
    };
    state.loading = false;
    match result {
        Ok(products) => {
            state.products = products;
            state.error = None;
        }
        Err(error) => {
            state.error = Some(error);
        }
    }
}

pub fn begin_product_compound_load(
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductCompoundLoadResults>,
) {
    if state.compounds_loading {
        return;
    }
    state.compounds_loaded = false;
    state.compounds_loading = true;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    thread::spawn(move || {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build();
        let result = match runtime {
            Ok(runtime) => runtime.block_on(async move {
                get_all_compound_models(&pool)
                    .await
                    .map(|models| {
                        models
                            .into_iter()
                            .map(|model| CompoundEntry {
                                id: model.id,
                                common_name: model.common_name,
                                iupac_name: model.iupac_name,
                                half_life_seconds: model.half_life_seconds,
                                image_path: model.image_path,
                            })
                            .collect()
                    })
                    .map_err(|error| format!("{error:?}"))
            }),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn apply_product_compound_load_results(
    results: Res<ProductCompoundLoadResults>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    while let Ok(result) = receiver.try_recv() {
        state.compounds_loading = false;
        match result {
            Ok(compounds) => {
                state.compounds = compounds;
                state.error = None;
                state.compounds_loaded = true;
                state.compounds_loading = false;
            }
            Err(error) => {
                state.compounds_loading = false;
                state.error = Some(format!("Could not load available compounds: {error}"));
            }
        }
    }
}

pub fn handle_product_form_buttons(
    add_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductAddButton>)>,
    edit_buttons: Query<(&Interaction, &ProductEditButton), (Changed<Interaction>, With<ProductEditButton>)>,
    cancel_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductCancelButton>)>,
    mut state: ResMut<ProductPageState>,
    mut form_overlay: Query<&mut Node, With<ProductFormOverlay>>,
) {
    let Ok(mut overlay) = form_overlay.single_mut() else {
        return;
    };
    let add_pressed = add_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    let edit_id = edit_buttons
        .iter()
        .find(|(interaction, _)| **interaction == Interaction::Pressed)
        .map(|(_, button)| button.0);
    let cancel_pressed = cancel_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    if cancel_pressed {
        if state.loading {
            return;
        }
        state.composition_load_generation = state.composition_load_generation.wrapping_add(1);
        state.composition_loading = false;
        state.composition_load_requested_id = None;
        state.serving_unit_selector_open = false;
        overlay.display = Display::None;
        state.error = None;
        return;
    }
    if add_pressed {
        state.composition_load_generation = state.composition_load_generation.wrapping_add(1);
        state.composition_loading = false;
        state.composition_load_requested_id = None;
        state.editing_id = None;
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
        state.focused_field = Some(ProductFormField::Name);
        state.error = None;
        overlay.display = Display::Flex;
    } else if let Some(id) = edit_id {
        let product = state
            .products
            .iter()
            .find(|product| product.id == id)
            .cloned();
        if let Some(product) = product {
            state.editing_id = Some(product.id);
            state.composition_load_generation = state.composition_load_generation.wrapping_add(1);
            state.composition_loading = false;
            state.composition_load_requested_id = None;
            state.name = product.name;
            state.route = product.route;
            state.brand = product.brand.unwrap_or_default();
            state.description = product.description.unwrap_or_default();
            state.selected_compounds.clear();
            state.image_path = product.image_path;
            state.serving_unit_selector_open = false;
            state.serving_size = product.serving_size.to_string();
            state.serving_unit = product.serving_unit.clone();
            state.serving_description = product.serving_description.clone().unwrap_or_default();
            state.focused_field = Some(ProductFormField::Name);
            state.error = None;
            overlay.display = Display::Flex;
        }
    }
}

pub fn focus_product_fields(
    serving_unit_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductServingUnitButton>)>,
    serving_unit_option_buttons: Query<(&Interaction, &ProductServingUnitOptionButton), Changed<Interaction>>,
    serving_description_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductServingDescriptionButton>)>,
    serving_size_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductServingSizeButton>)>,
    amount_buttons: Query<(&Interaction, &ProductCompositionAmountButton), Changed<Interaction>>,
    brand_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductBrandButton>)>,
    description_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductDescriptionButton>)>,
    name_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductNameButton>)>,
    route_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductRouteButton>)>,
    search_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductSearchButton>)>,
    mut state: ResMut<ProductPageState>,
    time: Res<Time>,
) {
    if brand_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(ProductFormField::Brand);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if description_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(ProductFormField::Description);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if serving_unit_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.serving_unit_selector_open = !state.serving_unit_selector_open;
        state.focused_field = None;
    }
    for (interaction, option) in &serving_unit_option_buttons {
        if *interaction == Interaction::Pressed {
            state.serving_unit = option.0.clone();
            state.serving_unit_selector_open = false;
            state.focused_field = None;
        }
    }
    if serving_description_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(ProductFormField::ServingDescription);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if serving_size_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(ProductFormField::ServingSize);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if name_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(ProductFormField::Name);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if route_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.route_selector_open = !state.route_selector_open;
        state.focused_field = None;
    }
    if search_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(ProductFormField::Search);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    for (interaction, button) in &amount_buttons {
        if *interaction == Interaction::Pressed {
            state.focused_field = Some(ProductFormField::CompoundAmount(button.0));
            state.caret_started_at = time.elapsed_secs_f64();
        }
    }
}

pub fn edit_product_fields(
    mut keyboard: MessageReader<KeyboardInput>,
    mut state: ResMut<ProductPageState>,
    time: Res<Time>,
) {
    let Some(field) = state.focused_field else {
        return;
    };
    for input in keyboard.read() {
        if input.state != ButtonState::Pressed {
            continue;
        }
        match input.key_code {
            KeyCode::Escape => {
                state.focused_field = None;
            }
            KeyCode::Backspace => {
                match field {
                    ProductFormField::Brand => { state.brand.pop(); }
                    ProductFormField::Description => { state.description.pop(); }
                    ProductFormField::Name => { state.name.pop(); }
                    ProductFormField::Search => { state.search_query.pop(); }
                    ProductFormField::Route => {}
                    ProductFormField::CompoundAmount(compound_id) => {
                        if let Some(compound) = state.selected_compounds.iter_mut().find(|compound| compound.compound_id == compound_id) {
                            compound.compound_amount.pop();
                        }
                    }
                    ProductFormField::ServingDescription => { state.serving_description.pop(); }
                    ProductFormField::ServingSize => { state.serving_size.pop(); }
                    ProductFormField::ServingUnit => {}
                }
                state.caret_started_at = time.elapsed_secs_f64();
            }
            _ => {
                let Some(text) = input.text.as_deref() else {
                    continue;
                };
                match field {
                    ProductFormField::Brand => {
                        state.brand.extend(text.chars().filter(|c| !c.is_control()));
                    }
                    ProductFormField::Description => {
                        state.description.extend(text.chars().filter(|c| !c.is_control()));
                    }
                    ProductFormField::Name => {
                        state.name.extend(text.chars().filter(|c| !c.is_control()));
                    }
                    ProductFormField::Search => {
                        state.search_query.extend(text.chars().filter(|c| !c.is_control()));
                    }
                    ProductFormField::Route => {}
                    ProductFormField::CompoundAmount(compound_id) => {
                        if let Some(compound) = state.selected_compounds.iter_mut().find(|compound| compound.compound_id == compound_id) {
                            compound.compound_amount.extend(text.chars().filter(|c| c.is_ascii_digit()));
                        }
                    }
                    ProductFormField::ServingDescription => {
                        state.serving_description.extend(text.chars().filter(|c| !c.is_control()));
                    }
                    ProductFormField::ServingSize => {
                        state.serving_size.extend(text.chars().filter(|c| c.is_ascii_digit() || *c == '.'));
                    }
                    ProductFormField::ServingUnit => {}
                }
                state.caret_started_at = time.elapsed_secs_f64();
            }
        }
    }
}

pub fn select_product_route(
    buttons: Query<(&Interaction, &ProductRouteOptionButton), Changed<Interaction>>,
    mut state: ResMut<ProductPageState>,
) {
    for (interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        state.route = button.0.clone();
        state.route_selector_open = false;
        state.error = None;
    }
}

pub fn begin_product_composition_load(
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductCompositionLoadResults>,
) {
    let Some(product_id) = state.editing_id else {
        return;
    };
    if !state.compounds_loaded || state.composition_loading || state.composition_load_requested_id == Some(product_id) {
        return;
    }
    let generation = state.composition_load_generation;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    state.composition_loading = true;
    state.composition_load_requested_id = Some(product_id);
    bevy::tasks::IoTaskPool::get().spawn(async move {
        let result = get_product_compounds_by_id(&pool, product_id).await.map_err(|error| error.to_string());
        let _ = sender.send(result.map(|compounds| (generation, product_id, compounds)));
    }).detach();
}

pub fn apply_product_composition_load_results(
    mut results: ResMut<ProductCompositionLoadResults>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    let Ok(result) = receiver.try_recv() else {
        return;
    };
    match result {
        Ok((generation, product_id, compounds)) => {
            if state.composition_load_generation != generation || state.editing_id != Some(product_id) {
                return;
            }
            state.composition_loading = false;
            let selected_compounds: Option<Vec<ProductCompositionEntry>> = compounds.into_iter().map(|compound| {
                state.compounds.iter().find(|entry| entry.id == compound.compound_id).map(|entry| ProductCompositionEntry {
                    compound_id: compound.compound_id,
                    compound_name: entry.common_name.clone(),
                    compound_amount: compound.compound_amount.to_string(),
                    compound_units: compound.compound_units,
                })
            }).collect();
            match selected_compounds {
                Some(compounds) => {
                    state.selected_compounds = compounds;
                }
                None => {
                    state.error = Some("A compound in this product's composition could not be found.".to_string());
                }
            }
        }
        Err(error) => {
            state.composition_loading = false;
            state.error = Some(error);
        }
    }
}

pub fn toggle_product_compounds(
    interactions: Query<
        (&Interaction, &ProductCompoundToggleButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<ProductPageState>,
) {
    for (interaction, button) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let compound_id = button.0;
        if state.selected_compounds.iter().any(|compound| compound.compound_id == compound_id) {
            state.selected_compounds.retain(|compound| compound.compound_id != compound_id);
        } else if let Some(compound_name) = state.compounds.iter().find(|compound| compound.id == compound_id).map(|compound| compound.common_name.clone()) {
            state.selected_compounds.push(ProductCompositionEntry {
                compound_id,
                compound_name,
                compound_amount: "1".to_string(),
                compound_units: "mg".to_string(),
            });
        }
        state.error = None;
    }
}

pub fn update_product_compound_buttons(
    mut commands: Commands,
    state: Res<ProductPageState>,
    list_container: Query<Entity, With<ProductCompoundListContainer>>,
    old_items: Query<Entity, With<ProductCompoundChoiceText>>,
    mut previous_signature: Local<Option<Vec<(i32, String, bool)>>>,
) {
    let Ok(container) = list_container.single() else {
        return;
    };
    let choices: Vec<(i32, String, bool)> = state.compounds.iter()
        .map(|compound| (
            compound.id,
            compound.common_name.clone(),
            state.selected_compounds.iter().any(|selected| selected.compound_id == compound.id),
        ))
        .collect();
    if previous_signature.as_ref() == Some(&choices) {
        return;
    }
    *previous_signature = Some(choices.clone());
    for entity in &old_items {
        commands.entity(entity).despawn();
    }
    commands.entity(container).with_children(|parent| {
        for (id, name, selected) in choices {
            let label = format!("{} {}", if selected { "[x]" } else { "[ ]" }, name);
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(if selected { Color::srgb(0.2, 0.45, 0.25) } else { Color::srgb(0.2, 0.2, 0.2) }),
                ProductCompoundToggleButton(id),
                ProductCompoundChoiceText,
            )).with_children(|button| {
                button.spawn((
                    Text::new(label),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}

pub fn update_product_composition_rows(
    mut commands: Commands,
    state: Res<ProductPageState>,
    list_container: Query<Entity, With<ProductCompositionListContainer>>,
    old_items: Query<Entity, With<ProductCompositionItem>>,
    mut previous_signature: Local<Option<Vec<(i32, String, String, String)>>>,
) {
    let Ok(container) = list_container.single() else {
        return;
    };
    let signature: Vec<_> = state.selected_compounds.iter()
        .map(|compound| (
            compound.compound_id,
            compound.compound_name.clone(),
            compound.compound_amount.clone(),
            compound.compound_units.clone(),
        ))
        .collect();
    if previous_signature.as_ref() == Some(&signature) {
        return;
    }
    *previous_signature = Some(signature.clone());
    for entity in &old_items {
        commands.entity(entity).despawn();
    }
    commands.entity(container).with_children(|parent| {
        for (id, name, amount, units) in signature {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.17, 0.20)),
                ProductCompositionItem,
            )).with_children(|row| {
                row.spawn((
                    Text::new(name),
                    TextFont { font_size: 15.0, ..default() },
                    Node { flex_grow: 1.0, ..default() },
                ));
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(90.0),
                        min_height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductCompositionAmountButton(id),
                )).with_children(|button| {
                    button.spawn((
                        Text::new(amount),
                        ProductCompositionAmountText(id),
                    ));
                });
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(55.0),
                        min_height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.22, 0.30, 0.42)),
                    ProductCompositionUnitButton(id),
                )).with_children(|button| {
                    button.spawn(Text::new(units));
                });
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.48, 0.20, 0.20)),
                    ProductCompositionRemoveButton(id),
                )).with_children(|button| {
                    button.spawn(Text::new("-"));
                });
            });
        }
    });
}

pub fn update_product_list(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<ProductPageState>,
    mut list_texts: Query<&mut Text, With<ProductListText>>,
    containers: Query<Entity, With<ProductListContainer>>,
    old_items: Query<Entity, With<ProductListItem>>,
    mut rendered: Local<Option<(Entity, Vec<(i32, String, Option<String>)>, String, Option<i32>)>>,
) {
    let query = state.search_query.trim().to_lowercase();
    let filtered: Vec<_> = state.products.iter()
        .filter(|product| product.name.to_lowercase().contains(&query))
        .collect();
    let status = if let Some(error) = &state.error {
        format!("Could not load products: {error}")
    } else if state.loading {
        "Loading products...".to_owned()
    } else if state.products.is_empty() {
        "No products yet. Select Add Product to create one.".to_owned()
    } else {
        format!("{} of {} products", filtered.len(), state.products.len())
    };
    for mut text in &mut list_texts {
        text.0 = status.clone();
    }
    let Ok(container) = containers.single() else {
        return;
    };
    let signature: Vec<_> = filtered.iter()
        .map(|product| (product.id, product.name.clone(), product.image_path.clone()))
        .collect();
    if rendered.as_ref() == Some(&(container, signature.clone(), query.clone(), state.expanded_preview_id)) {
        return;
    }
    for entity in &old_items {
        commands.entity(entity).despawn();
    }
    commands.entity(container).with_children(|list| {
        for product in &filtered {
            list.spawn((
                Node {
                    width: percent(100),
                    min_height: px(48),
                    padding: UiRect::all(px(8)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(8),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.17, 0.20)),
                ProductListItem,
            )).with_children(|row| {
                row.spawn((
                    Text::new(product.name.clone()),
                    TextFont { font_size: 16.0, ..default() },
                    Node { flex_grow: 1.0, ..default() },
                ));
                row.spawn((
                    Button,
                    Node {
                        width: px(36),
                        height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.30, 0.30, 0.30)),
                    ProductPreviewButton(product.id),
                )).with_children(|button| {
                    button.spawn(Text::new("V"));
                });
                row.spawn((
                    Button,
                    Node {
                        width: px(36),
                        height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.22, 0.30, 0.42)),
                    ProductEditButton(product.id),
                )).with_children(|button| {
                    button.spawn(Text::new("E"));
                });
                row.spawn((
                    Button,
                    Node {
                        width: px(36),
                        height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.48, 0.20, 0.20)),
                    ProductDeleteButton(product.id),
                )).with_children(|button| {
                    button.spawn(Text::new("-"));
                });
            });
            if state.expanded_preview_id == Some(product.id) {
                list.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(220),
                        padding: UiRect::all(px(12)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.11, 0.13, 0.16)),
                    ProductListItem,
                )).with_children(|preview| {
                    if let Some(path) = &product.image_path {
                        let image_handle = asset_server.load::<Image>(path.clone());
                        preview.spawn((
                            ImageNode::new(image_handle.clone()),
                            ProductPreviewImage(image_handle),
                            Node { width: px(240), height: px(220), ..default() },
                        ));
                    } else {
                        preview.spawn(Text::new("No product image available."));
                    }
                });
            }
        }
    });
    *rendered = Some((container, signature, query, state.expanded_preview_id));
}

pub fn spawn_product_navigation(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            padding: UiRect::all(px(20)),
            flex_direction: FlexDirection::Column,
            row_gap: px(16),
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.10, 0.12)),
        Name::new("Product Page"),
        ProductNavigationRoot,
    )).with_children(|page| {
        page.spawn(Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }).with_children(|header| {
            header.spawn((Text::new("Products"), TextFont { font_size: 28.0, ..default() }));
            header.spawn((
                Button,
                ProductDashboardButton,
                Node { padding: UiRect::axes(px(12), px(8)), ..default() },
                BackgroundColor(Color::srgb(0.20, 0.24, 0.28)),
            )).with_children(|button| {
                button.spawn(Text::new("Dashboard"));
            });
        });
        page.spawn(Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            column_gap: px(12),
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|header| {
            header.spawn((
                Button,
                Node {
                    flex_grow: 1.0,
                    min_width: px(100),
                    height: px(42),
                    padding: UiRect::all(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                ProductSearchButton,
            )).with_children(|button| {
                button.spawn((Text::new("Search products..."), ProductSearchText));
            });
            header.spawn((
                Button,
                Node {
                    width: px(42),
                    height: px(42),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                ProductAddButton,
            )).with_children(|button| {
                button.spawn(Text::new("+"));
            });
        });
        page.spawn((Text::new("Loading products..."), ProductListText));
        page.spawn((
            Node {
                width: percent(100),
                flex_grow: 1.0,
                min_height: px(0),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ProductListContainer,
        ));
        page.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ProductFormOverlay,
        )).with_children(|overlay| {
            overlay.spawn((
                Node {
                    width: percent(75),
                    max_width: px(650),
                    max_height: percent(85),
                    padding: UiRect::all(px(20)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.14, 0.17)),
                ProductFormRoot,
            )).with_children(|form| {
                form.spawn(Text::new("Product Editor"));
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductNameButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Name: "), ProductNameText));
                });
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductRouteButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Route: "), ProductRouteText));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(4),
                        display: Display::None,
                        ..default()
                    },
                    ProductRouteOptionList,
                )).with_children(|routes| {
                    for route in base::product::PRODUCT_ROUTES {
                        routes.spawn((
                            Button,
                            Node { width: percent(100), padding: UiRect::all(px(8)), ..default() },
                            BackgroundColor(Color::srgb(0.20, 0.22, 0.25)),
                            ProductRouteOptionButton((*route).to_string()),
                        )).with_children(|button| {
                            button.spawn(Text::new(*route));
                        });
                    }
                });
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductBrandButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Brand: "), ProductBrandText));
                });
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductServingSizeButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Serving size: "), ProductServingSizeText));
                });
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductServingUnitButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Serving unit: "), ProductServingUnitText));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(4),
                        display: Display::None,
                        ..default()
                    },
                    ProductServingUnitOptionList,
                )).with_children(|units| {
                    for unit in ["mg", "mL", "unit"] {
                        units.spawn((
                            Button,
                            Node { width: percent(100), padding: UiRect::all(px(8)), ..default() },
                            BackgroundColor(Color::srgb(0.20, 0.22, 0.25)),
                            ProductServingUnitOptionButton(unit.to_string()),
                        )).with_children(|button| {
                            button.spawn(Text::new(unit));
                        });
                    }
                });
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductServingDescriptionButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Serving description: "), ProductServingDescriptionText));
                });
                form.spawn((
                    Button,
                    Node { width: percent(100), min_height: px(40), padding: UiRect::all(px(8)), ..default() },
                    BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                    ProductDescriptionButton,
                )).with_children(|button| {
                    button.spawn((Text::new("Description: "), ProductDescriptionText));
                });
                form.spawn(Text::new("Product image:"));
                form.spawn((
                    Button,
                    Node { width: px(180), height: px(38), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                    ProductImageUploadButton,
                )).with_children(|button| {
                    button.spawn(Text::new("Upload Image"));
                });
                form.spawn((
                    Button,
                    Node { width: px(180), height: px(38), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    BackgroundColor(Color::srgb(0.45, 0.18, 0.18)),
                    ProductImageRemoveButton,
                )).with_children(|button| {
                    button.spawn(Text::new("Remove Image"));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        height: px(180),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.08, 0.09, 0.11)),
                    ProductImagePreview,
                ));
                form.spawn(Text::new("Composition:"));
                form.spawn((
                    Node {
                        width: percent(100),
                        max_height: px(160),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    ProductCompoundListContainer,
                ));
                form.spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        ..default()
                    },
                    ProductCompositionListContainer,
                ));
                form.spawn((Text::new(""), ProductErrorText));
                form.spawn((
                    Button,
                    Node { width: px(120), height: px(40), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                    ProductSaveButton,
                )).with_children(|button| {
                    button.spawn((
                        Text::new("Save"),
                        ProductSaveButtonText
                    ));
                });
                form.spawn((
                    Button,
                    Node { width: px(120), height: px(40), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                    BackgroundColor(Color::srgb(0.32, 0.20, 0.20)),
                    ProductCancelButton,
                )).with_children(|button| {
                    button.spawn(Text::new("Cancel"));
                });
            });
        });
        page.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            ProductDeleteDialog,
        )).with_children(|dialog| {
            dialog.spawn((
                Node {
                    width: px(360),
                    padding: UiRect::all(px(20)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.17, 0.20)),
            )).with_children(|panel| {
                panel.spawn(Text::new("Delete product?"));
                panel.spawn((Text::new(""), ProductDeleteNameText));
                panel.spawn((Text::new(""), ProductDeleteErrorText));
                panel.spawn((Button, ProductDeleteConfirmButton)).with_children(|button| {
                    button.spawn(Text::new("Confirm Delete"));
                });
                panel.spawn((Button, ProductDeleteCancelButton)).with_children(|button| {
                    button.spawn(Text::new("Cancel"));
                });
            });
        });
    });
}

pub fn update_product_route_options(
    state: Res<ProductPageState>,
    mut route_lists: Query<&mut Node, With<ProductRouteOptionList>>,
) {
    for mut node in &mut route_lists {
        node.display = if state.route_selector_open { Display::Flex } else { Display::None };
    }
}

pub fn update_product_serving_unit_options(
    state: Res<ProductPageState>,
    mut serving_unit_lists: Query<&mut Node, With<ProductServingUnitOptionList>>,
) {
    for mut node in &mut serving_unit_lists {
        node.display = if state.serving_unit_selector_open { Display::Flex } else { Display::None };
    }
}

pub fn cycle_product_compound_units(
    mut buttons: Query<(&Interaction, &ProductCompositionUnitButton), Changed<Interaction>>,
    mut state: ResMut<ProductPageState>,
) {
    for (interaction, button) in &mut buttons {
        if *interaction == Interaction::Pressed {
            if let Some(compound) = state.selected_compounds.iter_mut().find(|compound| compound.compound_id == button.0) {
                compound.compound_units = match compound.compound_units.as_str() {
                    "mg" => "mL".to_string(),
                    _ => "mg".to_string(),
                };
            }
        }
    }
}

pub fn remove_product_compound(
    mut buttons: Query<(&Interaction, &ProductCompositionRemoveButton), Changed<Interaction>>,
    mut state: ResMut<ProductPageState>,
) {
    for (interaction, button) in &mut buttons {
        if *interaction == Interaction::Pressed {
            state.selected_compounds.retain(|compound| compound.compound_id != button.0);
        }
    }
}

pub fn save_product(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ProductSaveButton>)>,
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductSaveResults>,
) {
    if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) || state.loading {
        return;
    }
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
    let editing_id = state.editing_id;
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
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
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
}

pub fn apply_product_save_results(
    mut form_overlay: Query<&mut Node, With<ProductFormOverlay>>,
    mut results: Res<ProductSaveResults>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    let Ok(result) = receiver.try_recv() else {
        return;
    };
    state.loading = false;
    match result {
        Ok(products) => {
            state.products = products;
            state.brand.clear();
            state.description.clear();
            state.name.clear();
            state.route.clear();
            state.selected_compounds.clear();
            state.editing_id = None;
            state.focused_field = None;
            state.route_selector_open = false;
            state.error = None;
            if let Ok(mut node) = form_overlay.single_mut() {
                node.display = Display::None;
            }
        }
        Err(error) => {
            state.error = Some(error);
        }
    }
}

pub fn update_product_save_ui(
    mut buttons: Query<(&Children, &mut BackgroundColor), With<ProductSaveButton>>,
    mut button_texts: Query<&mut Text, (With<ProductSaveButtonText>, Without<ProductErrorText>)>,
    mut error_texts: Query<&mut Text, (With<ProductErrorText>, Without<ProductSaveButtonText>)>,
    state: Res<ProductPageState>,
) {
    for (children, mut background) in &mut buttons {
        *background = BackgroundColor(if state.loading {
            Color::srgb(0.25, 0.27, 0.29)
        } else {
            Color::srgb(0.16, 0.38, 0.28)
        });
        for child in children.iter() {
            if let Ok(mut text) = button_texts.get_mut(child) {
                text.0 = if state.loading { "Saving...".to_string() } else { "Save".to_string() };
            }
        }
    }
    for mut text in &mut error_texts {
        text.0 = state.error.clone().unwrap_or_default();
    }
}

pub fn detect_product_form_open(
    mut was_open: Local<bool>,
    form_overlay: Query<&Node, With<ProductFormOverlay>>,
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductCompoundLoadResults>,
) {
    let Ok(overlay) = form_overlay.single() else {
        return;
    };
    let is_open = overlay.display != Display::None;
    if is_open && !*was_open && !state.compounds_loading {
        state.compounds_loaded = false;
        begin_product_compound_load(database, state, results);
    }
    *was_open = is_open;
}

pub fn toggle_product_preview(
    mut buttons: Query<(&Interaction, &ProductPreviewButton), Changed<Interaction>>,
    mut state: ResMut<ProductPageState>,
) {
    for (interaction, button) in &mut buttons {
        if *interaction == Interaction::Pressed {
            state.expanded_preview_id = if state.expanded_preview_id == Some(button.0) { None } else { Some(button.0) };
        }
    }
}

pub fn toggle_product_composition(
    mut interactions: Query<
        (&Interaction, &ProductCompoundToggleButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<ProductPageState>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let id = button.0;
        if state.selected_compounds.iter().any(|compound| compound.compound_id == id) {
            state.selected_compounds.retain(|compound| compound.compound_id != id);
        } else if let Some(compound_name) = state.compounds.iter().find(|compound| compound.id == id).map(|compound| compound.common_name.clone()){
            state.selected_compounds.push(ProductCompositionEntry {
                compound_id: id,
                compound_name,
                compound_amount: String::new(),
                compound_units: "mg".to_string(),
            });
        }
        state.error = None;
    }
}

pub fn update_product_form_text(
    state: Res<ProductPageState>,
    time: Res<Time>,
    mut brand_text: Query<&mut Text, (With<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>)>,
    mut description_text: Query<&mut Text, (With<ProductDescriptionText>, Without<ProductBrandText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>)>,
    mut error_text: Query<&mut Text, (With<ProductErrorText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductCompositionAmountText>)>,
    mut name_text: Query<&mut Text, (With<ProductNameText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>)>,
    mut route_text: Query<&mut Text, (With<ProductRouteText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>)>,
    mut search_text: Query<&mut Text, (With<ProductSearchText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>)>,
    mut composition_amount_text: Query<(&mut Text, &ProductCompositionAmountText)>,
    mut serving_description_text: Query<&mut Text, (With<ProductServingDescriptionText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>)>,
    mut serving_size_text: Query<&mut Text, (With<ProductServingSizeText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>, Without<ProductServingDescriptionText>, Without<ProductServingUnitText>)>,
    mut serving_unit_text: Query<&mut Text, (With<ProductServingUnitText>, Without<ProductBrandText>, Without<ProductDescriptionText>, Without<ProductNameText>, Without<ProductRouteText>, Without<ProductSearchText>, Without<ProductErrorText>, Without<ProductCompositionAmountText>, Without<ProductServingDescriptionText>, Without<ProductServingSizeText>)>,
) {
    let blink_on = (time.elapsed_secs_f64() - state.caret_started_at) % 1.0 < 0.5;
    for mut text in &mut brand_text {
        let active = state.focused_field == Some(ProductFormField::Brand);
        let value = if state.brand.is_empty() && !active { "".to_owned() } else { state.brand.clone() };
        **text = format!("Brand: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut description_text {
        let active = state.focused_field == Some(ProductFormField::Description);
        let value = if state.description.is_empty() && !active { "".to_owned() } else { state.description.clone() };
        **text = format!("Description: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut serving_description_text {
        let active = state.focused_field == Some(ProductFormField::ServingDescription);
        let value = if state.serving_description.is_empty() && !active { "".to_owned() } else { state.serving_description.clone() };
        **text = format!("Serving description: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut serving_size_text {
        let active = state.focused_field == Some(ProductFormField::ServingSize);
        let value = if state.serving_size.is_empty() && !active { "".to_owned() } else { state.serving_size.clone() };
        **text = format!("Serving size: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut serving_unit_text {
        **text = format!("Serving unit: {}", state.serving_unit);
    }
    for mut text in &mut name_text {
        let active = state.focused_field == Some(ProductFormField::Name);
        let value = if state.name.is_empty() && !active { "".to_owned() } else { state.name.clone() };
        **text = format!("Name: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut route_text {
        **text = if state.route.is_empty() { "Route: Select route".to_string() } else { format!("Route: {}", state.route) };
    }
    for mut text in &mut search_text {
        let active = state.focused_field == Some(ProductFormField::Search);
        let value = if state.search_query.is_empty() && !active { "Search compounds...".to_owned() } else { state.search_query.clone() };
        **text = format!("{}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut error_text {
        **text = state.error.clone().unwrap_or_default();
    }
    for (mut text, amount_text) in &mut composition_amount_text {
        let active = state.focused_field == Some(ProductFormField::CompoundAmount(amount_text.0));
        let value = state.selected_compounds.iter().find(|compound| compound.compound_id == amount_text.0).map(|compound| compound.compound_amount.as_str()).unwrap_or("");
        **text = format!("{}{}", value, if active && blink_on { "|" } else { "" });
    }
}

pub fn handle_product_dashboard_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ProductDashboardButton>)>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    if buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        next_page.set(AppPage::Dashboard);
    }
}

pub fn begin_product_image_upload(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ProductImageUploadButton>)>,
    mut receiver: ResMut<ProductImageUploadReceiver>,
    mut state: ResMut<ProductPageState>,
) {
    if state.image_loading || !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }
    let (sender, channel_receiver) = std::sync::mpsc::channel();
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

pub fn poll_product_image_upload(
    mut receiver: ResMut<ProductImageUploadReceiver>,
    mut state: ResMut<ProductPageState>,
) {
    let result = {
        let mut slot = receiver.0.lock().unwrap();
        let Some(channel_receiver) = slot.as_ref() else {
            return;
        };
        match channel_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Image upload operation failed.".to_owned()))
            }
        }
    };
    if let Some(result) = result {
        state.image_loading = false;
        match result {
            Ok(Some(path)) => {
                state.image_path = Some(path);
                state.error = None;
            }
            Ok(None) => {}
            Err(error) => {
                state.error = Some(error);
            }
        }
    }
}

pub fn remove_product_image(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ProductImageRemoveButton>)>,
    mut state: ResMut<ProductPageState>,
) {
    if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }
    state.image_path = None;
    state.error = None;
}

pub fn update_product_image_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<ProductPageState>,
    previews: Query<(Entity, Option<&ProductImagePreviewHandle>), With<ProductImagePreview>>,
) {
    let Ok((entity, current_handle)) = previews.single() else {
        return;
    };
    let desired_path = state.image_path.as_deref();
    let current_path = current_handle.and_then(|component| component.0.path().map(|path| path.to_string()));
    if current_path.as_deref() == desired_path {
        return;
    }
    if let Some(path) = desired_path {
        let image = asset_server.load::<Image>(path.to_owned());
        commands.entity(entity).insert((
            ImageNode::new(image.clone()),
            ProductImagePreviewHandle(image),
        ));
    } else {
        commands.entity(entity).remove::<(ImageNode, ProductImagePreviewHandle)>();
    }
}

pub fn update_product_preview_image_sizes(
    images: Res<Assets<Image>>,
    mut previews: Query<(&ProductPreviewImage, &mut Node)>,
) {
    for (preview, mut node) in &mut previews {
        let Some(image) = images.get(&preview.0) else {
            continue;
        };
        let size = image.size();
        let scale = (240.0 / size.x as f32).min(220.0 / size.y as f32).min(1.0);
        node.width = px(size.x as f32 * scale);
        node.height = px(size.y as f32 * scale);
    }
}

pub fn handle_product_delete_buttons(
    mut cancel_buttons: Query<&Interaction, (Changed<Interaction>, With<ProductDeleteCancelButton>)>,
    mut delete_buttons: Query<(&Interaction, &ProductDeleteButton), Changed<Interaction>>,
    mut dialog: Query<&mut Node, With<ProductDeleteDialog>>,
    mut state: ResMut<ProductPageState>,
) {
    if cancel_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.deleting_id = None;
        state.deleting_name = None;
        state.delete_error = None;
        if let Ok(mut node) = dialog.single_mut() {
            node.display = Display::None;
        }
    }
    for (interaction, button) in &mut delete_buttons {
        if *interaction == Interaction::Pressed {
            if state.deleting {
                continue;
            }
            let product = state.products.iter().find(|product| product.id == button.0).map(|product| (product.id, product.name.clone()));
            if let Some((id, name)) = product {
                state.deleting_id = Some(id);
                state.deleting_name = Some(name);
                state.delete_error = None;
                if let Ok(mut node) = dialog.single_mut() {
                    node.display = Display::Flex;
                }
            }
        }
    }
}

pub fn begin_product_delete(
    mut buttons: Query<&Interaction, (Changed<Interaction>, With<ProductDeleteConfirmButton>)>,
    database: Res<Database>,
    mut results: ResMut<ProductDeleteResults>,
    mut state: ResMut<ProductPageState>,
) {
    if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) || state.deleting {
        return;
    }
    let Some(id) = state.deleting_id else {
        return;
    };
    let pool = database.0.clone();
    let (sender, receiver) = std::sync::mpsc::channel();
    *results.0.lock().unwrap() = Some(receiver);
    state.deleting = true;
    state.delete_error = None;
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let result = match runtime {
            Ok(runtime) => runtime.block_on(delete_product(&pool, id)).map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((id, result));
    });
}

pub fn poll_product_delete_results(
    mut dialog: Query<&mut Node, With<ProductDeleteDialog>>,
    mut results: ResMut<ProductDeleteResults>,
    mut state: ResMut<ProductPageState>,
) {
    let result = {
        let mut receiver = results.0.lock().unwrap();
        receiver.as_ref().and_then(|receiver| receiver.try_recv().ok())
    };
    let Some((id, result)) = result else {
        return;
    };
    *results.0.lock().unwrap() = None;
    state.deleting = false;
    match result {
        Ok(()) => {
            state.products.retain(|product| product.id != id);
            state.expanded_preview_id = if state.expanded_preview_id == Some(id) { None } else { state.expanded_preview_id };
            state.deleting_id = None;
            state.deleting_name = None;
            state.delete_error = None;
            if let Ok(mut node) = dialog.single_mut() {
                node.display = Display::None;
            }
        }
        Err(error) => {
            state.delete_error = Some(error);
        }
    }
}

pub fn update_product_delete_dialog_text(
    state: Res<ProductPageState>,
    mut error_text: Query<&mut Text, (With<ProductDeleteErrorText>, Without<ProductDeleteNameText>)>,
    mut name_text: Query<&mut Text, (With<ProductDeleteNameText>, Without<ProductDeleteErrorText>)>,
) {
    for mut text in &mut error_text {
        text.0 = state.delete_error.clone().unwrap_or_default();
    }
    for mut text in &mut name_text {
        text.0 = state.deleting_name.as_ref().map(|name| format!("Are you sure you want to delete {}?", name)).unwrap_or_default();
    }
}

pub fn cleanup_product_navigation(
    mut commands: Commands,
    roots: Query<Entity, With<ProductNavigationRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}
