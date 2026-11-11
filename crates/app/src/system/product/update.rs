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


use bevy::input_focus::tab_navigation::TabIndex;
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy::ui_widgets::{Button, TextInput};
use crate::component::product::{ProductBrandText, ProductCompositionAmountText, ProductCompositionItem, ProductCompositionListContainer, ProductCompositionRemoveButton, ProductCompositionSearchText, ProductCompositionUnitButton, ProductCompoundChoiceText, ProductCompoundListContainer, ProductCompoundToggleButton, ProductDeleteButton, ProductDeleteDialog, ProductDeleteErrorText, ProductDeleteNameText, ProductDescriptionText, ProductEditButton, ProductErrorText, ProductFormOverlay, ProductImagePreview, ProductImagePreviewHandle, ProductListContainer, ProductListItem, ProductListText, ProductNameText, ProductPreviewButton, ProductPreviewContent, ProductPreviewImage, ProductPreviewOverlay, ProductPreviewPanel, ProductRouteOptionList, ProductSaveButton, ProductSaveButtonText, ProductSearchText, ProductServingDescriptionText, ProductServingSizeText, ProductServingUnitOptionList};
use crate::resource::product_resource::{ProductDeletionState, ProductFormState, ProductOverlay, ProductPageState};
use crate::system::product::activate::add_edit::{cycle_product_compound_units, edit_product, remove_product_compound, toggle_product_composition};
use crate::system::product::activate::delete::delete_product;
use crate::system::product::activate::preview::view_product_preview;

pub fn update_product_compound_buttons(
    mut commands: Commands,
    page_state: Res<ProductPageState>,
    form_state: Res<ProductFormState>,
    list_container: Query<Entity, With<ProductCompoundListContainer>>,
    old_items: Query<Entity, With<ProductCompoundChoiceText>>,
    mut previous_signature: Local<Option<Vec<(i32, String, bool)>>>,
) {
    let Ok(container) = list_container.single() else {
        return;
    };
    let query = form_state.composition_search_query.to_lowercase();
    let choices: Vec<(i32, String, bool)> = page_state.compounds.iter()
        .filter(|compound| compound.common_name.to_lowercase().contains(&query))
        .map(|compound| (
            compound.id,
            compound.common_name.clone(),
            form_state.selected_compounds.iter().any(|selected| selected.compound_id == compound.id),
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
                    TextFont { font_size: FontSize::Px(16.0), ..default() },
                    TextColor(Color::WHITE),
                ));
            }).observe(toggle_product_composition);
        }
    });
}

pub fn update_product_composition_rows(
    mut commands: Commands,
    form_state: Res<ProductFormState>,
    list_container: Query<Entity, With<ProductCompositionListContainer>>,
    old_items: Query<Entity, With<ProductCompositionItem>>,
    mut previous_signature: Local<Option<Vec<(i32, String, String)>>>
) {
    let Ok(container) = list_container.single() else {
        return;
    };
    let signature: Vec<_> = form_state.selected_compounds.iter()
        .map(|compound| (
            compound.compound_id,
            compound.compound_name.clone(),
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
        for (id, name, units) in signature {
            let amount = form_state.selected_compounds.iter()
                .find(|compound| compound.compound_id == id)
                .map(|compound| compound.compound_amount.clone())
                .unwrap_or_default();
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: px(42), 
                    padding: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                ProductCompositionItem,
            )).with_children(|row| {
                row.spawn((
                    Text::new(name),
                    TextFont { font_size: FontSize::Px(15.0), ..default() },
                    Node { flex_grow: 1.0, ..default() },
                ));
                row.spawn((
                    Node {
                        width: Val::Px(90.0),
                        height: px(36), 
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                    TextInput,
                    EditableText::new(&amount),
                    TextLayout::no_wrap(),
                    TextCursorStyle::default(),
                    TextFont::from_font_size(FontSize::Px(15.0)),
                    TextColor(Color::WHITE),
                    TabIndex(8),
                    ProductCompositionAmountText(id),
                ));
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
                }).observe(cycle_product_compound_units);
                row.spawn((
                    Button,
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(36.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.32, 0.20, 0.20)),
                    ProductCompositionRemoveButton(id),
                )).with_children(|button| {
                    button.spawn(Text::new("-"));
                }).observe(remove_product_compound);
            });
        }
    });
}

