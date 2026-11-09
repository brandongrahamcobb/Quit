/* app/src/system/product/activate/origin.rs
 * The purpose of this module is to provide a root page observer for the product system.
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
use crate::page::AppPage;
use crate::resource::product_resource::{ProductOverlay, ProductPageState};


pub fn handle_product_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_page: ResMut<NextState<AppPage>>,
    mut state: ResMut<ProductPageState>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    match state.overlay {
        Some(ProductOverlay::Preview(_)) => {
            state.overlay = None;
        }
        Some(ProductOverlay::Delete(_)) => {
            if state.deleting {
                return;
            }
            state.overlay = None;
            state.deleting_name = None;
            state.delete_error = None;
        }
        Some(ProductOverlay::Form { .. }) => {
            state.composition_load_generation = state.composition_load_generation.wrapping_add(1);
            state.composition_loading = false;
            state.composition_load_requested_id = None;
            state.overlay = None;
            state.error = None;
        }
        None => {
            next_page.set(AppPage::Dashboard);
        }
    }
}
