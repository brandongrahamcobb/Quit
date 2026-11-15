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
use bevy::text::{EditableText};
use std::sync::mpsc::TryRecvError;

use crate::component::login::{
    LoginStatusDisplay, PasswordField, UsernameField
};
use crate::page::AppPage;
use crate::resource::login_resource::{
    AuthenticatedAccount, LoginFormState, LoginPageState, LoginReceiver, LoginResult
};

pub fn poll_login_fields(
    mut form: ResMut<LoginFormState>,
    inputs: Query<(&EditableText, Option<&UsernameField>, Option<&PasswordField>)>,
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

pub fn poll_login_results(
    mut commands: Commands,
    receiver: Res<LoginReceiver>,
    mut page_state: ResMut<LoginPageState>,
    mut next_page: ResMut<NextState<AppPage>>,
    mut status: Query<&mut Text, With<LoginStatusDisplay>>,
) {
    let result = {
        let Ok(mut slot) = receiver.0.lock() else {
            return;
        };
        let Some(result_receiver) = slot.as_ref() else {
            return;
        };
        match result_receiver.try_recv() {
            Ok(result) => Some(result),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                for mut text in &mut status {
                    text.0 = "Login result channel disconnected.".to_owned();
                }
                page_state.pending = false;
                return;
            }
        }
    };
    let Some(result) = result else {
        return;
    };
    page_state.pending = false;
    match result {
        LoginResult::Authenticated { id, accepted_tos, quit_status } => {
            commands.insert_resource(AuthenticatedAccount(id));
            next_page.set(if accepted_tos && quit_status.is_some() { AppPage::Dashboard } else { AppPage::Onboarding });
        }
        LoginResult::InvalidCredentials => {
            for mut text in &mut status {
                text.0 = "Invalid username or password.".to_owned();
            }
        }
        LoginResult::DatabaseUnavailable => {
            for mut text in &mut status {
                text.0 = "Unable to sign in because the database is unavailable.".to_owned();
            }
        }
    }
}
