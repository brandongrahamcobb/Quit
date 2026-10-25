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
use bevy::prelude::*;

use crate::component::login::{LoginScreen, LoginStatusDisplay, LoginSubmit, PasswordDisplay, PasswordField, UsernameDisplay, UsernameField};

pub fn setup_login(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        LoginScreen,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode::new(asset_server.load("quit_app_splash.png")).with_mode(NodeImageMode::Auto),
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

