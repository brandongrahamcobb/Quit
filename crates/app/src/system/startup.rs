use bevy::ui::RelativeCursorPosition;
/* app/src/system/startup.rs
 * The purpose of this module is to provide a startup system.
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
use bevy::{prelude::*, ui::FocusPolicy};

use crate::component::login::{LoginScreen, LoginStatusDisplay, LoginSubmit, PasswordDisplay, PasswordField, UsernameDisplay, UsernameField, WallpaperBackground, WallpaperChoicesList, WallpaperCloseButton, WallpaperModal, WallpaperModeChoice, WallpaperModeDropdown, WallpaperModeMenu, WallpaperOpenButton, WallpaperStretchFill, WallpaperStretchLabel, WallpaperStretchSlider, WallpaperTileXButton, WallpaperTileXKnob, WallpaperTileYButton, WallpaperTileYKnob};
use crate::resource::custom_resource::{WallpaperMode, WallpaperSettings};

pub fn setup_login(mut commands: Commands, asset_server: Res<AssetServer>, wallpaper: Res<WallpaperSettings>) {
    let image_mode = match wallpaper.mode {
        WallpaperMode::Auto => NodeImageMode::Auto,
        WallpaperMode::Stretch => NodeImageMode::Stretch,
        WallpaperMode::Tiled => NodeImageMode::Tiled {
            tile_x: wallpaper.tile_x,
            tile_y: wallpaper.tile_y,
            stretch_value: wallpaper.stretch_value,
        },
    };
    commands.spawn(Camera2d);

    commands.spawn((
        LoginScreen,
        WallpaperBackground,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode::new(asset_server.load(wallpaper.image.clone())).with_mode(image_mode),
        children![(
            Node {
                width: px(440),
                padding: UiRect::all(px(36)),
                flex_direction: FlexDirection::Column,
                row_gap: px(18),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
            children![
                (
                    Text::new("Welcome back"),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.96, 0.93)),
                ),
                (
                    Text::new("Sign in."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.68, 0.74, 0.73)),
                ),
                (
                    Button,
                    UsernameField,
                    Node {
                        width: percent(100),
                        height: px(52),
                        padding: UiRect::horizontal(px(14)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.28, 0.36, 0.37)),
                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                    children![(
                        Text::new("Username"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.94, 0.96, 0.93)),
                        Node {
                            width: percent(100),
                            ..default()
                        },
                        UsernameDisplay
                    )],
                ),
                (
                    Button,
                    PasswordField,
                    Node {
                        width: percent(100),
                        height: px(52),
                        padding: UiRect::horizontal(px(14)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.28, 0.36, 0.37)),
                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                    children![(
                        Text::new("Password"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.94, 0.96, 0.93)),
                        Node {
                            width: percent(100),
                            ..default()
                        },
                        PasswordDisplay,
                    )],
                ),
                (
                    Button,
                    LoginSubmit,
                    Node {
                        width: percent(100),
                        height: px(52),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.22, 0.48, 0.39)),
                    children![(
                        Text::new("Sign in"),
                        TextFont {
                            font_size: 17.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.98, 0.99, 0.97)),
                    )],
                ),
                (
                    Text::new(""),
                    LoginStatusDisplay,
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.57, 0.48)),
                ),
            ],
        )],
    ));
}

pub fn setup_wallpaper_controls(
    mut commands: Commands,
    screens: Query<Entity, With<LoginScreen>>,
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
                Text::new("Wallpaper"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
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
                            Text::new("Wallpaper settings"),
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
                            Text::new("×"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ),
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
                                    Text::new("Repeat horizontally"),
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
                                    Text::new("Repeat vertically"),
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
                                    Text::new("Tiling threshold"),
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
                )],
            ));
        });
    }
}
