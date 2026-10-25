/* app/src/plugin/login_plugin.rs
* The purpose of this module is to serve the login flow for the Bevy client in a plugin.
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

use crate::component::login::LoginForm;
use crate::page::AppPage;
use crate::resource::login_resource::{LoginPending, LoginResults};
use crate::system::login::{edit_login_fields, poll_login_results, select_login_field, submit_login};
use crate::system::login::despawn_login;
use crate::system::startup::setup_login;

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<LoginForm>()
            .init_resource::<LoginPending>()
            .init_resource::<LoginResults>()
            .add_systems(Startup, setup_login)
            .add_systems(Update, (edit_login_fields, select_login_field, submit_login, poll_login_results).run_if(in_state(AppPage::Login)))
            .add_systems(OnExit(AppPage::Login), despawn_login);
    }
}
