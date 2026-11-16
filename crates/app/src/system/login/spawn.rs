/* app/src/system/login.rs
 * The purpose of this module is to provide a login system.
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
use bevy::ui_widgets::{Button, TextInput, observe};

use crate::component::input::TextInputPlaceholder;
use crate::component::login::{
    LoginPage, LoginStatusDisplay, LoginSubmit, PasswordField, UsernameField
};
use crate::component::page::PageRoot;
use crate::system::login::activate::sign_in::submit_login;


pub fn spawn_login_navigation(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        PageRoot,
        LoginPage,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode::new(asset_server.load("quit_app_splash.png")).with_mode(NodeImageMode::Auto),
    )).with_children(|page| {
        page.spawn((
            Node {
                width: px(440),
                padding: UiRect::all(px(36)),
                flex_direction: FlexDirection::Column,
                row_gap: px(18),
                ..default()
            },
            TabGroup::new(0),
            BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
        )).with_children(|form| {
            spawn_login_form(form);
        });
    });
}

fn spawn_login_field<'a, T: Component>(
    parent: &'a mut ChildSpawnerCommands,
    label: &str,
    marker: T,
    color: Color,
    tab_index: i8,
) -> EntityCommands<'a> {
    let mut wrap = parent.spawn((
        Node {
            width: percent(100),
            height: px(52),
            padding: UiRect::horizontal(px(14)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(color),
    ));
    wrap.with_children(|w| {
        w.spawn((
            Node {
                width: percent(100),
                height: px(22),
                ..default()
            },
            TextInput,
            EditableText::default(),
            TextLayout::new(Justify::Left, LineBreak::NoWrap),
            TextCursorStyle::default(),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
            TabIndex(tab_index as i32),
            marker,
        ))
        .with_child((
            Node {
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                ..default()
            },
            Text::new(label),
            TextFont::from_font_size(16.0),
            TextColor(Color::srgb(0.5, 0.5, 0.5)),
            TextInputPlaceholder,
        ));
    });
    wrap
}

fn spawn_login_form(
    parent: &mut ChildSpawnerCommands,
) {
    parent.spawn(
        (
            Text::new("Welcome back"),
            TextFont {
                font_size: FontSize::Px(32.0),
                ..default()
            },
            TextColor(Color::srgb(0.94, 0.96, 0.93)),
        )
    );
    spawn_login_field(parent, "Username", UsernameField, Color::srgb(0.07, 0.09, 0.10), 0);
    spawn_login_field(parent, "Password", PasswordField, Color::srgb(0.07, 0.09, 0.10), 1);
    parent.spawn((
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
            observe(submit_login),
            children![(
                Text::new("Sign in"),
                TextFont {
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextColor(Color::srgb(0.98, 0.99, 0.97)),
            )],
        ),
        (
            Text::new(""),
            LoginStatusDisplay,
            TextFont {
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.57, 0.48)),
        ),
    ));
}
