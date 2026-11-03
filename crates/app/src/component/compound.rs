/* app/src/components/compound.rs
 * The purpose of this module is to provide compound components.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.rs
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct OpenCompoundsButton;

#[derive(Component)]
pub struct CompoundNavigationRoot;

#[derive(Component)]
pub struct CompoundListText;

#[derive(Component)]
pub struct CompoundAddButton;

#[derive(Component)]
pub struct CompoundEditButton(pub i32);

#[derive(Component)]
pub struct CompoundFormRoot;

#[derive(Component)]
pub struct CompoundCommonNameButton;

#[derive(Component)]
pub struct CompoundCommonNameText;

#[derive(Component)]
pub struct CompoundIupacNameButton;

#[derive(Component)]
pub struct CompoundIupacNameText;

#[derive(Component)]
pub struct CompoundHalfLifeButton;

#[derive(Component)]
pub struct CompoundHalfLifeText;

#[derive(Component)]
pub struct CompoundMetaboliteToggleButton(pub i32);

#[derive(Component)]
pub struct CompoundMetaboliteListText;

#[derive(Component)]
pub struct CompoundSaveButton;

#[derive(Component)]
pub struct CompoundCancelButton;

#[derive(Component)]
pub struct CompoundErrorText;

#[derive(Component)]
pub struct CompoundListContainer;

#[derive(Component)]
pub struct CompoundListItem;

#[derive(Component)]
pub struct CompoundMetaboliteItem;

#[derive(Component)]
pub struct CompoundSearchButton;

#[derive(Component)]
pub struct CompoundSearchText;

#[derive(Component)]
pub struct CompoundPreviewButton(pub i32);

#[derive(Component)]
pub struct CompoundDeleteButton(pub i32);

#[derive(Component)]
pub struct CompoundMetaboliteListContainer;

#[derive(Component)]
pub struct CompoundFormOverlay;

#[derive(Component)]
pub struct CompoundImageUploadButton;

#[derive(Component)]
pub struct CompoundImageRemoveButton;

#[derive(Component)]
pub struct CompoundImagePreview;

#[derive(Component)]
pub struct CompoundImagePreviewPath(pub String);

#[derive(Component)]
pub struct CompoundImagePreviewHandle(pub Handle<Image>);

#[derive(Component)]
pub struct CompoundDeleteDialog;

#[derive(Component)]
pub struct CompoundDeleteConfirmButton;

#[derive(Component)]
pub struct CompoundDeleteCancelButton;

#[derive(Component)]
pub struct CompoundDeleteErrorText;

#[derive(Component)]
pub struct CompoundDeleteNameText;
