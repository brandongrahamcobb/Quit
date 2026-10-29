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

use crate::resource::substance_resource::SubstanceState;
use crate::page::AppPage;
use crate::system::substance::spawn_substance_navigation;

pub struct SubstancePlugin;

impl Plugin for SubstancePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<SubstanceState>()
            // .init_resource::<SubstanceEntriens>()
            // .init_resource::<SubstanceEntryLoadResults>()
            // .init_resource::<SubstanceEntryForm>()
            // .init_resource::<SubstanceEntrySaveResults>()
            // .add_systems(
            //     Update,
            //     (
            //         handle_calendar_navigation,
            //         select_calendar_view,
            //         select_calendar_date,
            //         request_calendar_entries,
            //         apply_calendar_entry_results,
            //         refresh_calendar_page,
            //         toggle_calendar_entry_form,
            //         focus_calendar_entry_field,
            //         cycle_calendar_entry_unit,
            //         edit_calendar_entry_fields,
            //         save_calendar_entry,
            //         apply_calendar_entry_save_results,
            //         update_calendar_entry_error,
            //         update_calendar_entries_text,
            //    ).chain().run_if(in_state(AppPage::Substance))
            // )
            .add_systems(
                OnEnter(AppPage::Substance),
                (
                    spawn_substance_navigation,
                ).chain()
            );
            // .add_systems(OnExit(AppPage::Substance), cleanup_calendar_navigation);
    }
}
