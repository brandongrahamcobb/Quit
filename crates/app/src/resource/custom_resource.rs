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


use bevy::ecs::entity::Entity;
use bevy::ecs::resource::Resource;
use db::pool::DbPool;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ClientMap(pub HashMap<i32, Entity>);

#[derive(Clone, Resource)]
pub struct Database(pub DbPool);
