/* app/src/system/substance.rs
 * The purpose of this module is to provide a substance system.
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
use crate::component::substance::{OpenSubstanceButton, SubstanceNavigationRoot};
use crate::page::AppPage;
use crate::resource::substance_resource::SubstanceState;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::SystemTime;
use tokio::runtime::Builder;

pub fn open_substances(
    buttons: Query<&Interaction, (Changed<Interaction>, With<OpenSubstanceButton>)>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    if buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        next_page.set(AppPage::Substance);
    }
}

pub fn spawn_substance_navigation(mut commands: Commands, substance: Res<SubstanceState>) {
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
            SubstanceNavigationRoot,
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
            ));
        });
}
