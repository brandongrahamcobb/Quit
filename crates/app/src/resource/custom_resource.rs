/* app/src/resource/custom_resource.rs
 * The purpose of this module is to define client resources.
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

use std::collections::HashMap;

use bevy::ecs::entity::Entity;
use bevy::ecs::resource::Resource;
use db::pool::DbPool;
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
use std::sync::Mutex;

#[derive(Resource)]
pub struct ClientMap(pub HashMap<i32, Entity>);

#[derive(Clone, Resource)]
pub struct Database(pub DbPool);

#[derive(Debug)]
pub enum LoginResult {
    Authenticated { id: i32, username: String },
    InvalidCredentials,
    DatabaseUnavailable,
}

#[derive(Resource)]
pub struct LoginResults {
    pub sender: Sender<LoginResult>,
    pub receiver: Mutex<Receiver<LoginResult>>,
}

impl Default for LoginResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Resource, Default)]
pub struct LoginPending(pub bool);

#[derive(Resource)]
pub struct AuthenticatedAccount {
    pub id: i32,
    pub username: String,
}

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
            image: "S-[3-(N-methylpyrrolid-2-yl)]pyridine.png".to_owned(),
            mode: WallpaperMode::Tiled,
            tile_x: true,
            tile_y: true,
            stretch_value: 0.1,
        }
    }
}

#[derive(Resource, Default)]
pub struct WallpaperFiles(pub Vec<String>);
