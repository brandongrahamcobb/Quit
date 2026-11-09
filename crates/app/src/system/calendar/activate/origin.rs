/* app/src/system/calendar/activate/origin.rs
 * The purpose of this module is to provide an observer for the main page of the calendar system.
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
use bevy::ui_widgets::Activate;
use crate::component::calendar::CalendarDateCell;
use crate::resource::calendar_resource::{
    CalendarOverlay, CalendarPageState, CalendarView 
};
use crate::page::AppPage;

pub fn handle_calendar_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_page: ResMut<NextState<AppPage>>,
    mut state: ResMut<CalendarPageState>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }
    match state.overlay {
        Some(CalendarOverlay::Form { editing_id }) => {
            state.overlay = None;
            // state.error = None;
        }
        Some(CalendarOverlay::Delete(_)) => {
            // if state.deleting {
            //     return;
            // }
            state.overlay = None;
            // state.deleting_name = None;
            // state.delete_error = None;
        }
        None => {
            next_page.set(AppPage::Dashboard);
        }
    }
}

pub fn handle_previous_button(
    _activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
) {
    calendar.visible_date = match calendar.view {
        CalendarView::Day => calendar
            .visible_date
            .checked_sub_days(chrono::Days::new(1))
            .unwrap_or(calendar.visible_date),
        CalendarView::Week => calendar
            .visible_date
            .checked_sub_days(chrono::Days::new(7))
            .unwrap_or(calendar.visible_date),
        CalendarView::Month => calendar
            .visible_date
            .checked_sub_months(chrono::Months::new(1))
            .unwrap_or(calendar.visible_date),
    };
    calendar.selected_date = calendar.visible_date;
}

pub fn handle_today_button(
    _activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
) {
    let today = chrono::Local::now().date_naive();
    calendar.visible_date = today;
    calendar.selected_date = today;
}

pub fn handle_next_button(
    _activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
) {
    calendar.visible_date = match calendar.view {
        CalendarView::Day => calendar
            .visible_date
            .checked_add_days(chrono::Days::new(1))
            .unwrap_or(calendar.visible_date),
        CalendarView::Week => calendar
            .visible_date
            .checked_add_days(chrono::Days::new(7))
            .unwrap_or(calendar.visible_date),
        CalendarView::Month => calendar
            .visible_date
            .checked_add_months(chrono::Months::new(1))
            .unwrap_or(calendar.visible_date),
    };
    calendar.selected_date = calendar.visible_date;
}

pub fn handle_day_button(
    _activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
) {
    calendar.view = CalendarView::Day;
}

pub fn handle_week_button(
    _activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
) {
    calendar.view = CalendarView::Week;
}

pub fn handle_month_button(
    _activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
) {
    calendar.view = CalendarView::Month;
}

pub fn handle_cell_button(
    activate: On<Activate>,
    mut calendar: ResMut<CalendarPageState>,
    cells: Query<&CalendarDateCell>,
) {
    let Ok(cell) = cells.get(activate.entity) else {
        return;
    };
    calendar.selected_date = cell.date;
    calendar.visible_date = cell.date;
}

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




