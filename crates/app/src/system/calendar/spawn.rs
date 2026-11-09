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
use bevy::ui_widgets::{Activate, Button};
use chrono::{Datelike, Days, Timelike};
use crate::component::calendar::{
    CalendarBackButton, CalendarDateCell, CalendarDayViewButton, CalendarEntryAmountField, CalendarEntryAmountUnitButton, CalendarMonthViewButton, CalendarNavigationRoot, CalendarNextButton, CalendarPeriodTitle, CalendarPreviousButton, CalendarTodayButton, CalendarWeekViewButton,
};
use crate::component::entry::{CalendarAddEntryButton, CalendarEntriesText, CalendarEntryCloseButton, CalendarEntryErrorText, CalendarEntryModal, CalendarEntryProductButton, CalendarEntryProductList, CalendarEntrySaveButton, CalendarEntryTimeField};
use crate::component::page::PageRoot;
use crate::resource::calendar_resource::{CalendarPageState, CalendarView};
use crate::system::calendar::activate::origin::{handle_cell_button, handle_day_button, handle_month_button, handle_next_button, handle_today_button, handle_week_button};
use crate::system::calendar::helper::calendar_period_label;

pub fn spawn_calendar_navigation(mut commands: Commands, calendar: Res<CalendarPageState>) {
    let dates = calendar.month_grid_dates();
    let visible_month = calendar.visible_date.month();
    let visible_year = calendar.visible_date.year();
    commands
        .spawn((
            PageRoot,
            Node {
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(20)),
                flex_direction: FlexDirection::Column,
                row_gap: px(12),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.10, 0.12)),
            Name::new("Calendar Page"),
            CalendarNavigationRoot,
        ))
        .with_children(|page| {
            page.spawn((
                Node {
                    width: percent(100),
                    height: px(48),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: px(12),
                    ..default()
                },
                Name::new("Calendar Navigation"),
            ))
            .with_children(|toolbar| {
                spawn_calendar_button(toolbar, "<", CalendarPreviousButton).observe(handle_next_button);
                spawn_calendar_button(toolbar, "Today", CalendarTodayButton).observe(handle_today_button);
                spawn_calendar_button(toolbar, ">", CalendarNextButton).observe(handle_next_button);
                toolbar.spawn((
                    Text::new(calendar_period_label(&calendar)),
                    TextFont::from_font_size(18.0),
                    TextColor(Color::WHITE),
                    CalendarPeriodTitle,
                ));
                spawn_calendar_view_button(
                    toolbar,
                    "Day",
                    CalendarDayViewButton,
                    calendar.view == CalendarView::Day,
                ).observe(handle_day_button);
                spawn_calendar_view_button(
                    toolbar,
                    "Week",
                    CalendarWeekViewButton,
                    calendar.view == CalendarView::Week,
                ).observe(handle_week_button);
                spawn_calendar_view_button(
                    toolbar,
                    "Month",
                    CalendarMonthViewButton,
                    calendar.view == CalendarView::Month,
                ).observe(handle_month_button);
            });
            page.spawn((
                Node {
                    width: percent(100),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(4),
                    ..default()
                },
                Name::new("Calendar Month Grid"),
            ))
            .with_children(|grid| {
                if calendar.view == CalendarView::Day {
                    spawn_calendar_day_timeline(grid, calendar.visible_date);
                    return;
                }
                if calendar.view == CalendarView::Week {
                    spawn_calendar_week_timeline(grid, calendar.visible_date, calendar.selected_date);
                    return;
                }
                grid.spawn((
                    Node {
                        width: percent(100),
                        height: px(28),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    Name::new("Calendar Weekdays"),
                ))
                .with_children(|weekdays| {
                   for label in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                        weekdays.spawn((
                            Node {
                                width: percent(14.2857),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Text::new(label),
                            TextFont::from_font_size(13.0),
                            TextColor(Color::srgb(0.65, 0.70, 0.72)),
                        ));
                    }
                });
                grid.spawn((
                    Node {
                        width: percent(100),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    Name::new("Calendar Dates"),
                ))
                .with_children(|cells| {
                    for date in dates {
                        let in_visible_month =
                            date.month() == visible_month && date.year() == visible_year;
                        let text_color = if in_visible_month {
                            Color::WHITE
                        } else {
                            Color::srgb(0.42, 0.46, 0.48)
                        };
                        let background_color = if date == calendar.selected_date {
                            Color::srgb(0.16, 0.34, 0.38)
                        } else {
                            Color::srgb(0.11, 0.14, 0.16)
                        };
                        cells.spawn((
                            Button,
                            Node {
                                width: percent(14.2857),
                                height: percent(16.6667),
                                padding: UiRect::all(px(8)),
                                ..default()
                            },
                            BackgroundColor(background_color),
                            CalendarDateCell {
                                date,
                                in_visible_month,
                            },
                            children![(
                                Text::new(date.day().to_string()),
                                TextFont::from_font_size(14.0),
                                TextColor(text_color),
                            )],
                        )).observe(handle_cell_button);
                    }
                });
            });
            page.spawn((
                Node {
                    width: percent(100),
                    padding: UiRect::all(px(16)),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.11, 0.14, 0.16)),
                children![
                    (
                        Text::new(format!(
                            "Usage on {}",
                            calendar.selected_date.format("%A, %B %e, %Y")
                        )),
                        TextFont::from_font_size(16.0),
                        TextColor(Color::WHITE),
                    ),
                    (
                        Text::new("No usage entries for this day yet."),
                        TextFont::from_font_size(13.0),
                        TextColor(Color::srgb(0.65, 0.70, 0.72)),
                        CalendarEntriesText,
                    ),
                    (
                        CalendarAddEntryButton,
                        Node {
                            width: px(120),
                            height: px(36),
                            padding: UiRect::horizontal(px(12)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.17, 0.42, 0.46)),
                        children![(
                            Text::new("Add entry"),
                            TextFont::from_font_size(14.0),
                            TextColor(Color::WHITE),
                        )],
                    )
                ],
            ));
            page.spawn((
                CalendarBackButton,
                Node {
                    height: px(36),
                    padding: UiRect::horizontal(px(12)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
                children![(
                    Text::new("Back to Dashboard"),
                    TextFont::from_font_size(14.0),
                    TextColor(Color::WHITE),
                )],
            ));
            page.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(0),
                    top: px(0),
                    bottom: px(0),
                    display: Display::None,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
                CalendarEntryModal,
            ))
            .with_children(|overlay| {
                overlay
                    .spawn((
                        Node {
                            width: px(420),
                            padding: UiRect::all(px(24)),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(16),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.11, 0.14, 0.16)),
                    ))
                    .with_children(|modal| {
                        modal.spawn((
                            Text::new("New entry"),
                            TextFont::from_font_size(20.0),
                            TextColor(Color::WHITE),
                        ));
                        modal.spawn((
                            CalendarEntryProductButton,
                            Node {
                                width: px(340),
                                height: px(42),
                                padding: UiRect::horizontal(px(12)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            Text::new("Product: Select product"),
                            TextFont::from_font_size(15.0),
                            TextColor(Color::WHITE),
                        ));
                        modal.spawn((
                            Node {
                                width: px(340),
                                max_height: px(180),
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::scroll_y(),
                                display: Display::None,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            CalendarEntryProductList,
                        ));
                        modal.spawn((
                            CalendarEntryAmountField,
                            Node {
                                width: px(340),
                                height: px(42),
                                padding: UiRect::horizontal(px(12)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            Text::new("Amount: 1"),
                            TextFont::from_font_size(15.0),
                            TextColor(Color::WHITE),
                        ));
                        modal.spawn((
                            CalendarEntryAmountUnitButton,
                            Node {
                                width: px(340),
                                height: px(42),
                                padding: UiRect::horizontal(px(12)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            Text::new("Unit: unit"),
                            TextFont::from_font_size(15.0),
                            TextColor(Color::WHITE),
                        ));
                        modal.spawn((
                            CalendarEntryTimeField,
                            Node {
                                width: px(340),
                                height: px(42),
                                padding: UiRect::horizontal(px(12)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            Text::new("Time: "),
                            TextFont::from_font_size(15.0),
                            TextColor(Color::WHITE),
                        ));
                        modal.spawn((
                            Text::new(""),
                            TextFont::from_font_size(13.0),
                            TextColor(Color::srgb(1.0, 0.45, 0.4)),
                            CalendarEntryErrorText,
                        ));
                        modal.spawn((
                            CalendarEntrySaveButton,
                            Node {
                                width: px(120),
                                height: px(40),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.18, 0.42, 0.34)),
                            children![(
                                Text::new("Save entry"),
                                TextFont::from_font_size(15.0),
                                TextColor(Color::WHITE),
                            )],
                        ));
                        modal.spawn((
                            Button,
                            CalendarEntryCloseButton,
                            Node {
                                width: px(40),
                                height: px(36),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
                            children![(
                                Text::new("x"),
                                TextFont::from_font_size(20.0),
                                TextColor(Color::WHITE),
                            )],
                        ));
                    });
            });
        });
}


fn spawn_calendar_button<'a, T>(
    parent: &'a mut ChildSpawnerCommands,
    label: &str,
    marker: T,
) -> EntityCommands<'a>
where
    T: Component,
{
    let mut entity = parent
        .spawn((
            Node {
                min_width: Val::Px(44.0),
                height: Val::Px(36.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
            marker,
        ));
    entity.with_child((
        Text::new(label),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));
    entity
}

fn spawn_calendar_day_timeline(
    parent: &mut ChildSpawnerCommands,
    date: chrono::NaiveDate,
) {
    parent
        .spawn((
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Name::new(format!("Usage Timeline for {date}")),
        ))
        .with_children(|timeline| {
            let now = chrono::Local::now();
            let today = now.date_naive();
            let current_hour = now.hour();
            let minute_offset = now.minute() as f32 / 60.0 * 100.0;
            for hour in 0..24 {
                let is_current_slot = date == today && hour == current_hour;
                timeline
                    .spawn((
                        Node {
                            position_type: PositionType::Relative,
                            width: percent(100),
                            height: px(28),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: px(8),
                            ..default()
                        },
                        Name::new(format!("{hour:02}:00")),
                    ))
                    .with_children(|row| {
                        row.spawn((
                            Node {
                                width: px(48),
                                ..default()
                            },
                            Text::new(format!("{hour:02}:00")),
                            TextFont::from_font_size(12.0),
                            TextColor(Color::srgb(0.65, 0.70, 0.72)),
                        ));
                        row.spawn((
                            Node {
                                flex_grow: 1.0,
                                height: px(1),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.24, 0.28, 0.30)),
                        ));
                        if is_current_slot {
                            row.spawn((
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: px(48),
                                    right: px(0),
                                    top: percent(minute_offset),
                                    height: px(2),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.95, 0.30, 0.30)),
                            ));
                        }
                    });
            }
        });
}

fn spawn_calendar_week_timeline(
    parent: &mut ChildSpawnerCommands,
    date: chrono::NaiveDate,
    selected_date: chrono::NaiveDate
) {
    let monday = date
        .checked_sub_days(Days::new(date.weekday().num_days_from_monday() as u64))
        .unwrap_or(date);
    let dates = std::array::from_fn::<_, 7, _>(|index| {
        monday
            .checked_add_days(Days::new(index as u64))
            .unwrap_or(monday)
    });
    parent
        .spawn((
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Name::new("Calendar Week Timeline"),
        ))
        .with_children(|timeline| {
            timeline
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(36),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    Name::new("Calendar Week Headings"),
                ))
                .with_children(|heading| {
                    heading.spawn(Node {
                        width: px(48),
                        ..default()
                    });
                    for day in dates.iter().copied() {
                        let background_color = if day == selected_date {
                            Color::srgb(0.16, 0.34, 0.38)
                        } else {
                            Color::srgb(0.11, 0.14, 0.16)
                        };
                        heading
                            .spawn((
                                Button,
                                Node {
                                    flex_grow: 1.0,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(background_color),
                                CalendarDateCell {
                                    date: day,
                                    in_visible_month: true,
                                },
                            ))
                            .with_child((
                                Text::new(day.format("%a %e").to_string()),
                                TextFont::from_font_size(12.0),
                                TextColor(Color::WHITE),
                            )).observe(handle_cell_button);
                    }
                });
            let now = chrono::Local::now();
            let today = now.date_naive();
            let current_hour = now.hour();
            let minute_offset = now.minute() as f32 / 60.0 * 100.0;
            for hour in 0..24 {
                timeline
                    .spawn((
                        Node {
                            width: percent(100),
                            height: px(26),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Name::new(format!("{hour:02}:00")),
                    ))
                    .with_children(|row| {
                        row.spawn((
                            Node {
                                width: px(48),
                                ..default()
                            },
                            Text::new(format!("{hour:02}:00")),
                            TextFont::from_font_size(11.0),
                            TextColor(Color::srgb(0.65, 0.70, 0.72)),
                        ));
                        for day in dates.iter().copied() {
                            let mut day_column = row.spawn((
                                Node {
                                    position_type: PositionType::Relative,
                                    flex_grow: 1.0,
                                    height: percent(100),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.11, 0.14, 0.16)),
                            ));
                            if day == today && hour == current_hour {
                                day_column.with_child((
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: px(0),
                                        right: px(0),
                                        top: percent(minute_offset),
                                        height: px(2),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.95, 0.30, 0.30)),
                                ));
                            }
                        }
                    });
            }
        });
}

fn spawn_calendar_view_button<'a, T>(
    parent: &'a mut ChildSpawnerCommands,
    label: &str,
    marker: T,
    active: bool,
) -> EntityCommands<'a> 
where
    T: Component
{
    let background_color = if active {
        Color::srgb(0.17, 0.42, 0.46)
    } else {
        Color::srgb(0.16, 0.20, 0.24)
    };
    let mut entity = parent
        .spawn((
            Button,
            Node {
                min_width: Val::Px(44.0),
                height: Val::Px(36.0),
                padding: UiRect::horizontal(Val::Px(12.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(background_color),
            marker,
        ));
    entity.with_child((
        Text::new(label),
        TextFont::from_font_size(16.0),
        TextColor(Color::WHITE),
    ));
    entity
}
