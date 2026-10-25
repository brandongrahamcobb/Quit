/* app/src/component/settings.rs
 * The purpose of this module is to provide a set of settings modal components.
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

use crate::resource::settings_resource::WallpaperMode;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPage {
    General,
    Wallpaper,
}

#[derive(Component)]
pub struct SettingsPageButton(pub SettingsPage);

#[derive(Component)]
pub struct SettingsPagePanel(pub SettingsPage);

#[derive(Component)]
pub struct WallpaperBackground;

#[derive(Component)]
pub struct WallpaperOpenButton;

#[derive(Component)]
pub struct WallpaperCloseButton;

#[derive(Component)]
pub struct WallpaperModal;

#[derive(Component)]
pub struct WallpaperChoicesList;

#[derive(Component)]
pub struct WallpaperImageChoice(pub String);

#[derive(Component)]
pub struct WallpaperModeDropdown;

#[derive(Component)]
pub struct WallpaperModeMenu;

#[derive(Component)]
pub struct WallpaperModeChoice(pub WallpaperMode);

#[derive(Component)]
pub struct WallpaperTileXButton;

#[derive(Component)]
pub struct WallpaperTileYButton;

#[derive(Component)]
pub struct WallpaperTileXKnob;

#[derive(Component)]
pub struct WallpaperTileYKnob;

#[derive(Component)]
pub struct WallpaperStretchSlider;

#[derive(Component)]
pub struct WallpaperStretchFill;

#[derive(Component)]
pub struct WallpaperStretchLabel;
