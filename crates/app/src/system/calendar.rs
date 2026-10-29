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
use chrono::{Datelike, Days, Local, NaiveTime, Timelike};
use crate::component::calendar::{
    CalendarBackButton, CalendarDateCell, CalendarDayViewButton, CalendarMonthViewButton, CalendarNavigationRoot, CalendarNextButton, CalendarPeriodTitle, CalendarPreviousButton, CalendarTodayButton, CalendarWeekViewButton, OpenCalendarButton,
};
use crate::component::entry::{CalendarAddEntryButton, CalendarEntriesText, CalendarEntryCloseButton, CalendarEntryErrorText, CalendarEntryMassField, CalendarEntryModal, CalendarEntrySaveButton, CalendarEntryTimeField, CalendarEntryUnitButton};
use crate::resource::calendar_resource::{
    CalendarEntries, CalendarEntryForm, CalendarEntryLoadResults, CalendarEntrySaveResults, CalendarState, CalendarView, EntryFormField, EntryUnit,
};
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::AuthenticatedAccount;
use crate::page::AppPage;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::SystemTime;
use tokio::runtime::Builder;

pub fn handle_calendar_navigation(
    mut calendar: ResMut<CalendarState>,
    previous_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarPreviousButton>),
    >,
    today_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarTodayButton>),
    >,
    next_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarNextButton>),
    >,
    back_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarBackButton>),
    >,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    if previous_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        calendar.move_previous();
    }
    if today_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        calendar.go_to_today();
    }
    if next_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        calendar.move_next();
    }
    if back_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        next_page.set(AppPage::Dashboard);
    }
}

pub fn spawn_calendar_navigation(mut commands: Commands, calendar: Res<CalendarState>) {
    let dates = calendar.month_grid_dates();
    let visible_month = calendar.visible_date.month();
    let visible_year = calendar.visible_date.year();
    commands
        .spawn((
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
                spawn_calendar_button(toolbar, "<", CalendarPreviousButton);
                spawn_calendar_button(toolbar, "Today", CalendarTodayButton);
                spawn_calendar_button(toolbar, ">", CalendarNextButton);
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
                );
                spawn_calendar_view_button(
                    toolbar,
                    "Week",
                    CalendarWeekViewButton,
                    calendar.view == CalendarView::Week,
                );
                spawn_calendar_view_button(
                    toolbar,
                    "Month",
                    CalendarMonthViewButton,
                    calendar.view == CalendarView::Month,
                );
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
                        ));
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
                        Button,
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
                Button,
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
                            Button,
                            CalendarEntryMassField,
                            Node {
                                width: px(340),
                                height: px(42),
                                padding: UiRect::horizontal(px(12)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            Text::new("Mass: "),
                            TextFont::from_font_size(15.0),
                            TextColor(Color::WHITE),
                        ));
                        modal.spawn((
                            Button,
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
                            Button,
                            CalendarEntryUnitButton,
                            Node {
                                width: px(340),
                                height: px(42),
                                padding: UiRect::horizontal(px(12)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                            Text::new("Unit: g"),
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
                            Button,
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


fn spawn_calendar_button<T: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    marker: T,
) {
    parent
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
            BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
            marker,
        ))
        .with_child((
            Text::new(label),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ));
}

pub fn open_calendar(
    buttons: Query<&Interaction, (Changed<Interaction>, With<OpenCalendarButton>)>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        next_page.set(AppPage::Calendar);
    }
}

pub fn cleanup_calendar_navigation(
    mut commands: Commands,
    roots: Query<Entity, With<CalendarNavigationRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn();
    }
}

fn calendar_period_label(calendar: &CalendarState) -> String {
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

pub fn update_calendar_period_title(
    calendar: Res<CalendarState>,
    mut titles: Query<&mut Text, With<CalendarPeriodTitle>>,
) {
    if !calendar.is_changed() {
        return;
    }
    let label = calendar_period_label(&calendar);
    for mut title in &mut titles {
        title.0 = label.clone();
    }
}

pub fn select_calendar_view(
    mut calendar: ResMut<CalendarState>,
    day_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarDayViewButton>),
    >,
    week_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarWeekViewButton>),
    >,
    month_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarMonthViewButton>),
    >,
) {
    if day_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        calendar.view = CalendarView::Day;
    }
    if week_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        calendar.view = CalendarView::Week;
    }
    if month_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        calendar.view = CalendarView::Month;
    }
}

