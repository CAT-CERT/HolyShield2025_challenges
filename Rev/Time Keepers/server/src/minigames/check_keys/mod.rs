// src/minigames/check_keys/mod.rs

mod constants;
mod crypto_core;
mod key_derivation;
mod validation;

use bevy::prelude::*;
use arboard::Clipboard;
use crate::components::*;
use bevy::input::ButtonState;
use bevy::input::keyboard::{KeyCode, KeyboardInput};
use crate::audio_system::{AudioManager, play_click_sound};
use crate::dialogue::DialogueManager;
use crate::minigames::first_minigame::MinigameState;

// 색상 팔레트
const BG_MAIN: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BG_HEADER: Color = Color::srgba(0.573, 0.573, 0.573, 1.000);
const BG_GAME: Color = Color::srgba(0.573, 0.573, 0.573, 1.000);
const HINT_BG: Color = Color::srgba(1.000, 0.992, 0.961, 1.000);
const INPUT_BG: Color = Color::srgba(1.000, 0.992, 0.961, 1.000);
const TEXT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const TEXT_GRAY: Color = Color::srgb(0.6, 0.6, 0.6);
const TEXT_DARK: Color = Color::srgb(0.2, 0.2, 0.2);

#[derive(Resource)]
pub struct KeyCheckState {
    pub is_active: bool,
    pub input_text: String,
    pub cursor_visible: bool,
    pub cursor_timer: Timer,
    pub correct_key: String,
    pub has_attempted: bool,
    pub is_showing_result: bool,
    pub result_timer: Timer,
    pub is_success: bool,
    pub is_all_selected: bool,
    pub cursor_position: usize,
}

impl Default for KeyCheckState {
    fn default() -> Self {
        Self {
            is_active: false,
            input_text: String::new(),
            cursor_visible: true,
            cursor_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            correct_key: String::new(),
            has_attempted: false,
            is_showing_result: false,
            result_timer: Timer::from_seconds(1.5, TimerMode::Once),
            is_success: false,
            is_all_selected: false,
            cursor_position: 0,
        }
    }
}

impl KeyCheckState {
    pub fn activate(&mut self) {
        if self.has_attempted {
            return;
        }
        self.is_active = true;
        self.input_text.clear();
        self.cursor_visible = true;
        self.cursor_timer = Timer::from_seconds(0.5, TimerMode::Repeating);
        self.result_timer = Timer::from_seconds(1.5, TimerMode::Once);
        self.is_showing_result = false;
        self.cursor_position = 0;

        self.correct_key = validation::generate_hint();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.input_text.clear();
        self.is_showing_result = false;
        self.cursor_position = 0;
    }

    pub fn verify_key(&self) -> bool {
        validation::secure_verify(&self.input_text)
    }
}

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $f:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $f, font_size: $s, color: $c }) }; }
macro_rules! btn { ($($f:ident: $v:expr),*) => { ButtonBundle { $($f: $v,)* border_radius: bevy::ui::BorderRadius::all(Val::Px(22.0)), ..default() } }; }

