/* app/src/system/product/spawn.rs
 * The purpose of this module is to provide a spawner for the product page.
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


use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy::ui_widgets::{Button, TextInput};
use crate::component::page::PageRoot;
use crate::component::product::{ProductAddButton, ProductBrandText, ProductCancelButton, ProductCompositionListContainer, ProductCompositionSearchText, ProductCompoundListContainer, ProductDashboardButton, ProductDeleteCancelButton, ProductDeleteConfirmButton, ProductDeleteDialog, ProductDeleteErrorText, ProductDeleteNameText, ProductDescriptionText, ProductErrorText, ProductFormOverlay, ProductFormRoot, ProductImagePreview, ProductImageRemoveButton, ProductImageUploadButton, ProductListContainer, ProductListText, ProductNameText, ProductNavigationRoot, ProductPreviewOverlay, ProductPreviewPanel, ProductRouteButton, ProductRouteOptionButton, ProductRouteOptionList, ProductRouteText, ProductSaveButton, ProductSaveButtonText, ProductSearchText, ProductServingDescriptionText, ProductServingSizeText, ProductServingUnitButton, ProductServingUnitOptionButton, ProductServingUnitOptionList, ProductServingUnitText};
use crate::system::product::activate::add_edit::{add_product, cancel_save, remove_product_image, save_product, select_product_route, select_product_serving_unit, toggle_product_route_dropdown, toggle_product_serving_unit_dropdown, upload_product_image};
use crate::system::product::activate::delete::{cancel_product_delete, confirm_product_delete};


pub fn spawn_product_navigation(mut commands: Commands) {
    commands.spawn((
        PageRoot,
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
        TabGroup::new(0),
    )).with_children(|page| {
        page.spawn(Node {
            width: percent(100),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }).with_children(|header| {
            header.spawn((Text::new("Products"), TextFont { font_size: FontSize::Px(28.0), ..default() }));
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
            header.spawn(Text::new("Search:"));
            header.spawn((
                Node {
                    flex_grow: 1.0,
                    min_width: px(100),
                    height: px(42),
                    padding: UiRect::all(px(10)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                TextInput,
                EditableText::default(),
                TextLayout::no_wrap(),
                TextCursorStyle::default(),
                TextFont::from_font_size(FontSize::Px(15.0)),
                TextColor(Color::WHITE),
                TabIndex(0),
                ProductSearchText,
            ));
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
            }).observe(add_product);
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
                    min_height: px(0),
                    padding: UiRect::all(px(20)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.14, 0.17)),
                ProductFormRoot,
                TabGroup::modal(),
            )).with_children(|form| {
                form.spawn(Text::new("Product Editor"));
                form.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(40),
                        padding: UiRect::all(px(8)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((
                        Text::new("Name:"),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                    ));
                    field.spawn((
                        Node {
                            flex_grow: 1.0,
                            min_width: px(0),
                            height: px(40),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        TextInput,
                        EditableText::default(),
                        TextLayout::no_wrap(),
                        TextCursorStyle::default(),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                        TabIndex(0),
                        ProductNameText,
                    ));
                });
               form.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(40),
                        padding: UiRect::all(px(8)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((
                        Text::new("Brand:"),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                    ));
                    field.spawn((
                        Node {
                            flex_grow: 1.0,
                            height: px(40),
                            min_width: px(0),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        TextInput,
                        EditableText::default(),
                        TextLayout::no_wrap(),
                        TextCursorStyle::default(),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                        TabIndex(1),
                        ProductBrandText,
                    ));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: px(12),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((Text::new("ROA:"), TextFont::from_font_size(FontSize::Px(15.0))));
                    field.spawn((
                        Node {
                            width: percent(100),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                    )).with_children(|dropdown| {
                        dropdown.spawn((
                            Button,
                            Node {
                                width: percent(100),
                                min_height: px(40),
                                padding: UiRect::all(px(8)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                            TabIndex(2),
                            ProductRouteButton,
                        )).with_children(|button| {
                            button.spawn((Text::new(""), TextFont::from_font_size(FontSize::Px(15.0)), ProductRouteText));
                        }).observe(toggle_product_route_dropdown);
                        dropdown.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                top: percent(100),
                                left: px(0),
                                width: percent(100),
                                max_height: px(180),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(4),
                                overflow: Overflow::scroll_y(),
                                display: Display::None,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                            GlobalZIndex(1),
                            OverrideClip,
                            ProductRouteOptionList,
                        )).with_children(|routes| {
                            for route in base::product::PRODUCT_ROUTES {
                                routes.spawn((
                                    Button,
                                    Node {
                                        width: percent(100),
                                        padding: UiRect::all(px(8)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.20, 0.22, 0.25)),
                                    ProductRouteOptionButton((*route).to_string()),
                                )).with_children(|button| {
                                    button.spawn(Text::new(*route));
                                }).observe(select_product_route);
                            }
                        });
                    });
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(40),
                        padding: UiRect::all(px(8)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((
                        Text::new("Serving Size:"),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                    ));
                    field.spawn((
                        Node {
                            flex_grow: 1.0,
                            height: px(40),
                            min_width: px(0),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        TextInput,
                        EditableText::default(),
                        TextLayout::no_wrap(),
                        TextCursorStyle::default(),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                        TabIndex(3),
                        ProductServingSizeText,
                    ));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: px(12),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((Text::new("Serving unit:"), TextFont::from_font_size(FontSize::Px(15.0))));
                    field.spawn((
                        Node {
                            width: percent(100),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                    )).with_children(|dropdown| {
                        dropdown.spawn((
                            Button,
                            Node {
                                width: percent(100),
                                min_height: px(40),
                                padding: UiRect::all(px(8)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                            TabIndex(4),
                            ProductServingUnitButton,
                        )).with_children(|button| {
                            button.spawn((Text::new(""), TextFont::from_font_size(FontSize::Px(15.0)), ProductServingUnitText));
                        }).observe(toggle_product_serving_unit_dropdown);;
                        dropdown.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                top: percent(100),
                                left: px(0),
                                width: percent(100),
                                max_height: px(180),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(4),
                                overflow: Overflow::scroll_y(),
                                display: Display::None,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                            GlobalZIndex(1),
                            OverrideClip,
                            ProductServingUnitOptionList,
                        )).with_children(|units| {
                            for unit in ["mg", "mL", "unit"] {
                                units.spawn((
                                    Button,
                                    Node {
                                        width: percent(100),
                                        padding: UiRect::all(px(8)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.20, 0.22, 0.25)),
                                    ProductServingUnitOptionButton(unit.to_string()),
                                )).with_children(|button| {
                                    button.spawn(Text::new(unit));
                                }).observe(select_product_serving_unit);
                            }
                        });
                    });
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(40),
                        padding: UiRect::all(px(8)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((
                        Text::new("Serving Description:"),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                    ));
                    field.spawn((
                        Node {
                            flex_grow: 1.0,
                            height: px(40),
                            min_width: px(0),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        TextInput,
                        EditableText::default(),
                        TextLayout::no_wrap(),
                        TextCursorStyle::default(),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                        TabIndex(5),
                        ProductServingDescriptionText,
                    ));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(40),
                        padding: UiRect::all(px(8)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    },
                )).with_children(|field| {
                    field.spawn((
                        Text::new("Product Description:"),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                    ));
                    field.spawn((
                        Node {
                            flex_grow: 1.0,
                            height: px(40),
                            min_width: px(0),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        TextInput,
                        EditableText::default(),
                        TextLayout::no_wrap(),
                        TextCursorStyle::default(),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                        TabIndex(6),
                        ProductDescriptionText,
                    ));
                });
                form.spawn((Text::new("Composition:"), TextFont::from_font_size(FontSize::Px(15.0))));
                form.spawn(Node {
                    width: percent(100),
                    flex_shrink: 0.0,
                    position_type: PositionType::Relative,
                    ..default()
                }).with_children(|wrap| {
                    wrap.spawn((
                        Node { width: percent(100), height: px(42), padding: UiRect::all(px(8)), align_items: AlignItems::Center, ..default() },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        TextInput,
                        EditableText::default(),
                        TextLayout::no_wrap(),
                        TextCursorStyle::default(),
                        TextFont::from_font_size(FontSize::Px(15.0)),
                        TextColor(Color::WHITE),
                        TabIndex(7),
                        ProductCompositionSearchText,
                    ));
                    wrap.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: percent(100),
                            left: px(0),
                            width: percent(100),
                            max_height: px(180),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(4),
                            overflow: Overflow::scroll_y(),
                            display: Display::None,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                        // GlobalZIndex(1),
                        OverrideClip,
                        ProductCompoundListContainer,
                    ));
                });
                form.spawn((
                    Node {
                        width: percent(100),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(6),
                        ..default()
                    },
                    ProductCompositionListContainer,
                ));
                form.spawn((Text::new("Image:"), TextFont::from_font_size(FontSize::Px(15.0))));
                form.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    }
                 )).with_children(|row| {
                    row.spawn((
                        Button,
                        Node { width: px(180), height: px(38), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                        BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                        ProductImageUploadButton,
                    )).with_children(|button| {
                        button.spawn((Text::new("Upload"), TextFont::from_font_size(FontSize::Px(15.0))));
                    }).observe(upload_product_image);
                    row.spawn((
                        Button,
                        Node { width: px(180), height: px(38), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                        BackgroundColor(Color::srgb(0.32, 0.20, 0.20)),
                        ProductImageRemoveButton,
                    )).with_children(|button| {
                        button.spawn((Text::new("Remove"), TextFont::from_font_size(FontSize::Px(15.0))));
                    }).observe(remove_product_image);
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
                form.spawn((Text::new(""), ProductErrorText));
                form.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(8),
                        ..default()
                    }
                 )).with_children(|row| {
                    row.spawn((
                        Button,
                        Node { width: px(120), height: px(40), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                        BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                        ProductSaveButton,
                    )).observe(save_product).with_children(|button| {
                        button.spawn((
                            Text::new("Save"),
                            ProductSaveButtonText
                        ));
                    });
                    row.spawn((
                        Button,
                        Node { width: px(120), height: px(40), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
                        BackgroundColor(Color::srgb(0.32, 0.20, 0.20)),
                        ProductCancelButton,
                    )).observe(cancel_save).with_children(|button| {
                        button.spawn(Text::new("Cancel"));
                    });
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
                    button.spawn(Text::new("Confirm Delete")).observe(confirm_product_delete);
                });
                panel.spawn((Button, ProductDeleteCancelButton)).with_children(|button| {
                    button.spawn(Text::new("Cancel")).observe(cancel_product_delete);
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
            ProductPreviewOverlay,
        )).with_children(|overlay| {
            overlay.spawn((
                Node {
                    width: percent(75),
                    max_width: px(650),
                    max_height: percent(85),
                    padding: UiRect::all(px(20)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(12),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.14, 0.17)),
                ProductPreviewPanel,
            ));
        });
    });
}
