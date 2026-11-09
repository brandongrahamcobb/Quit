/* app/src/system/product/activate/preview.rs
 * The purpose of this module is to provide a preview observer for the product system.
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
use bevy::ui_widgets::Activate;
use crate::component::product::ProductPreviewButton;
use crate::resource::product_resource::{ProductOverlay, ProductPageState};

pub fn view_product_preview(
    activate: On<Activate>,
    preview_buttons: Query<&ProductPreviewButton>,
    mut state: ResMut<ProductPageState>,
) {
    let Ok(button) = preview_buttons.get(activate.entity) else {
        return;
    };
    let overlay = ProductOverlay::Preview(button.0);
    state.overlay = if state.overlay == Some(overlay) { None } else { Some(overlay) };
}
