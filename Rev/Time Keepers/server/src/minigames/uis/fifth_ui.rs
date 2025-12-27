use bevy::prelude::*;
use crate::components::*;
use crate::audio_system::{AudioManager, play_click_sound};
use crate::minigames::fifth_minigame::{MathGame, MATH_DATA};  // 한줄로 통합
use crate::minigames::first_minigame::MinigameState;
use bevy::ecs::system::ParamSet;

// 기존 코드
const BG_MAIN: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BG_HEADER: Color = Color::srgba(0.647, 0.808, 0.686, 1.000);
const BG_GAME: Color = Color::srgba(0.647, 0.808, 0.686, 1.000);
const BG_QUESTION: Color = Color::srgba(0.949, 0.988, 0.949, 1.000);
const BG_ANSWER: Color = Color::srgba(0.949, 0.988, 0.949, 1.000);
const TEXT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const TEXT_DARK: Color = Color::srgba(0.455, 0.537, 0.447, 1.000);
const ROUND_BADGE: Color = Color::srgba(0.451, 0.549, 0.451, 1.000);

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $f:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $f, font_size: $s, color: $c }) }; }

pub fn setup_math_minigame(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        MathUI,
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
                border_radius: bevy::ui::BorderRadius::all(Val::Px(47.0))
            ),
            GameCard
        )).with_children(|card| {
            // 팝업
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
                        "Math Game",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        50.0,
                        TEXT_WHITE
                    ));
                    text_container.spawn(text!(
                        "시간 안에 올바른 정답을 클릭하세요!",
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
                    height: Val::Px(450.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        left: Val::Px(25.0),
                        right: Val::Px(25.0),
                        top: Val::Px(15.0),
                        bottom: Val::Px(15.0),
                    },
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
                background_color: BG_GAME.into(),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(47.0))
            )).with_children(|game_box| {
                // 그리드
                game_box.spawn(node!(
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(15.0)),
                        column_gap: Val::Px(10.0),
                        ..default()
                    }
                )).with_children(|info_row| {
                    // 라운드
                    info_row.spawn((
                        node!(
                            style: Style {
                                width: Val::Percent(33.33),
                                height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
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
                            MathDisplay
                        ));
                    });

                    // 라운드
                    info_row.spawn((
                        node!(
                            style: Style {
                                width: Val::Percent(66.67),
                                height: Val::Px(64.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: ROUND_BADGE.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
                        ),
                        ScoreDisplay
                    )).with_children(|score_badge| {
                        score_badge.spawn((
                            text!(
                                "0/4",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                40.0,
                                TEXT_WHITE
                            ),
                            ScoreText
                        ));
                    });
                });

                // 수학 게임 영역
                game_box.spawn((
                    node!(
                        style: Style {
                            width: Val::Percent(97.5),
                            height: Val::Px(340.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        }
                    ),
                    MathGameArea
                )).with_children(|area| {
                    // 질문 영역
                    area.spawn((
                        node!(
                            style: Style {
                                width: Val::Percent(100.0),
                                min_height: Val::Px(190.0),
                                padding: UiRect {
                                    left: Val::Px(15.0),
                                    right: Val::Px(15.0),
                                    top: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BG_QUESTION.into(),
                            border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0))
                        ),
                        MathQuestionContainer
                    )).with_children(|q_box| {
                        q_box.spawn((
                            text!(
                                "",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                80.0,
                                TEXT_DARK
                            ),
                            MathQuestionText
                        ));
                    });

                    // 답변 선택지 영역 (2x2 그리드)
                    area.spawn((
                        node!(
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(150.0),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(10.0),
                                ..default()
                            }
                        ),
                        MathAnswerArea
                    )).with_children(|ac| {
                        // 1번째 행
                        ac.spawn(node!(
                            style: Style {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            }
                        )).with_children(|row| {
                            for i in 0..2 {
                                row.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(320.0),
                                            height: Val::Px(70.0),
                                            padding: UiRect::all(Val::Px(12.0)),
                                            ..default()
                                        },
                                        background_color: BG_ANSWER.into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)),
                                        ..default()
                                    },
                                    MathAnswerButton { answer_index: i }
                                )).with_children(|b| {
                                    b.spawn(node!(
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(65.0),
                                            ..default()
                                        }
                                    )).with_children(|label_container| {
                                        label_container.spawn(node!(
                                            style: Style {
                                                width: Val::Px(65.0),
                                                height: Val::Px(55.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::left(Val::Px(2.0)),
                                                ..default()
                                            },
                                            background_color: ROUND_BADGE.into(),
                                            border_radius: bevy::ui::BorderRadius::all(Val::Px(10.0))
                                        )).with_children(|label_box| {
                                            label_box.spawn(text!(
                                                format!("A{}.", i + 1),
                                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                                30.0,
                                                TEXT_WHITE
                                            ));
                                        });

                                        label_container.spawn((
                                            text!(
                                                "",
                                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                                30.0,
                                                TEXT_DARK
                                            ),
                                            MathAnswerText
                                        ));
                                    });
                                });
                            }
                        });

                        // 2번째 행
                        ac.spawn(node!(
                            style: Style {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            }
                        )).with_children(|row| {
                            for i in 2..4 {
                                row.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(320.0),
                                            height: Val::Px(70.0),
                                            padding: UiRect::all(Val::Px(12.0)),
                                            ..default()
                                        },
                                        background_color: BG_ANSWER.into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)),
                                        ..default()
                                    },
                                    MathAnswerButton { answer_index: i }
                                )).with_children(|b| {
                                    b.spawn(node!(
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(65.0),
                                            ..default()
                                        }
                                    )).with_children(|label_container| {
                                        label_container.spawn(node!(
                                            style: Style {
                                                width: Val::Px(65.0),
                                                height: Val::Px(55.0),
                                                justify_content: JustifyContent::FlexEnd,
                                                align_items: AlignItems::Center,
                                                padding: UiRect::right(Val::Px(12.0)),
                                                ..default()
                                            },
                                            background_color: ROUND_BADGE.into(),
                                            border_radius: bevy::ui::BorderRadius::all(Val::Px(10.0))
                                        )).with_children(|label_box| {
                                            label_box.spawn(text!(
                                                format!("A{}.", i + 1),
                                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                                30.0,
                                                TEXT_WHITE
                                            ));
                                        });

                                        label_container.spawn((
                                            text!(
                                                "",
                                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                                30.0,
                                                TEXT_DARK
                                            ),
                                            MathAnswerText
                                        ));
                                    });
                                });
                            }
                        });
                    });
                });
            });

            // 버튼 컨테이너
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
                        background_color: Color::srgba(0.914, 0.949, 0.918, 1.000).into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)),
                        ..default()
                    },
                    MathStartButton
                )).with_children(|play_btn| {
                    play_btn.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(75.0),
                            height: Val::Px(77.0),
                            margin: UiRect::left(Val::Px(12.0)),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/triangle5.png")),
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
                    background_color: Color::srgba(0.647, 0.808, 0.686, 1.000).into(),
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

pub fn handle_math_start_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MathStartButton>)>,
    mut game: ResMut<MathGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<ReadyOverlay>>,
    retry_overlay_query: Query<Entity, With<RetryOverlay>>,
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                // ReadyOverlay 제거
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // RetryOverlay 제거
                if let Ok(entity) = retry_overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                // 게임 초기화
                *game = MathGame::default();
                game.game_started = true;
                
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

pub fn handle_math_answer_buttons(
    mut q: Query<(&Interaction, &mut BackgroundColor, &MathAnswerButton, &Children), With<Button>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text, With<MathAnswerText>>,
    mut game: ResMut<MathGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
) {
    // 게임이 시작되지 않았거나, 이미 답변했거나, 피드백 표시 중이면 버튼 비활성화
    if !game.game_started || game.is_answered || game.is_showing_feedback { 
        return; 
    }
    
    for (interaction, mut color, btn, btn_children) in q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                let correct_answer = MATH_DATA[game.current_round - 1].2;
                game.selected_answer = Some(btn.answer_index);
                game.is_answered = true;
                game.is_correct = Some(btn.answer_index == correct_answer);
                
                if btn.answer_index == correct_answer {
                    game.score += 1;
                }
                
                *color = Color::srgba(0.608, 0.710, 0.608, 1.000).into();
                
                for &child in btn_children.iter() {
                    if let Ok(container_children) = children_query.get(child) {
                        for &grandchild in container_children.iter() {
                            if let Ok(mut text) = text_query.get_mut(grandchild) {
                                text.sections[0].style.color = TEXT_WHITE;
                            }
                        }
                    }
                }
                
                // 정오답 모두 1.5초 피드백(성공/실패 이미지) 표시 후 처리
                game.is_showing_feedback = true;
                game.show_feedback_timer.reset();
            }
            Interaction::Hovered if !game.is_answered => {  // 조건 추가
                *color = Color::srgba(0.757, 0.855, 0.757, 1.000).into(); 
            }
            _ if !game.is_answered => {  // 조건 추가
                *color = BG_ANSWER.into(); 
            }
            _ => {}
        }
    }
}

