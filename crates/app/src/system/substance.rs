/* app/src/system/substance.rs
 * The purpose of this module is to provide a substance system.
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


use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::component::substance::{OpenSubstanceButton, SubstanceAddButton, SubstanceFormCancelButton, SubstanceFormErrorText, SubstanceFormKindButton, SubstanceFormKindText, SubstanceFormNameButton, SubstanceFormNameText, SubstanceFormRoot, SubstanceFormSaveButton, SubstanceListText, SubstanceNavigationRoot};
use crate::page::AppPage;
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::substance_resource::{SubstanceEntries, SubstanceEntryLoadResults, SubstanceEntrySaveResults, SubstanceForm};
use std::sync::mpsc::TryRecvError;
use std::thread;
use tokio::runtime::Builder;
use db::substance::model::NewSubstanceModel;


pub fn open_substances(buttons: Query<&Interaction, (Changed<Interaction>, With<OpenSubstanceButton>)>, mut next_page: ResMut<NextState<AppPage>>) {
    if buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        next_page.set(AppPage::Substance);
    }
}

pub fn spawn_substance_navigation(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            padding: UiRect::all(px(20)),
            flex_direction: FlexDirection::Column,
            row_gap: px(12),
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.10, 0.12)),
        Name::new("Substance Page"),
        SubstanceNavigationRoot,
    )).with_children(|page| {
        page.spawn((Text::new("Substances"), TextFont { font_size: 28.0, ..default() }));
        page.spawn((
            Button,
            Node {
                width: px(180),
                height: px(42),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
            SubstanceAddButton,
        )).with_children(|button| {
            button.spawn(Text::new("Add Substance"));
        });
        page.spawn((
            Text::new("Loading substances…"),
            SubstanceListText,
            Node {
                width: percent(100),
                ..default()
            },
        ));
        page.spawn((
            Node {
                width: percent(100),
                padding: UiRect::all(px(16)),
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.14, 0.17)),
            SubstanceFormRoot,
        )).with_children(|form| {
            form.spawn(Text::new("Create Substance"));
            form.spawn((
                Button,
                Node {
                    width: percent(100),
                    min_height: px(40),
                    padding: UiRect::all(px(8)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                SubstanceFormNameButton,
            )).with_children(|button| {
                button.spawn((Text::new("Name: "), SubstanceFormNameText));
            });
            form.spawn((
                Button,
                Node {
                    width: px(180),
                    height: px(40),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                SubstanceFormKindButton,
            )).with_children(|button| {
                button.spawn((Text::new("Type: simple"), SubstanceFormKindText));
            });
            form.spawn((
                Button,
                Node {
                    width: px(120),
                    height: px(40),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                SubstanceFormSaveButton,
            )).with_children(|button| {
                button.spawn(Text::new("Save"));
            });
            form.spawn((
                Button,
                Node {
                    width: px(120),
                    height: px(40),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.32, 0.20, 0.20)),
                SubstanceFormCancelButton,
            )).with_children(|button| {
                button.spawn(Text::new("Cancel"));
            });
            form.spawn((Text::new(""), SubstanceFormErrorText));
        });
    });
}

pub fn request_substance_entries(database: Res<Database>, account: Res<AuthenticatedAccount>, mut entries: ResMut<SubstanceEntries>, results: Res<SubstanceEntryLoadResults>) {
    let acc_id = account.id;
    if entries.loaded_for == Some(acc_id) || entries.loading_for == Some(acc_id) || entries.failed_for == Some(acc_id) {
        return;
    }
    entries.loading_for = Some(acc_id);
    entries.error = None;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime.block_on(db::substance::getters::get_substance_models_by_acc_id(&pool, acc_id)).map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((acc_id, result));
    });
}

pub fn apply_substance_entry_results(results: Res<SubstanceEntryLoadResults>, account: Res<AuthenticatedAccount>, mut entries: ResMut<SubstanceEntries>) {
    let receiver = match results.receiver.lock() {
        Ok(receiver) => receiver,
        Err(error) => {
            entries.error = Some(error.to_string());
            return;
        }
    };
    loop {
        match receiver.try_recv() {
            Ok((acc_id, result)) => {
                if acc_id != account.id || entries.loading_for != Some(acc_id) {
                    continue;
                }
                entries.loading_for = None;
                match result {
                    Ok(substances) => {
                        entries.entries = substances;
                        entries.loaded_for = Some(acc_id);
                        entries.failed_for = None;
                        entries.error = None;
                    }
                    Err(error) => {
                        entries.failed_for = Some(acc_id);
                        entries.error = Some(error);
                    }
                }
            }
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }
}

pub fn toggle_substance_form(open_buttons: Query<&Interaction, (Changed<Interaction>, With<SubstanceAddButton>)>, close_buttons: Query<&Interaction, (Changed<Interaction>, With<SubstanceFormCancelButton>)>, mut form: ResMut<SubstanceForm>, mut nodes: Query<&mut Node, With<SubstanceFormRoot>>) {
    if open_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        form.open = true;
        form.name_focused = true;
        form.error = None;
    }
    if close_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        form.open = false;
    }
    for mut node in &mut nodes {
        node.display = if form.open { Display::Flex } else { Display::None };
    }
}

pub fn focus_substance_name_field(buttons: Query<&Interaction, (Changed<Interaction>, With<SubstanceFormNameButton>)>, mut form: ResMut<SubstanceForm>) {
    if form.open && buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        form.name_focused = true;
    }
}

pub fn cycle_substance_kind(buttons: Query<&Interaction, (Changed<Interaction>, With<SubstanceFormKindButton>)>, mut form: ResMut<SubstanceForm>) {
    if form.open && buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        form.kind = if form.kind == "simple" { "complex".to_owned() } else { "simple".to_owned() };
    }
}

pub fn edit_substance_name(mut keyboard: MessageReader<KeyboardInput>, mut form: ResMut<SubstanceForm>) {
    if !form.open || !form.name_focused {
        return;
    }
    for input in keyboard.read() {
        if input.state != ButtonState::Pressed {
            continue;
        }
        match input.key_code {
            KeyCode::Backspace => {
                form.name.pop();
            }
            KeyCode::Escape => {
                form.name_focused = false;
            }
            _ => {
                if let Some(text) = input.text.as_deref() {
                    form.name.extend(text.chars().filter(|character| !character.is_control()));
                }
            }
        }
    }
}

pub fn save_substance(buttons: Query<&Interaction, (Changed<Interaction>, With<SubstanceFormSaveButton>)>, mut form: ResMut<SubstanceForm>, account: Res<AuthenticatedAccount>, database: Res<Database>, results: Res<SubstanceEntrySaveResults>) {
    if !form.open || !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }
    let name = form.name.trim().to_owned();
    if name.is_empty() {
        form.error = Some("Enter a substance name.".to_owned());
        return;
    }
    let acc_id = account.id;
    let new_substance = NewSubstanceModel {
        acc_id,
        name,
        kind: form.kind.clone(),
    };
    let pool = database.0.clone();
    let sender = results.sender.clone();
    form.error = None;
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime.block_on(db::substance::setters::insert_substance(&pool, new_substance)).map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((acc_id, result));
    });
}

pub fn apply_substance_entry_save_results(results: Res<SubstanceEntrySaveResults>, account: Res<AuthenticatedAccount>, mut form: ResMut<SubstanceForm>, mut entries: ResMut<SubstanceEntries>) {
    let receiver = match results.receiver.lock() {
        Ok(receiver) => receiver,
        Err(error) => {
            form.error = Some(error.to_string());
            return;
        }
    };
    loop {
        match receiver.try_recv() {
            Ok((acc_id, result)) if acc_id == account.id => match result {
                Ok(_) => {
                    form.open = false;
                    form.name.clear();
                    form.kind = "simple".to_owned();
                    form.name_focused = true;
                    form.error = None;
                    entries.loaded_for = None;
                    entries.loading_for = None;
                    entries.failed_for = None;
                    entries.error = None;
                }
                Err(error) => {
                    form.error = Some(error);
                }
            },
            Ok(_) => {}
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }
}

pub fn update_substance_text(
    entries: Res<SubstanceEntries>,
    account: Res<AuthenticatedAccount>,
    form: Res<SubstanceForm>,
    mut list_texts: Query<&mut Text, (With<SubstanceListText>, Without<SubstanceFormNameText>, Without<SubstanceFormKindText>, Without<SubstanceFormErrorText>)>,
    mut name_texts: Query<&mut Text, (With<SubstanceFormNameText>, Without<SubstanceListText>, Without<SubstanceFormKindText>, Without<SubstanceFormErrorText>)>,
    mut kind_texts: Query<&mut Text, (With<SubstanceFormKindText>, Without<SubstanceListText>, Without<SubstanceFormNameText>, Without<SubstanceFormErrorText>)>,
    mut error_texts: Query<&mut Text, (With<SubstanceFormErrorText>, Without<SubstanceListText>, Without<SubstanceFormNameText>, Without<SubstanceFormKindText>)>,
) {
    let content = if let Some(error) = &entries.error {
        format!("Could not load substances: {error}")
    } else if entries.loading_for == Some(account.id) {
        "Loading substances…".to_owned()
    } else if entries.loaded_for != Some(account.id) {
        "Substances have not been loaded.".to_owned()
    } else if entries.entries.is_empty() {
        "No substances yet. Select Add Substance to create one.".to_owned()
    } else {
        entries.entries.iter().map(|substance| format!("{} ({})", substance.name, substance.kind)).collect::<Vec<_>>().join("\n")
    };
    for mut text in &mut list_texts {
        text.0 = content.clone();
    }
    for mut text in &mut name_texts {
        text.0 = format!("Name: {}{}", form.name, if form.name_focused { "  |" } else { "" });
    }
    for mut text in &mut kind_texts {
        text.0 = format!("Type: {}", form.kind);
    }
    for mut text in &mut error_texts {
        text.0 = form.error.clone().unwrap_or_default();
    }
}
