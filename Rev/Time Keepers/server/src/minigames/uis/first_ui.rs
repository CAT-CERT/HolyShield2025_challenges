use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use crate::components::*;
use crate::audio_system::{AudioManager, play_click_sound};
use crate::minigames::first_minigame::{ButtonSequenceGame, MinigameState};

const ROUND_DATA: [(usize, usize); 4] = [(0, 2), (2, 3), (5, 4), (9, 15)];

// Figma에서 가져온 정확한 색상 팔레트 (채도를 낮춘 버전)
const BG_MAIN: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BG_HEADER: Color = Color::srgba(0.94, 0.83, 0.83, 1.0);  // 더 밝은 핑크
const BG_GAME: Color = Color::srgba(0.96, 0.90, 0.89, 1.0);  // 핑크빛 베이지
const BTN_NORMAL: Color = Color::srgba(0.91, 0.68, 0.66, 1.0);  // 더 핑크한 버튼
const BTN_HOVER: Color = Color::srgba(0.81, 0.58, 0.56, 1.0);  // 더 핑크한 호버
const BTN_ACTIVE: Color = Color::srgba(0.71, 0.48, 0.46, 1.0);  // 더 핑크한 액티브
const BTN_HIGHLIGHT: Color = Color::srgb(0.85, 0.54, 0.64);  // 더 핑크한 하이라이트
const TEXT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const ROUND_BADGE: Color = Color::srgba(0.94, 0.83, 0.83, 1.0);  // BG_HEADER와 동일

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $f:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $f, font_size: $s, color: $c }) }; }
macro_rules! btn { ($($f:ident: $v:expr),*) => { ButtonBundle { $($f: $v,)* border_radius: bevy::ui::BorderRadius::all(Val::Px(22.0)), ..default() } }; }

pub fn setup_button_sequence_minigame(mut commands: Commands, asset_server: Res<AssetServer>, _: Query<&Window>) {
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
        MinigameUI,
        GameUI
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
                        "Button Sequence Game",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        50.0,
                        TEXT_WHITE
                    ));
                    text_container.spawn(text!(
                        "불이 들어오는 버튼의 순서를 기억하세요!",
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
                        top: Val::Px(25.0),
                        bottom: Val::Px(15.0), // 25.0에서 15.0으로 감소 (10px 증가)
                    },
                    margin: UiRect::top(Val::Px(5.0)),
                    ..default()
                },
                background_color: BG_GAME.into(),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(47.0))
            )).with_children(|game_box| {
                // 라운드 표시
                game_box.spawn((
                    node!(
                        style: Style {
                            width: Val::Px(210.0),
                            height: Val::Px(64.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(35.0)),
                            ..default()
                        },
                        background_color: ROUND_BADGE.into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
                    ),
                    RoundDisplay
                )).with_children(|badge| {
                    badge.spawn((
                        text!(
                            "1/4",
                            asset_server.load("fonts/Galmuri11-Bold.ttf"),
                            40.0,
                            TEXT_WHITE
                        ),
                        MinigameDisplay
                    ));
                });

                // 버튼 그리드
                game_box.spawn((
                    node!(
                        style: Style {
                            width: Val::Px(610.0),
                            height: Val::Px(280.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(120.0))
                    ),
                    GameButtonGrid
                )).with_children(|grid| {
                    for i in 0..2 {
                        grid.spawn(node!(
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(135.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                column_gap: Val::Px(30.0),
                                ..default()
                            }
                        )).with_children(|row| {
                            for j in 0..2 {
                                row.spawn((
                                    btn!(
                                        style: Style {
                                            width: Val::Px(290.0),
                                            height: Val::Px(135.0),
                                            border: UiRect::all(Val::Px(0.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: BTN_NORMAL.into()
                                    ),
                                    MinigameButton { button_id: i * 2 + j }
                                )).with_children(|btn_child| {
                                    btn_child.spawn(text!(
                                        format!("{}", i * 2 + j + 1),
                                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                        40.0,
                                        TEXT_WHITE
                                    ));
                                });
                            }
                        });
                    }
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
                // 게임 상태 메시지
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

        // "Are you ready?" 오버레이 - 카드 밖에 배치
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
                // 플레이 버튼 (원형) - 이미지 버전
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
                    PlayButton
                )).with_children(|play_btn| {
                    play_btn.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(75.0),
                            height: Val::Px(77.0),
                            margin: UiRect::left(Val::Px(12.0)),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/triangle.png")),
                        ..default()
                    });
                });

                // "are you ready?" 메시지
                content.spawn(node!(
                    style: Style {
                        width: Val::Px(530.0),
                        height: Val::Px(145.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(25.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.94, 0.83, 0.83, 1.0).into(),
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

pub fn handle_minigame_start_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PlayButton>)>,
    mut game: ResMut<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<ReadyOverlay>>,
    mut status_query: Query<&mut Style, With<StatusContainer>>,
    mut grid_query: Query<&mut BackgroundColor, (With<MinigameButton>, Without<PlayButton>)>
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                *game = ButtonSequenceGame::default();
                game.game_started = true;
                game.is_in_round_delay = true; // 시작 딜레이 활성화
                game.round_delay_timer.reset(); // 0.5초 타이머 시작

                for mut style in status_query.iter_mut() {
                    style.display = Display::Flex;
                }

                for mut bg_color in grid_query.iter_mut() {
                    *bg_color = BTN_NORMAL.into();
                }

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

pub fn handle_minigame_close_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MinigameCloseButton>)>,
    mut state: ResMut<NextState<MinigameState>>,
    mut dm: ResMut<crate::dialogue::DialogueManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>
) {
    for (interaction, mut color) in &mut q {
        *color = match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                state.set(MinigameState::None);
                dm.next_line();
                BTN_ACTIVE
            }
            Interaction::Hovered => BTN_HOVER,
            _ => BTN_NORMAL,
        }.into();
    }
}

pub fn handle_minigame_buttons(
    mut queries: ParamSet<(
        Query<(&Interaction, &mut BackgroundColor, &MinigameButton), (Changed<Interaction>, With<Button>)>,
        Query<(&mut BackgroundColor, &MinigameButton)>
    )>,
    mut game: ResMut<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>
) {
    if !game.is_accepting_input { return; }
    
    let mut game_failed = false;
    for (interaction, mut color, btn) in queries.p0().iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                game.player_input.push(btn.button_id);
                
                *color = Color::srgba(0.890, 0.458, 0.443, 1.0).into();
                
                check_input(&mut game);
                
                if game.is_game_failed {
                    game_failed = true;
                }
            },
            Interaction::Hovered if game.is_accepting_input => {
                *color = BTN_HOVER.into();
            },
            _ => {
                *color = BTN_NORMAL.into();
            }
        }
    }
    
    if game_failed {
        queries.p1().iter_mut().for_each(|(mut c, _)| *c = BTN_NORMAL.into());
    }
}

