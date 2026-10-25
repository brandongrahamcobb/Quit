/* app/src/system/settings_wallpaper.rs
 * The purpose of this module is to provide a wallpaper settings system.
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
use bevy::ui::{FocusPolicy, RelativeCursorPosition};
use tokio::runtime::Builder;
use std::path::Path;
use std::sync::mpsc::TryRecvError;
use std::thread;

use crate::component::settings::{
    WallpaperBackground, WallpaperChoicesList, WallpaperCloseButton, WallpaperImageChoice, WallpaperModal, WallpaperModeChoice, WallpaperModeDropdown, WallpaperModeMenu, WallpaperOpenButton, WallpaperStretchFill, WallpaperStretchLabel, WallpaperStretchSlider, WallpaperTileXButton, WallpaperTileXKnob, WallpaperTileYButton, WallpaperTileYKnob,
};
use crate::resource::custom_resource::Database;
use crate::resource::settings_resource::{
    WallpaperFiles, WallpaperMode, WallpaperSettings,
};
use crate::resource::login_resource::AuthenticatedAccount;
use crate::resource::settings_resource::WallpaperSettingsLoadResults;

#[derive(Default)]
struct WallpaperSaveDebounce {
    dirty: bool,
    timer: Option<Timer>,
}

pub fn toggle_wallpaper_modal(
    open_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<WallpaperOpenButton>),
    >,
    close_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<WallpaperCloseButton>),
    >,
    mut modal: Query<&mut Node, With<WallpaperModal>>,
) {
    let open = open_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    let close = close_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    for mut node in &mut modal {
        if close {
            node.display = Display::None;
        } else if open {
            node.display = Display::Flex;
        }
    }
}

fn collect_pngs(directory: &Path, asset_root: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_pngs(&path, asset_root, files)?;
        } else if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
        {
            if let Ok(relative_path) = path.strip_prefix(asset_root) {
                files.push(relative_path.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    Ok(())
}

pub fn discover_wallpaper_files(mut files: ResMut<WallpaperFiles>) {
    files.0.clear();
    let asset_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    if let Err(error) = collect_pngs(&asset_root, &asset_root, &mut files.0) {
        eprintln!("could not scan the assets folder for wallpapers: {error}");
    }
    files.0.sort();
}

pub fn populate_wallpaper_choices(
    mut commands: Commands,
    lists: Query<Entity, With<WallpaperChoicesList>>,
    files: Res<WallpaperFiles>,
) {
    for list in &lists {
        commands.entity(list).with_children(|parent| {
            for file in &files.0 {
                parent.spawn((
                    Button,
                    WallpaperImageChoice(file.clone()),
                    Node {
                        width: percent(100),
                        height: px(44),
                        padding: UiRect::horizontal(px(12)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.07, 0.09, 0.10)),
                    Text::new(file.clone()),
                    TextFont {
                        font_size: 15.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            }
        });
    }
}

pub fn select_wallpaper(
    choices: Query<
        (&Interaction, &WallpaperImageChoice),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: ResMut<WallpaperSettings>,
) {
    for (interaction, choice) in &choices {
        if *interaction == Interaction::Pressed {
            settings.image = choice.0.clone();
        }
    }
}

pub fn apply_wallpaper(
    settings: Res<WallpaperSettings>,
    asset_server: Res<AssetServer>,
    mut backgrounds: Query<&mut ImageNode, With<WallpaperBackground>>,
) {
    if !settings.is_changed() {
        return;
    }
    let image_mode = match settings.mode {
        WallpaperMode::Auto => NodeImageMode::Auto,
        WallpaperMode::Stretch => NodeImageMode::Stretch,
        WallpaperMode::Tiled => NodeImageMode::Tiled {
            tile_x: settings.tile_x,
            tile_y: settings.tile_y,
            stretch_value: settings.stretch_value,
        },
    };
    for mut background in &mut backgrounds {
        background.image = asset_server.load(settings.image.clone());
        background.image_mode = image_mode.clone();
    }
}

pub fn toggle_wallpaper_mode_menu(
    buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<WallpaperModeDropdown>),
    >,
    mut menus: Query<&mut Node, With<WallpaperModeMenu>>,
) {
    let pressed = buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);
    if !pressed {
        return;
    }
    for mut menu in &mut menus {
        menu.display = if menu.display == Display::None {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn select_wallpaper_mode(
    choices: Query<
        (&Interaction, &WallpaperModeChoice),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: ResMut<WallpaperSettings>,
    mut menus: Query<&mut Node, With<WallpaperModeMenu>>,
) {
    for (interaction, choice) in &choices {
        if *interaction != Interaction::Pressed {
            continue;
        }
        settings.mode = choice.0;
        for mut menu in &mut menus {
            menu.display = Display::None;
        }
    }
}

pub fn update_wallpaper_mode_label(
    settings: Res<WallpaperSettings>,
    mut labels: Query<&mut Text, With<WallpaperModeDropdown>>,
) {
    if !settings.is_changed() {
        return;
    }
    let mode = match settings.mode {
        WallpaperMode::Auto => "Auto",
        WallpaperMode::Stretch => "Stretch",
        WallpaperMode::Tiled => "Tiled",
    };
    for mut label in &mut labels {
        label.0 = format!("{mode} ▾");
    }
}

pub fn toggle_wallpaper_tiles(
    tile_x_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<WallpaperTileXButton>),
    >,
    tile_y_buttons: Query<
        &Interaction,
        (Changed<Interaction>, With<WallpaperTileYButton>),
    >,
    mut settings: ResMut<WallpaperSettings>,
) {
    if settings.mode != WallpaperMode::Tiled {
        return;
    }
    if tile_x_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        settings.tile_x = !settings.tile_x;
    }
    if tile_y_buttons
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed)
    {
        settings.tile_y = !settings.tile_y;
    }
}

pub fn adjust_wallpaper_stretch(
    mouse: Res<ButtonInput<MouseButton>>,
    sliders: Query<(&Interaction, &RelativeCursorPosition), With<WallpaperStretchSlider>>,
    mut settings: ResMut<WallpaperSettings>,
) {
    if settings.mode != WallpaperMode::Tiled || !mouse.pressed(MouseButton::Left) {
        return;
    }
    for (interaction, cursor) in &sliders {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Some(position) = cursor.normalized else {
            continue;
        };
        let value = (position.x + 0.5).clamp(0.0, 1.0);
        if (settings.stretch_value - value).abs() > 0.005 {
            settings.stretch_value = value;
        }
    }
}

pub fn update_tiled_controls(
    settings: Res<WallpaperSettings>,
    mut tile_x_buttons: Query<
        (&mut BackgroundColor, &mut FocusPolicy),
        (
            With<WallpaperTileXButton>,
            Without<WallpaperTileYButton>,
            Without<WallpaperStretchSlider>,
            Without<WallpaperStretchFill>,
        ),
    >,
    mut tile_y_buttons: Query<
        (&mut BackgroundColor, &mut FocusPolicy),
        (
            With<WallpaperTileYButton>,
            Without<WallpaperTileXButton>,
            Without<WallpaperStretchSlider>,
            Without<WallpaperStretchFill>,
        ),
    >,
    mut tile_x_knobs: Query<
        &mut Node,
        (With<WallpaperTileXKnob>, Without<WallpaperTileYKnob>),
    >,
    mut tile_y_knobs: Query<
        &mut Node,
        (With<WallpaperTileYKnob>, Without<WallpaperTileXKnob>),
    >,
    mut slider: Query<
        (&mut BackgroundColor, &mut FocusPolicy),
        (
            With<WallpaperStretchSlider>,
            Without<WallpaperTileXButton>,
            Without<WallpaperTileYButton>,
            Without<WallpaperStretchFill>,
        ),
    >,
    mut fill: Query<
        &mut Node,
        (
            With<WallpaperStretchFill>,
            Without<WallpaperTileXKnob>,
            Without<WallpaperTileYKnob>,
        ),
    >,
    mut fill_color: Query<
        &mut BackgroundColor,
        (
            With<WallpaperStretchFill>,
            Without<WallpaperTileXButton>,
            Without<WallpaperTileYButton>,
            Without<WallpaperStretchSlider>,
        ),
    >,
    mut labels: Query<&mut Text, With<WallpaperStretchLabel>>,
) {
    if !settings.is_changed() {
        return;
    }
    let enabled = settings.mode == WallpaperMode::Tiled;
    let active_color = Color::srgb(0.22, 0.48, 0.39);
    let inactive_color = Color::srgb(0.28, 0.32, 0.34);
    let disabled_color = Color::srgb(0.18, 0.20, 0.21);
    for (mut color, mut focus) in &mut tile_x_buttons {
        *color = BackgroundColor(if enabled && settings.tile_x {
            active_color
        } else if enabled {
            inactive_color
        } else {
            disabled_color
        });
        *focus = if enabled {
            FocusPolicy::Block
        } else {
            FocusPolicy::Pass
        };
    }
    for (mut color, mut focus) in &mut tile_y_buttons {
        *color = BackgroundColor(if enabled && settings.tile_y {
            active_color
        } else if enabled {
            inactive_color
        } else {
            disabled_color
        });
        *focus = if enabled {
            FocusPolicy::Block
        } else {
            FocusPolicy::Pass
        };
    }
    for mut knob in &mut tile_x_knobs {
        knob.left = px(if settings.tile_x { 27 } else { 3 });
    }
    for mut knob in &mut tile_y_knobs {
        knob.left = px(if settings.tile_y { 27 } else { 3 });
    }
    for (mut color, mut focus) in &mut slider {
        *color = BackgroundColor(if enabled {
            inactive_color
        } else {
            disabled_color
        });
        *focus = if enabled {
            FocusPolicy::Block
        } else {
            FocusPolicy::Pass
        };
    }
    for mut node in &mut fill {
        node.width = percent(settings.stretch_value * 100.0);
    }
    for mut color in &mut fill_color {
        *color = BackgroundColor(if enabled {
            active_color
        } else {
            disabled_color
        });
    }
    for mut label in &mut labels {
        label.0 = format!("{:.2}", settings.stretch_value);
    }
}

pub fn load_wallpaper_settings(
    database: Res<Database>,
    account: Res<AuthenticatedAccount>,
    results: Res<WallpaperSettingsLoadResults>,
) {
    let account_id = account.id;
    let pool = database.0.clone();
    let sender = results.sender.clone();
    thread::spawn(move || {
        let result = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime
                .block_on(
                    db::settings::wallpaper::getters::get_wallpaper_model_by_acc_id(
                        &pool,
                        account_id,
                    ),
                )
                .map_err(|error| error.to_string()),
            Err(error) => Err(error.to_string()),
        };
        let _ = sender.send(result);
    });
}

pub fn apply_loaded_wallpaper_settings(
    results: Res<WallpaperSettingsLoadResults>,
    mut settings: ResMut<WallpaperSettings>,
) {
    let result = match results.receiver.lock() {
        Ok(receiver) => receiver.try_recv(),
        Err(error) => {
            eprintln!("wallpaper settings result lock failed: {error}");
            return;
        }
    };
    let model = match result {
        Ok(Ok(model)) => model,
        Ok(Err(error)) => {
            eprintln!("could not load wallpaper settings: {error}");
            return;
        }
        Err(TryRecvError::Empty) => return,
        Err(TryRecvError::Disconnected) => {
            eprintln!("wallpaper settings result channel disconnected");
            return;
        }
    };
    let mode = match model.mode.as_str() {
        "Auto" => WallpaperMode::Auto,
        "Stretch" => WallpaperMode::Stretch,
        "Tiled" => WallpaperMode::Tiled,
        other => {
            eprintln!("unknown wallpaper mode in database: {other}");
            return;
        }
    };
    if let Some(file_name) = model.file_name {
        settings.image = file_name;
    }
    settings.mode = mode;
    settings.tile_x = model.tile_x;
    settings.tile_y = model.tile_y;
    settings.stretch_value = model.scale;
}

pub fn save_wallpaper_settings(
    settings: Res<WallpaperSettings>,
    account: Res<AuthenticatedAccount>,
    database: Res<Database>,
    time: Res<Time>,
    mut debounce: Local<WallpaperSaveDebounce>,
) {
    if settings.is_changed() {
        debounce.dirty = true;
        debounce.timer = Some(Timer::from_seconds(0.5, TimerMode::Once));
    }
    if !debounce.dirty {
        return;
    }
    let Some(timer) = debounce.timer.as_mut() else {
        return;
    };
    if !timer.tick(time.delta()).just_finished() {
        return;
    }
    debounce.dirty = false;
    let account_id = account.id;
    let file_name = settings.image.clone();
    let mode = match settings.mode {
        WallpaperMode::Auto => "Auto",
        WallpaperMode::Stretch => "Stretch",
        WallpaperMode::Tiled => "Tiled",
    }
    .to_owned();
    let tile_x = settings.tile_x;
    let tile_y = settings.tile_y;
    let scale = settings.stretch_value;
    let pool = database.0.clone();
    std::thread::spawn(move || {
        let runtime = match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => runtime,
            Err(error) => {
                eprintln!("could not start wallpaper save runtime: {error}");
                return;
            }
        };
        let result = runtime.block_on(async {
            db::settings::wallpaper::setters::set_wallpaper_file_name_by_acc_id(
                &pool,
                account_id,
                file_name,
            )
            .await?;
            db::settings::wallpaper::setters::set_mode_by_acc_id(
                &pool,
                account_id,
                mode,
            )
            .await?;
            db::settings::wallpaper::setters::set_tile_x_by_acc_id(
                &pool,
                account_id,
                tile_x,
            )
            .await?;
            db::settings::wallpaper::setters::set_tile_y_by_acc_id(
                &pool,
                account_id,
                tile_y,
            )
            .await?;
            db::settings::wallpaper::setters::set_tile_scale_by_acc_id(
                &pool,
                account_id,
                scale,
            )
            .await?;

            Ok::<(), db::error::DatabaseError>(())
        });
        if let Err(error) = result {
            eprintln!("could not save wallpaper settings: {error}");
        }
    });
}
