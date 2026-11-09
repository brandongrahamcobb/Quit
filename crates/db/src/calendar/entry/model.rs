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

use crate::schema::{entries, entry_compounds};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel;
use std::time::SystemTime;

#[derive(Clone, Queryable)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EntryModel {
    pub id: i32,
    pub acc_id: i32,
    pub occurred_at: SystemTime,
    pub product_id: i32,
    pub amount: BigDecimal,
    pub amount_unit: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEntryModel {
    pub acc_id: i32,
    pub occurred_at: SystemTime,
    pub product_id: i32,
    pub amount: BigDecimal,
    pub amount_unit: String,
}

#[derive(Insertable)]
#[diesel(table_name = entry_compounds)]
pub struct NewEntryCompoundModel {
    pub entry_id: i32,
    pub compound_id: i32,
    pub compound_amount: BigDecimal,
    pub compound_units: String,
}