pub fn refresh_calendar_page(
    mut commands: Commands,
    calendar: Res<CalendarState>,
    roots: Query<Entity, With<CalendarNavigationRoot>>,
) {
    if !calendar.is_changed() {
        return;
    }
    for root in &roots {
        commands.entity(root).despawn();
    }
    spawn_calendar_navigation(commands, calendar);
}

pub fn select_calendar_date(
    mut calendar: ResMut<CalendarState>,
    cells: Query<
        (&Interaction, &CalendarDateCell),
        Changed<Interaction>,
    >,
) {
    for (interaction, cell) in &cells {
        if *interaction == Interaction::Pressed {
            calendar.select_date(cell.date);
        }
    }
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
            let now = Local::now();
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
                            ));
                    }
                });
            let now = Local::now();
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

fn spawn_calendar_view_button<T: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    marker: T,
    active: bool,
) {
    let background_color = if active {
        Color::srgb(0.17, 0.42, 0.46)
    } else {
        Color::srgb(0.16, 0.20, 0.24)
    };
    parent
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
        ))
        .with_child((
            Text::new(label),
            TextFont::from_font_size(16.0),
            TextColor(Color::WHITE),
        ));
}

fn local_day_boundary(date: chrono::NaiveDate) -> Option<SystemTime> {
    date.and_hms_opt(0, 0, 0)?
        .and_local_timezone(Local)
        .earliest()
        .map(SystemTime::from)
}

pub fn request_calendar_entries(
    database: Res<Database>,
    account: Res<AuthenticatedAccount>,
    calendar: Res<CalendarState>,
    mut entries: ResMut<CalendarEntries>,
    results: Res<CalendarEntryLoadResults>,
) {
    let date = calendar.selected_date;
    if entries.requested_date == Some(date) {
        return;
    }
    let Some(next_date) = date.succ_opt() else {
        entries.error = Some("Could not determine the next calendar date.".to_owned());
        return;
    };
    let (Some(start), Some(end)) = (
        local_day_boundary(date),
        local_day_boundary(next_date),
    ) else {
        entries.error = Some("Could not determine the selected day's local time range.".to_owned());
        return;
    };
    entries.requested_date = Some(date);
    entries.loaded_date = None;
    entries.entries.clear();
    entries.error = None;
    let acc_id = account.id;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime
                .block_on(
                    db::calendar::entry::getters::get_entries_by_acc_id_between(
                        &pool, acc_id, start, end,
                    ),
                )
                .map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((acc_id, date, result));
    });
}