fn check_input(game: &mut ButtonSequenceGame) {
    let (start, len) = ROUND_DATA[game.current_round.saturating_sub(1).min(3)];
    
    if let Some(&expected) = game.sequence.get(start + game.player_input.len() - 1) {
        if game.player_input.last() != Some(&expected) {
            game.is_game_failed = true;
            game.is_accepting_input = false;
            game.is_showing_fail = true; // fail 이미지 표시
            game.fail_timer.reset();
            return;
        }
    }
    
    if game.player_input.len() >= len {
        if game.current_round >= game.total_rounds {
            game.is_game_complete = 4;
            game.is_accepting_input = false;
            game.key_obtained = true;
        } else {
            game.current_round += 1;
            game.is_game_complete += 1;
            game.player_input.clear();
            game.is_accepting_input = false;
            game.is_showing_success = true;
            game.success_timer.reset();
        }
    }
}

pub fn update_sequence_display(
    time: Res<Time>,
    mut game: ResMut<ButtonSequenceGame>,
    mut btns: Query<(&mut BackgroundColor, &MinigameButton)>
) {
    let dt = time.delta();
    
    if game.is_showing_fail {
        game.fail_timer.tick(dt);
        if game.fail_timer.just_finished() {
            game.is_showing_fail = false;
            // is_game_failed는 유지하여 retry 오버레이 표시
        }
    } else if game.is_showing_success {
        game.success_timer.tick(dt);
        if game.success_timer.just_finished() {
            game.is_showing_success = false;
            game.is_in_round_delay = true;
            game.round_delay_timer.reset();
        }
    } else if game.is_in_round_delay {
        if game.player_input.is_empty() && game.round_delay_timer.elapsed_secs() == 0.0 {
            reset_colors(&mut btns);
    
            // ✅ 4라운드 들어가기 직전 속도 변경
            if game.current_round == 4 {
                game.sequence_timer.set_duration(std::time::Duration::from_secs_f32(0.05));
                game.gap_timer.set_duration(std::time::Duration::from_secs_f32(0.02));
            } else {
                game.sequence_timer.set_duration(std::time::Duration::from_secs_f32(0.4));
                game.gap_timer.set_duration(std::time::Duration::from_secs_f32(0.1));
            }
        }
        
        game.round_delay_timer.tick(dt);
        if game.round_delay_timer.just_finished() {
            game.is_in_round_delay = false;
            game.is_showing_sequence = true;
            game.current_showing_index = 0;
            game.sequence_timer.reset();
        }
    } else if game.is_in_end_delay {
        game.end_delay_timer.tick(dt);
        if game.end_delay_timer.just_finished() {
            game.is_in_end_delay = false;
            game.is_accepting_input = true;
            reset_colors(&mut btns);
        }
    } else if game.is_in_gap {
        game.gap_timer.tick(dt);
        if game.gap_timer.just_finished() {
            game.is_in_gap = false;
            game.sequence_timer.reset();
            let (start, _) = ROUND_DATA[game.current_round.saturating_sub(1).min(3)];
            if let Some(&id) = game.sequence.get(start + game.current_showing_index) {
                highlight_btn(&mut btns, id);
            }
        }
    } else if game.is_showing_sequence {
        if game.current_showing_index == 0 && game.sequence_timer.elapsed_secs() == 0.0 {
            let (start, _) = ROUND_DATA[game.current_round.saturating_sub(1).min(3)];
            if let Some(&id) = game.sequence.get(start) {
                highlight_btn(&mut btns, id);
            }
        }
        
        game.sequence_timer.tick(dt);
        if game.sequence_timer.just_finished() {
            let (_, len) = ROUND_DATA[game.current_round.saturating_sub(1).min(3)];
            game.current_showing_index += 1;
            
            if game.current_showing_index >= len {
                game.is_showing_sequence = false;
                game.is_in_end_delay = true;
                game.current_showing_index = 0;
                game.end_delay_timer.reset();
                reset_colors(&mut btns);
            } else {
                reset_colors(&mut btns);
                game.is_in_gap = true;
                game.gap_timer.reset();
            }
        }
    }
}

