// src/minigames/uis/third_ui.rs
use bevy::prelude::*;
use bevy::audio::Volume;
use crate::components::*;
use crate::dialogue::DialogueManager;
use crate::minigames::third_minigame::{PianoMinigameState, PianoTileGame, PianoNote, CompletedPianoMinigames, PianoGamePhase};
use crate::audio_system::{AudioManager, play_click_sound};

// 색상 팔레트
const BG_MAIN: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BG_HEADER: Color = Color::srgba(0.733, 0.835, 0.941, 1.000);
const BG_GAME: Color = Color::srgba(0.902, 0.949, 1.000, 1.000);
const TEXT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const ROUND_BADGE: Color = Color::srgba(0.733, 0.835, 0.941, 1.000);

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $f:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $f, font_size: $s, color: $c }) }; }

#[derive(Component)]
pub struct PianoKey {
    pub note: PianoNote,
}

#[derive(Component)]
pub struct SpeakerButton;

#[derive(Component)]
pub struct SpeakerImage;

#[derive(Component)]
pub struct SpeakerResetTimer {
    timer: Timer,
    target_child: Entity,
}

#[derive(Component)]
pub struct PianoBgmController {
    state: BgmControlState,
    wait_timer: Timer,
    sound_duration: Timer,
}

#[derive(PartialEq)]
enum BgmControlState {
    Idle,
    WaitingBeforeSound,
    PlayingSound,
    WaitingAfterSound,
}

