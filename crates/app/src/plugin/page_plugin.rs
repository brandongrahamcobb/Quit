/* app/src/plugin/page_plugin.rs
* The purpose of this module is to serve the page startup and teardown for the Bevy client in a plugin.
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

use crate::{component::page::PageRoot, page::AppPage};

pub struct PagePlugin;

impl Plugin for PagePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .add_systems(OnExit(AppPage::Calendar), despawn_page)
            .add_systems(OnExit(AppPage::Compounds), despawn_page)
            .add_systems(OnExit(AppPage::Dashboard), despawn_page)
            .add_systems(OnExit(AppPage::Login), despawn_page)
            .add_systems(OnExit(AppPage::Products), despawn_page);
    }
}

pub fn despawn_page(
    mut commands: Commands,
    page_roots: Query<Entity, With<PageRoot>>,
) {
    for entity in &page_roots {
        commands.entity(entity).despawn();
    }
}
