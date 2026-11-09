/* app/src/bin/quit.rs
 * The purpose of this module is to provide a Bevy client for quitting smoking.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.git
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

use std::collections::HashMap;

use bevy::DefaultPlugins;
use bevy::app::{App, Startup};
use bevy::ecs::system::Commands;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use db::pool::DbPool;
// use quit::plugin::calendar_plugin::CalendarPlugin;
// use quit::plugin::compound_plugin::CompoundPlugin;
use quit::plugin::dashboard_plugin::DashboardPlugin;
use quit::plugin::login_plugin::LoginPlugin;
use quit::plugin::page_plugin::PagePlugin;
use quit::plugin::scroll_plugin::ScrollPlugin;
use quit::plugin::startup_plugin::StartupPlugin;
// use quit::plugin::onboarding_plugin::OnboardingPlugin;
use quit::plugin::product_plugin::ProductPlugin;
use quit::resource::custom_resource::{ClientMap, Database};
use tracing_subscriber::EnvFilter;

fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();
    let mut app: App = App::new();
    app.add_plugins((
        DefaultPlugins,
        TabNavigationPlugin,
        StartupPlugin,
        PagePlugin,
        LoginPlugin,
        DashboardPlugin,
        // OnboardingPlugin,
        // CalendarPlugin,
        // CompoundPlugin,
        ProductPlugin,
        ScrollPlugin
    ));
    let db: DbPool = db::pool::new().expect("failed to create database pool");
    app.insert_resource(ClientMap(HashMap::new()));
    app.insert_resource(Database(db));
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        main();
    }
}