pub fn setup_piano_minigame(
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
        PianoMinigameUI,
        GameUI,
    )).with_children(|p| {
        // 메인 카드
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
            // 제목 헤더
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
                header.spawn(node!(
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    }
                )).with_children(|text_container| {
                    text_container.spawn(text!(
                        "Playing The Piano!",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        50.0,
                        TEXT_WHITE
                    ));
                    text_container.spawn(text!(
                        "스피커 아이콘을 눌러 소리를 듣고 알맞은 음을 클릭하세요!",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        20.0,
                        TEXT_WHITE
                    ));
                });
            });

            // 게임 영역
            card.spawn(node!(
                style: Style {
                    width: Val::Px(723.0),
                    height: Val::Px(580.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(25.0),
                        right: Val::Px(25.0),
                        top: Val::Px(50.0),
                        bottom: Val::Px(10.0),
                    },
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                background_color: BG_GAME.into(),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(47.0))
            )).with_children(|game_box| {
                // 라운드 표시와 스피커를 담는 컨테이너
                game_box.spawn(node!(
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(20.0),
                        margin: UiRect {
                            bottom: Val::Px(35.0),
                            right: Val::Px(0.0),
                            left: Val::Px(55.0),
                            top: Val::Px(0.0),
                        },
                        ..default()
                    }
                )).with_children(|top_row| {
                    // 라운드 표시
                    top_row.spawn((
                        node!(
                            style: Style {
                                width: Val::Px(210.0),
                                height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: ROUND_BADGE.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(30.0))
                        ),
                        RoundDisplay
                    )).with_children(|badge| {
                        badge.spawn((
                            text!(
                                "1/3",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                40.0,
                                TEXT_WHITE
                            ),
                            PianoDisplay
                        ));
                    });

                    // 스피커 버튼
                    top_row.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(80.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        },
                        SpeakerButton,
                        PianoListenButton,
                    )).with_children(|speaker| {
                        speaker.spawn((
                            ImageBundle {
                                style: Style {
                                    width: Val::Px(80.0),
                                    height: Val::Px(80.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("images/minigame/speaker.png")),
                                ..default()
                            },
                            SpeakerImage,
                        ));
                    });
                });

                // 피아노 건반 그리드
                game_box.spawn(
                    node!(
                        style: Style {
                            width: Val::Px(635.0),
                            height: Val::Px(245.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.650, 0.800, 0.880, 1.000).into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(30.0))
                    )
                ).with_children(|piano_bg| {
                    piano_bg.spawn((
                        node!(
                            style: Style {
                                width: Val::Px(690.0),
                                height: Val::Px(210.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::Center,
                                ..default()
                            }
                        ),
                        PianoKeyboard,
                    )).with_children(|piano_container| {
                        let notes = [
                            PianoNote::Do, PianoNote::Re, PianoNote::Mi, PianoNote::Fa,
                            PianoNote::Sol, PianoNote::La, PianoNote::Si
                        ];

                        for (index, note) in notes.iter().enumerate() {
                            piano_container.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(84.5),
                                        height: Val::Px(210.0),
                                        align_items: AlignItems::End,
                                        justify_content: JustifyContent::Center,
                                        padding: UiRect::bottom(Val::Px(15.0)),
                                        border: UiRect {
                                            left: if index == 0 { Val::Px(2.5) } else { Val::Px(1.0) },
                                            right: if index == notes.len() - 1 { Val::Px(2.5) } else { Val::Px(1.0) },
                                            top: Val::Px(2.5),
                                            bottom: Val::Px(2.5),
                                        },
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.98, 0.98, 1.0).into(),
                                    border_color: Color::srgb(0.5, 0.5, 0.7).into(),
                                    border_radius: bevy::ui::BorderRadius::all(Val::Px(10.0)),
                                    ..default()
                                },
                                PianoKey { note: *note },
                            )).with_children(|key| {
                                key.spawn(text!(
                                    note.to_korean(),
                                    asset_server.load("fonts/YOnepick-Bold.ttf"),
                                    23.0,
                                    Color::srgb(0.2, 0.2, 0.4)
                                ));
                            });
                        }

                        // 검은 건반 오버레이
                        piano_container.spawn(node!(
                            style: Style {
                                position_type: PositionType::Absolute,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            z_index: ZIndex::Local(10),
                            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))
                        )).with_children(|overlay| {
                            let white_key_width = 84.5;
                            let black_key_width = 52.0;
                            let black_key_height = 135.0;

                            let black_keys = [
                                (0usize, PianoNote::DoSharp),
                                (1usize, PianoNote::ReSharp),
                                (3usize, PianoNote::FaSharp),
                                (4usize, PianoNote::SolSharp),
                                (5usize, PianoNote::LaSharp),
                            ];

                            for (white_index, note) in black_keys {
                                let left_px = (white_index as f32 * white_key_width) + (white_key_width - black_key_width / 2.0) + 50.0;

                                overlay.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(left_px),
                                            top: Val::Px(0.0),
                                            width: Val::Px(black_key_width),
                                            height: Val::Px(black_key_height),
                                            align_items: AlignItems::End,
                                            justify_content: JustifyContent::Center,
                                            border: UiRect::all(Val::Px(2.0)),
                                            padding: UiRect::bottom(Val::Px(10.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.28, 0.32, 0.40).into(),
                                        border_color: Color::srgb(0.18, 0.22, 0.30).into(),
                                        border_radius: bevy::ui::BorderRadius {
                                            top_left: Val::Px(7.0),
                                            top_right: Val::Px(7.0),
                                            bottom_left: Val::Px(7.0),
                                            bottom_right: Val::Px(7.0),
                                        },
                                        ..default()
                                    },
                                    PianoKey { note },
                                )).with_children(|key| {
                                    key.spawn(text!(
                                        note.to_korean(),
                                        asset_server.load("fonts/YOnepick-Bold.ttf"),
                                        20.0,
                                        Color::srgb(0.9, 0.9, 1.0)
                                    ));
                                });
                            }
                        });
                    });
                });
            });

            // 컨트롤 버튼 컨테이너
            card.spawn((
                node!(
                    style: Style {
                        display: Display::None,
                        width: Val::Percent(100.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    }
                ),
                ButtonContainer
            )).with_children(|btn_container| {
                btn_container.spawn((
                    node!(
                        style: Style {
                            display: Display::None,
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }
                    ),
                    StatusContainer
                ));
            });
        });

        // "Are you ready?" 오버레이
        p.spawn((
            node!(
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                z_index: ZIndex::Global(1500)
            ),
            ReadyOverlay
        )).with_children(|overlay| {
            overlay.spawn(node!(
                style: Style {
                    width: Val::Px(812.0),
                    height: Val::Px(560.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(30.0),
                    ..default()
                }
            )).with_children(|content| {
                content.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(40.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.894, 0.909, 0.929, 1.000).into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)),
                        ..default()
                    },
                    PianoStartButton
                )).with_children(|play_btn| {
                    play_btn.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(75.0),
                            height: Val::Px(77.0),
                            margin: UiRect::left(Val::Px(12.0)),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/triangle3.png")),
                        ..default()
                    });
                });

                content.spawn(node!(
                    style: Style {
                        width: Val::Px(530.0),
                        height: Val::Px(145.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(25.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.733, 0.835, 0.941, 1.000).into(),
                    border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
                )).with_children(|msg_box| {
                    msg_box.spawn(text!(
                        "are you ready?",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        55.0,
                        TEXT_WHITE
                    ));
                });
            });
        });
    });
}

