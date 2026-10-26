/* app/src/resource/login_resource.rs
 * The purpose of this module is to define login resources.
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

use bevy::ecs::resource::Resource;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::Mutex;

#[derive(Debug)]
pub enum LoginResult {
    Authenticated { id: i32, username: String, accepted_tos: bool, quit_status: Option<String> },
    InvalidCredentials,
    DatabaseUnavailable,
}

#[derive(Resource)]
pub struct LoginResults {
    pub sender: Sender<LoginResult>,
    pub receiver: Mutex<Receiver<LoginResult>>,
}

impl Default for LoginResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Resource, Default)]
pub struct LoginPending(pub bool);

#[derive(Resource)]
pub struct AuthenticatedAccount {
    pub id: i32,
    pub username: String,
}
