/* db/src/product/query/setters.rs
 * The purpose of this module is to provide database setters for products.
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
use crate::schema::{products, product_compounds};
use crate::product::model::{NewProductCompoundModel, NewProductModel, ProductCompoundInput, ProductModel, UpdateProductModel};
use diesel::{Connection, PgConnection, expression_methods::*};
use diesel::{QueryDsl, RunQueryDsl};

pub async fn insert_product(
    pool: &DbPool,
    new_product: NewProductModel,
) -> Result<ProductModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::insert_into(products::table)
            .values(new_product)
            .get_result::<ProductModel>(conn)
    })
    .await
}

pub async fn update_product(
    pool: &DbPool,
    product_id: i32,
    updated_product: UpdateProductModel,
) -> Result<(), DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(products::table.filter(products::id.eq(product_id)))
            .set((
                updated_product,
                products::updated_at.eq(std::time::SystemTime::now()),
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

pub async fn delete_product(
    pool: &DbPool,
    product_id: i32,
) -> Result<(), DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::delete(products::table.filter(products::id.eq(product_id)))
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

pub async fn set_product_compounds(
    pool: &DbPool,
    product_id: i32,
    compounds: Vec<NewProductCompoundModel>,
) -> Result<(), DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        conn.transaction::<(), diesel::result::Error, _>(|conn| {
            let mut unique_ids: Vec<i32> = compounds.iter().map(|compound| compound.compound_id).collect();
            unique_ids.sort_unstable();
            unique_ids.dedup();
            if unique_ids.len() != compounds.len() {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            if compounds.iter().any(|compound| compound.product_id != product_id) {
                return Err(diesel::result::Error::RollbackTransaction);
            }
            diesel::delete(
                product_compounds::table
                    .filter(product_compounds::product_id.eq(product_id)),
            )
            .execute(conn)?;
            if !compounds.is_empty() {
                diesel::insert_into(product_compounds::table)
                    .values(&compounds)
                    .execute(conn)?;
            }
            Ok(())
        })
    })
    .await
}

fn replace_product_compounds(
    conn: &mut PgConnection,
    product_id: i32,
    compounds: Vec<ProductCompoundInput>,
) -> Result<(), diesel::result::Error> {
    let mut unique_ids: Vec<i32> = compounds.iter().map(|compound| compound.compound_id).collect();
    unique_ids.sort_unstable();
    unique_ids.dedup();
    if unique_ids.len() != compounds.len() || compounds.iter().any(|compound| compound.compound_amount <= 0 || !["mg", "mL"].contains(&compound.compound_units.as_str())) {
        return Err(diesel::result::Error::RollbackTransaction);
    }
    diesel::delete(product_compounds::table.filter(product_compounds::product_id.eq(product_id))).execute(conn)?;
    let new_compounds: Vec<NewProductCompoundModel> = compounds.into_iter().map(|compound| NewProductCompoundModel {
        product_id,
        compound_id: compound.compound_id,
        compound_amount: compound.compound_amount,
        compound_units: compound.compound_units,
    }).collect();
    if !new_compounds.is_empty() {
        diesel::insert_into(product_compounds::table).values(&new_compounds).execute(conn)?;
    }
    Ok(())
}

pub async fn insert_product_with_compounds(
    pool: &DbPool,
    new_product: NewProductModel,
    compounds: Vec<ProductCompoundInput>,
) -> Result<ProductModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        conn.transaction::<ProductModel, diesel::result::Error, _>(|conn| {
            let product = diesel::insert_into(products::table)
                .values(new_product)
                .get_result::<ProductModel>(conn)?;
            replace_product_compounds(conn, product.id, compounds)?;
            Ok(product)
        })
    }).await
}

pub async fn update_product_with_compounds(
    pool: &DbPool,
    product_id: i32,
    updated_product: UpdateProductModel,
    compounds: Vec<ProductCompoundInput>,
) -> Result<ProductModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        conn.transaction::<ProductModel, diesel::result::Error, _>(|conn| {
            let product = diesel::update(products::table.filter(products::id.eq(product_id)))
                .set((updated_product, products::updated_at.eq(std::time::SystemTime::now())))
                .get_result::<ProductModel>(conn)?;
            replace_product_compounds(conn, product.id, compounds)?;
            Ok(product)
        })
    }).await
}
