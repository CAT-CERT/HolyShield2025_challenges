// menu.rs - 웹툰 스타일 UI로 변경 - 3줄 레이아웃
use bevy::prelude::*;
use bevy::audio::Volume;
use bevy::time::TimerMode;
use crate::components::*;
use crate::game_state::GameState;
use crate::dialogue::VisitedLocations;
use crate::minigames::check_keys::KeyCheckState;
use crate::minigames::first_minigame::{ButtonSequenceGame, CompletedMinigames};
use crate::minigames::second_minigame::ConstellationGame;
use crate::minigames::fourth_minigame::QuizGame;
use crate::minigames::fifth_minigame::MathGame;
use crate::minigames::third_minigame::{PianoTileGame, CompletedPianoMinigames};
use crate::audio_system::{AudioManager, play_click_sound};
// Local marker for ambient rain so it can persist across states
#[derive(Component)]
pub struct RainAmbience;
use bevy::ui::Val;

// 웹툰 스타일 색상 팔레트
const BUTTON_NORMAL: Color = Color::srgba(1.0, 1.0, 1.0, 0.95);
const BUTTON_HOVERED: Color = Color::srgba(0.85, 0.85, 0.85, 1.0);
const BUTTON_PRESSED: Color = Color::srgba(0.7, 0.7, 0.7, 1.0);
const NEWGAME_NORMAL: Color = Color::srgba(1.0, 0.8, 0.9, 0.95);
const NEWGAME_HOVERED: Color = Color::srgba(1.0, 0.7, 0.8, 1.0);
const NEWGAME_PRESSED: Color = Color::srgba(1.0, 0.6, 0.7, 1.0);
const TEXT_PRIMARY: Color = Color::srgb(0.2, 0.3, 0.5);
const MENU_BACKGROUND_FADE_DURATION: f32 = 0.9;

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    // 현재 윈도우 크기 가져오기
    let window_size = if let Ok(window) = windows.get_single() {
        Vec2::new(window.width(), window.height())
    } else {
        Vec2::new(1280.0, 720.0)
    };

    // 메인 메뉴 배경
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/background.png"),
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                custom_size: Some(window_size),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            ..default()
        },
        MainMenu,
        MenuBackgroundFade {
            timer: Timer::from_seconds(MENU_BACKGROUND_FADE_DURATION, TimerMode::Once),
        },
    ));

    // 비 소리(레인 앰비언스)를 체육관 전까지 유지하기 위해 메인 메뉴에서부터 재생 시작
    // 이미 재생 중이면 중복 생성하지 않음
    // rain ambience now starts when dialogue begins (OnEnter Playing)

    // 메인 메뉴 UI 컨테이너
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // 왼쪽 하단 버튼 컨테이너
            parent.spawn(
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Vh(8.0),
                        left: Val::Vw(5.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        row_gap: Val::Vh(1.0),
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|button_parent| {
                // 첫 번째 줄: 새 시작 버튼
                button_parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Vw(25.0),
                                height: Val::Vh(8.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                padding: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            background_color: NEWGAME_NORMAL.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0)),
                            ..default()
                        },
                        NewGameButton,
                    ))
                    .with_children(|button| {
                        // 하트 아이콘 - 흰색
                        button.spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                margin: UiRect::right(Val::Px(12.0)),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        });

                        // 텍스트 - 흰색
                        button.spawn(TextBundle::from_section(
                            "새 시작",
                            TextStyle {
                                font: asset_server.load("fonts/YOnepick-Bold.ttf"),
                                font_size: 28.0,
                                color: Color::WHITE,
                            },
                        ));
                    });

                // 두 번째 줄: 설정 버튼
                button_parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Vw(25.0),
                                height: Val::Vh(8.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                padding: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            background_color: BUTTON_NORMAL.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0)),
                            ..default()
                        },
                        SettingsButton,
                    ))
                    .with_children(|button| {
                        // 설정 아이콘 - 회색
                        button.spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                margin: UiRect::right(Val::Px(12.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.6, 0.6, 0.6).into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        });

                        // 텍스트
                        button.spawn(TextBundle::from_section(
                            "설정",
                            TextStyle {
                                font: asset_server.load("fonts/YOnepick-Bold.ttf"),
                                font_size: 28.0,
                                color: TEXT_PRIMARY,
                            },
                        ));
                    });

                button_parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Vw(25.0),
                                height: Val::Vh(8.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                padding: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            background_color: BUTTON_NORMAL.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0)),
                            ..default()
                        },
                        ExitButton,
                    ))
                    .with_children(|button| {
                        // 종료 아이콘 - 회색
                        button.spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(24.0),
                                height: Val::Px(24.0),
                                margin: UiRect::right(Val::Px(12.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.6, 0.6, 0.6).into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(12.0)),
                            ..default()
                        });

                        // 텍스트
                        button.spawn(TextBundle::from_section(
                            "종료",
                            TextStyle {
                                font: asset_server.load("fonts/YOnepick-Bold.ttf"),
                                font_size: 28.0,
                                color: TEXT_PRIMARY,
                            },
                        ));
                    });
            });
    });
}

