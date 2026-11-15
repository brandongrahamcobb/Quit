use std::sync::mpsc::channel;

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
use bevy::prelude::*;
use bevy::ui_widgets::{Activate};
use db::account::getters::get_acc_model_by_username;
use tokio::runtime::Builder;

use crate::component::login::{
    LoginStatusDisplay,
};
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::{LoginFormState, LoginPageState, LoginReceiver, LoginResult};

pub fn submit_login_on_enter(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut page_state: ResMut<LoginPageState>,
    mut form_state: ResMut<LoginFormState>,
    database: Res<Database>,
    receiver: Res<LoginReceiver>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Enter) {
        return;
    }
    start_login(page_state.as_mut(), form_state.as_mut(), database.as_ref(), receiver.as_ref(), &mut status);
}

pub fn submit_login(
    _activate: On<Activate>,
    mut page_state: ResMut<LoginPageState>,
    mut form_state: ResMut<LoginFormState>,
    database: Res<Database>,
    receiver: Res<LoginReceiver>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    start_login(page_state.as_mut(), form_state.as_mut(), database.as_ref(), receiver.as_ref(), &mut status);
}

pub fn start_login(
    page_state: &mut LoginPageState,
    form_state: &mut LoginFormState,
    database: &Database,
    receiver: &LoginReceiver,
    status: &mut Query<&mut Text, With<LoginStatusDisplay>>,
) {
    if page_state.pending {
        return;
    }
    let username = form_state.username.trim().to_owned();
    let password = std::mem::take(&mut form_state.password);
    if username.is_empty() || password.is_empty() {
        for mut text in status.iter_mut() {
            text.0 = "Enter your username and password.".to_owned();
        }
        return;
    }
    page_state.pending = true;
    for mut text in status.iter_mut() {
        text.0 = "Signing in…".to_owned();
    }
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        //form_state.saving = false;
        form_state.error = Some("Could not initialize login.".to_string());
        return;
    }
    std::thread::spawn(move || {
        let outcome = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime.block_on(async move {
                match get_acc_model_by_username(&pool, username).await {
                    Ok(account) if account.banned => LoginResult::InvalidCredentials,
                    Ok(account) => match inc::account::authenticate(account.password, password) {
                        Ok(true) => match account.id {
                            Some(id) => LoginResult::Authenticated {
                                id,
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
