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
use crate::resource::product_resource::{ProductCompositionLoadResults, ProductCompoundLoadResults, ProductDeleteResults, ProductImageUploadReceiver, ProductLoadReceiver, ProductPageState, ProductSaveResults};
use crate::system::product::{apply_product_composition_load_results, apply_product_compound_load_results, apply_product_save_results, begin_product_composition_load, begin_product_delete, begin_product_image_upload, begin_product_load, cleanup_product_navigation, cycle_product_compound_units, detect_product_form_open, edit_product_fields, focus_product_fields, handle_product_dashboard_button, handle_product_delete_buttons, handle_product_form_buttons, poll_product_delete_results, poll_product_image_upload, poll_product_load, remove_product_compound, remove_product_image, save_product, select_product_route, spawn_product_navigation, toggle_product_composition, toggle_product_preview, update_product_composition_rows, update_product_compound_buttons, update_product_delete_dialog_text, update_product_form_text, update_product_image_preview, update_product_list, update_product_preview_image_sizes, update_product_route_options, update_product_save_ui};

pub struct ProductPlugin;

impl Plugin for ProductPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<ProductPageState>()
            .init_resource::<ProductLoadReceiver>()
            .init_resource::<ProductSaveResults>()
            .init_resource::<ProductImageUploadReceiver>()
            .init_resource::<ProductDeleteResults>()
            .init_resource::<ProductCompoundLoadResults>()
            .init_resource::<ProductCompositionLoadResults>()
            .add_systems(
                Update,
                (
                    poll_product_load,
                    update_product_list,
                    toggle_product_preview,
                    handle_product_form_buttons,
                    detect_product_form_open,
                )
                    .chain()
                    .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                Update,
                (
                    apply_product_compound_load_results,
                    update_product_route_options,
                    begin_product_composition_load,
                    apply_product_composition_load_results,
                    update_product_compound_buttons,
                    update_product_composition_rows,
                    toggle_product_composition,
                    cycle_product_compound_units,
                    remove_product_compound,
                    select_product_route,
                )
                    .chain()
                    .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                Update,
                (
                    focus_product_fields,
                    edit_product_fields,
                    update_product_form_text,
                    save_product,
                    apply_product_save_results,
                    update_product_save_ui,
                    handle_product_dashboard_button,
                )
                    .chain()
                    .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                Update,
                (
                    begin_product_image_upload,
                    poll_product_image_upload,
                    update_product_image_preview,
                    update_product_preview_image_sizes,
                    remove_product_image,
                    handle_product_delete_buttons,
                    begin_product_delete,
                    poll_product_delete_results,
                    update_product_delete_dialog_text,
                )
                    .chain()
                    .run_if(in_state(AppPage::Products)),
            )
            .add_systems(
                OnEnter(AppPage::Products),
                (begin_product_load, spawn_product_navigation).chain(),
            )
            .add_systems(OnExit(AppPage::Products), cleanup_product_navigation);
    }
}
