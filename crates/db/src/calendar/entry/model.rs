/* db/src/calendar/entry/model.rs
 * The purpose of this module is to provide a calendar entry db model and associated methods.
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

use crate::calendar::entry::error::EntryModelError;
use crate::schema::entries;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable)]
#[diesel(table_name = entries)]
pub struct EntryModel {
    pub id: i32,
    pub acc_id: i32,
    pub occurred_at: SystemTime,
    pub mass: i32,
    pub units: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
pub struct NewEntryModel {
    pub acc_id: i32,
    pub occurred_at: SystemTime,
    pub mass: i32,
    pub units: String,
}
