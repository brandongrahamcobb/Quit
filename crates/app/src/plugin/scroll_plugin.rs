/* app/src/plugin/scroll_plugin.rs
* The purpose of this module is to serve scrolling flows for the Bevy client in a plugin.
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

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub struct UiScroll {
    pub entity: Entity,
    pub delta: Vec2,
}

pub struct ScrollPlugin;

impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_scroll_events)
            .add_observer(handle_scroll);
    }
}

pub fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= 21.0;
        }
        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(UiScroll { entity, delta });
            }
        }
    }
}

pub fn handle_scroll(
    mut scroll: On<UiScroll>,
    mut lists: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut position, node, computed)) = lists.get_mut(scroll.entity) else {
        return;
    };
    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
    let delta = &mut scroll.delta;
    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0.0 {
        let at_limit = if delta.y > 0.0 {
            position.y >= max_offset.y
        } else {
            position.y <= 0.0
        };
        if !at_limit {
            position.y = (position.y + delta.y).clamp(0.0, max_offset.y);
            delta.y = 0.0;
        }
    }
    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}
