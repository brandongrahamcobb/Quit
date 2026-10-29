/* db/src/substance/model.rs
 * The purpose of this module is to provide a substance db model and associated methods.
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

use crate::substance::error::SubstanceModelError;
use crate::schema::substances;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable)]
#[diesel(table_name = substances)]
pub struct SubstanceModel {
    pub id: i32,
    pub acc_id: i32,
    pub name: String,
    pub kind: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = substances)]
pub struct NewSubstanceModel {
    pub acc_id: i32,
    pub name: String,
    pub kind: String,
}

impl SubstanceModel {
    pub fn get_created_at(&self) -> Result<SystemTime, SubstanceModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(SubstanceModelError::NoCreatedAt(self.id))
        }
    }
}