pub fn update_math_display(
    mut game: ResMut<MathGame>,
    time: Res<Time>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<MathDisplay>>,
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<MathQuestionText>>,
        Query<&mut Text, With<MathAnswerText>>,
    )>,
    children_query: Query<&Children>,
    mut answer_query: Query<(&MathAnswerButton, &Children, &mut BackgroundColor)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    retry_overlay_query: Query<Entity, With<RetryOverlay>>,
    minigame_ui_query: Query<Entity, With<MathUI>>,
    popup_query: Query<Entity, With<KeyPopup>>,
) {
    // 게임 타이머
    if game.game_started && game.is_game_complete < 4 && !game.is_answered {
        game.game_timer.tick(time.delta());
        
        if game.game_timer.just_finished() {
            game.is_game_complete = 4;
        }
    }
    
    if game.is_showing_feedback {
        game.show_feedback_timer.tick(time.delta());
        if game.show_feedback_timer.just_finished() {
            game.is_showing_feedback = false;
            
            if game.current_round < game.total_rounds {
                if game.is_correct == Some(true) {
                    game.current_round += 1;
                    game.is_answered = false;
                    game.selected_answer = None;
                    game.is_correct = None;
                    game.game_timer.reset();
                } else {
                    // 오답이면 게임 종료
                    game.is_game_complete = 4;
                }
            } else {
                game.is_game_complete = 4;
                if game.score >= 4 {
                    game.key_obtained = true;
                }
            }
        }
    }
    
    // 라운드 표시
    for mut text in text_queries.p0().iter_mut() {
        if game.game_started {
            text.sections[0].value = format!("{}/4", game.current_round);
        }
    }
    
    // 스코어/타이머 표시
    for mut text in text_queries.p1().iter_mut() {
        if game.is_game_complete >= 4 {
            text.sections[0].value = format!("Score : {}/4", game.score);
        } else {
            let remaining = game.game_timer.duration().as_secs_f32() - game.game_timer.elapsed_secs();
            text.sections[0].value = format!("Time : {:.1}s", remaining.max(0.0));
        }
    }
    
    // 질문/답변 업데이트
    if game.game_started && game.is_game_complete < 4 {
        let (question, answers, _correct_idx) = &MATH_DATA[game.current_round - 1];
        
        for mut text in text_queries.p2().iter_mut() {
            text.sections[0].value = format!("{}", question);
        }
        
        for (btn, btn_children, mut bg_color) in answer_query.iter_mut() {
            for &child in btn_children.iter() {
                if let Ok(container_children) = children_query.get(child) {
                    for &grandchild in container_children.iter() {
                        if let Ok(mut text) = text_queries.p3().get_mut(grandchild) {
                            text.sections[0].value = answers[btn.answer_index].to_string();
                            
                            if !game.is_answered {
                                text.sections[0].style.color = TEXT_DARK;
                            }
                        }
                    }
                }
            }
            
            if game.is_answered {
                if Some(btn.answer_index) == game.selected_answer {
                    *bg_color = Color::srgba(0.608, 0.710, 0.608, 1.000).into();
                } else {
                    *bg_color = BG_ANSWER.into();
                }
            }
        }
    }
    
    // 게임 종료 처리 - 피드백이 완전히 끝난 후에만
    if game.is_game_complete >= 4 && !game.is_showing_feedback {
        if game.key_obtained {
            // 성공 팝업
            if popup_query.is_empty() {
                if let Ok(ue) = minigame_ui_query.get_single() {
                    let encrypted_key = game.encrypted_key.clone();
                    
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
                                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                                z_index: ZIndex::Global(2000)
                            ),
                            KeyPopup
                        )).with_children(|pop| {
                            // ... 기존 성공 팝업 코드 동일
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
                                        background_color: Color::srgba(0.647, 0.808, 0.686, 1.000).into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(30.0))
                                    )).with_children(|header| {
                                        header.spawn(text!(
                                            "Congratulations!",
                                            asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                            55.0,
                                            TEXT_WHITE
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
                                        background_color: Color::srgba(0.647, 0.808, 0.686, 1.000).into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                                    )).with_children(|body| {
                                        body.spawn(text!(
                                            "과학실의 열쇠를 얻었습니다! 축하드립니다!",
                                            asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                            25.0,
                                            Color::srgba(0.333, 0.459, 0.373, 1.000)
                                        ));

                                        body.spawn(ImageBundle {
                                            style: Style {
                                                width: Val::Px(70.0),
                                                height: Val::Px(70.0),
                                                ..default()
                                            },
                                            image: UiImage::new(asset_server.load("images/minigame/star5.png")),
                                            ..default()
                                        });

                                        body.spawn(text!(
                                            encrypted_key,
                                            asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                            20.0,
                                            Color::srgba(0.333, 0.459, 0.373, 1.000)
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
                                        background_color: Color::srgba(0.647, 0.808, 0.686, 1.000).into(),
                                        ..default()
                                    },
                                    MathCloseButton
                                )).with_children(|next_btn| {
                                    next_btn.spawn(text!(
                                        "NEXT",
                                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                        40.0,
                                        TEXT_WHITE
                                    ));
                                });
                            });
                        });
                    });
                }
            }
        } else {
            // 재시도 화면 - key_obtained가 false면 무조건 실패
            if retry_overlay_query.is_empty() {
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
                                    ..default()
                                },
                                background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                                z_index: ZIndex::Global(1500)
                            ),
                            RetryOverlay
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
                                    MathStartButton
                                )).with_children(|retry_btn| {
                                    retry_btn.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(150.0),
                                            margin: UiRect::left(Val::Px(10.0)),
                                            ..default()
                                        },
                                        image: UiImage::new(asset_server.load("images/minigame/retry5.png")),
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
                                    background_color: Color::srgba(0.647, 0.808, 0.686, 1.000).into(),
                                    border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
                                )).with_children(|msg_box| {
                                    msg_box.spawn(text!(
                                        "Let's retry",
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
    }
}

pub fn handle_math_close_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<MathCloseButton>)>,
    mut state: ResMut<NextState<MinigameState>>,
    mut dm: ResMut<crate::dialogue::DialogueManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    game: Res<MathGame>,  // ✅ 추가
    mut completed: ResMut<crate::minigames::first_minigame::CompletedMinigames>,  // ✅ 추가
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                // ✅ 성공한 경우에만 completed에 추가
                if game.key_obtained {
                    if let Some(scene_id) = dm.current_scene.as_ref().map(|s| s.scene_id.clone()) {
                        let key = (scene_id, dm.current_line_index);
                        completed.0.insert(key);
                    }
                }
                
                state.set(MinigameState::None);
                dm.next_line();
                *color = Color::srgba(0.647, 0.808, 0.686, 1.000).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.597, 0.758, 0.636, 1.000).into();
            }
            _ => {
                *color = Color::srgba(0.647, 0.808, 0.686, 1.000).into();
            }
        }
    }
}

