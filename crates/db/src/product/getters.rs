/* db/src/product/getters.rs
 * The purpose of this module is to provide database getters for products.
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
use crate::schema::{products, product_compounds};
use crate::product::model::{ProductCompoundModel, ProductModel};
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn get_product_model_by_id(
    pool: &DbPool,
    product_id: i32,
) -> Result<ProductModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        products::table
            .filter(products::id.eq(product_id))
            .first::<ProductModel>(conn)
    })
    .await
}

pub async fn get_all_product_models(
    pool: &DbPool,
) -> Result<Vec<ProductModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        products::table
            .order(products::name.asc())
            .get_results::<ProductModel>(conn)
    })
    .await
}

pub async fn get_product_compounds_by_id(
    pool: &DbPool,
    product_id: i32,
) -> Result<Vec<ProductCompoundModel>, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        product_compounds::table
            .filter(product_compounds::product_id.eq(product_id))
            .order(product_compounds::compound_id.asc())
            .get_results::<ProductCompoundModel>(conn)
    })
    .await
}