pub fn cleanup_piano_minigame(
    mut commands: Commands,
    minigame_query: Query<Entity, With<PianoMinigameUI>>,
) {
    for entity in &minigame_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_piano_start_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PianoStartButton>)>,
    mut piano_game: ResMut<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<ReadyOverlay>>,
    mut key_query: Query<(&mut BackgroundColor, &PianoKey), Without<PianoStartButton>>,  // 추가
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // 모든 건반 색깔 초기화
                for (mut key_color, piano_key) in key_query.iter_mut() {
                    *key_color = if piano_key.note.is_black_key() {
                        Color::srgb(0.28, 0.32, 0.40).into()
                    } else {
                        Color::srgb(0.98, 0.98, 1.0).into()
                    };
                }
                
                piano_game.start_game();
                *color = Color::srgba(0.9, 0.9, 0.9, 1.0).into();
            }
            Interaction::Hovered => *color = Color::srgba(0.95, 0.95, 0.95, 1.0).into(),
            Interaction::None => *color = Color::srgba(0.894, 0.909, 0.929, 1.000).into(),
        }
    }
}

pub fn handle_piano_listen_button(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<PianoListenButton>)>,
    mut image_query: Query<&mut UiImage, With<SpeakerImage>>,
    mut piano_game: ResMut<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 클릭 불가능하면 모든 인터랙션 무시
    if !piano_game.can_click_speaker() {
        return;
    }

    for (interaction, children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for &child in children.iter() {
                if let Ok(mut image) = image_query.get_mut(child) {
                    *image = UiImage::new(asset_server.load("images/minigame/speaker_clicked.png"));
                    
                    piano_game.set_round_targets();
                    piano_game.start_playing_sound();
                    
                    commands.spawn(PianoBgmController {
                        state: BgmControlState::WaitingBeforeSound,
                        wait_timer: Timer::from_seconds(0.2, TimerMode::Once),
                        sound_duration: Timer::from_seconds(2.0, TimerMode::Once),
                    });
                    
                    commands.spawn(SpeakerResetTimer {
                        timer: Timer::from_seconds(2.4, TimerMode::Once),
                        target_child: child,
                    });
                }
            }
        }
    }
}

