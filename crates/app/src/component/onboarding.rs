/* app/src/component/onboarding.rs
 * The purpose of this module is to provide onboarding components.
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

#[derive(Component)]
pub struct OnboardingScreen;

#[derive(Component)]
pub struct TermsCheckbox;

#[derive(Component)]
pub struct TermsContinue;

#[derive(Component)]
pub struct TermsStatus;

#[derive(Component)]
pub struct TermsCheckboxLabel;

#[derive(Resource, Default)]
pub struct OnboardingForm {
    pub accepted_terms: bool,
    pub submitting: bool,
    pub quit_status: Option<QuitStatus>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuitStatus {
    NotReady,
    ReadyToQuit,
    AlreadyQuit,
}

#[derive(Component)]
pub struct QuitChoice(pub QuitStatus);

#[derive(Component)]
pub struct QuitStatusDisplay;

impl QuitStatus {
    pub fn as_db_value(self) -> &'static str {
        match self {
            Self::NotReady => "not_ready",
            Self::ReadyToQuit => "ready_to_quit",
            Self::AlreadyQuit => "already_quit",
        }
    }
}
