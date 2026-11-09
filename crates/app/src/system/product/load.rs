/* app/src/system/product/load.rs
 * The purpose of this module is to provide loaders for the product system.
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
use db::compound::getters::get_all_compound_models;
use db::product::getters::{get_all_product_models, get_product_compounds_by_id};
use crate::resource::compound_resource::CompoundEntry;
use crate::resource::custom_resource::Database;
use crate::resource::product_resource::{ProductCompositionLoadResults, ProductCompoundLoadResults, ProductEntry, ProductLoadReceiver, ProductOverlay, ProductPageState};
use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Builder;


pub fn begin_product_load(database: Res<Database>, mut state: ResMut<ProductPageState>, receiver: Res<ProductLoadReceiver>) {
    if state.loading {
        return;
    }
    state.loading = true;
    state.error = None;
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        state.loading = false;
        state.error = Some("Could not initialize product loading.".to_string());
        return;
    }
    thread::spawn(move || {
        let result = match Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime
                .block_on(get_all_product_models(&pool))
                .map(|models| {
                    models
                        .into_iter()
                        .map(|model| ProductEntry {
                            id: model.id,
                            name: model.name,
                            route: model.route,
                            brand: model.brand,
                            description: model.description,
                            image_path: model.image_path,
                            serving_size: model.serving_size,
                            serving_unit: model.serving_unit,
                            serving_description: model.serving_description,
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|error| format!("{error:?}")),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn begin_product_compound_load(
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductCompoundLoadResults>,
) {
    if state.compounds_loading || state.compounds_loaded {
        return;
    }
    state.compounds_loading = true;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    thread::spawn(move || {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build();
        let result = match runtime {
            Ok(runtime) => runtime.block_on(async move {
                get_all_compound_models(&pool)
                    .await
                    .map(|models| {
                        models
                            .into_iter()
                            .map(|model| CompoundEntry {
                                id: model.id,
                                common_name: model.common_name,
                                iupac_name: model.iupac_name,
                                half_life_seconds: model.half_life_seconds,
                                image_path: model.image_path,
                            })
                            .collect()
                    })
                    .map_err(|error| format!("{error:?}"))
            }),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn begin_product_composition_load(
    database: Res<Database>,
    mut state: ResMut<ProductPageState>,
    results: Res<ProductCompositionLoadResults>,
) {
    let Some(ProductOverlay::Form { editing_id: Some(product_id) }) = state.overlay else {
        return;
    };
    if !state.compounds_loaded || state.composition_loading || state.composition_load_requested_id == Some(product_id) {
        return;
    }
    let generation = state.composition_load_generation;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    state.composition_loading = true;
    state.composition_load_requested_id = Some(product_id);
    bevy::tasks::IoTaskPool::get().spawn(async move {
        let result = get_product_compounds_by_id(&pool, product_id).await.map_err(|error| error.to_string());
        let _ = sender.send(result.map(|compounds| (generation, product_id, compounds)));
    }).detach();
}
