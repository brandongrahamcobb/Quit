/* app/src/system/dashboard.rs
 * The purpose of this module is to provide a dashboard system.
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

use crate::component::login::LoginScreen;
use crate::resource::custom_resource::AuthenticatedAccount;

pub fn despawn_login(mut commands: Commands, screens: Query<Entity, With<LoginScreen>>) {
    for screen in &screens {
        commands.entity(screen).despawn();
    }
}

pub fn setup_dashboard(mut commands: Commands, account: Res<AuthenticatedAccount>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.055, 0.075, 0.09)),
        children![(
            Node {
                padding: UiRect::all(px(36)),
                flex_direction: FlexDirection::Column,
                row_gap: px(12),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
            children![
                (
                    Text::new(format!("Welcome, {}.", account.username)),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.96, 0.93)),
                ),
                (
                    Text::new("Your quit journey starts here."),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.68, 0.74, 0.73)),
                ),
            ],
        )],
    ));
}
