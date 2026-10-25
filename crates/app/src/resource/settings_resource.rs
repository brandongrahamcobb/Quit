/* app/src/resource/settings_resource.rs
 * The purpose of this module is to define settings resources.
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

use bevy::ecs::resource::Resource;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WallpaperMode {
    Auto,
    Stretch,
    #[default]
    Tiled,
}

#[derive(Resource)]
pub struct WallpaperSettings {
    pub image: String,
    pub mode: WallpaperMode,
    pub tile_x: bool,
    pub tile_y: bool,
    pub stretch_value: f32,
}

impl Default for WallpaperSettings {
    fn default() -> Self {
        Self {
            image: "quit_app_splash.png".to_owned(),
            mode: WallpaperMode::Tiled,
            tile_x: true,
            tile_y: true,
            stretch_value: 0.1,
        }
    }
}

#[derive(Resource, Default)]
pub struct WallpaperFiles(pub Vec<String>);

pub type WallpaperSettingsLoadResult = Result<db::settings::wallpaper::model::WallpaperSettingsModel, String>;

#[derive(Resource)]
pub struct WallpaperSettingsLoadResults {
    pub sender: Sender<WallpaperSettingsLoadResult>,
    pub receiver: Mutex<Receiver<WallpaperSettingsLoadResult>>,
}

impl Default for WallpaperSettingsLoadResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}