pub fn update_piano_bgm_control(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    piano_game: Res<PianoTileGame>,
    mut bgm_query: Query<(Entity, &mut PianoBgmController)>,
    audio_sinks: Query<&AudioSink, With<BackgroundMusic>>,
) {
    for (entity, mut controller) in bgm_query.iter_mut() {
        match controller.state {
            BgmControlState::WaitingBeforeSound => {
                controller.wait_timer.tick(time.delta());
                
                if controller.wait_timer.just_finished() {
                    for audio_sink in audio_sinks.iter() {
                        audio_sink.pause();
                    }
                    
                    let round = piano_game.current_round.clamp(1, piano_game.total_rounds);
                    let path = format!("sounds/piano/sound{}.ogg", round);
                    let vol = audio_manager.get_effective_volume("sfx");
                    
                    commands.spawn(AudioBundle {
                        source: asset_server.load(path),
                        settings: PlaybackSettings::DESPAWN
                            .with_volume(Volume::new(vol)),
                    });                    
                    
                    controller.state = BgmControlState::PlayingSound;
                }
            }
            BgmControlState::PlayingSound => {
                controller.sound_duration.tick(time.delta());
                
                if controller.sound_duration.just_finished() {
                    controller.state = BgmControlState::WaitingAfterSound;
                    controller.wait_timer = Timer::from_seconds(0.2, TimerMode::Once);
                }
            }
            BgmControlState::WaitingAfterSound => {
                controller.wait_timer.tick(time.delta());
                
                if controller.wait_timer.just_finished() {
                    for audio_sink in audio_sinks.iter() {
                        audio_sink.play();
                    }
                    
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

pub fn update_speaker_image(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer_query: Query<(Entity, &mut SpeakerResetTimer)>,
    mut image_query: Query<&mut UiImage, With<SpeakerImage>>,
    mut piano_game: ResMut<PianoTileGame>,
) {
    for (entity, mut reset_timer) in timer_query.iter_mut() {
        reset_timer.timer.tick(time.delta());
        
        if reset_timer.timer.just_finished() {
            if let Ok(mut image) = image_query.get_mut(reset_timer.target_child) {
                *image = UiImage::new(asset_server.load("images/minigame/speaker.png"));
            }
            piano_game.start_accepting_input();
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn handle_piano_close_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PianoCloseButton>)>,
    mut _piano_state: ResMut<NextState<PianoMinigameState>>,
    mut _dialogue_manager: ResMut<DialogueManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgba(0.733, 0.835, 0.941, 1.000).into();
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                _piano_state.set(PianoMinigameState::Inactive);
                _dialogue_manager.next_line();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.703, 0.805, 0.911, 1.000).into();
            }
            Interaction::None => {
                *color = Color::srgba(0.733, 0.835, 0.941, 1.000).into();
            }
        }
    }
}

pub fn handle_piano_keys(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &PianoKey), (Changed<Interaction>, With<PianoKey>)>,
    mut piano_game: ResMut<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
    audio_manager: Res<AudioManager>,
) {
    // 입력 불가능하면 모든 인터랙션 무시 (호버링 포함)
    if !piano_game.can_accept_input() {
        return;
    }

    let mut pressed_note: Option<PianoNote> = None;

    for (interaction, mut color, piano_key) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = if piano_key.note.is_black_key() {
                    Color::srgb(0.5, 0.5, 0.6).into()
                } else {
                    Color::srgb(0.8, 0.9, 1.0).into()
                };
                
                let audio_path = format!("sounds/piano/{}", piano_key.note.to_audio_filename());
                let volume = audio_manager.get_effective_volume("sfx");

                commands.spawn(AudioBundle {
                    source: asset_server.load(audio_path),
                    settings: PlaybackSettings::DESPAWN
                        .with_volume(Volume::new(volume)),
                });
                
                if pressed_note.is_none() {
                    pressed_note = Some(piano_key.note);
                }
            }
            Interaction::Hovered => {
                *color = if piano_key.note.is_black_key() {
                    Color::srgb(0.4, 0.4, 0.5).into()
                } else {
                    Color::srgb(0.9, 0.95, 1.0).into()
                };
            }
            Interaction::None => {
                *color = if piano_key.note.is_black_key() {
                    Color::srgb(0.28, 0.32, 0.40).into()
                } else {
                    Color::srgb(0.98, 0.98, 1.0).into()
                };
            }
        }
    }

    if let Some(note) = pressed_note {
        piano_game.add_note(note);
    }
}

pub fn update_piano_sequence_display(
    mut display_query: Query<&mut Text, With<PianoDisplay>>,
    piano_game: Res<PianoTileGame>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut text) = display_query.get_single_mut() {
        text.sections[0].value = if piano_game.phase == PianoGamePhase::PlayingSound {
            "Loading...".to_string()
        } else {
            format!("{}/{}", piano_game.current_round, piano_game.total_rounds)
        };
        text.sections[0].style.font = asset_server.load("fonts/Galmuri11-Bold.ttf");
        text.sections[0].style.font_size = 45.0;
        text.sections[0].style.color = TEXT_WHITE;
    }
}

