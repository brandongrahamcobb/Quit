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
use crate::resource::product_resource::{ProductCompositionEntry, ProductCompositionLoadResults, ProductCompoundLoadResults, ProductDeleteResults, ProductDeletionState, ProductFormState, ProductImageUploadReceiver, ProductLoadReceiver, ProductOverlay, ProductPageState, ProductSaveResults};
use std::sync::mpsc::TryRecvError;


pub fn poll_product_load(receiver: Res<ProductLoadReceiver>, mut page_state: ResMut<ProductPageState>) {
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
    page_state.loading = false;
    match result {
        Ok(products) => {
            page_state.products = products;
            page_state.error = None;
        }
        Err(error) => {
            page_state.error = Some(error);
        }
    }
}

pub fn poll_product_image_upload(
    receiver: ResMut<ProductImageUploadReceiver>,
    mut form_state: ResMut<ProductFormState>,
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
        form_state.image_loading = false;
        match result {
            Ok(Some(path)) => {
                form_state.image_path = Some(path);
                form_state.error = None;
            }
            Ok(None) => {}
            Err(error) => {
                form_state.error = Some(error);
            }
        }
    }
}

pub fn poll_product_delete_results(
    results: ResMut<ProductDeleteResults>,
    mut page_state: ResMut<ProductPageState>,
    mut deletion_state: ResMut<ProductDeletionState>,
) {
    let result = {
        let receiver = results.0.lock().unwrap();
        receiver.as_ref().and_then(|receiver| receiver.try_recv().ok())
    };
    let Some((id, result)) = result else {
        return;
    };
    *results.0.lock().unwrap() = None;
    deletion_state.deleting = false;
    match result {
        Ok(()) => {
            page_state.products.retain(|product| product.id != id);
            if page_state.overlay == Some(ProductOverlay::Preview(id)) || page_state.overlay == Some(ProductOverlay::Delete(id)) {
                page_state.overlay = None;
            }
            deletion_state.name = None;
            deletion_state.error = None;
        }
        Err(error) => {
            deletion_state.error = Some(error);
        }
    }
}

pub fn poll_product_compound_load_results(
    results: Res<ProductCompoundLoadResults>,
    mut page_state: ResMut<ProductPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    while let Ok(result) = receiver.try_recv() {
        page_state.compounds_loading = false;
        match result {
            Ok(compounds) => {
                page_state.compounds = compounds;
                page_state.compounds_loaded = true;
                page_state.compounds_loading = false;
                page_state.error = None;
            }
            Err(error) => {
                page_state.compounds_loading = false;
                page_state.error = Some(format!("Could not load available compounds: {error}"));
            }
        }
    }
}

pub fn poll_product_composition_load_results(
    results: ResMut<ProductCompositionLoadResults>,
    mut page_state: ResMut<ProductPageState>,
    mut form_state: ResMut<ProductFormState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    let Ok(result) = receiver.try_recv() else {
        return;
    };
    match result {
        Ok((generation, product_id, compounds)) => {
            if page_state.composition_load_generation != generation || page_state.overlay != Some(ProductOverlay::Form { editing_id: Some(product_id) }) {
                return;
            }
            page_state.composition_loading = false;
            let selected_compounds: Option<Vec<ProductCompositionEntry>> = compounds.into_iter().map(|compound| {
                page_state.compounds.iter().find(|entry| entry.id == compound.compound_id).map(|entry| ProductCompositionEntry {
                    compound_id: compound.compound_id,
                    compound_name: entry.common_name.clone(),
                    compound_amount: compound.compound_amount.to_string(),
                    compound_units: compound.compound_units,
                })
            }).collect();
            match selected_compounds {
                Some(compounds) => {
                    form_state.selected_compounds = compounds;
                }
                None => {
                    page_state.error = Some("A compound in this product's composition could not be found.".to_string());
                }
            }
        }
        Err(error) => {
            page_state.composition_loading = false;
            page_state.error = Some(error);
        }
    }
}

pub fn poll_product_save_results(
    mut form_overlay: Query<&mut Node, With<ProductFormOverlay>>,
    results: Res<ProductSaveResults>,
    mut page_state: ResMut<ProductPageState>,
    mut form_state: ResMut<ProductFormState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    let Ok(result) = receiver.try_recv() else {
        return;
    };
    match result {
        Ok(products) => {
            page_state.products = products;
            form_state.brand.clear();
            form_state.description.clear();
            form_state.name.clear();
            form_state.route.clear();
            form_state.selected_compounds.clear();
            form_state.route_selector_open = false;
            form_state.error = None;
            if let Ok(mut node) = form_overlay.single_mut() {
                node.display = Display::None;
            }
        }
        Err(error) => {
            form_state.error = Some(error);
        }
    }
}
