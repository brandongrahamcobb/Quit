/* app/src/system/dashboard.rs
 * The purpose of this module is to provide a dashboard system.
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
use chrono::{Datelike, Days, Local, Timelike};
use crate::component::calendar::{
    CalendarBackButton, CalendarDateCell, CalendarDayViewButton, CalendarMonthViewButton, CalendarNavigationRoot, CalendarNextButton, CalendarPeriodTitle, CalendarPreviousButton, CalendarTodayButton, CalendarWeekViewButton, OpenCalendarButton,
};
use crate::resource::calendar_resource::{CalendarState, CalendarView};
use crate::page::AppPage;

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
                spawn_calendar_button(toolbar, "‹", CalendarPreviousButton);
                spawn_calendar_button(toolbar, "Today", CalendarTodayButton);
                spawn_calendar_button(toolbar, "›", CalendarNextButton);
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
                    ),
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
            format!("{} – {}", start.format("%b %e"), end.format("%b %e, %Y"))
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
