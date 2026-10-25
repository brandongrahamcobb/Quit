/* app/src/component/login.rs
 * The purpose of this module is to provide login page components.
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

#[derive(Component)]
pub struct UsernameField;

#[derive(Component)]
pub struct PasswordField;

#[derive(Component)]
pub struct UsernameDisplay;

#[derive(Component)]
pub struct PasswordDisplay;

#[derive(Resource, Default)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub focused: LoginFocus,
    pub show_caret: bool,
    pub caret_started_at: f64
}

#[derive(Clone, Copy, Default)]
pub enum LoginFocus {
    #[default]
    Username,
    Password,
}

#[derive(Component)]
pub struct LoginSubmit;

#[derive(Component)]
pub struct LoginStatusDisplay;

#[derive(Component)]
pub struct LoginScreen;
