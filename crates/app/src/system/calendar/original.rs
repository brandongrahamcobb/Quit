// /* app/src/system/calendar.rs
//  * The purpose of this module is to provide a calendar system.
//  *
//  * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.git
//  *
//  * This program is free software: you can redistribute it and/or modify
//  * it under the terms of the GNU Affero General Public License as published by
//  * the Free Software Foundation, either version 3 of the License, or
//  * (at your option) any later version.
//  *
//  * This program is distributed in the hope that it will be useful,
//  * but WITHOUT ANY WARRANTY; without even the implied warranty of
//  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  * GNU Affero General Public License for more details.
//  *
//  * You should have received a copy of the GNU Affero General Public License
//  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
//  */
//
//
// use bevy::input::ButtonState;
// use bevy::input::keyboard::KeyboardInput;
// use bevy::prelude::*;
// use bigdecimal::BigDecimal;
// use chrono::{Datelike, Days, Local, NaiveTime, Timelike};
// use db::product::getters::get_all_product_models;
// use crate::component::calendar::{
//     CalendarBackButton, CalendarDateCell, CalendarDayViewButton, CalendarEntryAmountField, CalendarEntryAmountUnitButton, CalendarMonthViewButton, CalendarNavigationRoot, CalendarNextButton, CalendarPeriodTitle, CalendarPreviousButton, CalendarTodayButton, CalendarWeekViewButton, OpenCalendarButton,
// };
// use crate::component::entry::{CalendarAddEntryButton, CalendarEntriesText, CalendarEntryCloseButton, CalendarEntryErrorText, CalendarEntryModal, CalendarEntryProductButton, CalendarEntryProductList, CalendarEntryProductOption, CalendarEntrySaveButton, CalendarEntryTimeField};
// use crate::resource::calendar_resource::{
//     CalendarEntries, CalendarEntryForm, CalendarEntryLoadResults, CalendarEntrySaveResults, CalendarProductEntry, CalendarProductLoadReceiver, CalendarProducts, CalendarState, CalendarView, EntryFormField, 
// };
// use crate::resource::custom_resource::Database;
// use crate::resource::login_resource::AuthenticatedAccount;
// use crate::page::AppPage;
// use std::sync::mpsc::{TryRecvError, channel};
// use std::thread;
// use std::time::SystemTime;
// use tokio::runtime::Builder;
//
// pub fn handle_calendar_navigation(
//     mut calendar: ResMut<CalendarState>,
//     previous_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarPreviousButton>),
//     >,
//     today_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarTodayButton>),
//     >,
//     next_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarNextButton>),
//     >,
//     back_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarBackButton>),
//     >,
//     mut next_page: ResMut<NextState<AppPage>>,
// ) {
//     if previous_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         calendar.move_previous();
//     }
//     if today_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         calendar.go_to_today();
//     }
//     if next_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         calendar.move_next();
//     }
//     if back_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         next_page.set(AppPage::Dashboard);
//     }
// }
//
// pub fn open_calendar(
//     buttons: Query<&Interaction, (Changed<Interaction>, With<OpenCalendarButton>)>,
//     mut next_page: ResMut<NextState<AppPage>>,
// ) {
//     if buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         next_page.set(AppPage::Calendar);
//     }
// }
//
// pub fn cleanup_calendar_navigation(
//     mut commands: Commands,
//     roots: Query<Entity, With<CalendarNavigationRoot>>,
// ) {
//     for entity in &roots {
//         commands.entity(entity).despawn();
//     }
// }
//
// fn calendar_period_label(calendar: &CalendarState) -> String {
//     match calendar.view {
//         CalendarView::Day => calendar
//             .visible_date
//             .format("%A, %B %e, %Y")
//             .to_string(),
//         CalendarView::Week => {
//             let start = calendar
//                 .visible_date
//                 .checked_sub_days(Days::new(
//                     calendar.visible_date.weekday().num_days_from_monday() as u64,
//                 ))
//                 .unwrap_or(calendar.visible_date);
//             let end = start
//                 .checked_add_days(Days::new(6))
//                 .unwrap_or(start);
//             format!("{} - {}", start.format("%b %e"), end.format("%b %e, %Y"))
//         }
//         CalendarView::Month => calendar.visible_date.format("%B %Y").to_string(),
//     }
// }
//
// pub fn update_calendar_period_title(
//     calendar: Res<CalendarState>,
//     mut titles: Query<&mut Text, With<CalendarPeriodTitle>>,
// ) {
//     if !calendar.is_changed() {
//         return;
//     }
//     let label = calendar_period_label(&calendar);
//     for mut title in &mut titles {
//         title.0 = label.clone();
//     }
// }
//
// pub fn select_calendar_view(
//     mut calendar: ResMut<CalendarState>,
//     day_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarDayViewButton>),
//     >,
//     week_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarWeekViewButton>),
//     >,
//     month_buttons: Query<
//         &Interaction,
//         (Changed<Interaction>, With<CalendarMonthViewButton>),
//     >,
// ) {
//     if day_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         calendar.view = CalendarView::Day;
//     }
//     if week_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         calendar.view = CalendarView::Week;
//     }
//     if month_buttons
//         .iter()
//         .any(|interaction| *interaction == Interaction::Pressed)
//     {
//         calendar.view = CalendarView::Month;
//     }
// }
//
// pub fn refresh_calendar_page(
//     mut commands: Commands,
//     calendar: Res<CalendarState>,
//     roots: Query<Entity, With<CalendarNavigationRoot>>,
// ) {
//     if !calendar.is_changed() {
//         return;
//     }
//     for root in &roots {
//         commands.entity(root).despawn();
//     }
//     spawn_calendar_navigation(commands, calendar);
// }
//
// pub fn select_calendar_date(
//     mut calendar: ResMut<CalendarState>,
//     cells: Query<
//         (&Interaction, &CalendarDateCell),
//         Changed<Interaction>,
//     >,
// ) {
//     for (interaction, cell) in &cells {
//         if *interaction == Interaction::Pressed {
//             calendar.select_date(cell.date);
//         }
//     }
// }
//
// fn local_day_boundary(date: chrono::NaiveDate) -> Option<SystemTime> {
//     date.and_hms_opt(0, 0, 0)?
//         .and_local_timezone(Local)
//         .earliest()
//         .map(SystemTime::from)
// }
//
// pub fn request_calendar_entries(
//     database: Res<Database>,
//     account: Res<AuthenticatedAccount>,
//     calendar: Res<CalendarState>,
//     mut entries: ResMut<CalendarEntries>,
//     results: Res<CalendarEntryLoadResults>,
// ) {
//     let date = calendar.selected_date;
//     if entries.requested_date == Some(date) {
//         return;
//     }
//     let Some(next_date) = date.succ_opt() else {
//         entries.error = Some("Could not determine the next calendar date.".to_owned());
//         return;
//     };
//     let (Some(start), Some(end)) = (
//         local_day_boundary(date),
//         local_day_boundary(next_date),
//     ) else {
//         entries.error = Some("Could not determine the selected day's local time range.".to_owned());
//         return;
//     };
//     entries.requested_date = Some(date);
//     entries.loaded_date = None;
//     entries.entries.clear();
//     entries.error = None;
//     let acc_id = account.id;
//     let pool = database.0.clone();
//     let sender = results.sender.clone();
//     thread::spawn(move || {
//         let result = match Builder::new_current_thread().enable_all().build() {
//             Ok(runtime) => runtime
//                 .block_on(
//                     db::calendar::entry::getters::get_entries_by_acc_id_between(
//                         &pool, acc_id, start, end,
//                     ),
//                 )
//                 .map_err(|error| error.to_string()),
//             Err(error) => Err(error.to_string()),
//         };
//         let _ = sender.send((acc_id, date, result));
//     });
// }
//
// pub fn apply_calendar_entry_results(
//     results: Res<CalendarEntryLoadResults>,
//     account: Res<AuthenticatedAccount>,
//     mut calendar_entries: ResMut<CalendarEntries>,
// ) {
//     let receiver = match results.receiver.lock() {
//         Ok(receiver) => receiver,
//         Err(error) => {
//             calendar_entries.error = Some(error.to_string());
//             return;
//         }
//     };
//     loop {
//         match receiver.try_recv() {
//             Ok((acc_id, date, result)) => {
//                 if acc_id != account.id
//                     || calendar_entries.requested_date != Some(date)
//                 {
//                     continue;
//                 }
//                 match result {
//                     Ok(entries) => {
//                         calendar_entries.loaded_date = Some(date);
//                         calendar_entries.entries = entries;
//                         calendar_entries.error = None;
//                     }
//                     Err(error) => {
//                         calendar_entries.error = Some(error);
//                     }
//                 }
//             }
//             Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
//         }
//     }
// }
//
// pub fn update_calendar_entries_text(
//     entries: Res<CalendarEntries>,
//     products: Res<CalendarProducts>,
//     mut texts: Query<&mut Text, With<CalendarEntriesText>>,
// ) {
//     let content = if let Some(error) = &entries.error {
//         format!("Could not load entries: {error}")
//     } else if entries.loaded_date != entries.requested_date {
//         "Loading entries…".to_owned()
//     } else if !products.loaded {
//         "Loading products…".to_owned()
//     } else if entries.entries.is_empty() {
//         "No usage entries for this day yet.".to_owned()
//     } else {
//         entries.entries.iter().map(|entry| {
//             let time = chrono::DateTime::<Local>::from(entry.occurred_at);
//             let name = products.products.iter().find(|product| product.id == entry.product_id).map(|product| product.name.as_str()).unwrap_or("Unknown product");
//             format!("{} · {}", time.format("%H:%M"), name)
//         }).collect::<Vec<_>>().join("\n")
//     };
//     for mut text in &mut texts {
//         **text = content.clone();
//     }
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
// pub fn focus_calendar_entry_field(
//     mut amount_fields: Query<&Interaction, (Changed<Interaction>, With<CalendarEntryAmountField>)>,
//     mut fields: Query<&Interaction, (Changed<Interaction>, With<CalendarEntryTimeField>)>,
//     mut form: ResMut<CalendarEntryForm>,
// ) {
//     if !form.open {
//         return;
//     }
//     for interaction in &mut amount_fields {
//         if *interaction == Interaction::Pressed {
//             form.focused = EntryFormField::Amount;
//         }
//     }
//     for interaction in &mut fields {
//         if *interaction == Interaction::Pressed {
//             form.focused = EntryFormField::Time;
//         }
//     }
// }
//
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
//     buttons: Query<&Interaction, (Changed<Interaction>, With<CalendarEntrySaveButton>)>,
//     mut form: ResMut<CalendarEntryForm>,
//     calendar: Res<CalendarState>,
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
// pub fn apply_calendar_entry_save_results(
//     results: Res<CalendarEntrySaveResults>,
//     account: Res<AuthenticatedAccount>,
//     mut form: ResMut<CalendarEntryForm>,
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
//
// pub fn update_calendar_entry_error(
//     form: Res<CalendarEntryForm>,
//     mut texts: Query<&mut Text, With<CalendarEntryErrorText>>,
// ) {
//     let error = form.error.as_deref().unwrap_or("");
//     for mut text in &mut texts {
//         text.0 = error.to_owned();
//     }
// }
//
// pub fn begin_calendar_product_load(database: Res<Database>, mut products: ResMut<CalendarProducts>, receiver: Res<CalendarProductLoadReceiver>) {
//     if products.loading || products.loaded {
//         return;
//     }
//     products.loading = true;
//     products.error = None;
//     let pool = database.0.clone();
//     let (sender, result_receiver) = channel();
//     if let Ok(mut slot) = receiver.0.lock() {
//         *slot = Some(result_receiver);
//     } else {
//         products.loading = false;
//         products.error = Some("Could not initialize calendar product loading.".to_string());
//         return;
//     }
//     thread::spawn(move || {
//         let result = match Builder::new_current_thread().enable_all().build() {
//             Ok(runtime) => runtime.block_on(get_all_product_models(&pool)).map(|models| {
//                 models.into_iter().map(|model| CalendarProductEntry {
//                     id: model.id,
//                     name: model.name,
//                     serving_size: model.serving_size,
//                     serving_unit: model.serving_unit,
//                 }).collect::<Vec<_>>()
//             }).map_err(|error| format!("{error:?}")),
//             Err(error) => Err(error.to_string()),
//         };
//         let _ = sender.send(result);
//     });
// }
//
// pub fn poll_calendar_product_load(receiver: Res<CalendarProductLoadReceiver>, mut products: ResMut<CalendarProducts>) {
//     let result = {
//         let Ok(mut slot) = receiver.0.lock() else {
//             return;
//         };
//         let Some(result_receiver) = slot.as_ref() else {
//             return;
//         };
//         match result_receiver.try_recv() {
//             Ok(result) => {
//                 *slot = None;
//                 Some(result)
//             }
//             Err(TryRecvError::Empty) => None,
//             Err(TryRecvError::Disconnected) => {
//                 *slot = None;
//                 Some(Err("Calendar product loading worker disconnected.".to_string()))
//             }
//         }
//     };
//     let Some(result) = result else {
//         return;
//     };
//     products.loading = false;
//     match result {
//         Ok(loaded_products) => {
//             products.products = loaded_products;
//             products.loaded = true;
//             products.error = None;
//         }
//         Err(error) => {
//             products.error = Some(error);
//         }
//     }
// }
//
// pub fn update_calendar_product_options(
//     mut commands: Commands,
//     products: Res<CalendarProducts>,
//     lists: Query<(Entity, Option<&Children>), With<CalendarEntryProductList>>,
//     options: Query<Entity, With<CalendarEntryProductOption>>,
// ) {
//     if !products.loaded {
//         return;
//     }
//     let Ok((list_entity, children)) = lists.single() else {
//         return;
//     };
//     if children.is_some_and(|children| !children.is_empty()) {
//         return;
//     }
//     for entity in &options {
//         commands.entity(entity).despawn();
//     }
//     for product in &products.products {
//         commands.spawn((
//             Button,
//             CalendarEntryProductOption(product.id),
//             ChildOf(list_entity),
//             Node {
//                 width: px(340),
//                 min_height: px(38),
//                 padding: UiRect::horizontal(px(12)),
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             BackgroundColor(Color::srgb(0.09, 0.11, 0.12)),
//             Text::new(product.name.clone()),
//             TextFont::from_font_size(14.0),
//             TextColor(Color::WHITE),
//         ));
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

