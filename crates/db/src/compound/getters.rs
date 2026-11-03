/* db/src/compound/getters.rs
 * The purpose of this module is to provide database getters for compounds.
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

use crate::error::DatabaseError;
use crate::pool::{self, DbPool};
use crate::schema::compounds;
use crate::schema::compound_metabolites::dsl::{compound_metabolites, parent_compound_id, metabolite_compound_id};
use crate::compound::model::CompoundModel;
use diesel::expression_methods::*;
use diesel::{JoinOnDsl, QueryDsl, RunQueryDsl};

pub async fn get_compound_model_by_id(
    pool: &DbPool,
    compound_id: i32,
) -> Result<CompoundModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        compounds::table
            .filter(compounds::id.eq(compound_id))
            .first::<CompoundModel>(conn)
    })
    .await
}

pub async fn get_metabolites_by_compound_id(
    pool: &DbPool,
    compound_id: i32,
) -> Result<Vec<CompoundModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        compound_metabolites
            .inner_join(compounds::table.on(compounds::id.eq(metabolite_compound_id)))
            .filter(parent_compound_id.eq(compound_id))
            .select(compounds::all_columns)
            .load::<CompoundModel>(conn)
    })
    .await
}

pub async fn get_all_compound_models(
    pool: &DbPool,
) -> Result<Vec<CompoundModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        compounds::table
            .order(compounds::common_name.asc())
            .load::<CompoundModel>(conn)
    })
    .await
}
