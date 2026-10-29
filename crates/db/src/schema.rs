/* db/src/schema.rs
 * The purpose of this module is to frame the database schema for diesel.
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

diesel::table! {
    accounts (id) {
        id -> Nullable<Int4>,
        username -> Text,
        password -> Text,
        pic -> Nullable<Text>,
        last_login_at -> Nullable<Timestamp>,
        gender -> Text,
        accepted_tos -> Bool,
        banned -> Bool,
        admin -> Bool,
        quit_status -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    settings_wallpaper (acc_id) {
        acc_id -> Int4,
        file_name -> Nullable<Text>,
        tile_x -> Bool,
        tile_y -> Bool,
        scale -> Float4,
        mode -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    compounds (id) {
        id -> Int4,
        acc_id -> Int4,
        name -> Text,
        half_life_seconds -> Nullable<Float8>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    compound_metabolites (parent_compound_id, metabolite_compound_id) {
        parent_compound_id -> Int4,
        metabolite_compound_id -> Int4,
    }
}

diesel::table! {
    entries (id) {
        id -> Int4,
        acc_id -> Int4,
        occurred_at -> Timestamp,
        mass -> Int4,
        units -> Text,
        substance_id -> Int4,
        route_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    routes (id) {
        id -> Int4,
        acc_id -> Int4,
        name -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    substance_compounds (substance_id, compound_id) {
        substance_id -> Int4,
        compound_id -> Int4,
        compound_amount -> Int8,
        compound_units -> Text,
        substance_amount -> Int8,
        substance_units -> Text,
    }
}

diesel::table! {
    substance_routes (substance_id, route_id) {
        substance_id -> Int4,
        route_id -> Int4,
    }
}

diesel::table! {
    substances (id) {
        id -> Int4,
        acc_id -> Int4,
        name -> Text,
        kind -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    compounds,
    compound_metabolites,
    entries,
    routes,
    settings_wallpaper,
    substance_compounds,
    substance_routes,
    substances,
);


