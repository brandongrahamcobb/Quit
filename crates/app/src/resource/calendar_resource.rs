/* app/src/resource/custom_resource.rs
 * The purpose of this module is to define client resources.
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

use std::sync::{Mutex, mpsc::{Receiver, Sender, channel}};

use bevy::prelude::*;
use chrono::{Datelike, Duration, Local, NaiveDate};
use db::calendar::entry::model::EntryModel;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarView {
    Day,
    Week,
    #[default]
    Month,
}

#[derive(Resource)]
pub struct CalendarState {
    pub view: CalendarView,
    pub visible_date: NaiveDate,
    pub selected_date: NaiveDate,
}

impl Default for CalendarState {
    fn default() -> Self {
        let today = Local::now().date_naive();

        Self {
            view: CalendarView::Month,
            visible_date: today,
            selected_date: today,
        }
    }
}

impl CalendarState {
    pub fn go_to_today(&mut self) {
        let today = Local::now().date_naive();
        self.visible_date = today;
        self.selected_date = today;
    }

    pub fn move_next(&mut self) {
        self.visible_date = match self.view {
            CalendarView::Day => self
                .visible_date
                .checked_add_days(chrono::Days::new(1))
                .unwrap_or(self.visible_date),
            CalendarView::Week => self
                .visible_date
                .checked_add_days(chrono::Days::new(7))
                .unwrap_or(self.visible_date),
            CalendarView::Month => self
                .visible_date
                .checked_add_months(chrono::Months::new(1))
                .unwrap_or(self.visible_date),
        };
        self.selected_date = self.visible_date;
    }

    pub fn move_previous(&mut self) {
        self.visible_date = match self.view {
            CalendarView::Day => self
                .visible_date
                .checked_sub_days(chrono::Days::new(1))
                .unwrap_or(self.visible_date),
            CalendarView::Week => self
                .visible_date
                .checked_sub_days(chrono::Days::new(7))
                .unwrap_or(self.visible_date),
            CalendarView::Month => self
                .visible_date
                .checked_sub_months(chrono::Months::new(1))
                .unwrap_or(self.visible_date),
        };
        self.selected_date = self.visible_date;
    }

    pub fn select_date(&mut self, date: NaiveDate) {
        self.selected_date = date;
        self.visible_date = date;
    }

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

#[derive(Resource, Default)]
pub struct CalendarEntries {
    pub loaded_date: Option<NaiveDate>,
    pub requested_date: Option<NaiveDate>,
    pub entries: Vec<EntryModel>,
    pub error: Option<String>,
}

pub type CalendarEntryLoadResult =
    (i32, NaiveDate, Result<Vec<EntryModel>, String>);

#[derive(Resource)]
pub struct CalendarEntryLoadResults {
    pub sender: Sender<CalendarEntryLoadResult>,
    pub receiver: Mutex<Receiver<CalendarEntryLoadResult>>,
}

impl Default for CalendarEntryLoadResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EntryUnit {
    #[default]
    Grams,
    Milligrams,
    Micrograms,
}

impl EntryUnit {
    pub const fn database_value(self) -> &'static str {
        match self {
            Self::Grams => "g",
            Self::Milligrams => "mg",
            Self::Micrograms => "ug",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EntryFormField {
    #[default]
    Mass,
    Time,
}

#[derive(Resource)]
pub struct CalendarEntryForm {
    pub open: bool,
    pub mass: String,
    pub time: String,
    pub unit: EntryUnit,
    pub focused: EntryFormField,
    pub error: Option<String>,
}

impl Default for CalendarEntryForm {
    fn default() -> Self {
        Self {
            open: false,
            mass: String::new(),
            time: Local::now().format("%H:%M").to_string(),
            unit: EntryUnit::default(),
            focused: EntryFormField::default(),
            error: None,
        }
    }
}

pub type CalendarEntrySaveResult =
    (i32, NaiveDate, Result<EntryModel, String>);

#[derive(Resource)]
pub struct CalendarEntrySaveResults {
    pub sender: Sender<CalendarEntrySaveResult>,
    pub receiver: Mutex<Receiver<CalendarEntrySaveResult>>,
}

impl Default for CalendarEntrySaveResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}
