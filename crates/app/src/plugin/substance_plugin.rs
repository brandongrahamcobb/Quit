/* app/src/plugin/substance_plugin.rs
* The purpose of this module is to serve the substance flow for the Bevy client in a plugin.
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
use crate::resource::substance_resource::{SubstanceEntries, SubstanceEntryLoadResults, SubstanceEntrySaveResults, SubstanceForm};
use crate::system::substance::{apply_substance_entry_results, apply_substance_entry_save_results, cycle_substance_kind, edit_substance_name, focus_substance_name_field, open_substances, request_substance_entries, save_substance, spawn_substance_navigation, toggle_substance_form, update_substance_text};

pub struct SubstancePlugin;

impl Plugin for SubstancePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<SubstanceEntries>()
            .init_resource::<SubstanceEntryLoadResults>()
            .init_resource::<SubstanceEntrySaveResults>()
            .init_resource::<SubstanceForm>()
            .add_systems(Update, open_substances)
            .add_systems(
                Update,
                (
                    request_substance_entries,
                    apply_substance_entry_results,
                    toggle_substance_form,
                    focus_substance_name_field,
                    cycle_substance_kind,
                    edit_substance_name,
                    save_substance,
                    apply_substance_entry_save_results,
                    update_substance_text,
                )
                    .chain()
                    .run_if(in_state(AppPage::Substance)),
            )
            .add_systems(
                OnEnter(AppPage::Substance),
                (
                    spawn_substance_navigation,
                ).chain()
            );
            // .add_systems(OnExit(AppPage::Substance), cleanup_calendar_navigation);
    }
}
