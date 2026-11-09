/* app/src/plugin/compound_plugin.rs
* The purpose of this module is to serve the compound flow for the Bevy client in a plugin.
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
use crate::resource::compound_resource::{CompoundDeleteResults, CompoundImageUploadReceiver, CompoundLoadReceiver, CompoundMetaboliteLoadResults, CompoundPageState, CompoundSaveResults};
use crate::system::compound::{apply_compound_metabolite_load_results, apply_compound_save_results, begin_compound_delete, begin_compound_image_upload, begin_compound_load, begin_compound_metabolite_load, cleanup_compound_navigation, edit_compound_fields, focus_compound_fields, handle_compound_dashboard_button, handle_compound_delete_buttons, handle_compound_form_buttons, poll_compound_delete_results, poll_compound_image_upload, poll_compound_load, remove_compound_image, save_compound, spawn_compound_navigation, toggle_compound_metabolites, toggle_compound_preview, update_compound_delete_dialog_text, update_compound_form_text, update_compound_image_preview, update_compound_list, update_compound_metabolite_buttons, update_compound_preview_image_sizes};

pub struct CompoundPlugin;

impl Plugin for CompoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<CompoundPageState>()
            .init_resource::<CompoundLoadReceiver>()
            .init_resource::<CompoundSaveResults>()
            .init_resource::<CompoundMetaboliteLoadResults>()
            .init_resource::<CompoundImageUploadReceiver>()
            .init_resource::<CompoundDeleteResults>()
            .add_systems(
                Update,
                (
                    poll_compound_load,
                    toggle_compound_preview,
                    update_compound_list,
                    update_compound_metabolite_buttons,
                    handle_compound_form_buttons,
                    begin_compound_metabolite_load,
                    focus_compound_fields,
                    edit_compound_fields,
                    toggle_compound_metabolites,
                    update_compound_form_text,
                    handle_compound_dashboard_button,
                )
                    .chain()
                    .run_if(in_state(AppPage::Compounds)),
            )
            .add_systems(
                Update,
                (
                    save_compound,
                    apply_compound_save_results,
                    apply_compound_metabolite_load_results,
                )
                    .chain()
                    .run_if(in_state(AppPage::Compounds)),
            )
            .add_systems(
                Update,
                (
                    begin_compound_image_upload,
                    poll_compound_image_upload,
                    update_compound_image_preview,
                    update_compound_preview_image_sizes,
                    remove_compound_image,
                    handle_compound_delete_buttons,
                    begin_compound_delete,
                    poll_compound_delete_results,
                    update_compound_delete_dialog_text,
                )
                    .chain()
                    .run_if(in_state(AppPage::Compounds)),
            )
            .add_systems(
                OnEnter(AppPage::Compounds),
                (
                    begin_compound_load,
                    spawn_compound_navigation,
                ).chain()
            );
    }
}
