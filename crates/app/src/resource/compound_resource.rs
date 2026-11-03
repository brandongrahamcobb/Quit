/* app/src/resource/compound_resource.rs
 * The purpose of this module is to define compound resources.
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
use db::compound::model::CompoundModel;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::Mutex;

#[derive(Resource, Default)]
pub struct CompoundPageState {
    pub compounds: Vec<CompoundEntry>,
    pub metabolites: Vec<CompoundEntry>,
    pub editing_id: Option<i32>,
    pub common_name: String,
    pub iupac_name: String,
    pub half_life_seconds: String,
    pub selected_metabolite_ids: Vec<i32>,
    pub focused_field: Option<CompoundFormField>,
    pub error: Option<String>,
    pub loading: bool,
    pub search_query: String,
    pub image_path: Option<String>,
    pub image_loading: bool,
    pub caret_started_at: f64,
    pub expanded_preview_id: Option<i32>,
    pub deleting_id: Option<i32>,
    pub deleting_name: Option<String>,
    pub delete_error: Option<String>,
    pub deleting: bool,
}

#[derive(Clone, Debug)]
pub struct CompoundEntry {
    pub id: i32,
    pub common_name: String,
    pub iupac_name: String,
    pub half_life_seconds: Option<i64>,
    pub image_path: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompoundFormField {
    CommonName,
    IupacName,
    HalfLife,
    Search
}

#[derive(Resource, Default)]
pub struct CompoundLoadReceiver(
    pub Mutex<Option<Receiver<Result<Vec<CompoundEntry>, String>>>>,
);

#[derive(Resource)]
pub struct CompoundSaveResults {
    pub sender: Sender<Result<Vec<CompoundEntry>, String>>,
    pub receiver: Mutex<Receiver<Result<Vec<CompoundEntry>, String>>>,
}

impl Default for CompoundSaveResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Resource)]
pub struct CompoundMetaboliteLoadResults {
    pub sender: Sender<Result<(i32, Vec<i32>), String>>,
    pub receiver: Mutex<Receiver<Result<(i32, Vec<i32>), String>>>,
}

impl Default for CompoundMetaboliteLoadResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}

#[derive(Resource, Default)]
pub struct CompoundImageUploadReceiver(
    pub Mutex<Option<Receiver<Result<Option<String>, String>>>>,
);

#[derive(Resource, Default)]
pub struct CompoundDeleteResults(pub Mutex<Option<Receiver<(i32, Result<(), String>)>>>);
