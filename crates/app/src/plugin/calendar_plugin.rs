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
use crate::resource::calendar_resource::{CalendarEntries, CalendarEntryForm, CalendarEntryLoadResults, CalendarEntrySaveResults, CalendarProductLoadReceiver, CalendarProducts, CalendarPageState};
use crate::system::calendar::activate::origin::handle_calendar_escape;
use crate::system::calendar::load::{begin_calendar_entries_load, begin_calendar_product_load};
use crate::system::calendar::poll::{poll_calendar_entry_load_results, poll_calendar_entry_save_results, poll_calendar_product_load};
use crate::system::calendar::spawn::spawn_calendar_navigation;
use crate::system::calendar::update::{update_calendar_entries_text, update_calendar_entry_error, update_calendar_page, update_calendar_period_title, update_calendar_product_options};

pub struct CalendarPlugin;

impl Plugin for CalendarPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<CalendarPageState>()
            .init_resource::<CalendarEntries>()
            .init_resource::<CalendarEntryLoadResults>()
            .init_resource::<CalendarEntryForm>()
            .init_resource::<CalendarEntrySaveResults>()
            .init_resource::<CalendarProducts>()
            .init_resource::<CalendarProductLoadReceiver>()
            .add_systems(
                OnEnter(AppPage::Calendar),
                (
                    begin_calendar_product_load,
                    begin_calendar_entries_load
                )
            )
            .add_systems(
                Update,
                (
                    handle_calendar_escape,
                )
                    .chain()
                    .run_if(in_state(AppPage::Calendar)),
            )
            .add_systems(
                Update,
                (
                    poll_calendar_product_load,
                    poll_calendar_entry_load_results,
                    poll_calendar_entry_save_results,
                )
                .chain().run_if(in_state(AppPage::Calendar))
            )
            .add_systems(
                Update,
                (
                    update_calendar_entries_text,
                    update_calendar_product_options,
                    update_calendar_entry_error,
                    update_calendar_period_title,
                    update_calendar_page 
                )
                .chain().run_if(in_state(AppPage::Calendar)),
            )
            .add_systems(
                OnEnter(AppPage::Calendar),
                spawn_calendar_navigation,
            );
    }
}
