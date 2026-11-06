/* db/src/account/query/setters.rs
 * The purpose of this module is to provide database setters for accounts.
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

use crate::calendar::entry::model::{EntryModel, NewEntryCompoundModel, NewEntryModel};
use crate::error::DatabaseError;
use crate::pool::{self, DbPool};
use crate::schema::{entries, entry_compounds, product_compounds, products};
use diesel::RunQueryDsl;
use diesel::prelude::*;

pub async fn insert_entry(
    pool: &DbPool,
    new_entry: NewEntryModel,
) -> Result<EntryModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        conn.transaction::<EntryModel, diesel::result::Error, _>(|conn| {
            let (serving_size, serving_unit): (bigdecimal::BigDecimal, String) = products::table
                .filter(products::id.eq(new_entry.product_id))
                .select((products::serving_size, products::serving_unit))
                .first(conn)?;
            if serving_unit != new_entry.amount_unit {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            let compositions: Vec<(i32, bigdecimal::BigDecimal, String)> = product_compounds::table
                .filter(product_compounds::product_id.eq(new_entry.product_id))
                .select((
                    product_compounds::compound_id,
                    product_compounds::compound_amount,
                    product_compounds::compound_units,
                ))
                .load(conn)?;
            let entry = diesel::insert_into(entries::table)
                .values(&new_entry)
                .get_result::<EntryModel>(conn)?;
            let ratio = new_entry.amount / serving_size;
            let snapshots: Vec<NewEntryCompoundModel> = compositions
                .into_iter()
                .map(|(compound_id, compound_amount, compound_units)| NewEntryCompoundModel {
                    entry_id: entry.id,
                    compound_id,
                    compound_amount: compound_amount * ratio.clone(),
                    compound_units,
                })
                .collect();
            if !snapshots.is_empty() {
                diesel::insert_into(entry_compounds::table)
                    .values(&snapshots)
                    .execute(conn)?;
            }
            Ok(entry)
        })
    })
    .await
}
