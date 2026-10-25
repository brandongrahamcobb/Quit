/* app/src/components/entry.rs
 * The purpose of this module is to provide a drug entry component.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.rs
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
use bevy::ecs::component::Component;
use std::time::SystemTime;

#[derive(Component)]
struct Entry {
    timestamp: SystemTime,
    route: RouteOfAdministration,
    chemical: Chemical
}

enum RouteOfAdministration {
    Gum,
    Patch,
    Snus,
    Pouch,
    Lozenge,
    Dip,
    Leaf,
    Flower,
    Oil,
    Vape
}

pub struct Chemical {
    common_name: String,
    iupac_name: String
}
