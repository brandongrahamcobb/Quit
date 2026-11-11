/* app/src/plugin/product_plugin.rs
* The purpose of this module is to serve the product flow for the Bevy client in a plugin.
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
use crate::resource::product_resource::{ProductCompositionLoadResults, ProductCompoundLoadResults, ProductDeleteResults, ProductDeletionState, ProductFormState, ProductImageUploadReceiver, ProductLoadReceiver, ProductPageState, ProductSaveResults};
// use crate::system::product::activate::add_edit::detect_product_form_open;
use crate::system::product::activate::origin::handle_product_escape;
use crate::system::product::load::{begin_product_composition_load, begin_product_compound_load, begin_product_load};
use crate::system::product::poll::{poll_product_composition_load_results, poll_product_compound_load_results, poll_product_delete_results, poll_product_image_upload, poll_product_load, poll_product_save_results};
use crate::system::product::spawn::spawn_product_navigation;
use crate::system::product::update::{update_product_composition_rows, update_product_compound_buttons, update_product_delete_dialog_text, update_product_dropdowns, update_product_fields, update_product_list, update_product_overlays, update_product_preview, update_product_save_ui};

pub struct ProductPlugin;

impl Plugin for ProductPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<ProductPageState>()
            .init_resource::<ProductFormState>()
            .init_resource::<ProductDeletionState>()
            .init_resource::<ProductLoadReceiver>()
            .init_resource::<ProductSaveResults>()
            .init_resource::<ProductImageUploadReceiver>()
            .init_resource::<ProductDeleteResults>()
            .init_resource::<ProductCompoundLoadResults>()
            .init_resource::<ProductCompositionLoadResults>()
            .add_systems(
                OnEnter(AppPage::Products),
                (
                    begin_product_load,
                ).chain(),
            )
            .add_systems(
                Update,
                (
                    begin_product_compound_load,
                    begin_product_composition_load,
                ).chain()
                .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                Update,
                (
                    handle_product_escape,
                )
                    .chain()
                    .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                Update,
                (
                    poll_product_load,
                    poll_product_compound_load_results,
                    poll_product_composition_load_results,
                    poll_product_save_results,
                    poll_product_image_upload,
                    poll_product_delete_results,
                )
            )
            .add_systems(
                Update,
                (
                    update_product_fields,
                    update_product_overlays,
                    update_product_preview,
                    update_product_list,
                    update_product_compound_buttons,
                    update_product_composition_rows,
                    update_product_save_ui,
                    update_product_delete_dialog_text,
                    update_product_dropdowns
                )
                    .chain()
                    .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                OnEnter(AppPage::Products), spawn_product_navigation
            );
 
    }
}
