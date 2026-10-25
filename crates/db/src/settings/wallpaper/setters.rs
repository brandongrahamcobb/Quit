/* db/src/account/query/setters.rs
 * The purpose of this module is to provide database setters for accounts.
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

use crate::error::DatabaseError;
use crate::pool;
use crate::pool::DbPool;
use crate::schema::settings_wallpaper;
use crate::settings::wallpaper::model::WallpaperSettingsModel;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn set_tile_x_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    tile_x: bool,
) -> Result<WallpaperSettingsModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(settings_wallpaper::table.filter(settings_wallpaper::acc_id.eq(acc_id)))
            .set(settings_wallpaper::tile_x.eq(tile_x))
            .get_result::<WallpaperSettingsModel>(conn)
    })
    .await
}

pub async fn set_tile_y_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    tile_y: bool,
) -> Result<WallpaperSettingsModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(settings_wallpaper::table.filter(settings_wallpaper::acc_id.eq(acc_id)))
            .set(settings_wallpaper::tile_y.eq(tile_y))
            .get_result::<WallpaperSettingsModel>(conn)
    })
    .await
}

pub async fn set_wallpaper_file_name_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    file_name: String,
) -> Result<WallpaperSettingsModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(settings_wallpaper::table.filter(settings_wallpaper::acc_id.eq(acc_id)))
            .set(settings_wallpaper::file_name.eq(Some(file_name)))
            .get_result::<WallpaperSettingsModel>(conn)
    })
    .await
}

pub async fn set_tile_scale_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    scale: f32,
) -> Result<WallpaperSettingsModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(settings_wallpaper::table.filter(settings_wallpaper::acc_id.eq(acc_id)))
            .set(settings_wallpaper::scale.eq(scale))
            .get_result::<WallpaperSettingsModel>(conn)
    })
    .await
}

pub async fn set_mode_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    mode: String,
) -> Result<WallpaperSettingsModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(settings_wallpaper::table.filter(settings_wallpaper::acc_id.eq(acc_id)))
            .set(settings_wallpaper::mode.eq(mode))
            .get_result::<WallpaperSettingsModel>(conn)
    })
    .await
}