pub fn show_piano_success_image(
    piano_game: Res<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    success_query: Query<Entity, With<SuccessImage>>,
    piano_ui_query: Query<Entity, With<PianoMinigameUI>>
) {
    let should_show = piano_game.phase == PianoGamePhase::ShowingSuccess;
    let is_showing = !success_query.is_empty();
    
    if should_show && !is_showing {
        if let Ok(ui_entity) = piano_ui_query.get_single() {
            commands.entity(ui_entity).with_children(|parent| {
                parent.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        z_index: ZIndex::Global(1600)
                    ),
                    SuccessImage
                )).with_children(|overlay| {
                    overlay.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/success.png")),
                        ..default()
                    });
                });
            });
        }
    } else if !should_show && is_showing {
        for entity in success_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn show_piano_fail_image(
    piano_game: Res<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fail_query: Query<Entity, With<FailImage>>,
    piano_ui_query: Query<Entity, With<PianoMinigameUI>>
) {
    let should_show = piano_game.phase == PianoGamePhase::ShowingFail;
    let is_showing = !fail_query.is_empty();
    
    if should_show && !is_showing {
        if let Ok(ui_entity) = piano_ui_query.get_single() {
            commands.entity(ui_entity).with_children(|parent| {
                parent.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::top(Val::Px(150.0)),
                            ..default()
                        },
                        z_index: ZIndex::Global(1600)
                    ),
                    FailImage
                )).with_children(|overlay| {
                    overlay.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/fail.png")),
                        ..default()
                    });
                });
            });
        }
    } else if !should_show && is_showing {
        for entity in fail_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn show_piano_retry_overlay(
    piano_game: Res<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    retry_query: Query<Entity, With<FailOverlay>>,
    piano_ui_query: Query<Entity, With<PianoMinigameUI>>
) {
    let should_show = piano_game.phase == PianoGamePhase::ShowingRetry;
    let is_showing = !retry_query.is_empty();
    
    if should_show && !is_showing {
        if let Ok(ui_entity) = piano_ui_query.get_single() {
            commands.entity(ui_entity).with_children(|parent| {
                parent.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                        z_index: ZIndex::Global(1500)
                    ),
                    FailOverlay
                )).with_children(|overlay| {
                    overlay.spawn(node!(
                        style: Style {
                            width: Val::Px(812.0),
                            height: Val::Px(560.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            row_gap: Val::Px(30.0),
                            ..default()
                        }
                    )).with_children(|content| {
                        content.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(200.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(40.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(1.0, 1.0, 1.0, 1.0).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)),
                                ..default()
                            },
                            PianoRetryButton
                        )).with_children(|retry_btn| {
                            retry_btn.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(120.0),
                                    margin: UiRect::left(Val::Px(8.0)),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("images/minigame/retry3.png")),
                                ..default()
                            });
                        });

                        content.spawn(node!(
                            style: Style {
                                width: Val::Px(530.0),
                                height: Val::Px(145.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(25.0)),
                                ..default()
                            },
                            background_color: Color::srgba(0.733, 0.835, 0.941, 1.000).into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
                        )).with_children(|msg_box| {
                            msg_box.spawn(text!(
                                "let's retry!",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                55.0,
                                TEXT_WHITE
                            ));
                        });
                    });
                });
            });
        }
    }
}

pub fn handle_piano_retry_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PianoRetryButton>)>,
    mut piano_game: ResMut<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<FailOverlay>>,
    mut key_query: Query<(&mut BackgroundColor, &PianoKey), Without<PianoRetryButton>>,  // 추가
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // ✅ 누적 승리 수 초기화 추가
                piano_game.is_completed = 0;
                piano_game.key_obtained = false;
                
                // 모든 건반 색깔 초기화
                for (mut key_color, piano_key) in key_query.iter_mut() {
                    *key_color = if piano_key.note.is_black_key() {
                        Color::srgb(0.28, 0.32, 0.40).into()
                    } else {
                        Color::srgb(0.98, 0.98, 1.0).into()
                    };
                }
                
                piano_game.start_game();
                *color = Color::srgba(0.9, 0.9, 0.9, 1.0).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.95, 0.95, 0.95, 1.0).into();
            }
            _ => {
                *color = Color::srgba(1.0, 1.0, 1.0, 1.0).into();
            }
        }
    }
}

pub fn update_piano_minigame_display(
    mut piano_game: ResMut<PianoTileGame>,
    time: Res<Time>,
    mut completed_minigames: ResMut<CompletedPianoMinigames>,
    mut key_query: Query<(&mut BackgroundColor, &PianoKey)>,
) {
    match piano_game.phase {
        PianoGamePhase::ShowingSuccess => {
            piano_game.phase_timer.tick(time.delta());

            if piano_game.phase_timer.just_finished() {
                // ✅ 1. 라운드 성공 후 누적 카운트 증가
                piano_game.is_completed += 1;
                // ✅ 2. 모든 건반 색깔 초기화
                for (mut color, piano_key) in key_query.iter_mut() {
                    *color = if piano_key.note.is_black_key() {
                        Color::srgb(0.28, 0.32, 0.40).into()
                    } else {
                        Color::srgb(0.98, 0.98, 1.0).into()
                    };
                }

                // ✅ 3. 모든 라운드를 다 깼을 때만 승리 팝업 표시
                if piano_game.is_completed >= 3 {
                    piano_game.key_obtained = true;
                    completed_minigames.piano_tiles = true;
                    piano_game.show_key_popup();
                } else {
                    // ✅ 4. 아직 남은 라운드가 있으면 다음 라운드로 이동
                    piano_game.current_round += 1;
                    piano_game.phase = PianoGamePhase::WaitingForListen;
                    piano_game.player_sequence.clear();
                }
            }
        }

        PianoGamePhase::ShowingFail => {
            piano_game.phase_timer.tick(time.delta());
            if piano_game.phase_timer.just_finished() {
                // 실패 시에도 건반 색깔 초기화
                for (mut color, piano_key) in key_query.iter_mut() {
                    *color = if piano_key.note.is_black_key() {
                        Color::srgb(0.28, 0.32, 0.40).into()
                    } else {
                        Color::srgb(0.98, 0.98, 1.0).into()
                    };
                }

                piano_game.show_retry();
            }
        }

        _ => {}
    }
}

