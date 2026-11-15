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
use crate::resource::calendar_resource::{
    CalendarEntryLoadReceiver, CalendarFormState, CalendarPageState,
};
use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::product_resource::ProductLoadReceiver;
use std::sync::mpsc::TryRecvError;

pub fn poll_calendar_entry_load_results(
    receiver: Res<CalendarEntryLoadReceiver>,
    account: Res<AuthenticatedAccount>,
    mut page_state: ResMut<CalendarPageState>
) {
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
                Some((account.0, page_state.selected_date, Err("Calendar entry loading worker disconnected.".to_string())))
            }
        }
    };
    let Some((acc_id, date, result)) = result else {
        return;
    };
    if acc_id != account.0 || date != page_state.selected_date {
        return;
    }
    page_state.loading_entries = true;
    match result {
        Ok(entries) => {
            page_state.entries = entries;
            page_state.error = None;
        }
        Err(error) => {
            page_state.error = Some(error);
        }
    }
}

pub fn poll_calendar_products_load(receiver: Res<ProductLoadReceiver>, mut form_state: ResMut<CalendarFormState>) {
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
                Some(Err("Product loading worker disconnected.".to_string()))
            }
        }
    };
    let Some(result) = result else {
        return;
    };
    form_state.loading_products = false;
    match result {
        Ok(products) => {
            form_state.products = products;
            form_state.error = None;
        }
        Err(error) => {
            form_state.error = Some(error);
        }
    }
}

// pub fn poll_product_delete_results(
//     results: ResMut<ProductDeleteResults>,
//     mut page_state: ResMut<ProductPageState>,
//     mut deletion_state: ResMut<ProductDeletionState>,
// ) {
//     let result = {
//         let receiver = results.0.lock().unwrap();
//         receiver.as_ref().and_then(|receiver| receiver.try_recv().ok())
//     };
//     let Some((id, result)) = result else {
//         return;
//     };
//     *results.0.lock().unwrap() = None;
//     deletion_state.deleting = false;
//     match result {
//         Ok(()) => {
//             page_state.products.retain(|product| product.id != id);
//             if page_state.overlay == Some(ProductOverlay::Preview(id)) || page_state.overlay == Some(ProductOverlay::Delete(id)) {
//                 page_state.overlay = None;
//             }
//             deletion_state.name = None;
//             deletion_state.error = None;
//         }
//         Err(error) => {
//             deletion_state.error = Some(error);
//         }
//     }
// }

// pub fn poll_calendar_form_save_results(
//     results: Res<CalendarEntrySaveResults>,
//     account: Res<AuthenticatedAccount>,
//     mut form: ResMut<CalendarFormState>,
//     mut entries: ResMut<CalendarEntries>,
//     mut lists: Query<&mut Node, (With<CalendarEntryProductList>, Without<CalendarEntryModal>)>,
//     mut modals: Query<&mut Node, (With<CalendarEntryModal>, Without<CalendarEntryProductList>)>,
//     mut product_buttons: Query<&mut Text, With<CalendarEntryProductButton>>,
// ) {
//     let receiver = match results.receiver.lock() {
//         Ok(receiver) => receiver,
//         Err(error) => {
//             form.error = Some(error.to_string());
//             return;
//         }
//     };
//     loop {
//         match receiver.try_recv() {
//             Ok((acc_id, _date, result)) if acc_id == account.id => match result {
//                 Ok(_) => {
//                     form.open = false;
//                     form.product_id = None;
//                     form.product_selector_open = false;
//                     form.time = Local::now().format("%H:%M").to_string();
//                     form.amount = "1".to_string();
//                     form.amount_unit = "unit".to_string();
//                     form.error = None;
//                     entries.requested_date = None;
//                     for mut node in &mut modals {
//                         node.display = Display::None;
//                     }
//                     for mut node in &mut lists {
//                         node.display = Display::None;
//                     }
//                     for mut text in &mut product_buttons {
//                         **text = "Product: Select product".to_string();
//                     }
//                 }
//                 Err(error) => {
//                     form.error = Some(error);
//                 }
//             },
//             Ok(_) => {}
//             Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
//         }
//     }
// }
