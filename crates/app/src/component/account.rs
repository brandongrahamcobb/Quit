/* app/src/component/account.rs
 * The purpose of this module is to provide an account component.
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

use core::convert::From;

use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use db::account::model::AccountModel;

#[derive(Clone, Component)]
pub struct QuitAccount {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub pic: Option<String>,
    pub gender: String,
    pub accepted_tos: bool,
    pub banned: bool,
    pub admin: bool,
}

#[derive(Component)]
pub struct InAccount(pub Entity);

impl From<(AccountModel, i32)> for QuitAccount {
    fn from((model, id): (AccountModel, i32)) -> Self {
        Self {
            id,
            username: model.username,
            password: model.password,
            pic: model.pic,
            gender: model.gender,
            accepted_tos: model.accepted_tos,
            banned: model.banned,
            admin: model.admin,
        }
    }
}
