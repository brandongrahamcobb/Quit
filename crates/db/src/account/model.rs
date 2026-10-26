/* db/src/account/model.rs
 * The purpose of this module is to provide an account db model and associated methods.
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

use crate::account::error::AccountModelError;
use crate::schema::accounts;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = accounts)]
pub struct AccountModel {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub pic: Option<String>,
    pub last_login_at: Option<SystemTime>,
    pub gender: String,
    pub accepted_tos: bool,
    pub banned: bool,
    pub admin: bool,
    pub quit_status: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl AccountModel {
    pub fn get_id(&self) -> Result<i32, AccountModelError> {
        if let Some(oid) = self.id {
            Ok(oid)
        } else {
            Err(AccountModelError::NoId)
        }
    }

    pub fn get_pic(&self) -> Result<String, AccountModelError> {
        if let Some(pic) = self.pic.clone() {
            Ok(pic)
        } else {
            Err(AccountModelError::NoPic(self.get_id()?))
        }
    }

    pub fn get_created_at(&self) -> Result<SystemTime, AccountModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(AccountModelError::NoCreatedAt(self.get_id()?))
        }
    }
}
