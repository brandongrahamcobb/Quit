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
use bevy::input_focus::InputFocus;
use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use bevy::ui_widgets::{Activate, Button, TextInput, observe};
use db::account::getters::get_acc_model_by_username;
use std::sync::mpsc::TryRecvError;
use tokio::runtime::Builder;

use crate::component::login::{
    LoginForm, LoginScreen, LoginStatusDisplay, LoginSubmit, PasswordDisplay, UsernameDisplay
};
use crate::component::page::PageRoot;
use crate::page::AppPage;
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::{
    AuthenticatedAccount, LoginPending, LoginResult, LoginResults,
};


pub fn setup_login(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        PageRoot,
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
            TabGroup::new(0),
            BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
            children![
                (
                    Text::new("Welcome back"),
                    TextFont {
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.96, 0.93)),
                ),
                (
                    Text::new("Sign in."),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.68, 0.74, 0.73)),
                ),
                (
                   Text::new("Username"),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.68, 0.74, 0.73)),
                ),
                (
                    Node {
                        width: percent(100),
                        height: px(52),
                        padding: UiRect {
                            left: px(14),
                            right: px(14),
                            top: px(16),
                            bottom: px(16),
                        },
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                    BorderColor::all(Color::srgb(0.28, 0.36, 0.37)),
                    TextInput,
                    EditableText::default(),
                    TextLayout::new(Justify::Left, LineBreak::NoWrap),
                    TextCursorStyle::default(),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.96, 0.93)),
                    TabIndex(0),
                    UsernameDisplay,
                ),
                (
                    Text::new("Password"),
                    TextFont {
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.68, 0.74, 0.73)),
                ),
                (
                    Node {
                        width: percent(100),
                        height: px(52),
                        padding: UiRect {
                            left: px(14),
                            right: px(14),
                            top: px(16),
                            bottom: px(16),
                        },
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                    BorderColor::all(Color::srgb(0.28, 0.36, 0.37)),
                    TextInput,
                    EditableText::default(),
                    TextLayout::no_wrap(),
                    TextCursorStyle::default(),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.94, 0.96, 0.93)),
                    TabIndex(1),
                    PasswordDisplay,
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
            ],
        )],
    ));
}

pub fn sync_login_fields(
    mut form: ResMut<LoginForm>,
    inputs: Query<(&EditableText, Option<&UsernameDisplay>, Option<&PasswordDisplay>)>,
) {
    for (input, username, password) in &inputs {
        if username.is_some() {
            form.username = input.value().to_string();
        }
        if password.is_some() {
            form.password = input.value().to_string();
        }
    }
}

pub fn submit_login_on_enter(
    input_focus: Res<InputFocus>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    inputs: Query<(), Or<(With<UsernameDisplay>, With<PasswordDisplay>)>>,
    mut form: ResMut<LoginForm>,
    database: Res<Database>,
    results: Res<LoginResults>,
    mut pending: ResMut<LoginPending>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }
    if let Some(focused_entity) = input_focus.get() {
        if inputs.contains(focused_entity) {
            start_login(form.as_mut(), database.as_ref(), results.as_ref(), pending.as_mut(), &mut status);
        }
    }
}

pub fn submit_login(
    _activate: On<Activate>,
    mut form: ResMut<LoginForm>,
    database: Res<Database>,
    results: Res<LoginResults>,
    mut pending: ResMut<LoginPending>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    start_login(form.as_mut(), database.as_ref(), results.as_ref(), pending.as_mut(), &mut status);
}

pub fn start_login(
    form: &mut LoginForm,
    database: &Database,
    results: &LoginResults,
    pending: &mut LoginPending,
    status: &mut Query<&mut Text, With<LoginStatusDisplay>>,
) {
    if pending.0 {
        return;
    }
    let username = form.username.trim().to_owned();
    let password = std::mem::take(&mut form.password);
    if username.is_empty() || password.is_empty() {
        for mut text in status.iter_mut() {
            text.0 = "Enter your username and password.".to_owned();
        }
        return;
    }
    pending.0 = true;
    for mut text in status.iter_mut() {
        text.0 = "Signing in…".to_owned();
    }
    let pool = database.0.clone();
    let sender = results.sender.clone();
    std::thread::spawn(move || {
        let outcome = match Builder::new_current_thread().build() {
            Ok(runtime) => runtime.block_on(async move {
                match get_acc_model_by_username(&pool, username).await {
                    Ok(account) if account.banned => LoginResult::InvalidCredentials,
                    Ok(account) => match inc::account::authenticate(account.password, password) {
                        Ok(true) => match account.id {
                            Some(id) => LoginResult::Authenticated {
                                id,
                                username: account.username,
                                accepted_tos: account.accepted_tos,
                                quit_status: account.quit_status,
                            },
                            None => LoginResult::DatabaseUnavailable,
                        },
                        Ok(false) => LoginResult::InvalidCredentials,
                        Err(error) => {
                            eprintln!("password verification failed: {error}");
                            LoginResult::DatabaseUnavailable
                        }
                    },
                    Err(db::error::DatabaseError::DieselError(diesel::result::Error::NotFound)) => LoginResult::InvalidCredentials,
                    Err(error) => {
                        eprintln!("account lookup failed: {error}");
                        LoginResult::DatabaseUnavailable
                    }
                }
            }),
            Err(error) => {
                eprintln!("failed to start login runtime: {error}");
                LoginResult::DatabaseUnavailable
            }
        };
        let _ = sender.send(outcome);
    });
}

pub fn poll_login_results(
    mut commands: Commands,
    results: Res<LoginResults>,
    mut pending: ResMut<LoginPending>,
    mut next_page: ResMut<NextState<AppPage>>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    let result = match results.receiver.lock() {
        Ok(receiver) => match receiver.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                pending.0 = false;
                return;
            }
        },
        Err(_) => {
            pending.0 = false;
            return;
        }
    };
    pending.0 = false;
    match result {
        LoginResult::Authenticated { id, username, accepted_tos, quit_status } => {
            commands.insert_resource(AuthenticatedAccount { id, username });
            next_page.set(if accepted_tos && quit_status.is_some() { AppPage::Dashboard } else { AppPage::Onboarding });
        }
        LoginResult::InvalidCredentials => {
            for mut text in &mut status {
                text.0 = "Invalid username or password.".to_owned();
            }
        }
        LoginResult::DatabaseUnavailable => {
            for mut text in &mut status {
                text.0 = "Sign-in is unavailable right now. Try again shortly.".to_owned();
            }
        }
    }
}
