/* app/src/plugin/onboarding_plugin.rs
* The purpose of this module is to serve the onboarding flow for the Bevy client in a plugin.
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

use crate::component::onboarding::OnboardingForm;
use crate::page::AppPage;
use crate::resource::onboarding_resource::TermsAcceptanceResults;
use crate::system::onboarding::{
    despawn_onboarding, poll_terms_acceptance, select_quit_status, setup_onboarding, submit_terms_acceptance, toggle_terms_acceptance,
};

pub struct OnboardingPlugin;

impl Plugin for OnboardingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnboardingForm>()
            .init_resource::<TermsAcceptanceResults>()
            .add_systems(OnEnter(AppPage::Onboarding), setup_onboarding)
            .add_systems(
                Update,
                (toggle_terms_acceptance, select_quit_status, submit_terms_acceptance, poll_terms_acceptance).run_if(in_state(AppPage::Onboarding)),
            )
            .add_systems(OnExit(AppPage::Onboarding), despawn_onboarding);
    }
}
