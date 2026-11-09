/* app/src/system/dashboard.rs
 * The purpose of this module is to provide a dashboard system.
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
use bevy::ui_widgets::{Activate, Button};

use crate::component::calendar::OpenCalendarButton;
use crate::component::compound::OpenCompoundsButton;
use crate::component::dashboard::DashboardScreen;
use crate::component::page::PageRoot;
use crate::component::product::OpenProductsButton;
use crate::component::settings::WallpaperBackground;
use crate::page::AppPage;
use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::settings_resource::{WallpaperMode, WallpaperSettings};

pub fn setup_dashboard(
    mut commands: Commands,
    account: Res<AuthenticatedAccount>,
    asset_server: Res<AssetServer>,
    wallpaper_settings: Res<WallpaperSettings>,
) {
    let image_mode = match wallpaper_settings.mode {
        WallpaperMode::Auto => NodeImageMode::Auto,
        WallpaperMode::Stretch => NodeImageMode::Stretch,
        WallpaperMode::Tiled => NodeImageMode::Tiled {
            tile_x: wallpaper_settings.tile_x,
            tile_y: wallpaper_settings.tile_y,
            stretch_value: wallpaper_settings.stretch_value,
        },
    };
    commands.spawn((
        PageRoot,
        DashboardScreen,
        WallpaperBackground,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode::new(asset_server.load(wallpaper_settings.image.clone())).with_mode(image_mode),
    )).with_children(|parent| {
        parent.spawn((
            Node {
                padding: UiRect::all(px(36)),
                flex_direction: FlexDirection::Column,
                row_gap: px(12),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
        )).with_children(|panel| {
            panel.spawn((
                Text::new(format!("Welcome, {}.", account.username)),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::srgb(0.94, 0.96, 0.93)),
            ));
            panel.spawn((
                Text::new("Your quit journey starts here."),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.68, 0.74, 0.73)),
            ));
            panel.spawn((
                Button,
                OpenCalendarButton,
                Node {
                    height: px(44),
                    padding: UiRect::horizontal(px(16)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
            )).with_children(|button| {
                button.spawn((
                    Text::new("Calendar"),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::WHITE),
                ));
            }).observe(open_calendar);
            panel.spawn((
                Button,
                OpenCompoundsButton,
                Node {
                    height: px(44),
                    padding: UiRect::horizontal(px(16)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
            )).with_children(|button| {
                button.spawn((
                    Text::new("Compounds"),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::WHITE),
                ));
            }).observe(open_compounds);
            panel.spawn((
                Button,
                OpenProductsButton,
                Node {
                    height: px(44),
                    padding: UiRect::horizontal(px(16)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.20, 0.24)),
            )).with_children(|button| {
                button.spawn((
                    Text::new("Products"),
                    TextFont::from_font_size(16.0),
                    TextColor(Color::WHITE),
                ));
            }).observe(open_products);
        });
    });
}

pub fn open_products(
    _activate: On<Activate>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    next_page.set(AppPage::Products);
}

pub fn open_compounds(
    _activate: On<Activate>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    next_page.set(AppPage::Compounds);
}

pub fn open_calendar(
    _activate: On<Activate>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    next_page.set(AppPage::Calendar);
}

pub fn open_dashboard(
    _activate: On<Activate>,
    mut next_page: ResMut<NextState<AppPage>>,
) {
    next_page.set(AppPage::Dashboard);
}
