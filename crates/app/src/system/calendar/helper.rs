/* app/src/system/calendar/helper.rs
 * The purpose of this module is to provide helper functions for the calendar system.
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


use chrono::{Datelike, Days, Local};
use std::time::SystemTime;

use crate::resource::calendar_resource::{CalendarPageState, CalendarView};

pub fn calendar_period_label(calendar: &CalendarPageState) -> String {
    match calendar.view {
        CalendarView::Day => calendar
            .visible_date
            .format("%A, %B %e, %Y")
            .to_string(),
        CalendarView::Week => {
            let start = calendar
                .visible_date
                .checked_sub_days(Days::new(
                    calendar.visible_date.weekday().num_days_from_monday() as u64,
                ))
                .unwrap_or(calendar.visible_date);
            let end = start
                .checked_add_days(Days::new(6))
                .unwrap_or(start);
            format!("{} - {}", start.format("%b %e"), end.format("%b %e, %Y"))
        }
        CalendarView::Month => calendar.visible_date.format("%B %Y").to_string(),
    }
}

pub fn local_day_boundary(date: chrono::NaiveDate) -> Option<SystemTime> {
    date.and_hms_opt(0, 0, 0)?
        .and_local_timezone(Local)
        .earliest()
        .map(SystemTime::from)
}