pub fn setup_key_check_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        node!(
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::srgba(0.239, 0.239, 0.239, 0.80).into(),
            z_index: ZIndex::Global(1000)
        ),
        KeyCheckUI,
        GameUI
    )).with_children(|p| {
        p.spawn((
            node!(
                style: Style {
                    position_type: PositionType::Relative,
                    width: Val::Px(850.0),
                    height: Val::Px(620.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(25.0),
                        right: Val::Px(25.0),
                        top: Val::Px(15.0),
                        bottom: Val::Px(25.0),
                    },
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: BG_MAIN.into(),
                border_color: BorderColor(Color::srgba(1.0, 1.0, 1.0, 1.0)),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(47.0))
            ),
            GameCard
        )).with_children(|card| {
            card.spawn(node!(
                style: Style {
                    width: Val::Px(693.0),
                    height: Val::Px(90.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                background_color: BG_HEADER.into(),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
            )).with_children(|header| {
                header.spawn(text!(
                    "Final Key Verify",
                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                    50.0,
                    TEXT_WHITE
                ));
            });

            card.spawn(node!(
                style: Style {
                    width: Val::Px(723.0),
                    height: Val::Px(480.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect {
                        left: Val::Px(40.0),
                        right: Val::Px(40.0),
                        top: Val::Px(35.0),
                        bottom: Val::Px(40.0),
                    },
                    margin: UiRect::top(Val::Px(5.0)),
                    row_gap: Val::Px(35.0),
                    ..default()
                },
                background_color: BG_GAME.into(),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(47.0))
            )).with_children(|game_box| {
                game_box.spawn(node!(
                    style: Style {
                        width: Val::Px(650.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: HINT_BG.into(),
                    border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                )).with_children(|hint| {
                    hint.spawn(text!(
                        "Hint : all + hana + eunha + haejin + pick",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        35.0,
                        TEXT_DARK
                    ));
                });

                game_box.spawn(node!(
                    style: Style {
                        width: Val::Px(650.0),
                        height: Val::Px(300.0),
                        padding: UiRect::all(Val::Px(37.0)),
                        border: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    background_color: INPUT_BG.into(),
                    border_color: BorderColor(BG_HEADER),
                    border_radius: bevy::ui::BorderRadius::all(Val::Px(30.0))
                )).with_children(|grid_area| {
                    grid_area.spawn(node!(
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(10.0),
                            ..default()
                        }
                    )).with_children(|input_row| {
                        input_row.spawn((
                            node!(
                                style: Style {
                                    width: Val::Px(450.0),
                                    height: Val::Px(100.0),
                                    padding: UiRect {
                                        left: Val::Px(30.0),
                                        right: Val::Px(20.0),
                                        top: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                    },
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::FlexStart,
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                background_color: INPUT_BG.into(),
                                border_color: BorderColor(BG_HEADER),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(27.0))
                            ),
                            KeyInputBox,
                        )).with_children(|input_box| {
                            input_box.spawn((
                                text!(
                                    "Enter flag here....",
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    35.0,
                                    TEXT_GRAY
                                ),
                                KeyInputText,
                            ));
                        });

                        input_row.spawn((
                            btn!(
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(95.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(15.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(0.251, 0.667, 0.341, 1.000).into()
                            ),
                            KeyCheckButton,
                        )).with_children(|btn_child| {
                            btn_child.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(75.0),
                                    height: Val::Px(60.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("images/minigame/check.png")),
                                ..default()
                            });
                        });
                    });
                });
            });
        });
    });
}

pub fn handle_key_input(
    mut key_events: EventReader<KeyboardInput>,
    mut key_state: ResMut<KeyCheckState>,
    mut text_query: Query<&mut Text, With<KeyInputText>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if !key_state.is_active || key_state.is_showing_result {
        return;
    }

    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && keyboard_input.just_pressed(KeyCode::KeyA) {
        key_state.is_all_selected = true;
    }

    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && keyboard_input.just_pressed(KeyCode::KeyC) {
        if key_state.is_all_selected && !key_state.input_text.is_empty() {
            if let Ok(mut clipboard) = Clipboard::new() {
                let _ = clipboard.set_text(&key_state.input_text);
            }
        }
    }

    if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight])
        && keyboard_input.just_pressed(KeyCode::KeyV) {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Ok(text) = clipboard.get_text() {
                if key_state.is_all_selected {
                    key_state.input_text.clear();
                    key_state.is_all_selected = false;
                    key_state.cursor_position = 0;
                }

                let available_space = 100 - key_state.input_text.len();
                let to_add: String = text.chars().take(available_space).collect();

                let mut chars: Vec<char> = key_state.input_text.chars().collect();
                for c in to_add.chars() {
                    chars.insert(key_state.cursor_position, c);
                    key_state.cursor_position += 1;
                }
                key_state.input_text = chars.into_iter().collect();
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        if key_state.is_all_selected {
            key_state.is_all_selected = false;
            key_state.cursor_position = 0;
        } else if key_state.cursor_position > 0 {
            key_state.cursor_position -= 1;
        }
    }

    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        if key_state.is_all_selected {
            key_state.is_all_selected = false;
            key_state.cursor_position = key_state.input_text.chars().count();
        } else {
            let len = key_state.input_text.chars().count();
            if key_state.cursor_position < len {
                key_state.cursor_position += 1;
            }
        }
    }

    for event in key_events.read() {
        if event.state == ButtonState::Pressed {
            if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
                continue;
            }

            match event.key_code {
                KeyCode::Backspace => {
                    if key_state.is_all_selected {
                        key_state.input_text.clear();
                        key_state.is_all_selected = false;
                        key_state.cursor_position = 0;
                    } else if key_state.cursor_position > 0 {
                        let mut chars: Vec<char> = key_state.input_text.chars().collect();
                        chars.remove(key_state.cursor_position - 1);
                        key_state.input_text = chars.into_iter().collect();
                        key_state.cursor_position -= 1;
                    }
                }
                KeyCode::Delete => {
                    if key_state.is_all_selected {
                        key_state.input_text.clear();
                        key_state.is_all_selected = false;
                        key_state.cursor_position = 0;
                    } else {
                        let mut chars: Vec<char> = key_state.input_text.chars().collect();
                        if key_state.cursor_position < chars.len() {
                            chars.remove(key_state.cursor_position);
                            key_state.input_text = chars.into_iter().collect();
                        }
                    }
                }
                KeyCode::Enter => {
                    if !key_state.input_text.is_empty() {
                        let is_success = key_state.verify_key();
                        key_state.is_success = is_success;
                        key_state.is_showing_result = true;
                        key_state.result_timer.reset();
                    }
                    key_state.is_all_selected = false;
                }
                KeyCode::Escape => {
                    if key_state.is_all_selected {
                        key_state.is_all_selected = false;
                    } else {
                        key_state.input_text.clear();
                        key_state.cursor_position = 0;
                    }
                }
                KeyCode::Space => {
                    if key_state.is_all_selected {
                        key_state.input_text.clear();
                        key_state.is_all_selected = false;
                        key_state.cursor_position = 0;
                    }
                    if key_state.input_text.len() < 100 {
                        let mut chars: Vec<char> = key_state.input_text.chars().collect();
                        chars.insert(key_state.cursor_position, ' ');
                        key_state.input_text = chars.into_iter().collect();
                        key_state.cursor_position += 1;
                    }
                }
                KeyCode::ArrowLeft | KeyCode::ArrowRight => {
                    // 이미 위에서 처리됨
                }
                _ => {
                    if let Some(c) = key_code_to_char(event.key_code, &keyboard_input) {
                        if key_state.is_all_selected {
                            key_state.input_text.clear();
                            key_state.is_all_selected = false;
                            key_state.cursor_position = 0;
                        }
                        if key_state.input_text.len() < 100 {
                            let mut chars: Vec<char> = key_state.input_text.chars().collect();
                            chars.insert(key_state.cursor_position, c);
                            key_state.input_text = chars.into_iter().collect();
                            key_state.cursor_position += 1;
                        }
                    }
                }
            }
        }
    }

    if key_state.input_text.len() > 100 {
        key_state.input_text.truncate(100);
    }

    for mut text in text_query.iter_mut() {
        if key_state.input_text.is_empty() {
            let cursor = if key_state.cursor_visible { "|" } else { "" };
            text.sections[0].value = format!("Enter flag here....{}", cursor);
            text.sections[0].style.color = TEXT_GRAY;
        } else {
            const MAX_VISIBLE_CHARS: usize = 24;
            let chars: Vec<char> = key_state.input_text.chars().collect();
            let total_len = chars.len();

            let (display_start, display_text) = if total_len > MAX_VISIBLE_CHARS {
                if key_state.cursor_position > MAX_VISIBLE_CHARS {
                    let start = key_state.cursor_position - MAX_VISIBLE_CHARS;
                    (start, chars[start..].iter().collect::<String>())
                } else {
                    (0, chars[..MAX_VISIBLE_CHARS].iter().collect::<String>())
                }
            } else {
                (0, key_state.input_text.clone())
            };

            let relative_cursor_pos = key_state.cursor_position.saturating_sub(display_start);
            let display_chars: Vec<char> = display_text.chars().collect();

            let before: String = display_chars[..relative_cursor_pos.min(display_chars.len())].iter().collect();
            let after: String = display_chars[relative_cursor_pos.min(display_chars.len())..].iter().collect();
            let cursor = if key_state.cursor_visible { "|" } else { "" };

            text.sections[0].value = format!("{}{}{}", before, cursor, after);
            text.sections[0].style.color = if key_state.is_all_selected {
                Color::srgb(0.3, 0.5, 0.8)
            } else {
                TEXT_DARK
            };
        }
    }
}

