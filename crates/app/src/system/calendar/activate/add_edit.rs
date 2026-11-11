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


use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::ui_widgets::Activate;
use bigdecimal::BigDecimal;
use chrono::{Local, NaiveTime};
use crate::component::calendar::{
    CalendarEntryAmountField, CalendarEntryAmountUnitButton, 
};
use crate::component::entry::{CalendarAddEntryButton, CalendarEntryCloseButton, CalendarEntryModal, CalendarEntryProductButton, CalendarEntryProductList, CalendarEntryProductOption, CalendarEntrySaveButton, CalendarEntryTimeField};
use crate::resource::calendar_resource::{
    CalendarEntryForm, CalendarEntrySaveResults, CalendarProducts, CalendarPageState, EntryFormField, 
};
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::AuthenticatedAccount;
use std::thread;
use std::time::SystemTime;
use tokio::runtime::Builder;

// pub fn edit_calendar_entry_fields(
//     mut amount_fields: Query<&mut Text, With<CalendarEntryAmountField>>,
//     mut amount_unit_buttons: Query<&mut Text, (With<CalendarEntryAmountUnitButton>, Without<CalendarEntryAmountField>)>,
//     mut keyboard: MessageReader<KeyboardInput>,
//     mut form: ResMut<CalendarEntryForm>,
//     mut time_fields: Query<&mut Text, (With<CalendarEntryTimeField>, Without<CalendarEntryAmountField>, Without<CalendarEntryAmountUnitButton>)>,
// ) {
//     if form.open {
//         for input in keyboard.read() {
//             if input.state != ButtonState::Pressed {
//                 continue;
//             }
//             match input.key_code {
//                 KeyCode::Backspace => {
//                     match form.focused {
//                         EntryFormField::Amount => { form.amount.pop(); }
//                         EntryFormField::Time => { form.time.pop(); }
//                         EntryFormField::Product => {}
//                     }
//                 }
//                 KeyCode::Tab => {
//                     form.focused = match form.focused {
//                         EntryFormField::Amount => EntryFormField::Time,
//                         _ => EntryFormField::Amount,
//                     };
//                 }
//                 _ => {
//                     let Some(text) = input.text.as_deref() else {
//                         continue;
//                     };
//                     match form.focused {
//                         EntryFormField::Amount => {
//                             form.amount.extend(text.chars().filter(|character| character.is_ascii_digit() || *character == '.'));
//                         }
//                         EntryFormField::Time => {
//                             form.time.extend(text.chars().filter(|character| character.is_ascii_digit() || *character == ':'));
//                         }
//                         EntryFormField::Product => {}
//                     }
//                 }
//             }
//         }
//     }
//     for mut text in &mut amount_fields {
//         **text = format!("Amount: {}", form.amount);
//     }
//     for mut text in &mut amount_unit_buttons {
//         **text = format!("Unit: {}", form.amount_unit);
//     }
//     for mut text in &mut time_fields {
//         **text = format!("Time: {}", form.time);
//     }
// }
//
// pub fn save_calendar_entry(
//     _activate: On<Activate>,
//     mut form: ResMut<CalendarEntryForm>,
//     calendar: Res<CalendarPageState>,
//     account: Res<AuthenticatedAccount>,
//     database: Res<Database>,
//     results: Res<CalendarEntrySaveResults>,
// ) {
//     if !form.open || !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
//         return;
//     }
//     let Some(product_id) = form.product_id else {
//         form.error = Some("Select a product.".to_owned());
//         return;
//     };
//     let Ok(amount) = form.amount.trim().parse::<BigDecimal>() else {
//         form.error = Some("Enter a valid amount.".to_owned());
//         return;
//     };
//     if amount <= BigDecimal::from(0) {
//         form.error = Some("Amount must be greater than zero.".to_owned());
//         return;
//     }
//     if !["mg", "mL", "unit"].contains(&form.amount_unit.as_str()) {
//         form.error = Some("Select a valid amount unit.".to_owned());
//         return;
//     }
//     let Ok(time) = NaiveTime::parse_from_str(&form.time, "%H:%M") else {
//         form.error = Some("Enter a time in HH:MM format.".to_owned());
//         return;
//     };
//     let Some(occurred_at) = calendar.selected_date.and_time(time).and_local_timezone(Local).earliest().map(SystemTime::from) else {
//         form.error = Some("That local time does not exist.".to_owned());
//         return;
//     };
//     let acc_id = account.id;
//     let date = calendar.selected_date;
//     let new_entry = db::calendar::entry::model::NewEntryModel {
//         acc_id,
//         occurred_at,
//         product_id,
//         amount,
//         amount_unit: form.amount_unit.clone(),
//     };
//     let pool = database.0.clone();
//     let sender = results.sender.clone();
//     form.error = None;
//     thread::spawn(move || {
//         let result = match Builder::new_current_thread().enable_all().build() {
//             Ok(runtime) => runtime.block_on(db::calendar::entry::setters::insert_entry(&pool, new_entry)).map_err(|error| error.to_string()),
//             Err(error) => Err(error.to_string()),
//         };
//         let _ = sender.send((acc_id, date, result));
//     });
// }
//
// pub fn toggle_calendar_entry_form(
//     open_buttons: Query<&Interaction, (Changed<Interaction>, With<CalendarAddEntryButton>)>,
//     close_buttons: Query<&Interaction, (Changed<Interaction>, With<CalendarEntryCloseButton>)>,
//     mut form: ResMut<CalendarEntryForm>,
//     mut lists: Query<&mut Node, (With<CalendarEntryProductList>, Without<CalendarEntryModal>)>,
//     mut modals: Query<&mut Node, (With<CalendarEntryModal>, Without<CalendarEntryProductList>)>,
//     mut product_buttons: Query<&mut Text, With<CalendarEntryProductButton>>,
// ) {
//     let opening = open_buttons.iter().any(|interaction| *interaction == Interaction::Pressed);
//     let closing = close_buttons.iter().any(|interaction| *interaction == Interaction::Pressed);
//     if opening {
//         form.open = true;
//         form.error = None;
//         form.product_id = None;
//         form.product_selector_open = false;
//         form.time = Local::now().format("%H:%M").to_string();
//         for mut text in &mut product_buttons {
//             **text = "Product: Select product".to_string();
//         }
//     }
//     if closing {
//         form.open = false;
//         form.product_selector_open = false;
//     }
//     for mut node in &mut modals {
//         node.display = if form.open { Display::Flex } else { Display::None };
//     }
//     for mut node in &mut lists {
//         node.display = if form.product_selector_open { Display::Flex } else { Display::None };
//     }
// }
//
// pub fn add_entry(
//     mut form: ResMut<CalendarEntryForm>,
//     mut lists: Query<&mut Node, (With<CalendarEntryProductList>, Without<CalendarEntryModal>)>,
//     mut modals: Query<&mut Node, (With<CalendarEntryModal>, Without<CalendarEntryProductList>)>,
//     mut product_buttons: Query<&mut Text, With<CalendarEntryProductButton>>,
// ) {
//     if opening {
//         form.open = true;
//         form.error = None;
//         form.product_id = None;
//         form.product_selector_open = false;
//         form.time = Local::now().format("%H:%M").to_string();
//         for mut text in &mut product_buttons {
//             **text = "Product: Select product".to_string();
//         }
//     }
//     if closing {
//         form.open = false;
//         form.product_selector_open = false;
//     }
//     for mut node in &mut modals {
//         node.display = if form.open { Display::Flex } else { Display::None };
//     }
//     for mut node in &mut lists {
//         node.display = if form.product_selector_open { Display::Flex } else { Display::None };
//     }
// }
//
// pub fn select_calendar_product(
//     mut form: ResMut<CalendarEntryForm>,
//     products: Res<CalendarProducts>,
//     mut buttons: Query<(&mut Text, &mut BackgroundColor), With<CalendarEntryProductButton>>,
//     options: Query<(&Interaction, &CalendarEntryProductOption), Changed<Interaction>>,
// ) {
//     for (interaction, option) in &options {
//         if *interaction != Interaction::Pressed {
//             continue;
//         }
//         let Some(product) = products.products.iter().find(|product| product.id == option.0) else {
//             continue;
//         };
//         form.amount = product.serving_size.to_string();
//         form.amount_unit = product.serving_unit.clone();
//         form.product_id = Some(product.id);
//         form.product_selector_open = false;
//         form.error = None;
//         for (mut text, mut background) in &mut buttons {
//             **text = format!("Product: {}", product.name);
//             *background = BackgroundColor(Color::srgb(0.07, 0.09, 0.10));
//         }
//     }
// }
//
//
// pub fn toggle_calendar_product_selector(
//     mut form: ResMut<CalendarEntryForm>,
//     mut lists: Query<&mut Node, With<CalendarEntryProductList>>,
//     buttons: Query<&Interaction, (Changed<Interaction>, With<CalendarEntryProductButton>)>,
// ) {
//     if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
//         return;
//     }
//     let Ok(mut node) = lists.single_mut() else {
//         return;
//     };
//     form.product_selector_open = !form.product_selector_open;
//     node.display = if form.product_selector_open { Display::Flex } else { Display::None };
// }

