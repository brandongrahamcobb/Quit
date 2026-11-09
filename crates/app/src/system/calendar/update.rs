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
use bevy::ui_widgets::Button;
use chrono::Local;
use crate::component::calendar::{CalendarNavigationRoot, CalendarPeriodTitle};
use crate::component::entry::{CalendarEntriesText, CalendarEntryErrorText, CalendarEntryProductList, CalendarEntryProductOption};
use crate::resource::calendar_resource::{
    CalendarEntries, CalendarEntryForm, CalendarProducts, CalendarPageState, 
};
use crate::system::calendar::helper::calendar_period_label;
use crate::system::calendar::spawn::spawn_calendar_navigation;

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

pub fn update_calendar_entries_text(
    entries: Res<CalendarEntries>,
    products: Res<CalendarProducts>,
    mut texts: Query<&mut Text, With<CalendarEntriesText>>,
) {
    let content = if let Some(error) = &entries.error {
        format!("Could not load entries: {error}")
    } else if entries.loaded_date != entries.requested_date {
        "Loading entries…".to_owned()
    } else if !products.loaded {
        "Loading products…".to_owned()
    } else if entries.entries.is_empty() {
        "No usage entries for this day yet.".to_owned()
    } else {
        entries.entries.iter().map(|entry| {
            let time = chrono::DateTime::<Local>::from(entry.occurred_at);
            let name = products.products.iter().find(|product| product.id == entry.product_id).map(|product| product.name.as_str()).unwrap_or("Unknown product");
            format!("{} · {}", time.format("%H:%M"), name)
        }).collect::<Vec<_>>().join("\n")
    };
    for mut text in &mut texts {
        **text = content.clone();
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

pub fn update_calendar_product_options(
    mut commands: Commands,
    products: Res<CalendarProducts>,
    lists: Query<(Entity, Option<&Children>), With<CalendarEntryProductList>>,
    options: Query<Entity, With<CalendarEntryProductOption>>,
) {
    if !products.loaded {
        return;
    }
    let Ok((list_entity, children)) = lists.single() else {
        return;
    };
    if children.is_some_and(|children| !children.is_empty()) {
        return;
    }
    for entity in &options {
        commands.entity(entity).despawn();
    }
    for product in &products.products {
        commands.spawn((
            Button,
            CalendarEntryProductOption(product.id),
            ChildOf(list_entity),
            Node {
                width: px(340),
                min_height: px(38),
                padding: UiRect::horizontal(px(12)),
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.09, 0.11, 0.12)),
            Text::new(product.name.clone()),
            TextFont::from_font_size(14.0),
            TextColor(Color::WHITE),
        ));
    }
}

pub fn update_calendar_page(
    mut commands: Commands,
    calendar: Res<CalendarPageState>,
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
