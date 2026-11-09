/* app/src/system/product/activate/delete.rs
 * The purpose of this module is to provide deletion observers for the product system.
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
use bevy::ui_widgets::Activate;
use db::product::setters::delete_product_by_id;
use crate::component::product::ProductDeleteButton;
use crate::resource::custom_resource::Database;
use crate::resource::product_resource::{ProductDeleteResults, ProductOverlay, ProductPageState};
use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Builder;

pub fn cancel_product_delete(
    _activate: On<Activate>,
    mut state: ResMut<ProductPageState>,
) {
    if state.deleting {
        return;
    }
    state.overlay = None;
    state.deleting_name = None;
    state.delete_error = None;
}

pub fn confirm_product_delete(
    _activate: On<Activate>,
    database: Res<Database>,
    mut results: ResMut<ProductDeleteResults>,
    mut state: ResMut<ProductPageState>,
) {
    if state.deleting {
        return;
    }
    let Some(ProductOverlay::Delete(id)) = state.overlay else {
        return;
    };
    let pool = database.0.clone();
    let (sender, receiver) = channel();
    *results.0.lock().unwrap() = Some(receiver);
    state.deleting = true;
    state.delete_error = None;
    thread::spawn(move || {
        let runtime = Builder::new_current_thread().enable_all().build();
        let result = match runtime {
            Ok(runtime) => runtime.block_on(delete_product_by_id(&pool, id)).map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((id, result));
    });
}

pub fn delete_product(
    activate: On<Activate>,
    delete_buttons: Query<&ProductDeleteButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = delete_buttons.get(activate.entity) else {
        return;
    };
    let id = button.0;
    let Some(name) = state.products.iter().find(|product| product.id == id).map(|product| product.name.clone()) else {
        return;
    };
    state.overlay = Some(ProductOverlay::Delete(id));
    state.deleting_name = Some(name);
    state.delete_error = None;
}