pub fn update_product_list(
    mut commands: Commands,
    page_state: Res<ProductPageState>,
    mut list_texts: Query<&mut Text, With<ProductListText>>,
    containers: Query<Entity, With<ProductListContainer>>,
    old_items: Query<Entity, With<ProductListItem>>,
    mut rendered: Local<Option<(Entity, Vec<(i32, String, Option<String>)>, String)>>,
) {
    let query = page_state.search_query.trim().to_lowercase();
    let filtered: Vec<_> = page_state.products.iter()
        .filter(|product| product.name.to_lowercase().contains(&query))
        .collect();
    let status = if let Some(error) = &page_state.error {
        format!("Could not load products: {error}")
    } else if page_state.loading {
        "Loading products...".to_owned()
    } else if page_state.products.is_empty() {
        "No products yet. Select Add Product to create one.".to_owned()
    } else {
        format!("{} of {} products", filtered.len(), page_state.products.len())
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
    if rendered.as_ref() == Some(&(container, signature.clone(), query.clone())) {
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
                    TextFont { font_size: FontSize::Px(16.0), ..default() },
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
                }).observe(view_product_preview);
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
                }).observe(edit_product);
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
                }).observe(delete_product);
            });
        }
    });
    *rendered = Some((container, signature, query));
}

pub fn update_product_dropdowns(
    page_state: Res<ProductPageState>,
    form_state: Res<ProductFormState>,
    mut dropdowns: ParamSet<(
        Query<&mut Node, With<ProductRouteOptionList>>,
        Query<&mut Node, With<ProductServingUnitOptionList>>,
        Query<&mut Node, With<ProductCompoundListContainer>>,
    )>,
) {
    for mut node in &mut dropdowns.p0() {
        node.display = if form_state.route_selector_open { Display::Flex } else { Display::None };
    }
    for mut node in &mut dropdowns.p1() {
        node.display = if form_state.serving_unit_selector_open { Display::Flex } else { Display::None };
    }
    let show = form_state.composition_search_query_open && matches!(page_state.overlay, Some(ProductOverlay::Form { .. }))
        && !form_state.composition_search_query.is_empty();
    for mut node in &mut dropdowns.p2() {
        node.display = if show { Display::Flex } else { Display::None };
    }
}

pub fn update_product_save_ui(
    mut buttons: Query<(&Children, &mut BackgroundColor), With<ProductSaveButton>>,
    mut button_texts: Query<&mut Text, (With<ProductSaveButtonText>, Without<ProductErrorText>)>,
    mut error_texts: Query<&mut Text, (With<ProductErrorText>, Without<ProductSaveButtonText>)>,
    page_state: Res<ProductPageState>,
) {
    for (children, mut background) in &mut buttons {
        *background = BackgroundColor(if page_state.loading {
            Color::srgb(0.25, 0.27, 0.29)
        } else {
            Color::srgb(0.16, 0.38, 0.28)
        });
        for child in children.iter() {
            if let Ok(mut text) = button_texts.get_mut(child) {
                text.0 = if page_state.loading { "Saving...".to_string() } else { "Save".to_string() };
            }
        }
    }
    for mut text in &mut error_texts {
        text.0 = page_state.error.clone().unwrap_or_default();
    }
}