pub fn show_math_success_image(
    game: Res<MathGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    success_query: Query<Entity, With<SuccessImage>>,
    math_ui_query: Query<Entity, With<MathUI>>
) {
    if game.is_showing_feedback && game.is_correct == Some(true) && success_query.is_empty() {
        if let Ok(ui_entity) = math_ui_query.get_single() {
            commands.entity(ui_entity).with_children(|parent| {
                parent.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::top(Val::Px(60.0)),
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
    } else if !(game.is_showing_feedback && game.is_correct == Some(true)) && !success_query.is_empty() {
        for entity in success_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn show_math_fail_image(
    game: Res<MathGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fail_query: Query<Entity, With<FailImage>>,
    math_ui_query: Query<Entity, With<MathUI>>
) {
    if game.is_showing_feedback && game.is_correct == Some(false) && fail_query.is_empty() {
        if let Ok(ui_entity) = math_ui_query.get_single() {  // ✅ 이 줄 추가
            commands.entity(ui_entity).with_children(|parent| {
                parent.spawn((
                    node!(
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::top(Val::Px(60.0)),
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
        }  // ✅ 이 줄 추가
    } else if !(game.is_showing_feedback && game.is_correct == Some(false)) && !fail_query.is_empty() {
        for entity in fail_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn handle_math_key_popup(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    game: Res<MathGame>,
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    popup_query: Query<Entity, With<KeyPopup>>,
    _minigame_ui_query: Query<Entity, With<MathUI>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) && game.key_obtained && game.is_game_complete >= 4 {
        keyboard_input.clear_just_pressed(KeyCode::Space);
        
        if let Ok(pe) = popup_query.get_single() {
            commands.entity(pe).despawn_recursive();
        }
    }
}

pub fn update_math_start_button_visibility(
    game: Res<MathGame>,
    mut button_query: Query<&mut Style, With<MathStartButton>>,
    ready_overlay_query: Query<(), With<ReadyOverlay>>,
) {
    if !ready_overlay_query.is_empty() {
        for mut style in button_query.iter_mut() {
            style.display = if game.game_started {
                Display::None
            } else {
                Display::Flex
            };
        }
    }
}

pub fn update_key_icon_fade(
    time: Res<Time>,
    mut query: Query<(&mut KeyIcon, &mut BackgroundColor)>,
) {
    for (mut icon, mut color) in query.iter_mut() {
        icon.fade_timer.tick(time.delta());
        
        let alpha = if icon.is_fading_in {
            icon.fade_timer.fraction()
        } else {
            1.0 - icon.fade_timer.fraction()
        };
        
        if let Color::Srgba(srgba) = color.0 {
            *color = Color::srgba(srgba.red, srgba.green, srgba.blue, alpha).into();
        }
        
        if icon.fade_timer.just_finished() {
            icon.is_fading_in = !icon.is_fading_in;
            icon.fade_timer.reset();
        }
    }
}

pub fn cleanup_math_minigame(mut commands: Commands, ui_query: Query<Entity, With<MathUI>>) {
    ui_query.iter().for_each(|e| commands.entity(e).despawn_recursive());
}