pub fn apply_calendar_entry_results(
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

pub fn update_calendar_entries_text(
    entries: Res<CalendarEntries>,
    mut texts: Query<&mut Text, With<CalendarEntriesText>>,
) {
    let content = if let Some(error) = &entries.error {
        format!("Could not load entries: {error}")
    } else if entries.loaded_date != entries.requested_date {
        "Loading entries…".to_owned()
    } else if entries.entries.is_empty() {
        "No usage entries for this day yet.".to_owned()
    } else {
        entries
            .entries
            .iter()
            .map(|entry| {
                let time = chrono::DateTime::<Local>::from(entry.occurred_at);
                format!("{} · {} {}", time.format("%H:%M"), entry.mass, entry.units)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    for mut text in &mut texts {
        text.0 = content.clone();
    }
}

pub fn toggle_calendar_entry_form(
    open_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarAddEntryButton>),
    >,
    close_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarEntryCloseButton>),
    >,
    mut form: ResMut<CalendarEntryForm>,
    mut modals: Query<&mut Node, With<CalendarEntryModal>>,
) {
    if open_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        form.open = true;
        form.error = None;
    }
    if close_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        form.open = false;
    }
    for mut node in &mut modals {
        node.display = if form.open {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn focus_calendar_entry_field(
    mut fields: Query<
        (
            &Interaction,
            Option<&CalendarEntryMassField>,
            Option<&CalendarEntryTimeField>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut form: ResMut<CalendarEntryForm>,
) {
    if !form.open {
        return;
    }
    for (interaction, mass, time) in &mut fields {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if mass.is_some() {
            form.focused = EntryFormField::Mass;
        } else if time.is_some() {
            form.focused = EntryFormField::Time;
        }
    }
}

pub fn cycle_calendar_entry_unit(
    buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarEntryUnitButton>),
    >,
    mut form: ResMut<CalendarEntryForm>,
) {
    if !form.open {
        return;
    }
    for interaction in &buttons {
        if *interaction == Interaction::Pressed {
            form.unit = match form.unit {
                EntryUnit::Grams => EntryUnit::Milligrams,
                EntryUnit::Milligrams => EntryUnit::Micrograms,
                EntryUnit::Micrograms => EntryUnit::Grams,
            };
        }
    }
}

pub fn edit_calendar_entry_fields(
    mut keyboard: MessageReader<KeyboardInput>,
    mut form: ResMut<CalendarEntryForm>,
    mut fields: Query<(
        &mut Text,
        Option<&CalendarEntryMassField>,
        Option<&CalendarEntryTimeField>,
        Option<&CalendarEntryUnitButton>,
    )>,
) {
    if form.open {
        for input in keyboard.read() {
            if input.state != ButtonState::Pressed {
                continue;
            }
            match input.key_code {
                KeyCode::Backspace => match form.focused {
                    EntryFormField::Mass => {
                        form.mass.pop();
                    }
                    EntryFormField::Time => {
                        form.time.pop();
                    }
                },
                KeyCode::Tab => {
                    form.focused = match form.focused {
                        EntryFormField::Mass => EntryFormField::Time,
                        EntryFormField::Time => EntryFormField::Mass,
                    };
                }
                _ => {
                    let Some(text) = input.text.as_deref() else {
                        continue;
                    };
                    match form.focused {
                        EntryFormField::Mass => {
                            form.mass.extend(text.chars().filter(char::is_ascii_digit));
                        }
                        EntryFormField::Time => {
                            form.time.extend(
                                text.chars()
                                    .filter(|character| character.is_ascii_digit() || *character == ':'),
                            );
                        }
                    }
                }
            }
        }
    }
    for (mut text, mass, time, unit) in &mut fields {
        if mass.is_some() {
            text.0 = format!("Mass: {}", form.mass);
        } else if time.is_some() {
            text.0 = format!("Time: {}", form.time);
        } else if unit.is_some() {
            text.0 = format!("Unit: {}", form.unit.database_value());
        }
    }
}

pub fn save_calendar_entry(
    buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CalendarEntrySaveButton>),
    >,
    mut form: ResMut<CalendarEntryForm>,
    calendar: Res<CalendarState>,
    account: Res<AuthenticatedAccount>,
    database: Res<Database>,
    results: Res<CalendarEntrySaveResults>,
) {
    if !form.open
        || !buttons
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
    {
        return;
    }
    let Some(mass) = form.mass.parse::<i32>().ok().filter(|mass| *mass > 0) else {
        form.error = Some("Enter a mass greater than zero.".to_owned());
        return;
    };
    let Ok(time) = NaiveTime::parse_from_str(&form.time, "%H:%M") else {
        form.error = Some("Enter a time in HH:MM format.".to_owned());
        return;
    };
    let Some(occurred_at) = calendar
        .selected_date
        .and_time(time)
        .and_local_timezone(Local)
        .earliest()
        .map(SystemTime::from)
    else {
        form.error = Some("That local time does not exist.".to_owned());
        return;
    };
    let acc_id = account.id;
    let date = calendar.selected_date;
    let new_entry = db::calendar::entry::model::NewEntryModel {
        acc_id,
        occurred_at,
        mass,
        units: form.unit.database_value().to_owned(),
    };
    let pool = database.0.clone();
    let sender = results.sender.clone();
    form.error = None;
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime
                .block_on(db::calendar::entry::setters::insert_entry(
                    &pool, new_entry,
                ))
                .map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((acc_id, date, result));
    });
}

pub fn apply_calendar_entry_save_results(
    results: Res<CalendarEntrySaveResults>,
    account: Res<AuthenticatedAccount>,
    mut form: ResMut<CalendarEntryForm>,
    mut entries: ResMut<CalendarEntries>,
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
                    form.mass.clear();
                    form.time = Local::now().format("%H:%M").to_string();
                    form.error = None;
                    entries.requested_date = None;
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

pub fn update_calendar_entry_error(
    form: Res<CalendarEntryForm>,
    mut texts: Query<&mut Text, With<CalendarEntryErrorText>>,
) {
    let error = form.error.as_deref().unwrap_or("");
    for mut text in &mut texts {
        text.0 = error.to_owned();
    }
}
