/* app/src/resource/calendar_resource.rs
 * The purpose of this module is to define calendar resources.
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
use chrono::{Datelike, Duration, Local, NaiveDate};
use db::calendar::entry::model::EntryModel;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;

use crate::resource::product_resource::ProductEntry;

#[derive(Resource)]
pub struct CalendarPageState {
    pub view: CalendarView,
    pub visible_date: NaiveDate,
    pub selected_date: NaiveDate,
    pub overlay: Option<CalendarOverlay>,
    pub loading_entries: bool,
    pub entries: Vec<EntryModel>,
    pub error: Option<String>
}

impl Default for CalendarPageState {
    fn default() -> Self {
        let today = Local::now().date_naive();
        Self {
            view: CalendarView::Month,
            visible_date: today,
            selected_date: today,
            overlay: None,
            loading_entries: false,
            entries: Vec::new(),
            error: None
        }
    }
}

impl CalendarPageState {
    pub fn month_grid_dates(&self) -> [NaiveDate; 42] {
        let first_day = self
            .visible_date
            .with_day(1)
            .unwrap_or(self.visible_date);
        let monday_offset = first_day.weekday().num_days_from_monday() as i64;
        let grid_start = first_day
            .checked_sub_signed(Duration::days(monday_offset))
            .unwrap_or(first_day);
        std::array::from_fn(|index| {
            grid_start
                .checked_add_signed(Duration::days(index as i64))
                .unwrap_or(grid_start)
        })
    }
}


#[derive(Default, Resource)]
pub struct CalendarFormState {
    pub time: String,
    pub loading_products: bool,
    pub products: Vec<ProductEntry>,
    pub product_id: Option<i32>,
    pub product_search_query: Option<String>,
    pub product_search_query_open: bool,
    pub servings: Option<f32>,
    pub error: Option<String>,
}

pub type CalendarEntryLoadResult = (i32, NaiveDate, Result<Vec<EntryModel>, String>);
#[derive(Default, Resource)]
pub struct CalendarEntryLoadReceiver(pub Mutex<Option<Receiver<CalendarEntryLoadResult>>>);
pub type CalendarFormSaveResult = Result<EntryModel>;
#[derive(Default, Resource)]
pub struct CalendarFormSaveReceiver(pub Mutex<Option<Receiver<CalendarFormSaveResult>>>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalendarOverlay {
    Form { editing_id: Option<i32> },
    Delete(i32),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarView {
    Day,
    Week,
    #[default]
    Month,
}


