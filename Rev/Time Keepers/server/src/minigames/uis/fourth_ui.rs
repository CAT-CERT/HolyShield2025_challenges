use bevy::prelude::*;
use crate::components::*;
use crate::audio_system::{AudioManager, play_click_sound};
use crate::minigames::fourth_minigame::{QuizGame, QUIZ_DATA};
use crate::minigames::first_minigame::MinigameState;
use bevy::ecs::system::ParamSet;

// 색상 팔레트
const BG_MAIN: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BG_HEADER: Color = Color::srgba(0.882, 0.831, 0.729, 1.000);
const BG_GAME: Color = Color::srgba(0.902, 0.851, 0.749, 1.000);
const BG_QUESTION: Color = Color::srgba(0.980, 0.961, 0.902, 1.000);
const BG_ANSWER: Color = Color::srgba(0.980, 0.961, 0.902, 1.000);
const TEXT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const TEXT_DARK: Color = Color::srgba(0.537, 0.447, 0.447, 1.000);
const ROUND_BADGE: Color = Color::srgba(0.733, 0.659, 0.580, 1.000);

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $f:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $f, font_size: $s, color: $c }) }; }

pub fn setup_quiz_minigame(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        "Quiz Game",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        50.0,
                        TEXT_WHITE
                    ));
                    text_container.spawn(text!(
                        "올바른 정답을 클릭하세요!",
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
                // 상단 정보 표시 영역 (라운드 + 스코어) - 1:2 비율
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
                    // 라운드 표시 (왼쪽) - 1/3
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
                            QuizDisplay
                        ));
                    });

                    // 스코어 표시 (오른쪽) - 2/3
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

                // 퀴즈 게임 영역
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
                    QuizGameArea
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
                        QuestionContainer
                    )).with_children(|q_box| {
                        q_box.spawn((
                            text!(
                                "",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                40.0,
                                TEXT_DARK
                            ),
                            QuizQuestion
                        ));
                    });

                    // 답변 선택지 영역 (2x2 그리드)
                    area.spawn((
                        node!(
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(150.0),
                                display: Display::Flex,  // Grid 대신 Flex 사용
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,  // 세로 가운데
                                align_items: AlignItems::Center,  // 가로 가운데
                                row_gap: Val::Px(10.0),
                                ..default()
                            }
                        ),
                        AnswerContainer
                    )).with_children(|ac| {
                        // 첫 번째 행
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
                                            width: Val::Px(320.0),  // 고정 너비
                                            height: Val::Px(70.0),
                                            padding: UiRect::all(Val::Px(12.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgba(0.980, 0.961, 0.902, 1.000).into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)),
                                        ..default()
                                    },
                                    QuizAnswerButton { answer_index: i }
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
                                            background_color: Color::srgba(0.733, 0.659, 0.580, 1.000).into(),
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
                                                18.0,
                                                TEXT_DARK
                                            ),
                                            QuizAnswerText
                                        ));
                                    });
                                });
                            }
                        });

                        // 두 번째 행
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
                                        background_color: Color::srgba(0.980, 0.961, 0.902, 1.000).into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)),
                                        ..default()
                                    },
                                    QuizAnswerButton { answer_index: i }
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
                                            background_color: Color::srgba(0.733, 0.659, 0.580, 1.000).into(),
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
                                                18.0,
                                                TEXT_DARK
                                            ),
                                            QuizAnswerText
                                        ));
                                    });
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
                        background_color: Color::srgba(0.949, 0.937, 0.914, 1.000).into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)),
                        ..default()
                    },
                    QuizStartButton
                )).with_children(|play_btn| {
                    play_btn.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(75.0),
                            height: Val::Px(77.0),
                            margin: UiRect::left(Val::Px(12.0)),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/triangle4.png")),
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
                    background_color: Color::srgba(0.851, 0.784, 0.749, 1.000).into(),
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

pub fn handle_quiz_start_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuizStartButton>)>,
    mut game: ResMut<QuizGame>,
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
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                if let Ok(entity) = retry_overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                *game = QuizGame::default();
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

pub fn handle_quiz_answer_buttons(
    mut q: Query<(&Interaction, &mut BackgroundColor, &QuizAnswerButton, &Children), With<Button>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text, With<QuizAnswerText>>,
    mut game: ResMut<QuizGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
) {
    if !game.game_started || game.is_answered || game.is_showing_feedback { return; }
    
    for (interaction, mut color, btn, btn_children) in q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                let correct_answer = QUIZ_DATA[game.current_round - 1].2;
                game.selected_answer = Some(btn.answer_index);
                game.is_answered = true;
                game.is_correct = Some(btn.answer_index == correct_answer);
                
                if btn.answer_index == correct_answer {
                    game.score += 1;
                }
                
                *color = Color::srgba(0.816, 0.741, 0.667, 1.000).into();
                
                // 클릭된 버튼의 텍스트 색상을 TEXT_WHITE로 변경
                for &child in btn_children.iter() {
                    if let Ok(container_children) = children_query.get(child) {
                        for &grandchild in container_children.iter() {
                            if let Ok(mut text) = text_query.get_mut(grandchild) {
                                text.sections[0].style.color = TEXT_WHITE;
                            }
                        }
                    }
                }
                
                game.is_showing_feedback = true;
                game.show_feedback_timer.reset();
            }
            Interaction::Hovered if !game.is_answered => { 
                *color = Color::srgba(0.866, 0.791, 0.717, 1.000).into();
            }
            _ if !game.is_answered => { 
                *color = BG_ANSWER.into(); 
            }
            _ => {}
        }
    }
}

