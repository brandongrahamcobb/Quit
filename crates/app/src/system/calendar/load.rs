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
    CalendarEntryLoadReceiver, CalendarFormState, CalendarPageState,
};
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::product_resource::{ProductEntry, ProductLoadReceiver};
use crate::system::calendar::helper::local_day_boundary;
use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Builder;


pub fn begin_calendar_entries_load(
    database: Res<Database>,
    account: Res<AuthenticatedAccount>,
    mut calendar: ResMut<CalendarPageState>,
    receiver: Res<CalendarEntryLoadReceiver>,
) {
    let date = calendar.selected_date;
    let Some(next_date) = date.succ_opt() else {
        calendar.error = Some("Could not determine the next calendar date.".to_owned());
        return;
    };
    let (Some(start), Some(end)) = (
        local_day_boundary(date),
        local_day_boundary(next_date),
    ) else {
        calendar.error = Some("Could not determine the selected day's local time range.".to_owned());
        return;
    };
    calendar.entries.clear();
    calendar.error = None;
    calendar.loading_entries = true;
    let acc_id = account.id;
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        calendar.loading_entries = false;
        calendar.error = Some("Could not initialize calendar entry loading.".to_string());
        return;
    }
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

pub fn begin_calendar_product_load(database: Res<Database>, receiver: Res<ProductLoadReceiver>, mut form_state: ResMut<CalendarFormState>) {
    if form_state.loading_products {
        return;
    }
    form_state.loading_products = true;
    form_state.error = None;
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        form_state.loading_products = false;
        form_state.error = Some("Could not initialize calendar product loading.".to_string());
        return;
    }
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime
                .block_on(get_all_product_models(&pool))
                .map(|models| {
                    models
                        .into_iter()
                        .map(|model| ProductEntry {
                            id: model.id,
                            name: model.name,
                            route: model.route,
                            brand: model.brand,
                            description: model.description,
                            image_path: model.image_path,
                            serving_size: model.serving_size,
                            serving_unit: model.serving_unit,
                            serving_description: model.serving_description,
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|error| format!("{error:?}")),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}