fn key_code_to_char(key: KeyCode, keyboard_input: &ButtonInput<KeyCode>) -> Option<char> {
    let shift = keyboard_input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    
    match key {
        KeyCode::KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift { 'H' } else { 'h' }),
        KeyCode::KeyI => Some(if shift { 'I' } else { 'i' }),
        KeyCode::KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift { 'L' } else { 'l' }),
        KeyCode::KeyM => Some(if shift { 'M' } else { 'm' }),
        KeyCode::KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift { 'O' } else { 'o' }),
        KeyCode::KeyP => Some(if shift { 'P' } else { 'p' }),
        KeyCode::KeyQ => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::KeyZ => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::Digit0 => Some(if shift { ')' } else { '0' }),
        KeyCode::Digit1 => Some(if shift { '!' } else { '1' }),
        KeyCode::Digit2 => Some(if shift { '@' } else { '2' }),
        KeyCode::Digit3 => Some(if shift { '#' } else { '3' }),
        KeyCode::Digit4 => Some(if shift { '$' } else { '4' }),
        KeyCode::Digit5 => Some(if shift { '%' } else { '5' }),
        KeyCode::Digit6 => Some(if shift { '^' } else { '6' }),
        KeyCode::Digit7 => Some(if shift { '&' } else { '7' }),
        KeyCode::Digit8 => Some(if shift { '*' } else { '8' }),
        KeyCode::Digit9 => Some(if shift { '(' } else { '9' }),
        KeyCode::Minus => Some(if shift { '_' } else { '-' }),
        KeyCode::Equal => Some(if shift { '+' } else { '=' }),
        KeyCode::BracketLeft => Some(if shift { '{' } else { '[' }),
        KeyCode::BracketRight => Some(if shift { '}' } else { ']' }),
        KeyCode::Backslash => Some(if shift { '|' } else { '\\' }),
        KeyCode::Semicolon => Some(if shift { ':' } else { ';' }),
        KeyCode::Quote => Some(if shift { '"' } else { '\'' }),
        KeyCode::Comma => Some(if shift { '<' } else { ',' }),
        KeyCode::Period => Some(if shift { '>' } else { '.' }),
        KeyCode::Slash => Some(if shift { '?' } else { '/' }),
        KeyCode::Backquote => Some(if shift { '~' } else { '`' }),
        _ => None,
    }
}

