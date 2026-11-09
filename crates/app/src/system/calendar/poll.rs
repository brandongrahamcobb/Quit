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
use chrono::Local;
use crate::component::entry::{CalendarEntryModal, CalendarEntryProductButton, CalendarEntryProductList};
use crate::resource::calendar_resource::{
    CalendarEntries, CalendarEntryForm, CalendarEntryLoadResults, CalendarEntrySaveResults, CalendarProductLoadReceiver, CalendarProducts,
};
use crate::resource::login_resource::AuthenticatedAccount;
use std::sync::mpsc::TryRecvError;

pub fn poll_calendar_entry_load_results(
    results: Res<CalendarEntryLoadResults>,
    account: Res<AuthenticatedAccount>,
    mut calendar_entries: ResMut<CalendarEntries>,
) {
    let receiver = match results.receiver.lock() {
        Ok(receiver) => receiver,
        Err(error) => {
            calendar_entries.error = Some(error.to_string());
            return;
        }
    };
    loop {
        match receiver.try_recv() {
            Ok((acc_id, date, result)) => {
                if acc_id != account.id
                    || calendar_entries.requested_date != Some(date)
                {
                    continue;
                }
                match result {
                    Ok(entries) => {
                        calendar_entries.loaded_date = Some(date);
                        calendar_entries.entries = entries;
                        calendar_entries.error = None;
                    }
                    Err(error) => {
                        calendar_entries.error = Some(error);
                    }
                }
            }
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }
}

pub fn poll_calendar_entry_save_results(
    results: Res<CalendarEntrySaveResults>,
    account: Res<AuthenticatedAccount>,
    mut form: ResMut<CalendarEntryForm>,
    mut entries: ResMut<CalendarEntries>,
    mut lists: Query<&mut Node, (With<CalendarEntryProductList>, Without<CalendarEntryModal>)>,
    mut modals: Query<&mut Node, (With<CalendarEntryModal>, Without<CalendarEntryProductList>)>,
    mut product_buttons: Query<&mut Text, With<CalendarEntryProductButton>>,
) {
    let receiver = match results.receiver.lock() {
        Ok(receiver) => receiver,
        Err(error) => {
            form.error = Some(error.to_string());
            return;
        }
    };
    loop {
        match receiver.try_recv() {
            Ok((acc_id, _date, result)) if acc_id == account.id => match result {
                Ok(_) => {
                    form.open = false;
                    form.product_id = None;
                    form.product_selector_open = false;
                    form.time = Local::now().format("%H:%M").to_string();
                    form.amount = "1".to_string();
                    form.amount_unit = "unit".to_string();
                    form.error = None;
                    entries.requested_date = None;
                    for mut node in &mut modals {
                        node.display = Display::None;
                    }
                    for mut node in &mut lists {
                        node.display = Display::None;
                    }
                    for mut text in &mut product_buttons {
                        **text = "Product: Select product".to_string();
                    }
                }
                Err(error) => {
                    form.error = Some(error);
                }
            },
            Ok(_) => {}
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }
}

pub fn poll_calendar_product_load(receiver: Res<CalendarProductLoadReceiver>, mut products: ResMut<CalendarProducts>) {
    let result = {
        let Ok(mut slot) = receiver.0.lock() else {
            return;
        };
        let Some(result_receiver) = slot.as_ref() else {
            return;
        };
        match result_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Calendar product loading worker disconnected.".to_string()))
            }
        }
    };
    let Some(result) = result else {
        return;
    };
    products.loading = false;
    match result {
        Ok(loaded_products) => {
            products.products = loaded_products;
            products.loaded = true;
            products.error = None;
        }
        Err(error) => {
            products.error = Some(error);
        }
    }
}
