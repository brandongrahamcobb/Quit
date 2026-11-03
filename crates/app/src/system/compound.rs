/* app/src/system/compound.rs
 * The purpose of this module is to provide a compound system.
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
use db::compound::getters::{get_all_compound_models, get_metabolites_by_compound_id};
use db::compound::model::NewCompoundModel;
use db::compound::setters::{delete_compound, insert_compound, set_compound_metabolites, update_compound};
use crate::component::compound::{CompoundAddButton, CompoundCancelButton, CompoundCommonNameButton, CompoundCommonNameText, CompoundDeleteButton, CompoundDeleteCancelButton, CompoundDeleteConfirmButton, CompoundDeleteDialog, CompoundDeleteErrorText, CompoundDeleteNameText, CompoundEditButton, CompoundErrorText, CompoundFormOverlay, CompoundFormRoot, CompoundHalfLifeButton, CompoundHalfLifeText, CompoundImagePreview, CompoundImagePreviewHandle, CompoundImageRemoveButton, CompoundImageUploadButton, CompoundIupacNameButton, CompoundIupacNameText, CompoundListContainer, CompoundListItem, CompoundListText, CompoundMetaboliteItem, CompoundMetaboliteListContainer, CompoundMetaboliteListText, CompoundMetaboliteToggleButton, CompoundNavigationRoot, CompoundPreviewButton, CompoundSaveButton, CompoundSearchButton, CompoundSearchText, OpenCompoundsButton};
use crate::page::AppPage;
use crate::resource::compound_resource::{CompoundDeleteResults, CompoundEntry, CompoundFormField, CompoundImageUploadReceiver, CompoundLoadReceiver, CompoundMetaboliteLoadResults, CompoundPageState, CompoundSaveResults};
use crate::resource::custom_resource::Database;
use crate::resource::login_resource::AuthenticatedAccount;
use std::sync::mpsc::{TryRecvError, channel};
use std::thread;
use tokio::runtime::Builder;

pub fn open_compounds(buttons: Query<&Interaction, (Changed<Interaction>, With<OpenCompoundsButton>)>, mut next_page: ResMut<NextState<AppPage>>) {
    if buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        next_page.set(AppPage::Compounds);
    }
}

pub fn begin_compound_load(
    database: Res<Database>,
    mut state: ResMut<CompoundPageState>,
    receiver: Res<CompoundLoadReceiver>,
) {
    if state.loading {
        return;
    }
    state.loading = true;
    state.error = None;
    let pool = database.0.clone();
    let (sender, result_receiver) = channel();
    if let Ok(mut slot) = receiver.0.lock() {
        *slot = Some(result_receiver);
    } else {
        state.loading = false;
        state.error = Some("Could not initialize compound loading.".to_string());
        return;
    }
    thread::spawn(move || {
        let result = match Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime
                .block_on(get_all_compound_models(&pool))
                .map(|models| {
                    models
                        .into_iter()
                        .map(|model| CompoundEntry {
                            id: model.id,
                            common_name: model.common_name,
                            iupac_name: model.iupac_name,
                            half_life_seconds: model.half_life_seconds,
                            image_path: model.image_path,
                        })
                        .collect::<Vec<_>>()
                })
                .map_err(|error| format!("{error:?}")),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn poll_compound_load(
    receiver: Res<CompoundLoadReceiver>,
    mut state: ResMut<CompoundPageState>,
) {
    let result = {
        let Ok(mut slot) = receiver.0.lock() else {
            return;
        };
        let Some(result_receiver) = slot.as_ref() else {
            return;
        };
        match result_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Compound loading worker disconnected.".to_string()))
            }
        }
    };
    let Some(result) = result else {
        return;
    };
    state.loading = false;
    match result {
        Ok(compounds) => {
            state.compounds = compounds;
            state.metabolites = state.compounds.clone();
            state.error = None;
        }
        Err(error) => {
            state.error = Some(error);
        }
    }
}

pub fn spawn_compound_navigation(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                padding: UiRect::all(px(20)),
                flex_direction: FlexDirection::Column,
                row_gap: px(16),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.10, 0.12)),
            Name::new("Compound Page"),
            CompoundNavigationRoot,
        ))
        .with_children(|page| {
            page.spawn((
                Text::new("Compounds"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
            ));
            page.spawn(Node {
                width: percent(100),
                flex_direction: FlexDirection::Row,
                column_gap: px(12),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Button,
                    Node {
                        flex_grow: 1.0,
                        min_width: px(100),
                        height: px(42),
                        padding: UiRect::all(px(10)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.16, 0.18, 0.21)),
                    CompoundSearchButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Search compounds..."),
                        CompoundSearchText,
                    ));
                });
                header.spawn((
                    Button,
                    Node {
                        width: px(42),
                        height: px(42),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                    CompoundAddButton,
                ))
                .with_children(|button| {
                    button.spawn(Text::new("+"));
                });
            });
            page.spawn((
                Text::new("Loading compounds..."),
                CompoundListText,
            ));
            page.spawn((
                Node {
                    width: percent(100),
                    flex_grow: 1.0,
                    min_height: px(0),
                    flex_direction: FlexDirection::Column,
                    row_gap: px(6),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
                CompoundListContainer,
            ));
            page.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(100),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                CompoundFormOverlay,
            ))
            .with_children(|overlay| {
                overlay.spawn((
                    Node {
                        width: percent(75),
                        max_width: px(650),
                        max_height: percent(85),
                        padding: UiRect::all(px(20)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(12),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.14, 0.17)),
                    CompoundFormRoot,
                ))
                .with_children(|form| {
                    form.spawn(Text::new("Compound Editor"));
                    form.spawn((
                        Button,
                        Node {
                            width: percent(100),
                            min_height: px(40),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                        CompoundCommonNameButton,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Common name: "),
                            CompoundCommonNameText,
                        ));
                    });
                    form.spawn((
                        Button,
                        Node {
                            width: percent(100),
                            min_height: px(40),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                        CompoundIupacNameButton,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("IUPAC name: "),
                            CompoundIupacNameText,
                        ));
                    });
                    form.spawn((
                        Button,
                        Node {
                            width: percent(100),
                            min_height: px(40),
                            padding: UiRect::all(px(8)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.18, 0.20, 0.23)),
                        CompoundHalfLifeButton,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Half-life (seconds): "),
                            CompoundHalfLifeText,
                        ));
                    });
                    form.spawn((
                        Text::new("Metabolites:"),
                        CompoundMetaboliteListText,
                    ));
                    form.spawn((
                        Node {
                            width: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(6),
                            ..default()
                        },
                        CompoundMetaboliteListContainer,
                    ));
                    form.spawn((
                        Button,
                        Node {
                            width: px(180),
                            height: px(38),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.16, 0.38, 0.28)),
                        CompoundImageUploadButton,
                    )).with_children(|button| {
                        button.spawn(Text::new("Upload Structure"));
                    });
                    form.spawn((
                        Button,
                        Node {
                            width: px(180),
                            height: px(38),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.45, 0.18, 0.18)),
                        CompoundImageRemoveButton,
                    )).with_children(|button| {
                        button.spawn(Text::new("Remove Structure"));
                    });
                    form.spawn((
                        Node {
                            width: percent(100),
                            height: px(240),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.08, 0.09, 0.11)),
                        CompoundImagePreview,
                    ));
                    form.spawn((Text::new(""), CompoundErrorText));
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
                        CompoundSaveButton,
                    ))
                    .with_children(|button| {
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
                        CompoundCancelButton,
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new("Cancel"));
                    });
                });
            });
            page.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(100),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
                CompoundDeleteDialog,
            ))
            .with_children(|dialog| {
                dialog.spawn((
                    Node {
                        width: px(360),
                        padding: UiRect::all(px(20)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(12),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.17, 0.20)),
                ))
                .with_children(|panel| {
                    panel.spawn(Text::new("Delete compound?"));
                    panel.spawn((Text::new(""), CompoundDeleteNameText));
                    panel.spawn((Text::new(""), CompoundDeleteErrorText));
                    panel.spawn((
                        Button,
                        CompoundDeleteConfirmButton,
                    )).with_children(|button| {
                        button.spawn(Text::new("Confirm Delete"));
                    });
                    panel.spawn((
                        Button,
                        CompoundDeleteCancelButton,
                    )).with_children(|button| {
                        button.spawn(Text::new("Cancel"));
                    });
                });
            });
        });
}

pub fn update_compound_list(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<CompoundPageState>,
    mut list_texts: Query<&mut Text, With<CompoundListText>>,
    containers: Query<Entity, With<CompoundListContainer>>,
    old_items: Query<Entity, With<CompoundListItem>>,
    mut rendered: Local<Option<(Entity, Vec<(i32, String, Option<String>)>, String, Option<i32>)>>,
) {
    let query = state.search_query.trim().to_lowercase();
    let filtered: Vec<_> = state.compounds.iter().filter(|compound| compound.common_name.to_lowercase().contains(&query)).collect();
    let status = if let Some(error) = &state.error {
        format!("Could not load compounds: {error}")
    } else if state.loading {
        "Loading compounds...".to_owned()
    } else if state.compounds.is_empty() {
        "No compounds yet. Select Add Compound to create one.".to_owned()
    } else {
        format!("{} of {} compounds", filtered.len(), state.compounds.len())
    };
    for mut text in &mut list_texts {
        text.0 = status.clone();
    }
    let Ok(container) = containers.single() else {
        return;
    };
    let signature: Vec<(i32, String, Option<String>)> = filtered.iter().map(|compound| (compound.id, compound.common_name.clone(), compound.image_path.clone())).collect();
    if rendered.as_ref() == Some(&(container, signature.clone(), query.clone(), state.expanded_preview_id)) {
        return;
    }
    for entity in &old_items {
        commands.entity(entity).despawn();
    }
    commands.entity(container).with_children(|list| {
        for compound in &filtered {
            list.spawn((
                Node {
                    width: percent(100),
                    min_height: px(48),
                    padding: UiRect::all(px(8)),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(8),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.17, 0.20)),
                CompoundListItem,
            ))
            .with_children(|row| {
                row.spawn((
                    Text::new(compound.common_name.clone()),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    Node {
                        flex_grow: 1.0,
                        ..default()
                    },
                ));
                row.spawn((
                    Button,
                    Node {
                        width: px(36),
                        height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.30, 0.30, 0.30)),
                    CompoundPreviewButton(compound.id),
                ))
                .with_children(|button| {
                    button.spawn(Text::new("V"));
                });
                row.spawn((
                    Button,
                    Node {
                        width: px(36),
                        height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.22, 0.30, 0.42)),
                    CompoundEditButton(compound.id),
                ))
                .with_children(|button| {
                    button.spawn(Text::new("E"));
                });
                row.spawn((
                    Button,
                    Node {
                        width: px(36),
                        height: px(36),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.48, 0.20, 0.20)),
                    CompoundDeleteButton(compound.id),
                ))
                .with_children(|button| {
                    button.spawn(Text::new("-"));
                });
            });
            if state.expanded_preview_id == Some(compound.id) {
                list.spawn((
                    Node {
                        width: percent(100),
                        min_height: px(220),
                        padding: UiRect::all(px(12)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.11, 0.13, 0.16)),
                    CompoundListItem,
                ))
                .with_children(|preview| {
                    if let Some(path) = &compound.image_path {
                        preview.spawn((
                            ImageNode::new(asset_server.load(path.clone())),
                            Node {
                                width: percent(100),
                                height: px(220),
                                ..default()
                            },
                        ));
                    } else {
                        preview.spawn(Text::new("No structural image available."));
                    }
                });
            }
        }
    });
    *rendered = Some((container, signature, query, state.expanded_preview_id));
}

pub fn handle_compound_form_buttons(
    add_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CompoundAddButton>),
    >,
    edit_buttons: Query<
        (&Interaction, &CompoundEditButton),
        Changed<Interaction>,
    >,
    cancel_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<CompoundCancelButton>),
    >,
    mut state: ResMut<CompoundPageState>,
    mut form_overlay: Query<&mut Node, With<CompoundFormOverlay>>,
) {
    let Ok(mut overlay) = form_overlay.single_mut() else {
        return;
    };
    let add_pressed = add_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    let edit_id = edit_buttons
        .iter()
        .find(|(interaction, _)| **interaction == Interaction::Pressed)
        .map(|(_, button)| button.0);
    let cancel_pressed = cancel_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    if cancel_pressed {
        overlay.display = Display::None;
        state.error = None;
        return;
    }
    if add_pressed {
        state.editing_id = None;
        state.common_name.clear();
        state.iupac_name.clear();
        state.half_life_seconds.clear();
        state.image_path = None;
        state.focused_field = Some(CompoundFormField::CommonName);
        state.error = None;
        overlay.display = Display::Flex;
    } else if let Some(id) = edit_id {
        let compound = state
            .compounds
            .iter()
            .find(|compound| compound.id == id)
            .cloned();
        if let Some(compound) = compound {
            state.editing_id = Some(compound.id);
            state.common_name = compound.common_name;
            state.iupac_name = compound.iupac_name;
            state.half_life_seconds = compound
                .half_life_seconds
                .map(|seconds| seconds.to_string())
                .unwrap_or_default();
            state.image_path = compound.image_path.clone();
            state.focused_field = Some(CompoundFormField::CommonName);
            state.error = None;
            overlay.display = Display::Flex;
        }
    }
}

pub fn focus_compound_fields(
    common_name_buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundCommonNameButton>)>,
    half_life_buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundHalfLifeButton>)>,
    iupac_name_buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundIupacNameButton>)>,
    search_buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundSearchButton>)>,
    mut state: ResMut<CompoundPageState>,
    time: Res<Time>,
) {
    if common_name_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(CompoundFormField::CommonName);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if half_life_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(CompoundFormField::HalfLife);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if iupac_name_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(CompoundFormField::IupacName);
        state.caret_started_at = time.elapsed_secs_f64();
    }
    if search_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.focused_field = Some(CompoundFormField::Search);
        state.caret_started_at = time.elapsed_secs_f64();
    }
}

pub fn edit_compound_fields(
    mut keyboard: MessageReader<KeyboardInput>,
    mut state: ResMut<CompoundPageState>,
    time: Res<Time>,
) {
    let Some(mut field) = state.focused_field else {
        return;
    };
    for input in keyboard.read() {
        if input.state != ButtonState::Pressed {
            continue;
        }
        match input.key_code {
            KeyCode::Escape => {
                state.focused_field = None;
            }
            KeyCode::Backspace => {
                match field {
                    CompoundFormField::CommonName => { state.common_name.pop(); }
                    CompoundFormField::HalfLife => { state.half_life_seconds.pop(); }
                    CompoundFormField::IupacName => { state.iupac_name.pop(); }
                    CompoundFormField::Search => { state.search_query.pop(); }
                }
            }
            _ => {
                let Some(text) = input.text.as_deref() else {
                    continue;
                };
                match field {
                    CompoundFormField::CommonName => {
                        state.common_name.extend(text.chars().filter(|c| !c.is_control()));
                        state.caret_started_at = time.elapsed_secs_f64();
                    }
                    CompoundFormField::HalfLife => {
                        state.half_life_seconds.extend(text.chars().filter(|c| c.is_ascii_digit()));
                        state.caret_started_at = time.elapsed_secs_f64();
                    }
                    CompoundFormField::IupacName => {
                        state.iupac_name.extend(text.chars().filter(|c| !c.is_control()));
                        state.caret_started_at = time.elapsed_secs_f64();
                    }
                    CompoundFormField::Search => {
                        state.search_query.extend(text.chars().filter(|c| !c.is_control()));
                        state.caret_started_at = time.elapsed_secs_f64();
                    }
                }
            }
        }
    }
}



pub fn save_compound(
    buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundSaveButton>)>,
    mut state: ResMut<CompoundPageState>,
    database: Res<Database>,
    results: Res<CompoundSaveResults>,
) {
    if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }
    let common_name = state.common_name.trim().to_owned();
    let half_life_input = state.half_life_seconds.trim().to_owned();
    let iupac_name = state.iupac_name.trim().to_owned();
    let editing_id = state.editing_id;
    let image_path = state.image_path.clone();
    let metabolite_ids = state.selected_metabolite_ids.clone();
    if common_name.is_empty() {
        state.error = Some("Enter a common name.".to_owned());
        return;
    }
    if iupac_name.is_empty() {
        state.error = Some("Enter an IUPAC name.".to_owned());
        return;
    }
    let half_life_seconds = if half_life_input.is_empty() {
        None
    } else {
        match half_life_input.parse::<i64>() {
            Ok(value) if value >= 0 => Some(value),
            _ => {
                state.error = Some("Half-life must be a nonnegative whole number of seconds.".to_owned());
                return;
            }
        }
    };
    let new_compound = NewCompoundModel {
        common_name,
        iupac_name,
        half_life_seconds,
        image_path,
    };
    let pool = database.0.clone();
    let sender = results.sender.clone();
    state.error = None;
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime.block_on(async {
                let compound_id = if let Some(id) = editing_id {
                    update_compound(
                        &pool,
                        id,
                        new_compound.common_name,
                        new_compound.iupac_name,
                        new_compound.half_life_seconds,
                        new_compound.image_path,
                    )
                    .await
                    .map_err(|error| error.to_string())?;
                    id
                } else {
                    insert_compound(&pool, new_compound)
                        .await
                        .map_err(|error| error.to_string())?
                        .id
                };
                set_compound_metabolites(&pool, compound_id, metabolite_ids)
                    .await
                    .map_err(|error| format!("Failed to save metabolites: {error}"))?;
                let models = get_all_compound_models(&pool)
                    .await
                    .map_err(|error| error.to_string())?;
                Ok::<Vec<CompoundEntry>, String>(
                    models.into_iter().map(|model| CompoundEntry {
                        id: model.id,
                        common_name: model.common_name,
                        iupac_name: model.iupac_name,
                        half_life_seconds: model.half_life_seconds,
                        image_path: model.image_path,
                    }).collect(),
                )
            }),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn apply_compound_save_results(
    results: Res<CompoundSaveResults>,
    mut state: ResMut<CompoundPageState>,
    mut form_nodes: Query<&mut Node, With<CompoundFormRoot>>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        state.error = Some("Could not read the compound save result.".to_owned());
        return;
    };
    loop {
        match receiver.try_recv() {
            Ok(Ok(compounds)) => {
                state.compounds = compounds;
                state.editing_id = None;
                state.common_name.clear();
                state.iupac_name.clear();
                state.half_life_seconds.clear();
                state.selected_metabolite_ids.clear();
                state.focused_field = None;
                state.error = None;
                for mut node in &mut form_nodes {
                    node.display = Display::None;
                }
            }
            Ok(Err(error)) => {
                state.error = Some(error);
            }
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
        }
    }
}


pub fn update_compound_form_text(
    state: Res<CompoundPageState>,
    time: Res<Time>,
    mut common_name_text: Query<&mut Text, (With<CompoundCommonNameText>, Without<CompoundIupacNameText>, Without<CompoundHalfLifeText>, Without<CompoundSearchText>, Without<CompoundErrorText>)>,
    mut error_text: Query<&mut Text, (With<CompoundErrorText>, Without<CompoundCommonNameText>, Without<CompoundIupacNameText>, Without<CompoundHalfLifeText>, Without<CompoundSearchText>)>,
    mut half_life_text: Query<&mut Text, (With<CompoundHalfLifeText>, Without<CompoundCommonNameText>, Without<CompoundIupacNameText>, Without<CompoundSearchText>, Without<CompoundErrorText>)>,
    mut iupac_name_text: Query<&mut Text, (With<CompoundIupacNameText>, Without<CompoundCommonNameText>, Without<CompoundHalfLifeText>, Without<CompoundSearchText>, Without<CompoundErrorText>)>,
    mut search_text: Query<&mut Text, (With<CompoundSearchText>, Without<CompoundCommonNameText>, Without<CompoundIupacNameText>, Without<CompoundHalfLifeText>, Without<CompoundErrorText>)>,
) {
    let blink_on = (time.elapsed_secs_f64() - state.caret_started_at) % 1.0 < 0.5;
    for mut text in &mut common_name_text {
        let active = state.focused_field == Some(CompoundFormField::CommonName);
        let value = if state.common_name.is_empty() && !active {
            "".to_owned()
        } else {
            state.common_name.clone()
        };
        **text = format!("Common name: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut iupac_name_text {
        let active = state.focused_field == Some(CompoundFormField::IupacName);
        let value = if state.iupac_name.is_empty() && !active {
            "".to_owned()
        } else {
            state.iupac_name.clone()
        };
        **text = format!("IUPAC name: {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut half_life_text {
        let active = state.focused_field == Some(CompoundFormField::HalfLife);
        let value = if state.half_life_seconds.is_empty() && !active {
            "".to_owned()
        } else {
            state.half_life_seconds.clone()
        };
        **text = format!("Half-life (seconds): {}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut search_text {
        let active = state.focused_field == Some(CompoundFormField::Search);
        let value = if state.search_query.is_empty() && !active {
            "Search compounds...".to_owned()
        } else {
            state.search_query.clone()
        };
        **text = format!("{}{}", value, if active && blink_on { "|" } else { "" });
    }
    for mut text in &mut error_text {
        **text = state.error.clone().unwrap_or_default();
    }
}

pub fn update_compound_metabolite_buttons(
    mut commands: Commands,
    state: Res<CompoundPageState>,
    list_container: Query<Entity, With<CompoundMetaboliteListContainer>>,
    old_items: Query<Entity, With<CompoundMetaboliteItem>>,
    mut previous_signature: Local<Option<(Option<i32>, Vec<(i32, String, bool)>)>>,
) {
    let Ok(container) = list_container.single() else {
        return;
    };
    let choices: Vec<(i32, String, bool)> = state.metabolites.iter()
        .filter(|compound| Some(compound.id) != state.editing_id)
        .map(|compound| (compound.id, compound.common_name.clone(), state.selected_metabolite_ids.contains(&compound.id)))
        .collect();
    let signature = (state.editing_id, choices.clone());
    if previous_signature.as_ref() == Some(&signature) {
        return;
    }
    *previous_signature = Some(signature);
    for entity in &old_items {
        commands.entity(entity).despawn();
    }
    commands.entity(container).with_children(|parent| {
        for (id, name, selected) in choices {
            let label = format!("{} {}", if selected { "[x]" } else { "[ ]" }, name);
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(if selected { Color::srgb(0.2, 0.45, 0.25) } else { Color::srgb(0.2, 0.2, 0.2) }),
                CompoundMetaboliteItem,
                CompoundMetaboliteToggleButton(id),
            )).with_children(|button| {
                button.spawn((
                    Text::new(label),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        }
    });
}

pub fn toggle_compound_metabolites(
    mut interactions: Query<
        (&Interaction, &CompoundMetaboliteToggleButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<CompoundPageState>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let id = button.0;
        if state.selected_metabolite_ids.contains(&id) {
            state.selected_metabolite_ids.retain(|&selected| selected != id);
        } else {
            state.selected_metabolite_ids.push(id);
        }
        state.error = None;
    }
}

pub fn begin_compound_metabolite_load(
    mut commands: Commands,
    buttons: Query<
        (&Interaction, &CompoundEditButton),
        (Changed<Interaction>, With<Button>),
    >,
    state: Res<CompoundPageState>,
    database: Res<Database>,
    results: Res<CompoundMetaboliteLoadResults>,
) {
    for (interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let compound_id = button.0;
        if state.editing_id != Some(compound_id) {
            continue;
        }
        let pool = database.0.clone();
        let sender = results.sender.clone();
        thread::spawn(move || {
            let runtime = Builder::new_current_thread()
                .enable_all()
                .build();
            let result = match runtime {
                Ok(runtime) => runtime.block_on(async move {
                    get_metabolites_by_compound_id(&pool, compound_id)
                        .await
                        .map(|models| {
                            (
                                compound_id,
                                models.into_iter().map(|model| model.id).collect(),
                            )
                        })
                        .map_err(|error| format!("{error:?}"))
                }),
                Err(error) => Err(error.to_string()),
            };
            let _ = sender.send(result);
        });
    }
}

pub fn apply_compound_metabolite_load_results(
    results: Res<CompoundMetaboliteLoadResults>,
    mut state: ResMut<CompoundPageState>,
) {
    let Ok(receiver) = results.receiver.lock() else {
        return;
    };
    while let Ok(result) = receiver.try_recv() {
        match result {
            Ok((compound_id, metabolite_ids)) => {
                if state.editing_id == Some(compound_id) {
                    state.selected_metabolite_ids = metabolite_ids;
                    state.error = None;
                }
            }
            Err(error) => {
                state.error = Some(format!(
                    "Could not load existing metabolites: {error}"
                ));
            }
        }
    }
}

pub fn begin_compound_image_upload(
    buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundImageUploadButton>)>,
    mut receiver: ResMut<CompoundImageUploadReceiver>,
    mut state: ResMut<CompoundPageState>,
) {
    if state.image_loading || !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }
    let (sender, channel_receiver) = std::sync::mpsc::channel();
    *receiver.0.lock().unwrap() = Some(channel_receiver);
    state.image_loading = true;
    thread::spawn(move || {
        let result = (|| {
            let Some(source) = rfd::FileDialog::new()
                .add_filter("Structure images", &["png", "jpg", "jpeg", "webp"])
                .pick_file()
            else {
                return Ok(None);
            };
            let extension = source.extension()
                .and_then(|extension| extension.to_str())
                .map(str::to_ascii_lowercase)
                .filter(|extension| ["png", "jpg", "jpeg", "webp"].contains(&extension.as_str()))
                .ok_or_else(|| "Unsupported image format.".to_owned())?;
            let directory = std::path::Path::new("assets/compound_images");
            std::fs::create_dir_all(directory)
                .map_err(|error| format!("Could not create image directory: {error}"))?;
            let filename = format!(
                "structure_{}.{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|error| error.to_string())?
                    .as_nanos(),
                extension
            );
            let asset_path = format!("compound_images/{filename}");
            let destination = directory.join(&filename);
            std::fs::copy(&source, &destination)
                .map_err(|error| format!("Could not copy image: {error}"))?;
            Ok(Some(asset_path))
        })();
        let _ = sender.send(result);
    });
}

pub fn poll_compound_image_upload(
    mut receiver: ResMut<CompoundImageUploadReceiver>,
    mut state: ResMut<CompoundPageState>,
) {
    let result = {
        let mut slot = receiver.0.lock().unwrap();
        let Some(channel_receiver) = slot.as_ref() else {
            return;
        };
        match channel_receiver.try_recv() {
            Ok(result) => {
                *slot = None;
                Some(result)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                *slot = None;
                Some(Err("Image upload operation failed.".to_owned()))
            }
        }
    };
    if let Some(result) = result {
        state.image_loading = false;
        match result {
            Ok(Some(path)) => {
                state.image_path = Some(path);
                state.error = None;
            }
            Ok(None) => {}
            Err(error) => {
                state.error = Some(error);
            }
        }
    }
}

pub fn remove_compound_image(
    buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundImageRemoveButton>)>,
    mut state: ResMut<CompoundPageState>,
) {
    if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        return;
    }
    state.image_path = None;
    state.error = None;
}

pub fn update_compound_image_preview(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<CompoundPageState>,
    previews: Query<(Entity, Option<&CompoundImagePreviewHandle>), With<CompoundImagePreview>>,
) {
    let Ok((entity, current_handle)) = previews.single() else {
        return;
    };
    let desired_path = state.image_path.as_deref();
    let current_path = current_handle.and_then(|component| component.0.path().map(|path| path.to_string()));
    if current_path.as_deref() == desired_path {
        return;
    }
    if let Some(path) = desired_path {
        let image = asset_server.load::<Image>(path.to_owned());
        commands.entity(entity).insert((
            ImageNode::new(image.clone()),
            CompoundImagePreviewHandle(image),
        ));
    } else {
        commands.entity(entity).remove::<(ImageNode, CompoundImagePreviewHandle)>();
    }
}

pub fn toggle_compound_preview(
    mut buttons: Query<(&Interaction, &CompoundPreviewButton), Changed<Interaction>>,
    mut state: ResMut<CompoundPageState>,
) {
    for (interaction, button) in &mut buttons {
        if *interaction == Interaction::Pressed {
            state.expanded_preview_id = if state.expanded_preview_id == Some(button.0) { None } else { Some(button.0) };
        }
    }
}

pub fn handle_compound_delete_buttons(
    mut cancel_buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundDeleteCancelButton>)>,
    mut delete_buttons: Query<(&Interaction, &CompoundDeleteButton), Changed<Interaction>>,
    mut dialog: Query<&mut Node, With<CompoundDeleteDialog>>,
    mut state: ResMut<CompoundPageState>,
) {
    if cancel_buttons.iter().any(|interaction| *interaction == Interaction::Pressed) {
        state.deleting_id = None;
        state.deleting_name = None;
        state.delete_error = None;
        if let Ok(mut node) = dialog.single_mut() {
            node.display = Display::None;
        }
    }
    for (interaction, button) in &mut delete_buttons {
        if *interaction == Interaction::Pressed {
            if state.deleting {
                continue;
            }
            let compound = state.compounds.iter().find(|compound| compound.id == button.0).map(|compound| (compound.id, compound.common_name.clone()));
            if let Some((id, name)) = compound {
                state.deleting_id = Some(id);
                state.deleting_name = Some(name);
                state.delete_error = None;
                if let Ok(mut node) = dialog.single_mut() {
                    node.display = Display::Flex;
                }
            }
            // if let Some(compound) = state.compounds.iter().find(|compound| compound.id == button.0) {
            //     state.deleting_id = Some(compound.id);
            //     state.deleting_name = Some(compound.common_name.clone());
            //     state.delete_error = None;
            //     if let Ok(mut node) = dialog.single_mut() {
            //         node.display = Display::Flex;
            //     }
            // }
        }
    }
}

pub fn begin_compound_delete(
    mut buttons: Query<&Interaction, (Changed<Interaction>, With<CompoundDeleteConfirmButton>)>,
    database: Res<Database>,
    mut results: ResMut<CompoundDeleteResults>,
    mut state: ResMut<CompoundPageState>,
) {
    if !buttons.iter().any(|interaction| *interaction == Interaction::Pressed) || state.deleting {
        return;
    }
    let Some(id) = state.deleting_id else {
        return;
    };
    let pool = database.0.clone();
    let (sender, receiver) = std::sync::mpsc::channel();
    *results.0.lock().unwrap() = Some(receiver);
    state.deleting = true;
    state.delete_error = None;
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let result = match runtime {
            Ok(runtime) => runtime.block_on(delete_compound(&pool, id)).map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send((id, result));
    });
}

pub fn poll_compound_delete_results(
    mut dialog: Query<&mut Node, With<CompoundDeleteDialog>>,
    mut results: ResMut<CompoundDeleteResults>,
    mut state: ResMut<CompoundPageState>,
) {
    let result = {
        let mut receiver = results.0.lock().unwrap();
        receiver.as_ref().and_then(|receiver| receiver.try_recv().ok())
    };
    let Some((id, result)) = result else {
        return;
    };
    *results.0.lock().unwrap() = None;
    state.deleting = false;
    match result {
        Ok(()) => {
            state.compounds.retain(|compound| compound.id != id);
            state.expanded_preview_id = if state.expanded_preview_id == Some(id) { None } else { state.expanded_preview_id };
            state.deleting_id = None;
            state.deleting_name = None;
            state.delete_error = None;
            if let Ok(mut node) = dialog.single_mut() {
                node.display = Display::None;
            }
        }
        Err(error) => {
            state.delete_error = Some(error);
        }
    }
}

pub fn update_compound_delete_dialog_text(
    state: Res<CompoundPageState>,
    mut error_text: Query<&mut Text, (With<CompoundDeleteErrorText>, Without<CompoundDeleteNameText>)>,
    mut name_text: Query<&mut Text, (With<CompoundDeleteNameText>, Without<CompoundDeleteErrorText>)>,
) {
    for mut text in &mut error_text {
        text.0 = state.delete_error.clone().unwrap_or_default();
    }
    for mut text in &mut name_text {
        text.0 = state.deleting_name.as_ref().map(|name| format!("Are you sure you want to delete {}?", name)).unwrap_or_default();
    }
}