pub fn update_quiz_display(
    mut game: ResMut<QuizGame>,
    time: Res<Time>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<QuizDisplay>>,
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<QuizQuestion>>,
        Query<&mut Text, With<QuizAnswerText>>,
    )>,
    children_query: Query<&Children>,
    mut answer_query: Query<(&QuizAnswerButton, &Children, &mut BackgroundColor)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    retry_overlay_query: Query<Entity, With<RetryOverlay>>,
    minigame_ui_query: Query<Entity, With<MinigameUI>>,
    popup_query: Query<Entity, With<KeyPopup>>,
) {
    if game.is_showing_feedback {
        game.show_feedback_timer.tick(time.delta());
        if game.show_feedback_timer.just_finished() {
            game.is_showing_feedback = false;
            
            if game.current_round < game.total_rounds {
                game.current_round += 1;
                game.is_answered = false;
                game.selected_answer = None;
                game.is_correct = None;
            } else {
                game.is_game_complete = 4;
                if game.score >= 3 {
                    game.key_obtained = true;
                }
            }
        }
    }
    
    // 라운드 표시 업데이트
    for mut text in text_queries.p0().iter_mut() {
        if game.game_started {
            text.sections[0].value = format!("{}/4", game.current_round);
        }
    }
    
    // 점수 표시 업데이트
    for mut text in text_queries.p1().iter_mut() {
        text.sections[0].value = format!("Score : {}/4", game.score);
    }
    
    // 질문 업데이트
    if game.game_started && game.is_game_complete < 4 {
        let (question, answers, _correct_idx) = &QUIZ_DATA[game.current_round - 1];
        
        for mut text in text_queries.p2().iter_mut() {
            text.sections[0].value = format!("Q. {}", question);
        }
        
        // 답변 텍스트 업데이트
        for (btn, btn_children, mut bg_color) in answer_query.iter_mut() {
            for &child in btn_children.iter() {
                if let Ok(container_children) = children_query.get(child) {
                    for &grandchild in container_children.iter() {
                        if let Ok(mut text) = text_queries.p3().get_mut(grandchild) {
                            text.sections[0].value = answers[btn.answer_index].to_string();
                            
                            // 답변이 선택되지 않았으면 텍스트 색상을 TEXT_DARK로 초기화
                            if !game.is_answered {
                                text.sections[0].style.color = TEXT_DARK;
                            }
                        }
                    }
                }
            }
            
            // 색상 처리
            if game.is_answered {
                if Some(btn.answer_index) == game.selected_answer {
                    *bg_color = Color::srgba(0.816, 0.741, 0.667, 1.000).into();
                } else {
                    *bg_color = BG_ANSWER.into();
                }
            }
        }
    }
    
    // 게임 완료 처리
    if game.is_game_complete >= 4 {
        if game.score >= 3 {
            // 3점 이상이면 키 팝업 자동 표시
            if popup_query.is_empty() && game.key_obtained {
                    if let Ok(ue) = minigame_ui_query.get_single() {
                        let encrypted_key = "52d6c9dd2b7f2c9bb326";
                    
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
                                        background_color: Color::srgba(0.851, 0.784, 0.749, 1.000).into(),
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
                                        background_color: Color::srgba(0.851, 0.784, 0.749, 1.000).into(),
                                        border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                                    )).with_children(|body| {
                                        body.spawn(text!(
                                            "도서관의 열쇠를 얻었습니다! 축하드립니다!",
                                            asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                            25.0,
                                            Color::srgba(0.459, 0.333, 0.333, 1.000)
                                        ));

                                        body.spawn(ImageBundle {
                                            style: Style {
                                                width: Val::Px(70.0),
                                                height: Val::Px(70.0),
                                                ..default()
                                            },
                                            image: UiImage::new(asset_server.load("images/minigame/star4.png")),
                                            ..default()
                                        });

                                        body.spawn(text!(
                                            encrypted_key,
                                            asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                            20.0,
                                            Color::srgba(0.459, 0.333, 0.333, 1.000)
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
                                        background_color: Color::srgba(0.851, 0.784, 0.749, 1.000).into(),
                                        ..default()
                                    },
                                    QuizCloseButton
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
            // 3점 미만이면 재시도 화면
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
                                    QuizStartButton
                                )).with_children(|retry_btn| {
                                    retry_btn.spawn(ImageBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(150.0),
                                            margin: UiRect::left(Val::Px(10.0)),
                                            ..default()
                                        },
                                        image: UiImage::new(asset_server.load("images/minigame/retry4.png")),
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
                                    background_color: Color::srgba(0.851, 0.784, 0.749, 1.000).into(),
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

pub fn handle_quiz_close_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<QuizCloseButton>)>,
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
                *color = Color::srgba(0.851, 0.784, 0.749, 1.000).into();
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.821, 0.754, 0.719, 1.000).into();
            }
            _ => {
                *color = Color::srgba(0.851, 0.784, 0.749, 1.000).into();
            }
        }
    }
}

