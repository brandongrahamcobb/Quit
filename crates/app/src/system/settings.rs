/* app/src/system/wallpaper.rs
 * The purpose of this module is to provide a wallpaper system.
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


use bevy::prelude::*;
use bevy::ui::{FocusPolicy, RelativeCursorPosition};

use crate::component::dashboard::DashboardScreen;
use crate::component::settings::{SettingsPage, SettingsPageButton, SettingsPagePanel, WallpaperChoicesList, WallpaperCloseButton, WallpaperModal, WallpaperModeChoice, WallpaperModeDropdown, WallpaperModeMenu, WallpaperOpenButton, WallpaperStretchFill, WallpaperStretchLabel, WallpaperStretchSlider, WallpaperTileXButton, WallpaperTileXKnob, WallpaperTileYButton, WallpaperTileYKnob};
use crate::resource::settings_resource::WallpaperMode;

pub fn select_settings_page(
    buttons: Query<
        (&Interaction, &SettingsPageButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut panels: Query<(&mut Node, &SettingsPagePanel)>,
) {
    for (interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        for (mut node, panel) in &mut panels {
            node.display = if panel.0 == button.0 {
                Display::Flex
            } else {
                Display::None
            };
        }
    }
}

pub fn setup_settings_controls(
    mut commands: Commands,
    screens: Query<Entity, With<DashboardScreen>>,
) {
    for screen in &screens {
        commands.entity(screen).with_children(|parent| {
            parent.spawn((
                Button,
                WallpaperOpenButton,
                Node {
                    position_type: PositionType::Absolute,
                    left: px(18),
                    bottom: px(18),
                    width: px(120),
                    height: px(44),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
                children![(
                    Text::new("Settings"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                )]
            ));
            parent.spawn((
                WallpaperModal,
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(100),
                    height: percent(100),
                    display: Display::None,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
                children![(
                    Node {
                        width: px(640),
                        max_width: percent(90),
                        min_height: px(360),
                        padding: UiRect::all(px(28)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(20),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
                    children![
                        (
                            Text::new("Settings"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ),
                        (
                            Button,
                            WallpaperCloseButton,
                            Node {
                                position_type: PositionType::Absolute,
                                top: px(12),
                                right: px(12),
                                width: px(36),
                                height: px(36),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.20, 0.23, 0.25)),
                            children![(
                                Text::new("x"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            )]
                        ),
                        (
                            Node {
                                width: percent(100),
                                flex_direction: FlexDirection::Row,
                                column_gap: px(16),
                                flex_grow: 1.0,
                                ..default()
                            },
                            children![
                                (
                                    Node {
                                        width: px(130),
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(8),
                                        ..default()
                                    },
                                    children![
                                        (
                                            Button,
                                            SettingsPageButton(SettingsPage::General),
                                            Node {
                                                width: percent(100),
                                                height: px(42),
                                                align_items: AlignItems::Center,
                                                padding: UiRect::horizontal(px(12)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                                            Text::new("General"),
                                            TextFont::default(),
                                            TextColor(Color::WHITE),
                                        ),
                                        (
                                            Button,
                                            SettingsPageButton(SettingsPage::Wallpaper),
                                            Node {
                                                width: percent(100),
                                                height: px(42),
                                                align_items: AlignItems::Center,
                                                padding: UiRect::horizontal(px(12)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                                            Text::new("Wallpaper"),
                                            TextFont::default(),
                                            TextColor(Color::WHITE),
                                        ),
                                    ],
                                ),
                                (
                                    SettingsPagePanel(SettingsPage::General),
                                    Node {
                                        width: px(0),
                                        flex_grow: 1.0,
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(12),
                                        ..default()
                                    },
                                    children![(
                                        Text::new("More settings coming soon."),
                                        TextFont::default(),
                                        TextColor(Color::WHITE),
                                    )],
                                ),
                                (
                                    SettingsPagePanel(SettingsPage::Wallpaper),
                                    Node {
                                        width: px(0),
                                        flex_grow: 1.0,
                                        display: Display::None,
                                        flex_direction: FlexDirection::Column,
                                        row_gap: px(12),
                                        ..default()
                                    },
                                    children![
                                        (
                                            WallpaperChoicesList,
                                            Node {
                                                width: percent(100),
                                                flex_direction: FlexDirection::Column,
                                                row_gap: px(8),
                                                ..default()
                                            },
                                        ),
                                        (
                                            Text::new("Image mode"),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ),
                                        (
                                            Button,
                                            WallpaperModeDropdown,
                                            Node {
                                                width: px(180),
                                                height: px(42),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                                            Text::new("Tiled ▾"),
                                            TextFont {
                                                font_size: 15.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ),
                                        (
                                            WallpaperModeMenu,
                                            Node {
                                                width: px(180),
                                                display: Display::None,
                                                flex_direction: FlexDirection::Column,
                                                row_gap: px(4),
                                                ..default()
                                            },
                                            children![
                                                (
                                                    Button,
                                                    WallpaperModeChoice(WallpaperMode::Auto),
                                                    Node {
                                                        width: percent(100),
                                                        height: px(38),
                                                        align_items: AlignItems::Center,
                                                        padding: UiRect::horizontal(px(10)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                                                    Text::new("Auto"),
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                                (
                                                    Button,
                                                    WallpaperModeChoice(WallpaperMode::Stretch),
                                                    Node {
                                                        width: percent(100),
                                                        height: px(38),
                                                        align_items: AlignItems::Center,
                                                        padding: UiRect::horizontal(px(10)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                                                    Text::new("Stretch"),
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                                (
                                                    Button,
                                                    WallpaperModeChoice(WallpaperMode::Tiled),
                                                    Node {
                                                        width: percent(100),
                                                        height: px(38),
                                                        align_items: AlignItems::Center,
                                                        padding: UiRect::horizontal(px(10)),
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                                                    Text::new("Tiled"),
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                            ],
                                        ),
                                        (
                                            Node {
                                                width: percent(100),
                                                justify_content: JustifyContent::SpaceBetween,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            children![
                                                (
                                                    Text::new("Tile X"),
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                                (
                                                    Button,
                                                    WallpaperTileXButton,
                                                    Node {
                                                        position_type: PositionType::Relative,
                                                        width: px(54),
                                                        height: px(30),
                                                        border_radius: BorderRadius::MAX,
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.22, 0.48, 0.39)),
                                                    children![(
                                                        WallpaperTileXKnob,
                                                        Node {
                                                            position_type: PositionType::Absolute,
                                                            left: px(27),
                                                            top: px(3),
                                                            width: px(24),
                                                            height: px(24),
                                                            border_radius: BorderRadius::MAX,
                                                            ..default()
                                                        },
                                                        BackgroundColor(Color::WHITE),
                                                        FocusPolicy::Pass,
                                                    )],
                                                ),
                                            ],
                                        ),
                                        (
                                            Node {
                                                width: percent(100),
                                                justify_content: JustifyContent::SpaceBetween,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            children![
                                                (
                                                    Text::new("Tile Y"),
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                                (
                                                    Button,
                                                    WallpaperTileYButton,
                                                    Node {
                                                        position_type: PositionType::Relative,
                                                        width: px(54),
                                                        height: px(30),
                                                        border_radius: BorderRadius::MAX,
                                                        ..default()
                                                    },
                                                    BackgroundColor(Color::srgb(0.22, 0.48, 0.39)),
                                                    children![(
                                                        WallpaperTileYKnob,
                                                        Node {
                                                            position_type: PositionType::Absolute,
                                                            left: px(27),
                                                            top: px(3),
                                                            width: px(24),
                                                            height: px(24),
                                                            border_radius: BorderRadius::MAX,
                                                            ..default()
                                                        },
                                                        BackgroundColor(Color::WHITE),
                                                        FocusPolicy::Pass,
                                                    )],
                                                ),
                                            ],
                                        ),
                                        (
                                            Node {
                                                width: percent(100),
                                                justify_content: JustifyContent::SpaceBetween,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            children![
                                                (
                                                    Text::new("Scale"),
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                                (
                                                    Text::new("0.10"),
                                                    WallpaperStretchLabel,
                                                    TextFont::default(),
                                                    TextColor(Color::WHITE),
                                                ),
                                            ],
                                        ),
                                        (
                                            Button,
                                            WallpaperStretchSlider,
                                            RelativeCursorPosition::default(),
                                            FocusPolicy::Pass,
                                            Node {
                                                position_type: PositionType::Relative,
                                                width: percent(100),
                                                height: px(16),
                                                border_radius: BorderRadius::MAX,
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.28, 0.32, 0.34)),
                                            children![(
                                                WallpaperStretchFill,
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    left: px(0),
                                                    top: px(0),
                                                    bottom: px(0),
                                                    width: percent(10),
                                                    border_radius: BorderRadius::MAX,
                                                    ..default()
                                                },
                                                BackgroundColor(Color::srgb(0.22, 0.48, 0.39)),
                                                FocusPolicy::Pass,
                                            )],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                )],
            ));
        });
    }
}

