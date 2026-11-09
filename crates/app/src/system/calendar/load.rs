/* app/src/system/calendar.rs
 * The purpose of this module is to provide a calendar system.
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
use db::product::getters::get_all_product_models;
use crate::resource::calendar_resource::{
    CalendarEntries, CalendarEntryLoadResults, CalendarPageState, CalendarProductEntry, CalendarProductLoadReceiver, CalendarProducts, 
};
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::AuthenticatedAccount;
use crate::system::calendar::helper::local_day_boundary;
use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Builder;

pub fn begin_calendar_entries_load(
    database: Res<Database>,
    account: Res<AuthenticatedAccount>,
    calendar: Res<CalendarPageState>,
    mut entries: ResMut<CalendarEntries>,
    results: Res<CalendarEntryLoadResults>,
) {
    let date = calendar.selected_date;
    if entries.requested_date == Some(date) {
        return;
    }
    let Some(next_date) = date.succ_opt() else {
        entries.error = Some("Could not determine the next calendar date.".to_owned());
        return;
    };
    let (Some(start), Some(end)) = (
        local_day_boundary(date),
        local_day_boundary(next_date),
    ) else {
        entries.error = Some("Could not determine the selected day's local time range.".to_owned());
        return;
    };
    entries.requested_date = Some(date);
    entries.loaded_date = None;
    entries.entries.clear();
    entries.error = None;
    let acc_id = account.id;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime
                .block_on(
                    db::calendar::entry::getters::get_entries_by_acc_id_between(
                        &pool, acc_id, start, end,
                    ),
                )
                .map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((acc_id, date, result));
    });
}

pub fn begin_calendar_product_load(database: Res<Database>, mut products: ResMut<CalendarProducts>, receiver: Res<CalendarProductLoadReceiver>) {
    if products.loading || products.loaded {
        return;
    }
    products.loading = true;
    products.error = None;
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        products.loading = false;
        products.error = Some("Could not initialize calendar product loading.".to_string());
        return;
    }
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime.block_on(get_all_product_models(&pool)).map(|models| {
                models.into_iter().map(|model| CalendarProductEntry {
                    id: model.id,
                    name: model.name,
                    serving_size: model.serving_size,
                    serving_unit: model.serving_unit,
                }).collect::<Vec<_>>()
            }).map_err(|error| format!("{error:?}")),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}
