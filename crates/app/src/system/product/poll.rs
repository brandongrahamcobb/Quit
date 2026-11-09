/* app/src/system/product/poll.rs
 * The purpose of this module is to provide polling systems to respond to product system loading.
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
use crate::component::product::ProductFormOverlay;
use crate::resource::product_resource::{ProductCompositionEntry, ProductCompositionLoadResults, ProductCompoundLoadResults, ProductDeleteResults, ProductImageUploadReceiver, ProductLoadReceiver, ProductOverlay, ProductPageState, ProductSaveResults};
use std::sync::mpsc::TryRecvError;


pub fn poll_product_load(receiver: Res<ProductLoadReceiver>, mut state: ResMut<ProductPageState>) {
    let result = {
        let Ok(mut slot) = receiver.0.lock() else {
            return;
        };
        let Some(result_receiver) = slot.as_ref() else {
            return;
        };
        match result_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Product loading worker disconnected.".to_string()))
            }
        }
    };
    let Some(result) = result else {
        return;
    };
    state.loading = false;
    match result {
        Ok(products) => {
            state.products = products;
            state.error = None;
        }
        Err(error) => {
            state.error = Some(error);
        }
    }
}

pub fn poll_product_image_upload(
    mut receiver: ResMut<ProductImageUploadReceiver>,
    mut state: ResMut<ProductPageState>,
) {
    let result = {
        let mut slot = receiver.0.lock().unwrap();
        let Some(channel_receiver) = slot.as_ref() else {
            return;
        };
        match channel_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Image upload operation failed.".to_owned()))
            }
        }
    };
    if let Some(result) = result {
        state.image_loading = false;
        match result {
            Ok(Some(path)) => {
                state.image_path = Some(path);
                state.error = None;
            }
            Ok(None) => {}
            Err(error) => {
                state.error = Some(error);
            }
        }
    }
}

pub fn poll_product_delete_results(
    mut results: ResMut<ProductDeleteResults>,
    mut state: ResMut<ProductPageState>,
) {
    let result = {
        let mut receiver = results.0.lock().unwrap();
        receiver.as_ref().and_then(|receiver| receiver.try_recv().ok())
    };
    let Some((id, result)) = result else {
        return;
    };
    *results.0.lock().unwrap() = None;
    state.deleting = false;
    match result {
        Ok(()) => {
            state.products.retain(|product| product.id != id);
            if state.overlay == Some(ProductOverlay::Preview(id)) || state.overlay == Some(ProductOverlay::Delete(id)) {
                state.overlay = None;
            }
            state.deleting_name = None;
            state.delete_error = None;
        }
        Err(error) => {
            state.delete_error = Some(error);
        }
    }
}



pub fn poll_product_compound_load_results(
    results: Res<ProductCompoundLoadResults>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    while let Ok(result) = receiver.try_recv() {
        state.compounds_loading = false;
        match result {
            Ok(compounds) => {
                state.compounds = compounds;
                state.error = None;
                state.compounds_loaded = true;
                state.compounds_loading = false;
            }
            Err(error) => {
                state.compounds_loading = false;
                state.error = Some(format!("Could not load available compounds: {error}"));
            }
        }
    }
}

pub fn poll_product_composition_load_results(
    mut results: ResMut<ProductCompositionLoadResults>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    let Ok(result) = receiver.try_recv() else {
        return;
    };
    match result {
        Ok((generation, product_id, compounds)) => {
            if state.composition_load_generation != generation || state.overlay != Some(ProductOverlay::Form { editing_id: Some(product_id) }) {
                return;
            }
            state.composition_loading = false;
            let selected_compounds: Option<Vec<ProductCompositionEntry>> = compounds.into_iter().map(|compound| {
                state.compounds.iter().find(|entry| entry.id == compound.compound_id).map(|entry| ProductCompositionEntry {
                    compound_id: compound.compound_id,
                    compound_name: entry.common_name.clone(),
                    compound_amount: compound.compound_amount.to_string(),
                    compound_units: compound.compound_units,
                })
            }).collect();
            match selected_compounds {
                Some(compounds) => {
                    state.selected_compounds = compounds;
                }
                None => {
                    state.error = Some("A compound in this product's composition could not be found.".to_string());
                }
            }
        }
        Err(error) => {
            state.composition_loading = false;
            state.error = Some(error);
        }
    }
}

pub fn poll_product_save_results(
    mut form_overlay: Query<&mut Node, With<ProductFormOverlay>>,
    mut results: Res<ProductSaveResults>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    let Ok(result) = receiver.try_recv() else {
        return;
    };
    state.loading = false;
    match result {
        Ok(products) => {
            state.products = products;
            state.brand.clear();
            state.description.clear();
            state.name.clear();
            state.route.clear();
            state.selected_compounds.clear();
            state.route_selector_open = false;
            state.error = None;
            if let Ok(mut node) = form_overlay.single_mut() {
                node.display = Display::None;
            }
        }
        Err(error) => {
            state.error = Some(error);
        }
    }
}