pub fn handle_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&NewGameButton>, Option<&SettingsButton>, Option<&ExitButton>),
        (
            Changed<Interaction>,
            With<Button>,
            Or<(
                With<NewGameButton>,
                With<SettingsButton>,
                With<ExitButton>,
            )>,
        ),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    mut dialogue_manager: ResMut<crate::dialogue::DialogueManager>,
    mut visited_locations: ResMut<VisitedLocations>,
    mut key_check_state: ResMut<KeyCheckState>,
    mut button_sequence_game: ResMut<ButtonSequenceGame>,
    mut completed_minigames: ResMut<CompletedMinigames>,
    mut constellation_game: ResMut<ConstellationGame>,
    mut quiz_game: ResMut<QuizGame>,
    mut math_game: ResMut<MathGame>,
    mut piano_game: ResMut<PianoTileGame>,
    mut completed_piano_minigames: ResMut<CompletedPianoMinigames>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
) {
    for (interaction, mut color, new_game, settings, exit) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if new_game.is_some() {
                    *color = NEWGAME_PRESSED.into();
                    play_click_sound(&mut commands, &asset_server, &audio_manager);
                    
                    visited_locations.completed_chapters.clear();
                    key_check_state.deactivate();
                    key_check_state.has_attempted = false;
                    *button_sequence_game = ButtonSequenceGame::default();
                    completed_minigames.0.clear();
                    constellation_game.hard_reset_for_entry();
                    *quiz_game = QuizGame::default();
                    *math_game = MathGame::default();
                    piano_game.reset();
                    completed_piano_minigames.piano_tiles = false;
                    *key_check_state = KeyCheckState::default();
                    
                    dialogue_manager.current_line_index = 0;
                    if let Some(first_scene) = dialogue_manager.scenes.first().cloned() {
                        dialogue_manager.current_scene = Some(first_scene);
                    }
                    
                    dialogue_manager.typing_effect.is_typing = false;
                    dialogue_manager.typing_effect.current_char_index = 0;
                    dialogue_manager.typing_effect.full_text = String::new();
                    dialogue_manager.typing_effect.displayed_text = String::new();
                    dialogue_manager.typing_effect.typing_timer.reset();

                    dialogue_manager.choice_system.is_choice_active = false;
                    dialogue_manager.choice_system.current_choices.clear();
                    
                    let first_text = if let Some(current_line) = dialogue_manager.get_current_line() {
                        Some(current_line.text.clone())
                    } else {
                        None
                    };
                    
                    if let Some(text) = first_text {
                        dialogue_manager.start_typing(text);
                    }
                    
                    next_state.set(GameState::Playing);
                } else if settings.is_some() {
                    *color = BUTTON_PRESSED.into();
                    play_click_sound(&mut commands, &asset_server, &audio_manager);
                    next_state.set(GameState::Settings);
                } else if exit.is_some() {
                    *color = BUTTON_PRESSED.into();
                    play_click_sound(&mut commands, &asset_server, &audio_manager);
                    app_exit_events.send(bevy::app::AppExit::Success);
                }
            }
            Interaction::Hovered => {
                if new_game.is_some() {
                    *color = NEWGAME_HOVERED.into();
                } else {
                    *color = BUTTON_HOVERED.into();
                }
            }
            Interaction::None => {
                if new_game.is_some() {
                    *color = NEWGAME_NORMAL.into();
                } else {
                    *color = BUTTON_NORMAL.into();
                }
            }
        }
    }
}

pub fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenu>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_main_menu_background_fade(time: Res<Time>, mut query: Query<(&mut Sprite, &mut MenuBackgroundFade)>) {
    for (mut sprite, mut fade) in query.iter_mut() {
        fade.timer.tick(time.delta());
        let duration = fade.timer.duration().as_secs_f32().max(f32::EPSILON);
        let alpha = (fade.timer.elapsed_secs() / duration).clamp(0.0, 1.0);
        let current = sprite.color.to_srgba();
        sprite.color = Color::srgba(current.red, current.green, current.blue, alpha);
    }
}

pub fn start_rain_on_dialogue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    existing_rain: Query<Entity, With<RainAmbience>>,
) {
    if existing_rain.is_empty() {
        let rain_volume = audio_manager.get_effective_volume("sfx") * 0.6;
        commands.spawn((
            AudioBundle {
                source: asset_server.load("sounds/bgms/rain.ogg"),
                settings: PlaybackSettings::LOOP.with_volume(Volume::new(rain_volume)),
            },
            RainAmbience,
        ));
    }
}

// 체육관(gym) 배경으로 진입하면 빗소리 종료
pub fn stop_rain_on_gym(
    mut commands: Commands,
    bm: Res<crate::character_system::BackgroundManager>,
    rain_q: Query<Entity, With<RainAmbience>>,
) {
    if bm.current_background.as_deref() == Some("gym") {
        for e in &rain_q {
            commands.entity(e).despawn();
        }
    }
}

// ESC 등으로 Playing을 떠날 때 빗소리 정리
pub fn stop_rain_ambience(
    mut commands: Commands,
    rain_q: Query<Entity, With<RainAmbience>>,
) {
    for e in &rain_q {
        commands.entity(e).despawn();
    }
}