pub fn update_product_image_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    form_state: Res<ProductFormState>,
    previews: Query<(Entity, Option<&ProductImagePreviewHandle>), With<ProductImagePreview>>,
) {
    let Ok((entity, current_handle)) = previews.single() else {
        return;
    };
    let desired_path = form_state.image_path.as_deref();
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

pub fn update_product_delete_dialog_text(
    deletion_state: Res<ProductDeletionState>,
    mut error_text: Query<&mut Text, (With<ProductDeleteErrorText>, Without<ProductDeleteNameText>)>,
    mut name_text: Query<&mut Text, (With<ProductDeleteNameText>, Without<ProductDeleteErrorText>)>,
) {
    for mut text in &mut error_text {
        text.0 = deletion_state.error.clone().unwrap_or_default();
    }
    for mut text in &mut name_text {
        text.0 = deletion_state.name.as_ref().map(|name| format!("Are you sure you want to delete {}?", name)).unwrap_or_default();
    }
}

pub fn update_product_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    panels: Query<Entity, With<ProductPreviewPanel>>,
    children: Query<Entity, With<ProductPreviewContent>>,
    page_state: Res<ProductPageState>,
    mut rendered: Local<Option<(i32, Option<String>)>>,
) {
    let Some(ProductOverlay::Preview(id)) = page_state.overlay else {
        if rendered.is_some() {
            for entity in &children {
                commands.entity(entity).despawn();
            }
            *rendered = None;
        }
        return;
    };
    let Some(product) = page_state.products.iter().find(|product| product.id == id) else {
        return;
    };
    let signature = (id, product.image_path.clone());
    if rendered.as_ref() == Some(&signature) {
        return;
    }
    for entity in &children {
        commands.entity(entity).despawn();
    }
    let Ok(panel) = panels.single() else {
        return;
    };
    commands.entity(panel).with_children(|panel| {
        panel.spawn((
            Node {
                width: percent(100),
                min_height: px(220),
                padding: UiRect::all(px(12)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ProductPreviewContent,
        )).with_children(|preview| {
            if let Some(path) = &product.image_path {
                let image_handle = asset_server.load::<Image>(path.clone());
                preview.spawn((
                    ImageNode::new(image_handle),
                    Node {
                        width: px(240),
                        height: px(220),
                        ..default()
                    },
                ));
            } else {
                preview.spawn(Text::new("No product image available."));
            }
        });
    });
    *rendered = Some(signature);
}

pub fn update_product_fields(
    brand_inputs: Query<&EditableText, With<ProductBrandText>>,
    description_inputs: Query<&EditableText, With<ProductDescriptionText>>,
    name_inputs: Query<&EditableText, With<ProductNameText>>,
    search_inputs: Query<&EditableText, With<ProductSearchText>>,
    serving_description_inputs: Query<&EditableText, With<ProductServingDescriptionText>>,
    serving_size_inputs: Query<&EditableText, With<ProductServingSizeText>>,
    composition_amount_inputs: Query<(&EditableText, &ProductCompositionAmountText)>,
    composition_search_inputs: Query<&EditableText, With<ProductCompositionSearchText>>,
    mut page_state: ResMut<ProductPageState>,
    mut form_state: ResMut<ProductFormState>,
) {
    for input in &search_inputs {
        page_state.search_query = input.value().to_string();
    }
    for input in &composition_search_inputs {
        let value = input.value().to_string();
        if value != form_state.composition_search_query {
            form_state.composition_search_query_open = !value.is_empty();
            form_state.composition_search_query = value;
        }
    }
    if !matches!(page_state.overlay, Some(ProductOverlay::Form { .. })) {
        return;
    }
    for input in &brand_inputs {
        form_state.brand = input.value().to_string();
    }
    for input in &description_inputs {
        form_state.description = input.value().to_string();
    }
    for input in &name_inputs {
        form_state.name = input.value().to_string();
    }
    for input in &serving_description_inputs {
        form_state.serving_description = input.value().to_string();
    }
    for input in &serving_size_inputs {
        form_state.serving_size = input.value().to_string();
    }
    for (input, amount_text) in &composition_amount_inputs {
        let value = input.value().to_string();
        if let Some(compound) = form_state.selected_compounds.iter_mut().find(|compound| compound.compound_id == amount_text.0) {
            compound.compound_amount = value;
        }
    }
}

pub fn update_product_overlays(
    mut overlays: ParamSet<(
        Query<&mut Node, With<ProductFormOverlay>>,
        Query<&mut Node, With<ProductDeleteDialog>>,
        Query<&mut Node, With<ProductPreviewOverlay>>,
    )>,
    page_state: Res<ProductPageState>,
) {
    let show_form = matches!(page_state.overlay, Some(ProductOverlay::Form { .. }));
    let show_delete = matches!(page_state.overlay, Some(ProductOverlay::Delete(_)));
    let show_preview = matches!(page_state.overlay, Some(ProductOverlay::Preview(_)));
    for mut node in &mut overlays.p0() {
        node.display = if show_form { Display::Flex } else { Display::None };
    }
    for mut node in &mut overlays.p1() {
        node.display = if show_delete { Display::Flex } else { Display::None };
    }
    for mut node in &mut overlays.p2() {
        node.display = if show_preview { Display::Flex } else { Display::None };
    }
}




