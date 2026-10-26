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
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use db::account::getters::get_acc_model_by_username;
use std::sync::mpsc::TryRecvError;
use tokio::runtime::Builder;
use crate::component::login::{
    LoginFocus, LoginForm, LoginScreen, LoginStatusDisplay, LoginSubmit, PasswordDisplay, PasswordField, UsernameDisplay, UsernameField
};
use crate::page::AppPage;
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::{
    AuthenticatedAccount, LoginPending, LoginResult, LoginResults,
};

pub fn despawn_login(mut commands: Commands, screens: Query<Entity, With<LoginScreen>>) {
    for screen in &screens {
        commands.entity(screen).despawn();
    }
}

pub fn edit_login_fields(
    mut keyboard: MessageReader<KeyboardInput>,
    time: Res<Time>,
    mut form: ResMut<LoginForm>,
    mut displays: Query<(&mut Text, Option<&UsernameDisplay>, Option<&PasswordDisplay>)>,
) {
    for input in keyboard.read() {
        if input.state != ButtonState::Pressed {
            continue;
        }
        match input.key_code {
            KeyCode::Backspace => match form.focused {
                LoginFocus::Username => {
                    form.username.pop();
                }
                LoginFocus::Password => {
                    form.password.pop();
                }
            },
            KeyCode::Tab => {
                form.focused = match form.focused {
                    LoginFocus::Username => LoginFocus::Password,
                    LoginFocus::Password => LoginFocus::Username,
                };
            }
            _ => {
                let Some(text) = input.text.as_deref() else {
                    continue;
                };
                let target = match form.focused {
                    LoginFocus::Username => &mut form.username,
                    LoginFocus::Password => &mut form.password,
                };
                target.extend(text.chars().filter(|character| !character.is_control()));
            }
        }
    }
    let blink_on = (time.elapsed_secs_f64() - form.caret_started_at) % 1.0 < 0.5;
    for (mut text, username_display, password_display) in &mut displays {
        if username_display.is_some() {
            let active = form.show_caret && matches!(form.focused, LoginFocus::Username);
            let caret = active && blink_on;
            text.0 = if form.username.is_empty() {
                if !active {
                    "Username".to_owned()
                } else if caret {
                    "|".to_owned()
                } else {
                    String::new()
                }
            } else if caret {
                format!("{}|", form.username)
            } else {
                form.username.clone()
            };
        }
        if password_display.is_some() {
            let active = form.show_caret && matches!(form.focused, LoginFocus::Password);
            let caret = active && blink_on;
            let masked_password = "*".repeat(form.password.chars().count());
            text.0 = if form.password.is_empty() {
                if !active {
                    "Password".to_owned()
                } else if caret {
                    "|".to_owned()
                } else {
                    String::new()
                }
            } else if caret {
                format!("{masked_password}|")
            } else {
                masked_password
            };
        }
    }
}

pub fn select_login_field(
    mut interactions: Query<
        (&Interaction, Option<&UsernameField>, Option<&PasswordField>),
        (Changed<Interaction>, With<Button>),
    >,
    time: Res<Time>,
    mut form: ResMut<LoginForm>,
) {
    for (interaction, username, password) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if username.is_some() {
            form.focused = LoginFocus::Username;
            form.caret_started_at = time.elapsed_secs_f64();
            form.show_caret = true;
        } else if password.is_some() {
            form.focused = LoginFocus::Password;
            form.caret_started_at = time.elapsed_secs_f64();
            form.show_caret = true;
        }
    }
}

pub fn submit_login(
    interactions: Query<&Interaction, (Changed<Interaction>, With<LoginSubmit>)>,
    mut keyboard: MessageReader<KeyboardInput>,
    mut form: ResMut<LoginForm>,
    database: Res<Database>,
    results: Res<LoginResults>,
    mut pending: ResMut<LoginPending>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    if pending.0 || !interactions.iter().any(|interaction| *interaction == Interaction::Pressed) {
        let mut enter_pressed = false;
        for input in keyboard.read() {
            enter_pressed |= input.state == ButtonState::Pressed
                && matches!(input.key_code, KeyCode::Enter | KeyCode::NumpadEnter);
        }
        let button_pressed = interactions
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed);
        if pending.0 || (!enter_pressed && !button_pressed) {
            return;
        }
    }
    let username = form.username.trim().to_owned();
    let password = std::mem::take(&mut form.password);
    if username.is_empty() || password.is_empty() {
        for mut text in &mut status {
            text.0 = "Enter your username and password.".to_owned();
        }
        return;
    }
    pending.0 = true;
    for mut text in &mut status {
        text.0 = "Signing in…".to_owned();
    }
    let pool = database.0.clone();
    let sender = results.sender.clone();
    std::thread::spawn(move || {
        let outcome = match Builder::new_current_thread().build() {
            Ok(runtime) => runtime.block_on(async move {
                match get_acc_model_by_username(&pool, username).await {
                    Ok(account) if account.banned => {
                        LoginResult::InvalidCredentials
                    }
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
                    Err(db::error::DatabaseError::DieselError(
                        diesel::result::Error::NotFound,
                    )) => LoginResult::InvalidCredentials,
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