pub fn show_quiz_success_image(
    game: Res<QuizGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    success_query: Query<Entity, With<SuccessImage>>,
    quiz_ui_query: Query<Entity, With<MinigameUI>>
) {
    if game.is_showing_feedback && game.is_correct == Some(true) && success_query.is_empty() {
        if let Ok(ui_entity) = quiz_ui_query.get_single() {
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

pub fn show_quiz_fail_image(
    game: Res<QuizGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fail_query: Query<Entity, With<FailImage>>,
    quiz_ui_query: Query<Entity, With<MinigameUI>>
) {
    if game.is_showing_feedback && game.is_correct == Some(false) && fail_query.is_empty() {
        if let Ok(ui_entity) = quiz_ui_query.get_single() {
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
        }
    } else if !(game.is_showing_feedback && game.is_correct == Some(false)) && !fail_query.is_empty() {
        for entity in fail_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn handle_quiz_key_popup(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    game: Res<QuizGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    popup_query: Query<Entity, With<KeyPopup>>,
    minigame_ui_query: Query<Entity, With<MinigameUI>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) && game.key_obtained && game.is_game_complete >= 4 {
        keyboard_input.clear_just_pressed(KeyCode::Space);
        
        if let Ok(pe) = popup_query.get_single() {
            commands.entity(pe).despawn_recursive();
        } else if let Ok(ue) = minigame_ui_query.get_single() {
            let encrypted_key = "52d6c9dd2b7f2c9bb326";
            
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
                                background_color: BG_HEADER.into(),
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
                                background_color: BG_HEADER.into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                            )).with_children(|body| {
                                body.spawn(text!(
                                    "도서관의 열쇠를 얻었습니다! 축하드립니다!",
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    25.0,
                                    Color::srgba(0.7, 0.5, 0.4, 1.0)
                                ));

                                body.spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(70.0),
                                        height: Val::Px(70.0),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("images/minigame/star4.png")),
                                    ..default()
                                });

                                body.spawn(text!(
                                    encrypted_key,
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    20.0,
                                    Color::srgba(0.7, 0.5, 0.4, 1.0)
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
                                background_color: BG_HEADER.into(),
                                ..default()
                            },
                            QuizCloseButton
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
}

pub fn cleanup_quiz_minigame(mut commands: Commands, ui_query: Query<Entity, With<MinigameUI>>) {
    ui_query.iter().for_each(|e| commands.entity(e).despawn_recursive());
}
