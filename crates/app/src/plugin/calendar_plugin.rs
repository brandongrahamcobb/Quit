/* app/src/plugin/calendar_plugin.rs
* The purpose of this module is to serve the calendar flow for the Bevy client in a plugin.
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
use crate::resource::calendar_resource::{CalendarEntries, CalendarEntryForm, CalendarEntryLoadResults, CalendarEntrySaveResults, CalendarState};
use crate::system::calendar::{apply_calendar_entry_results, apply_calendar_entry_save_results, cleanup_calendar_navigation, cycle_calendar_entry_unit, edit_calendar_entry_fields, focus_calendar_entry_field, handle_calendar_navigation, refresh_calendar_page, request_calendar_entries, save_calendar_entry, select_calendar_date, select_calendar_view, spawn_calendar_navigation, toggle_calendar_entry_form, update_calendar_entries_text, update_calendar_entry_error};

pub struct CalendarPlugin;

impl Plugin for CalendarPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<CalendarState>()
            .init_resource::<CalendarEntries>()
            .init_resource::<CalendarEntryLoadResults>()
            .init_resource::<CalendarEntryForm>()
            .init_resource::<CalendarEntrySaveResults>()
            .add_systems(
                Update,
                (
                    handle_calendar_navigation,
                    select_calendar_view,
                    select_calendar_date,
                    request_calendar_entries,
                    apply_calendar_entry_results,
                    refresh_calendar_page,
                    toggle_calendar_entry_form,
                    focus_calendar_entry_field,
                    cycle_calendar_entry_unit,
                    edit_calendar_entry_fields,
                    save_calendar_entry,
                    apply_calendar_entry_save_results,
                    update_calendar_entry_error,
                    update_calendar_entries_text,
               ).chain().run_if(in_state(AppPage::Calendar))
            )
            .add_systems(
                OnEnter(AppPage::Calendar),
                (
                    spawn_calendar_navigation
                ).chain()
            )
            .add_systems(OnExit(AppPage::Calendar), cleanup_calendar_navigation);
    }
}
