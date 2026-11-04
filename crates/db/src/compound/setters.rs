/* db/src/compound/query/setters.rs
 * The purpose of this module is to provide database setters for compounds.
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
use crate::schema::{compound_metabolites, compounds};
use crate::compound::model::{CompoundModel, NewCompoundModel, UpdateCompoundModel};
use diesel::expression_methods::*;
use diesel::{Connection, QueryDsl, RunQueryDsl};

pub async fn insert_compound(
    pool: &DbPool,
    new_compound: NewCompoundModel,
) -> Result<CompoundModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::insert_into(compounds::table)
            .values(new_compound)
            .get_result::<CompoundModel>(conn)
    })
    .await
}


pub async fn set_compound_metabolites(
    pool: &DbPool,
    parent_id: i32,
    metabolite_ids: Vec<i32>,
) -> Result<(), DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        conn.transaction::<(), diesel::result::Error, _>(|conn| {
            let mut unique_ids = metabolite_ids.clone();
            unique_ids.sort_unstable();
            unique_ids.dedup();
            if unique_ids.len() != metabolite_ids.len() {
                return Err(diesel::result::Error::RollbackTransaction.into());
            }
            if unique_ids.contains(&parent_id) {
                return Err(diesel::result::Error::RollbackTransaction.into());
            }
            diesel::delete(
                compound_metabolites::table
                    .filter(compound_metabolites::parent_compound_id.eq(parent_id)),
            )
            .execute(conn)?;
            let relationships: Vec<_> = metabolite_ids
                .into_iter()
                .map(|metabolite_id| {
                    (
                        compound_metabolites::parent_compound_id.eq(parent_id),
                        compound_metabolites::metabolite_compound_id.eq(metabolite_id),
                    )
                })
                .collect();
            if !relationships.is_empty() {
                diesel::insert_into(compound_metabolites::table)
                    .values(&relationships)
                    .execute(conn)?;
            }
            Ok(())
        })
    })
    .await
}

pub async fn update_compound(
    pool: &DbPool,
    compound_id: i32,
    updated_compound: UpdateCompoundModel,
) -> Result<(), DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(compounds::table.filter(compounds::id.eq(compound_id)))
            .set((
                updated_compound,
                compounds::updated_at.eq(std::time::SystemTime::now()),
            ))
            .execute(conn)
            .and_then(|rows| {
                if rows == 0 {
                    Err(diesel::result::Error::NotFound)
                } else {
                    Ok(())
                }
            })
    })
    .await
}

pub async fn delete_compound(pool: &DbPool, compound_id: i32) -> Result<(), DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::delete(compound_metabolites::table.filter(
                compound_metabolites::parent_compound_id.eq(compound_id).or(
                    compound_metabolites::metabolite_compound_id.eq(compound_id)
                )
            )).execute(conn)?;
            diesel::delete(compounds::table.filter(compounds::id.eq(compound_id))).execute(conn)?;
            Ok(())
        })
    }).await
}
