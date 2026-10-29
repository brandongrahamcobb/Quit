/* db/src/substance/query/setters.rs
 * The purpose of this module is to provide database setters for substances.
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
use crate::pool;
use crate::pool::DbPool;
use crate::schema::substances;
use crate::substance::model::{NewSubstanceModel, SubstanceModel};
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn insert_substance(
    pool: &DbPool,
    new_substance: NewSubstanceModel,
) -> Result<SubstanceModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::insert_into(substances::table)
            .values(new_substance)
            .get_result::<SubstanceModel>(conn)
    })
    .await
}
