/* app/src/components/calendar.rs
 * The purpose of this module is to provide calendar components.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.rs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;
use chrono::NaiveDate;

#[derive(Component)]
pub struct CalendarPreviousButton;

#[derive(Component)]
pub struct CalendarTodayButton;

#[derive(Component)]
pub struct CalendarNextButton;

#[derive(Component)]
pub struct OpenCalendarButton;

#[derive(Component)]
pub struct CalendarBackButton;

#[derive(Component)]
pub struct CalendarNavigationRoot;

#[derive(Component)]
pub struct CalendarPeriodTitle;

#[derive(Component)]
pub struct CalendarDayViewButton;

#[derive(Component)]
pub struct CalendarWeekViewButton;

#[derive(Component)]
pub struct CalendarMonthViewButton;

#[derive(Component)]
pub struct CalendarDateCell {
    pub date: NaiveDate,
    pub in_visible_month: bool,
}

#[derive(Component)]
pub struct CalendarFormServingsText;

#[derive(Component)]
pub struct CalendarFormTimeText;

#[derive(Component)]
pub struct CalendarFormProductSearchText;

#[derive(Component)]
pub struct CalendarFormProductListContainer;

#[derive(Component)]
pub struct CalendarFormOverlay;

#[derive(Component)]
pub struct CalendarFormRoot;

#[derive(Component)]
pub struct CalendarDeleteDialog;

#[derive(Component)]
pub struct CalendarDashboardButton;

#[derive(Component)]
pub struct CalendarDateCellText;

#[derive(Component)]
pub struct CalendarMonthContainer;

#[derive(Component)]
pub struct CalendarWeekContainer;

#[derive(Component)]
pub struct CalendarDayContainer;

#[derive(Component)]
pub struct CalendarWeekDateText;

#[derive(Component)]
pub struct CalendarWeekDateCell(pub usize);

#[derive(Component)]
pub struct CalendarMonthDateCell(pub usize);

#[derive(Component)]
pub struct CalendarAddEntryButton;




