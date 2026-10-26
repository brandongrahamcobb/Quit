/* app/src/resource/onboarding_resource.rs
 * The purpose of this module is to define onboarding resources.
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

use bevy::ecs::resource::Resource;
use std::sync::Mutex;
use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Debug)]
pub enum TermsAcceptanceResult {
    Saved,
    Failed,
}

#[derive(Resource)]
pub struct TermsAcceptanceResults {
    pub sender: Sender<TermsAcceptanceResult>,
    pub receiver: Mutex<Receiver<TermsAcceptanceResult>>,
}

impl Default for TermsAcceptanceResults {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender,
            receiver: Mutex::new(receiver),
        }
    }
}
