/* app/src/resource/product_resource.rs
 * The purpose of this module is to define product resources.
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

use bevy::prelude::*;
use bigdecimal::BigDecimal;
use db::product::model::ProductCompoundModel;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::Mutex;

use crate::resource::compound_resource::CompoundEntry;

#[derive(Resource, Default)]
pub struct ProductPageState {
    pub loading_products: bool,
    pub products: Vec<ProductEntry>,
    pub search_query: String,
    pub overlay: Option<ProductOverlay>,
    pub error: Option<String>,
}

#[derive(Resource, Default)]
pub struct ProductFormState {
    pub brand: String,
    pub compounds: Vec<CompoundEntry>,
    pub composition_search_query: String,
    pub composition_search_query_open: bool,
    pub description: String,
    pub error: Option<String>,
    pub image_path: Option<String>,
    pub loading_composition: bool,
    pub loading_compounds: bool,
    pub name: String,
    pub route: String,
    pub route_selector_open: bool,
    pub saving: bool,
    pub selected_compounds: Vec<ProductCompositionEntry>,
    pub serving_description: String,
    pub serving_size: String,
    pub serving_unit: String,
    pub serving_unit_selector_open: bool,
    pub uploading_image: bool,
}

#[derive(Resource, Default)]
pub struct ProductDeletionState {
    pub deleting: bool,
    pub name: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ProductEntry {
    pub id: i32,
    pub name: String,
    pub route: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub serving_size: BigDecimal,
    pub serving_unit: String,
    pub serving_description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ProductCompositionEntry {
    pub compound_id: i32,
    pub compound_name: String,
    pub compound_amount: String,
    pub compound_units: String,
}

#[derive(Resource, Default)]
pub struct ProductLoadReceiver(
    pub Mutex<Option<Receiver<Result<Vec<ProductEntry>, String>>>>,
);

#[derive(Resource)]
pub struct ProductSaveResults {
    pub sender: Sender<Result<Vec<ProductEntry>, String>>,
    pub receiver: Mutex<Receiver<Result<Vec<ProductEntry>, String>>>,
}

impl Default for ProductSaveResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Resource, Default)]
pub struct ProductImageUploadReceiver(
    pub Mutex<Option<Receiver<Result<Option<String>, String>>>>,
);

#[derive(Resource, Default)]
pub struct ProductDeleteResults(
    pub Mutex<Option<Receiver<(i32, Result<(), String>)>>>,
);

#[derive(Resource)]
pub struct ProductCompoundLoadResults {
    pub sender: Sender<Result<Vec<CompoundEntry>, String>>,
    pub receiver: Mutex<Receiver<Result<Vec<CompoundEntry>, String>>>,
}

impl Default for ProductCompoundLoadResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Resource)]
pub struct ProductCompositionLoadResults {
    pub sender: Sender<(i32, Result<Vec<ProductCompoundModel>, String>)>,
    pub receiver: Mutex<Receiver<(i32, Result<Vec<ProductCompoundModel>, String>)>>,
}
impl Default for ProductCompositionLoadResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver: Mutex::new(receiver) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductOverlay {
    Preview(i32),
    Form { editing_id: Option<i32> },
    Delete(i32),
}
