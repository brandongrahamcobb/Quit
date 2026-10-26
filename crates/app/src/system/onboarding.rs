/* app/src/system/onboarding.rs
 * The purpose of this module is to provide an onboarding system.
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
use db::account::setters::{accept_tos_by_acc_id, set_quit_status_by_acc_id};
use std::sync::mpsc::TryRecvError;
use tokio::runtime::Builder;

use crate::component::onboarding::{
    OnboardingForm, OnboardingScreen, QuitChoice, QuitStatus, QuitStatusDisplay, TermsCheckbox, TermsCheckboxLabel, TermsContinue, TermsStatus,
};
use crate::page::AppPage;
use crate::resource::custom_resource::{
    Database,
};
use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::onboarding_resource::{
    TermsAcceptanceResult,
    TermsAcceptanceResults,
};

pub fn setup_onboarding(mut commands: Commands) {
    commands.spawn((
        OnboardingScreen,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.11, 0.12)),
        children![(
            Node {
                width: px(620),
                padding: UiRect::all(px(32)),
                flex_direction: FlexDirection::Column,
                row_gap: px(18),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.13, 0.15)),
            children![
                (
                    Text::new("Before you begin"),
                    TextFont { font_size: 30.0, ..default() },
                    TextColor(Color::srgb(0.94, 0.96, 0.93)),
                ),
                (
                        Text::new("Where are you in your quit journey?"),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(Color::srgb(0.94, 0.96, 0.93)),
                    ),
                    (
                        Button,
                        QuitChoice(QuitStatus::NotReady),
                        Node {
                            height: px(48),
                            align_items: AlignItems::Center,
                            padding: UiRect::horizontal(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                        children![(Text::new("Not ready to quit"), TextFont { font_size: 16.0, ..default() }, TextColor(Color::srgb(0.94, 0.96, 0.93)))],
                    ),
                    (
                        Button,
                        QuitChoice(QuitStatus::ReadyToQuit),
                        Node {
                            height: px(48),
                            align_items: AlignItems::Center,
                            padding: UiRect::horizontal(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                        children![(Text::new("Ready to quit"), TextFont { font_size: 16.0, ..default() }, TextColor(Color::srgb(0.94, 0.96, 0.93)))],
                    ),
                    (
                        Button,
                        QuitChoice(QuitStatus::AlreadyQuit),
                        Node {
                            height: px(48),
                            align_items: AlignItems::Center,
                            padding: UiRect::horizontal(px(12)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                        children![(Text::new("Already quit"), TextFont { font_size: 16.0, ..default() }, TextColor(Color::srgb(0.94, 0.96, 0.93)))],
                    ),
                    (
                        Text::new("Choose one to continue."),
                        QuitStatusDisplay,
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.68, 0.74, 0.73)),
                    ),
                (
                    Text::new("Terms of Service"),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.68, 0.74, 0.73)),
                ),
                (
                    Text::new("Replace this placeholder with the terms you want users to review."),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.88, 0.90, 0.87)),
                ),
                (
                    Button,
                    TermsCheckbox,
                    Node {
                        height: px(48),
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(px(12)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                    children![(
                        Text::new("[ ] I have read and accept the Terms of Service"),
                        TermsCheckboxLabel,
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(0.94, 0.96, 0.93)),
                    )],
                ),
                (
                    Button,
                    TermsContinue,
                    Node {
                        height: px(48),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.22, 0.48, 0.39)),
                    children![(
                        Text::new("Continue"),
                        TextFont { font_size: 17.0, ..default() },
                        TextColor(Color::srgb(0.98, 0.99, 0.97)),
                    )],
                ),
                (
                    Text::new(""),
                    TermsStatus,
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.95, 0.57, 0.48)),
                ),
            ],
        )],
    ));
}

pub fn toggle_terms_acceptance(
    interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<TermsCheckbox>),
    >,
    mut form: ResMut<OnboardingForm>,
    mut labels: Query<&mut Text, With<TermsCheckboxLabel>>,
) {
    if !interactions
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        return;
    }
    form.accepted_terms = !form.accepted_terms;
    for mut label in &mut labels {
        label.0 = if form.accepted_terms {
            "[x] I have read and accept the Terms of Service".to_owned()
        } else {
            "[ ] I have read and accept the Terms of Service".to_owned()
        };
    }
}

pub fn despawn_onboarding(
    mut commands: Commands,
    screens: Query<Entity, With<OnboardingScreen>>,
) {
    for screen in &screens {
        commands.entity(screen).despawn();
    }
}

pub fn submit_terms_acceptance(
    interactions: Query<&Interaction, (Changed<Interaction>, With<TermsContinue>)>,
    mut form: ResMut<OnboardingForm>,
    account: Res<AuthenticatedAccount>,
    database: Res<Database>,
    results: Res<TermsAcceptanceResults>,
    mut status: Query<&mut Text, With<TermsStatus>>,
) {
    if form.submitting
        || !interactions
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
    {
        return;
    }
    if !form.accepted_terms {
        for mut text in &mut status {
            text.0 = "Please accept the Terms of Service to continue.".to_owned();
        }
        return;
    }
    let Some(quit_status) = form.quit_status else {
        for mut text in &mut status {
            text.0 = "Choose where you are in your quit journey to continue.".to_owned();
        }
        return;
    };
    let quit_status = quit_status.as_db_value().to_owned();
    form.submitting = true;
    for mut text in &mut status {
        text.0 = "Saving your acceptance…".to_owned();
    }
    let pool = database.0.clone();
    let account_id = account.id;
    let sender = results.sender.clone();
    std::thread::spawn(move || {
        let result = match Builder::new_current_thread().build() {
            Ok(runtime) => runtime.block_on(async {
                match set_quit_status_by_acc_id(&pool, account_id, quit_status).await {
                    Ok(_) => match accept_tos_by_acc_id(&pool, account_id).await {
                        Ok(_) => TermsAcceptanceResult::Saved,
                        Err(error) => {
                            eprintln!("failed to save terms acceptance: {error}");
                            TermsAcceptanceResult::Failed
                        }
                    },
                    Err(error) => {
                        eprintln!("failed to save quit status: {error}");
                        TermsAcceptanceResult::Failed
                    }
                }
            }),
            Err(error) => {
                eprintln!("failed to start onboarding runtime: {error}");
                TermsAcceptanceResult::Failed
            }
        };
        let _ = sender.send(result);
    });
}

pub fn poll_terms_acceptance(
    results: Res<TermsAcceptanceResults>,
    mut form: ResMut<OnboardingForm>,
    mut next_page: ResMut<NextState<AppPage>>,
    mut status: Query<&mut Text, With<TermsStatus>>,
) {
    let result = match results.receiver.lock() {
        Ok(receiver) => match receiver.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                form.submitting = false;
                return;
            }
        },
        Err(_) => {
            form.submitting = false;
            return;
        }
    };
    form.submitting = false;
    match result {
        TermsAcceptanceResult::Saved => next_page.set(AppPage::Dashboard),
        TermsAcceptanceResult::Failed => {
            for mut text in &mut status {
                text.0 = "Could not save your acceptance. Please try again.".to_owned();
            }
        }
    }
}

pub fn select_quit_status(
    interactions: Query<
        (&Interaction, &QuitChoice),
        (Changed<Interaction>, With<Button>),
    >,
    mut form: ResMut<OnboardingForm>,
    mut display: Query<&mut Text, With<QuitStatusDisplay>>,
) {
    for (interaction, choice) in &interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        form.quit_status = Some(choice.0);
        for mut text in &mut display {
            text.0 = match choice.0 {
                QuitStatus::NotReady => "Selected: Not ready to quit",
                QuitStatus::ReadyToQuit => "Selected: Ready to quit",
                QuitStatus::AlreadyQuit => "Selected: Already quit",
            }
            .to_owned();
        }
    }
}
