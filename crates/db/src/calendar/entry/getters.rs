/* db/src/calendar/entry/getters.rs
 * The purpose of this module is to provide database getters for calendar entries.
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

use crate::calendar::entry::model::EntryModel;
use crate::error::DatabaseError;
use crate::pool::{self, DbPool};
use crate::schema::entries;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn get_entries_by_acc_id_between(
    pool: &DbPool,
    acc_id: i32,
    start: std::time::SystemTime,
    end: std::time::SystemTime,
) -> Result<Vec<EntryModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        entries::table
            .filter(entries::acc_id.eq(acc_id))
            .filter(entries::occurred_at.ge(start))
            .filter(entries::occurred_at.lt(end))
            .order(entries::occurred_at.asc())
            .load::<EntryModel>(conn)
    })
    .await
}
