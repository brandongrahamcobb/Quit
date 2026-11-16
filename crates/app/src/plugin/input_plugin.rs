/* app/src/plugin/input_plugin.rs
* The purpose of this module is to serve text input field handling Bevy client in a plugin.
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

use bevy::input_focus::InputFocus;
use bevy::picking::hover::HoverMap;
use bevy::prelude::*;
use bevy::text::EditableText;

use crate::component::input::TextInputPlaceholder;

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_text_input_placeholder);
    }
}

pub fn update_text_input_placeholder(
    input_focus: Res<InputFocus>,
    mut placeholders: Query<(&ChildOf, &mut Visibility), With<TextInputPlaceholder>>,
    inputs: Query<&EditableText>,
) {
    for (parent, mut visibility) in &mut placeholders {
        let input_entity = parent.parent();
        let focused = input_focus.get() == Some(input_entity);
        let has_text = inputs.get(input_entity).is_ok_and(|input| !input.value().to_string().is_empty());
        *visibility = if focused || has_text {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