pub fn handle_piano_key_popup(
    piano_game: Res<PianoTileGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    popup_query: Query<Entity, With<KeyPopup>>,
    piano_ui_query: Query<Entity, With<PianoMinigameUI>>
) {
    let should_show = piano_game.phase == PianoGamePhase::ShowingKeyPopup;
    let is_showing = !popup_query.is_empty();
    
    if should_show && !is_showing {
        if let Ok(ue) = piano_ui_query.get_single() {
            let encrypted_key = piano_game.encrypted_key.clone();
            
            commands.entity(ue).with_children(|p| {
                p.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::top(Val::Px(100.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.75).into(),
                        z_index: ZIndex::Global(2000)
                    ),
                    KeyPopup
                )).with_children(|pop| {
                    pop.spawn(node!(
                        style: Style {
                            width: Val::Px(812.0),
                            height: Val::Px(560.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            row_gap: Val::Px(10.0),
                            ..default()
                        }
                    )).with_children(|container| {
                        container.spawn(node!(
                            style: Style {
                                width: Val::Px(510.0),
                                height: Val::Px(320.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect {
                                    left: Val::Px(30.0),
                                    right: Val::Px(30.0),
                                    top: Val::Px(15.0),
                                    bottom: Val::Px(15.0),
                                },
                                border: UiRect::all(Val::Px(3.0)),
                                row_gap: Val::Px(10.0),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0)),
                            background_color: Color::srgb(1.0, 1.0, 1.0).into()
                        )).with_children(|modal| {
                            modal.spawn(node!(
                                style: Style {
                                    width: Val::Px(469.0),
                                    height: Val::Px(95.0),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgba(0.650, 0.800, 0.880, 1.000).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(60.0))
                            )).with_children(|header| {
                                header.spawn(text!(
                                    "Congratulations!",
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    55.0,
                                    Color::srgb(1.0, 1.0, 1.0)
                                ));
                            });

                            modal.spawn(node!(
                                style: Style {
                                    width: Val::Px(469.0),
                                    height: Val::Px(176.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    row_gap: Val::Px(10.0),
                                    padding: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(0.650, 0.800, 0.880, 1.000).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                            )).with_children(|body| {
                                body.spawn(text!(
                                    "음악실의 열쇠를 얻었습니다! 축하드립니다!",
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    25.0,
                                    Color::srgba(0.412, 0.557, 0.706, 1.000)
                                ));

                                body.spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(70.0),
                                        height: Val::Px(70.0),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("images/minigame/star3.png")),
                                    ..default()
                                });

                                body.spawn(text!(
                                    encrypted_key,
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    20.0,
                                    Color::srgba(0.443, 0.612, 0.780, 1.000)
                                ));
                            });
                        });

                        container.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(151.0),
                                    height: Val::Px(51.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0)),
                                background_color: Color::srgba(0.650, 0.800, 0.880, 1.000).into(),
                                ..default()
                            },
                            PianoCloseButton
                        )).with_children(|next_btn| {
                            next_btn.spawn(text!(
                                "NEXT",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                40.0,
                                Color::srgb(1.0, 1.0, 1.0)
                            ));
                        });
                    });
                });
            });
        }
    }
}

pub fn update_key_icon_fade(
    time: Res<Time>,
    mut key_query: Query<(&mut KeyIcon, &Children)>,
    mut text_query: Query<&mut Text>
) {
    for (mut key_icon, children) in key_query.iter_mut() {
        if key_icon.is_fading_in {
            key_icon.fade_timer.tick(time.delta());
            let alpha = (key_icon.fade_timer.elapsed_secs() / key_icon.fade_timer.duration().as_secs_f32()).clamp(0.0, 1.0);
            
            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if let Some(section) = text.sections.get_mut(0) {
                        section.style.color = Color::srgba(0.8, 0.6, 0.2, alpha);
                    }
                }
            }
            
            if key_icon.fade_timer.just_finished() {
                key_icon.is_fading_in = false;
            }
        }
    }
}
