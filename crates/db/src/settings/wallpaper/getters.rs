/* db/src/settings/wallpaper/getters.rs
 * The purpose of this module is to provide database getters for wallpaper settings.
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

use crate::account::model::AccountModel;
use crate::error::DatabaseError;
use crate::pool::{self, DbPool};
use crate::schema::settings_wallpaper;
use crate::settings::wallpaper::model::WallpaperSettingsModel;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn get_wallpaper_model_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
) -> Result<WallpaperSettingsModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        settings_wallpaper::table
            .filter(settings_wallpaper::acc_id.eq(acc_id))
            .first::<WallpaperSettingsModel>(conn)
    })
    .await
}
