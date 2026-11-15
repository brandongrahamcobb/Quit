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
use chrono::{Datelike, Days};
use crate::component::calendar::{CalendarDateCell, CalendarDateCellText, CalendarDayContainer, CalendarDayViewButton, CalendarDeleteDialog, CalendarFormOverlay, CalendarMonthContainer, CalendarMonthDateCell, CalendarMonthViewButton, CalendarPeriodTitle, CalendarWeekContainer, CalendarWeekDateCell, CalendarWeekDateText, CalendarWeekViewButton};
use crate::resource::calendar_resource::{
    CalendarOverlay, CalendarPageState, CalendarView, 
};
use crate::system::calendar::helper::calendar_period_label;

pub fn update_calendar_period_title(
    calendar: Res<CalendarPageState>,
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

// pub fn update_calendar_entry_error(
//     form: Res<CalendarFormState>,
//     mut texts: Query<&mut Text, With<CalendarEntryErrorText>>,
// ) {
//     if !form.is_changed() {
//         return;
//     }
//     let error = form.error.as_deref().unwrap_or("");
//     for mut text in &mut texts {
//         text.0 = error.to_owned();
//     }
// }

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

pub fn update_calendar_overlays(
    mut overlays: ParamSet<(
        Query<&mut Node, With<CalendarFormOverlay>>,
        Query<&mut Node, With<CalendarDeleteDialog>>,
    )>,
    page_state: Res<CalendarPageState>,
) {
    let show_form = matches!(page_state.overlay, Some(CalendarOverlay::Form { .. }));
    let show_delete = matches!(page_state.overlay, Some(CalendarOverlay::Delete(_)));
    for mut node in &mut overlays.p0() {
        node.display = if show_form { Display::Flex } else { Display::None };
    }
    for mut node in &mut overlays.p1() {
        node.display = if show_delete { Display::Flex } else { Display::None };
    }
}

pub fn update_calendar_month_cells(
    calendar: Res<CalendarPageState>,
    mut cells: Query<(&mut CalendarDateCell, &mut BackgroundColor, &Children, &CalendarMonthDateCell)>,
    mut texts: Query<&mut Text, With<CalendarDateCellText>>,
    mut text_colors: Query<&mut TextColor, With<CalendarDateCellText>>,
) {
    if !calendar.is_changed() {
        return;
    }
    if calendar.view != CalendarView::Month {
        return;
    }
    let dates = calendar.month_grid_dates();
    let visible_month = calendar.visible_date.month();
    let visible_year = calendar.visible_date.year();
    for (mut cell, mut background, children, index) in &mut cells {
        let Some(date) = dates.get(index.0) else { continue; };
        cell.date = *date;
        cell.in_visible_month = date.month() == visible_month && date.year() == visible_year;
        *background = BackgroundColor(if *date == calendar.selected_date {
            Color::srgb(0.16, 0.34, 0.38)
        } else {
            Color::srgb(0.11, 0.14, 0.16)
        });
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                text.0 = date.day().to_string();
            }
            if let Ok(mut color) = text_colors.get_mut(child) {
                color.0 = if cell.in_visible_month {
                    Color::WHITE
                } else {
                    Color::srgb(0.42, 0.46, 0.48)
                };
            }
        }
    }
}

pub fn update_calendar_view_buttons(
    calendar: Res<CalendarPageState>,
    mut buttons: Query<
        (&mut BackgroundColor, Option<&CalendarDayViewButton>, Option<&CalendarWeekViewButton>, Option<&CalendarMonthViewButton>),
        Or<(With<CalendarDayViewButton>, With<CalendarWeekViewButton>, With<CalendarMonthViewButton>)>,
    >,
) {
    if !calendar.is_changed() {
        return;
    }
    for (mut background, day, week, month) in &mut buttons {
        let selected = (day.is_some() && calendar.view == CalendarView::Day)
            || (week.is_some() && calendar.view == CalendarView::Week)
            || (month.is_some() && calendar.view == CalendarView::Month);
        *background = BackgroundColor(if selected {
            Color::srgb(0.16, 0.34, 0.38)
        } else {
            Color::srgb(0.20, 0.24, 0.28)
        });
    }
}

pub fn update_calendar_view_visibility(
    calendar: Res<CalendarPageState>,
    mut containers: ParamSet<(
        Query<&mut Node, With<CalendarMonthContainer>>,
        Query<&mut Node, With<CalendarWeekContainer>>,
        Query<&mut Node, With<CalendarDayContainer>>,
    )>,
) {
    if !calendar.is_changed() {
        return;
    }
    for mut node in &mut containers.p0() {
        node.display = if calendar.view == CalendarView::Month { Display::Flex } else { Display::None };
    }
    for mut node in &mut containers.p1() {
        node.display = if calendar.view == CalendarView::Week { Display::Flex } else { Display::None };
    }
    for mut node in &mut containers.p2() {
        node.display = if calendar.view == CalendarView::Day { Display::Flex } else { Display::None };
    }
}

pub fn update_calendar_week_headings(
    calendar: Res<CalendarPageState>,
    mut cells: Query<(&mut CalendarDateCell, &mut BackgroundColor, &Children, &CalendarWeekDateCell)>,
    mut texts: Query<&mut Text, With<CalendarWeekDateText>>,
) {
    if !calendar.is_changed() || calendar.view != CalendarView::Week {
        return;
    }
    let monday = calendar.visible_date.checked_sub_days(Days::new(calendar.visible_date.weekday().num_days_from_monday() as u64)).unwrap_or(calendar.visible_date);
    for (mut cell, mut background, children, index) in &mut cells {
        let date = monday.checked_add_days(Days::new(index.0 as u64)).unwrap_or(monday);
        cell.date = date;
        *background = BackgroundColor(if date == calendar.selected_date {
            Color::srgb(0.16, 0.34, 0.38)
        } else {
            Color::srgb(0.11, 0.14, 0.16)
        });
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                text.0 = date.format("%a %e").to_string();
            }
        }
    }
}