fn reset_colors(q: &mut Query<(&mut BackgroundColor, &MinigameButton)>) {
    q.iter_mut().for_each(|(mut c, _)| *c = BTN_NORMAL.into());
}

fn highlight_btn(q: &mut Query<(&mut BackgroundColor, &MinigameButton)>, id: usize) {
    for (mut c, btn) in q.iter_mut() {
        if btn.button_id == id {
            *c = BTN_HIGHLIGHT.into();
            break;
        }
    }
}

pub fn update_minigame_display(
    minigame: Res<ButtonSequenceGame>,
    mut display_query: Query<&mut Text, With<MinigameDisplay>>,
) {
    if !minigame.is_changed() { return; }

    for mut text in display_query.iter_mut() {
        // 시퀀스를 보여주는 중이거나 라운드 딜레이 중일 때만 Loading... 표시
        if minigame.is_showing_sequence || minigame.is_in_round_delay {
            text.sections[0].value = "Loading...".to_string();
        } else {
            text.sections[0].value = format!("{}/{}", minigame.current_round, minigame.total_rounds);
        }
    }
}

pub fn show_success_image(
    minigame: Res<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    success_query: Query<Entity, With<SuccessImage>>,
    minigame_ui_query: Query<Entity, With<MinigameUI>>
) {
    if minigame.is_showing_success && success_query.is_empty() {
        if let Ok(ui_entity) = minigame_ui_query.get_single() {
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
    } else if !minigame.is_showing_success && !success_query.is_empty() {
        for entity in success_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn handle_key_popup(
    minigame: Res<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    popup_query: Query<Entity, With<KeyPopup>>,
    minigame_ui_query: Query<Entity, With<MinigameUI>>
) {
    if minigame.is_game_complete >= 4 && popup_query.is_empty() {
        if let Ok(ue) = minigame_ui_query.get_single() {
            let encrypted_key = minigame.encrypted_key.clone();
            
            commands.entity(ue).with_children(|p| {
                p.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::top(Val::Px(50.0)),
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
                                background_color: Color::srgba(0.961, 0.855, 0.855, 1.0).into(),
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
                                background_color: Color::srgba(0.961, 0.855, 0.855, 1.0).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                            )).with_children(|body| {
                                body.spawn(text!(
                                    "체육관의 열쇠를 얻었습니다! 축하드립니다!",
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    25.0,
                                    Color::srgba(0.757, 0.576, 0.569, 1.000)
                                ));

                                body.spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(70.0),
                                        height: Val::Px(70.0),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("images/minigame/star.png")),
                                    ..default()
                                });

                                body.spawn(text!(
                                    encrypted_key,
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    20.0,
                                    Color::srgba(0.757, 0.576, 0.569, 1.000)
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
                                background_color: Color::srgba(0.945, 0.780, 0.776, 1.0).into(),
                                ..default()
                            },
                            NextButton
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
    mut image_query: Query<&mut BackgroundColor, With<Handle<Image>>>
) {
    for (mut key_icon, children) in key_query.iter_mut() {
        if key_icon.is_fading_in {
            key_icon.fade_timer.tick(time.delta());
            let alpha = (key_icon.fade_timer.elapsed_secs() / key_icon.fade_timer.duration().as_secs_f32()).clamp(0.0, 1.0);
            
            for &child in children {
                if let Ok(mut bg) = image_query.get_mut(child) {
                    *bg = BackgroundColor(Color::srgba(1.0, 1.0, 1.0, alpha));
                }
            }
            
            if key_icon.fade_timer.just_finished() {
                key_icon.is_fading_in = false;
            }
        }
    }
}

pub fn show_fail_image(
    minigame: Res<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fail_query: Query<Entity, With<FailImage>>,
    minigame_ui_query: Query<Entity, With<MinigameUI>>
) {
    if minigame.is_showing_fail && fail_query.is_empty() {
        if let Ok(ui_entity) = minigame_ui_query.get_single() {
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
    } else if !minigame.is_showing_fail && !fail_query.is_empty() {
        for entity in fail_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn show_retry_overlay(
    minigame: Res<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    retry_query: Query<Entity, With<FailOverlay>>,
    minigame_ui_query: Query<Entity, With<MinigameUI>>
) {
    // fail 이미지가 끝나고 게임이 실패 상태일 때만 retry 오버레이 표시
    if minigame.is_game_failed && !minigame.is_showing_fail && retry_query.is_empty() {
        if let Ok(ui_entity) = minigame_ui_query.get_single() {
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
                            row_gap: Val::Px(30.0), // 시작 버튼과 동일
                            ..default()
                        }
                    )).with_children(|content| {
                        // 재시작 버튼 (원형) - 시작 버튼과 동일한 크기와 위치
                        content.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0), // 170에서 200으로
                                    height: Val::Px(200.0), // 170에서 200으로
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(40.0)), // 시작 버튼과 동일
                                    ..default()
                                },
                                background_color: Color::srgba(1.0, 1.0, 1.0, 1.0).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)), // 85에서 100으로
                                ..default()
                            },
                            RetryButton
                        )).with_children(|retry_btn| {
                            retry_btn.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(120.0),
                                    margin: UiRect::left(Val::Px(8.0)),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("images/minigame/retry.png")),
                                ..default()
                            });
                        });

                        // "let's retry!" 메시지 - 시작 버튼과 동일한 크기
                        content.spawn(node!(
                            style: Style {
                                width: Val::Px(530.0), // 500에서 530으로
                                height: Val::Px(145.0), // 90에서 145로
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(25.0)), // 20에서 25로
                                ..default()
                            },
                            background_color: Color::srgba(0.94, 0.83, 0.83, 1.0).into(),
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

pub fn handle_next_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<NextButton>)>,
    mut state: ResMut<NextState<MinigameState>>,
    mut dm: ResMut<crate::dialogue::DialogueManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                state.set(MinigameState::None);
                dm.next_line();
                *color = Color::srgba(0.885, 0.720, 0.716, 1.0).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.845, 0.680, 0.676, 1.0).into();
            }
            _ => {
                *color = Color::srgba(0.945, 0.780, 0.776, 1.0).into();
            }
        }
    }
}

pub fn handle_retry_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RetryButton>)>,
    mut game: ResMut<ButtonSequenceGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<FailOverlay>>,
    mut status_query: Query<&mut Style, With<StatusContainer>>,
    mut grid_query: Query<&mut BackgroundColor, (With<MinigameButton>, Without<RetryButton>)>
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                *game = ButtonSequenceGame::default();
                game.game_started = true;
                game.is_in_round_delay = true;
                game.round_delay_timer.reset();

                for mut style in status_query.iter_mut() {
                    style.display = Display::Flex;
                }

                for mut bg_color in grid_query.iter_mut() {
                    *bg_color = BTN_NORMAL.into();
                }

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

pub fn cleanup_minigame(mut commands: Commands, ui_query: Query<Entity, With<MinigameUI>>) {
    ui_query.iter().for_each(|e| commands.entity(e).despawn_recursive());
}
