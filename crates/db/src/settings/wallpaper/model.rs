/* db/src/account/model.rs
 * The purpose of this module is to provide an account db model and associated methods.
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

use crate::schema::settings_wallpaper;
use crate::settings::wallpaper::error::WallpaperSettingsModelError;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = settings_wallpaper)]
pub struct WallpaperSettingsModel {
    pub acc_id: i32,
    pub file_name: Option<String>,
    pub tile_x: bool,
    pub tile_y: bool,
    pub scale: f32,
    pub mode: String,
    pub created_at: Option<SystemTime>,
    pub updated_at: SystemTime,
}

impl WallpaperSettingsModel {
    pub fn get_file_name(&self) -> Result<String, WallpaperSettingsModelError> {
        self.file_name
            .as_ref()
            .cloned()
            .ok_or(WallpaperSettingsModelError::NoFileName)
    }

    pub fn get_created_at(&self) -> Result<SystemTime, WallpaperSettingsModelError> {
        if let Some(created_at) = self.created_at {
            Ok(created_at)
        } else {
            Err(WallpaperSettingsModelError::NoCreatedAt(self.acc_id))
        }
    }
}