pub fn handle_key_check_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<KeyCheckButton>)>,
    mut key_state: ResMut<KeyCheckState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                if key_state.is_showing_result || key_state.input_text.is_empty() {
                    return;
                }
                
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                let is_success = key_state.verify_key();
                key_state.is_success = is_success;
                key_state.is_showing_result = true;
                key_state.result_timer.reset();
                
                *color = Color::srgba(0.251, 0.667, 0.341, 1.000).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.201, 0.617, 0.291, 1.000).into();
            }
            _ => {
                *color = Color::srgba(0.251, 0.667, 0.341, 1.000).into();
            }
        }
    }
}

pub fn show_result_image(
    key_state: Res<KeyCheckState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    result_query: Query<Entity, With<ResultImage>>,
    ui_query: Query<Entity, With<KeyCheckUI>>
) {
    if key_state.is_showing_result && result_query.is_empty() {
        if let Ok(ui_entity) = ui_query.get_single() {
            let image_path = if key_state.is_success {
                "images/minigame/success.png"
            } else {
                "images/minigame/fail.png"
            };
            
            commands.entity(ui_entity).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::top(Val::Px(60.0)),
                            ..default()
                        },
                        z_index: ZIndex::Global(1600),
                        ..default()
                    },
                    ResultImage
                )).with_children(|overlay| {
                    overlay.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load(image_path)),
                        ..default()
                    });
                });
            });
        }
    } else if !key_state.is_showing_result && !result_query.is_empty() {
        for entity in result_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_result_display(
    time: Res<Time>,
    mut key_state: ResMut<KeyCheckState>,
    mut state: ResMut<NextState<MinigameState>>,
    mut dm: ResMut<DialogueManager>,
) {
    if !key_state.is_showing_result {
        return;
    }

    key_state.result_timer.tick(time.delta());
    
    if key_state.result_timer.just_finished() { 
        if key_state.is_success {
            key_state.has_attempted = true;
        } else {
            key_state.has_attempted = false;
        }
        
        let target_scene = if key_state.is_success { "end_1" } else { "end_2" };
        
        dm.move_to_scene(target_scene);
        
        if let Some(first_line) = dm.get_current_line() {
            let text = first_line.text.clone();
            if !text.is_empty() && first_line.event_image.is_none() {
                dm.start_typing(text);
            }
        }
        
        key_state.deactivate();
        state.set(MinigameState::None);
    }
}

pub fn update_cursor_blink(
    time: Res<Time>,
    mut key_state: ResMut<KeyCheckState>,
) {
    if key_state.is_active && !key_state.is_showing_result {
        key_state.cursor_timer.tick(time.delta());
        if key_state.cursor_timer.just_finished() {
            key_state.cursor_visible = !key_state.cursor_visible;
        }
    }
}

pub fn check_key_trigger(
    dm: Res<DialogueManager>,
    state: Res<State<MinigameState>>,
    mut next_state: ResMut<NextState<MinigameState>>,
    mut key_state: ResMut<KeyCheckState>,
) {
    if key_state.has_attempted {
        return;
    }

    if *state.get() != MinigameState::None || dm.is_typing() {
        return;
    }

    let trigger = dm.pending_minigame.as_ref().or_else(|| {
        dm.get_current_line()
            .and_then(|l| l.minigame_trigger.as_ref())
            .filter(|_| !dm.is_choice_active())
    });

    if trigger.map_or(false, |t| t == "check_key") {
        key_state.activate();
        next_state.set(MinigameState::KeyCheck);
    }
}

pub fn cleanup_key_check(mut commands: Commands, ui_query: Query<Entity, With<KeyCheckUI>>) {
    ui_query.iter().for_each(|e| commands.entity(e).despawn_recursive());
}

#[derive(Component)]
pub struct KeyCheckUI;

#[derive(Component)]
pub struct GameCard;

#[derive(Component)]
pub struct KeyInputBox;

#[derive(Component)]
pub struct KeyInputText;

#[derive(Component)]
pub struct KeyCheckButton;

#[derive(Component)]
pub struct ResultImage;