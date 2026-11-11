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
use crate::resource::product_resource::{ProductDeletionState, ProductFormState, ProductOverlay, ProductPageState};


pub fn handle_product_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_page: ResMut<NextState<AppPage>>,
    mut page_state: ResMut<ProductPageState>,
    mut form_state: ResMut<ProductFormState>,
    mut deletion_state: ResMut<ProductDeletionState>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    match page_state.overlay {
        Some(ProductOverlay::Preview(_)) => {
            page_state.overlay = None;
        }
        Some(ProductOverlay::Delete(_)) => {
            if deletion_state.deleting {
                return;
            }
            page_state.overlay = None;
            deletion_state.name = None;
            deletion_state.error = None;
        }
        Some(ProductOverlay::Form { .. }) => {
            if form_state.loading_compounds || form_state.loading_composition || form_state.saving {
                return;
            }
            page_state.overlay = None;
            form_state.error = None;
        }
        None => {
            next_page.set(AppPage::Dashboard);
        }
    }
}
