/* app/src/plugin/dashboard_plugin.rs
* The purpose of this module is to serve the dashboard flow for the Bevy client in a plugin.
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

use crate::page::AppPage;
use crate::resource::settings_resource::{WallpaperFiles, WallpaperSettings, WallpaperSettingsLoadResults};
use crate::system::dashboard::setup_dashboard;
use crate::system::settings::{setup_settings_controls, select_settings_page};
use crate::system::settings_wallpaper::{adjust_wallpaper_stretch, apply_loaded_wallpaper_settings, apply_wallpaper, discover_wallpaper_files, load_wallpaper_settings, populate_wallpaper_choices, select_wallpaper, select_wallpaper_mode, toggle_wallpaper_modal, toggle_wallpaper_mode_menu, toggle_wallpaper_tiles, update_tiled_controls, update_wallpaper_mode_label};

pub struct DashboardPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppPage>()
            .init_resource::<WallpaperFiles>()
            .init_resource::<WallpaperSettings>()
            .init_resource::<WallpaperSettingsLoadResults>()
            .add_systems(
                Update,
                (
                    select_settings_page,
                    apply_loaded_wallpaper_settings,
                    toggle_wallpaper_modal,
                    toggle_wallpaper_mode_menu,
                    select_wallpaper,
                    select_wallpaper_mode,
                    toggle_wallpaper_tiles,
                    adjust_wallpaper_stretch,
                    apply_wallpaper,
                    update_tiled_controls,
                    update_wallpaper_mode_label
                ).chain().run_if(in_state(AppPage::Dashboard))
            )
            .add_systems(
                OnEnter(AppPage::Dashboard),
                (
                    discover_wallpaper_files,
                    setup_dashboard,
                    setup_settings_controls,
                    populate_wallpaper_choices,
                    load_wallpaper_settings,
                ).chain()
            );
    }
}
