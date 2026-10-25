/* app/src/system/wallpaper.rs
 * The purpose of this module is to provide a wallpaper system.
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
use std::path::Path;

use crate::component::login::{
    WallpaperBackground, WallpaperChoicesList, WallpaperCloseButton, WallpaperImageChoice, WallpaperModal, WallpaperModeChoice, WallpaperModeDropdown, WallpaperModeMenu, WallpaperOpenButton, WallpaperStretchFill, WallpaperStretchLabel, WallpaperStretchSlider, WallpaperTileXButton, WallpaperTileXKnob, WallpaperTileYButton, WallpaperTileYKnob,
};
use crate::resource::custom_resource::{
    WallpaperFiles,
    WallpaperMode,
    WallpaperSettings,
};

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
