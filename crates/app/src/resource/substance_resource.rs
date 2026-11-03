/* app/src/resource/substance_resource.rs
 * The purpose of this module is to define substance resources.
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
use db::substance::model::SubstanceModel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

#[derive(Resource, Default)]
pub struct SubstanceEntries {
    pub entries: Vec<SubstanceModel>,
    pub error: Option<String>,
    pub failed_for: Option<i32>,
    pub loaded_for: Option<i32>,
    pub loading_for: Option<i32>,
}

#[derive(Resource)]
pub struct SubstanceEntryLoadResults {
    pub receiver: Mutex<Receiver<(i32, Result<Vec<SubstanceModel>, String>)>>,
    pub sender: Sender<(i32, Result<Vec<SubstanceModel>, String>)>,
}

impl Default for SubstanceEntryLoadResults {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            receiver: Mutex::new(receiver),
            sender,
        }
    }
}

#[derive(Resource)]
pub struct SubstanceEntrySaveResults {
    pub receiver: Mutex<Receiver<(i32, Result<SubstanceModel, String>)>>,
    pub sender: Sender<(i32, Result<SubstanceModel, String>)>,
}

impl Default for SubstanceEntrySaveResults {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            receiver: Mutex::new(receiver),
            sender,
        }
    }
}

#[derive(Resource)]
pub struct SubstanceForm {
    pub error: Option<String>,
    pub kind: String,
    pub name: String,
    pub name_focused: bool,
    pub open: bool,
}

impl Default for SubstanceForm {
    fn default() -> Self {
        Self {
            error: None,
            kind: "simple".to_owned(),
            name: String::new(),
            name_focused: true,
            open: false,
        }
    }
}
