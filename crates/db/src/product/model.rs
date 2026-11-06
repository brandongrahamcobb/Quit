/* db/src/product/model.rs
 * The purpose of this module is to provide a product db model and associated methods.
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

use crate::product::error::ProductModelError;
use crate::schema::{products, product_compounds};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Queryable)]
#[diesel(table_name = products)]
pub struct ProductModel {
    pub id: i32,
    pub name: String,
    pub route: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub serving_size: BigDecimal,
    pub serving_unit: String,
    pub serving_description: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl ProductModel {
    pub fn get_created_at(&self) -> Result<SystemTime, ProductModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(ProductModelError::NoCreatedAt(self.id))
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProductModel {
    pub name: String,
    pub route: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub serving_size: BigDecimal,
    pub serving_unit: String,
    pub serving_description: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = products)]
#[diesel(treat_none_as_null = true)]
pub struct UpdateProductModel {
    pub name: String,
    pub route: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub serving_size: BigDecimal,
    pub serving_unit: String,
    pub serving_description: Option<String>,
}

#[derive(Clone, Queryable, Identifiable)]
#[diesel(table_name = product_compounds)]
#[diesel(primary_key(product_id, compound_id))]
pub struct ProductCompoundModel {
    pub product_id: i32,
    pub compound_id: i32,
    pub compound_amount: BigDecimal,
    pub compound_units: String,
}

#[derive(Insertable)]
#[diesel(table_name = product_compounds)]
pub struct NewProductCompoundModel {
    pub product_id: i32,
    pub compound_id: i32,
    pub compound_amount: BigDecimal,
    pub compound_units: String,
}

#[derive(Clone)]
pub struct ProductCompoundInput {
    pub compound_id: i32,
    pub compound_amount: BigDecimal,
    pub compound_units: String,
}

