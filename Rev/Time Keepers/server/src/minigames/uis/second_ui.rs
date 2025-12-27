use bevy::prelude::*;
use crate::components::*;
use crate::audio_system::{AudioManager, play_click_sound};
use crate::minigames::first_minigame::MinigameState;
use crate::minigames::second_minigame::{ConstellationGame};

const STAR_DIAMETER: f32 = 32.0;
const STAR_RADIUS: f32 = STAR_DIAMETER / 2.0;
const STAR_LABEL_SIZE: f32 = 14.0;

// Figma에서 가져온 정확한 색상 팔레트
const BG_MAIN: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const BG_HEADER: Color = Color::srgba(0.784, 0.753, 0.843, 1.0);
const BG_GAME: Color = Color::srgba(0.784, 0.753, 0.843, 1.0);
const TEXT_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
const ROUND_BADGE: Color = Color::srgba(0.686, 0.631, 0.753, 1.0);

// 별 색상 - 이미지 기반
const STAR_COLORS: [Color; 3] = [
    Color::srgba(0.9, 0.9, 0.9, 1.0),      // 기본: 밝은 회색/흰색
    Color::srgba(1.0, 1.0, 1.0, 1.0),      // 호버: 완전 흰색
    Color::srgba(0.686, 0.478, 0.686, 1.0), // 연결됨: 보라색
];

// 연결선 색상
const LINE_COLOR: Color = Color::srgba(0.478, 0.408, 0.545, 1.0);

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $f:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $f, font_size: $s, color: $c }) }; }

#[derive(Component)]
pub struct ConstellationStar { pub star_id: usize }

pub fn setup_constellation_minigame(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _: Query<&Window>,
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
        ConstellationUI,
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
                    padding: UiRect {
                        left: Val::Px(12.0),
                        right: Val::Px(12.0),
                        top: Val::Px(8.0),
                        bottom: Val::Px(12.0),
                    },
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
                        "Connecting Stars",
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        50.0,
                        TEXT_WHITE
                    ));
                    text_container.spawn(text!(
                        "별을 차례대로 연결해보세요",
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
                    height: Val::Px(550.0),
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
                // 상단 정보 표시 영역
                game_box.spawn(node!(
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(15.0)),
                        column_gap: Val::Px(5.0),  // 10.0에서 5.0으로 감소
                        ..default()
                    }
                )).with_children(|info_row| {
                    // 라운드 표시 (왼쪽)
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
                                "1/3",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                40.0,
                                TEXT_WHITE
                            ),
                            ConstellationRoundDisplay
                        ));
                    });

                    // 별자리 이름 표시 (오른쪽)
                    info_row.spawn(
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
                    )).with_children(|name_box| {
                        name_box.spawn((
                            text!(
                                "Cassiopeia",
                                asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                35.0,
                                TEXT_WHITE
                            ),
                            ConstellationNameDisplay
                        ));
                    });
                });
                // 별자리 게임 영역
                game_box.spawn((
                    node!(
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(330.0),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        background_color: Color::srgba(0.686, 0.631, 0.753, 1.0).into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(25.0))
                    ),
                    ConstellationGameArea
                ));
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
                    row_gap: Val::Px(20.0),  // 30.0에서 20.0으로 감소
                    ..default()
                }
            )).with_children(|content| {
                // 플레이 버튼 (원형)
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
                        background_color: Color::srgba(0.933, 0.914, 0.945, 1.0).into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)),
                        ..default()
                    },
                    ConstellationStartButton
                )).with_children(|play_btn| {
                    play_btn.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(75.0),
                            height: Val::Px(77.0),
                            margin: UiRect::left(Val::Px(12.0)),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("images/minigame/triangle2.png")),
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
                        padding: UiRect {
                            left: Val::Px(25.0),
                            right: Val::Px(25.0),
                            top: Val::Px(25.0),
                            bottom: Val::Px(35.0),
                        },
                        ..default()
                    },
                    background_color: Color::srgba(0.784, 0.753, 0.843, 1.0).into(),
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

// 별들을 동적으로 생성
pub fn update_constellation_stars(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game: Res<ConstellationGame>,
    game_area_query: Query<Entity, With<ConstellationGameArea>>,
    star_query: Query<Entity, With<ConstellationStar>>,
) {
    if !game.is_changed() { return; }

    // 기존 별들 제거
    for entity in star_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if !game.game_started || game.is_game_failed || game.is_game_complete >= 3 {
        return;
    }

    if let Ok(game_area) = game_area_query.get_single() {
        let constellation = game.get_current_constellation();

        commands.entity(game_area).with_children(|parent| {
            for (i, &(x, y)) in constellation.stars.iter().enumerate() {
                let is_connected = game.stars_connected.contains(&i);
                let color = if is_connected { STAR_COLORS[2] } else { STAR_COLORS[0] };

                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(x - STAR_RADIUS),
                            top: Val::Px(y - STAR_RADIUS),
                            width: Val::Px(STAR_DIAMETER),
                            height: Val::Px(STAR_DIAMETER),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: color.into(),
                        border_radius: bevy::ui::BorderRadius::all(Val::Px(STAR_RADIUS)),
                        ..default()
                    },
                    ConstellationStar { star_id: i }
                )).with_children(|star| {
                    star.spawn(text!(
                        format!("{}", i + 1),
                        asset_server.load("fonts/Galmuri11-Bold.ttf"),
                        STAR_LABEL_SIZE,
                        Color::srgba(0.3, 0.3, 0.3, 1.0)
                    ));
                });
            }
        });
    }
}

// 연결선 그리기
pub fn update_constellation_lines(
    mut commands: Commands,
    game: Res<ConstellationGame>,
    game_area_query: Query<Entity, With<ConstellationGameArea>>,
    line_query: Query<Entity, With<ConstellationLine>>,
) {
    if !game.is_changed() { return; }

    for e in line_query.iter() {
        commands.entity(e).despawn_recursive();
    }

    if !game.game_started || game.is_game_failed || game.is_game_complete >= 3 {
        return;
    }

    if let Ok(game_area) = game_area_query.get_single() {
        commands.entity(game_area).with_children(|parent| {
            for (start_pos, end_pos) in &game.connection_lines {
                let length = start_pos.distance(*end_pos);
                let center_x = (start_pos.x + end_pos.x) / 2.0;
                let center_y = (start_pos.y + end_pos.y) / 2.0;
                let angle = (end_pos.y - start_pos.y).atan2(end_pos.x - start_pos.x);

                parent.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(center_x - length / 2.0),
                            top: Val::Px(center_y - 2.0),
                            width: Val::Px(length),
                            height: Val::Px(4.0),
                            ..default()
                        },
                        background_color: LINE_COLOR.into(),  // 회색 연결선
                        transform: Transform::from_rotation(Quat::from_rotation_z(angle)),
                        ..default()
                    },
                    ConstellationLine,
                ));
            }
        });
    }
}

pub fn handle_constellation_start_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ConstellationStartButton>)>,
    mut game: ResMut<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<ReadyOverlay>>,
    mut status_query: Query<&mut Style, With<StatusContainer>>,
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                game.hard_reset_for_entry();
                game.game_started = true;

                for mut style in status_query.iter_mut() {
                    style.display = Display::Flex;
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

pub fn handle_constellation_stars(
    mut star_query: Query<(&Interaction, &mut BackgroundColor, &ConstellationStar), (Changed<Interaction>, With<Button>)>,
    mut game: ResMut<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
) {
    if !game.game_started || game.is_game_complete >= 3 || game.is_game_failed {
        return;
    }

    for (interaction, mut color, star) in star_query.iter_mut() {
        let is_connected = game.stars_connected.contains(&star.star_id);

        match *interaction {
            Interaction::Pressed if !is_connected => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);

                if game.connect_star(star.star_id) {
                    *color = STAR_COLORS[2].into();  // 보라색
                } else if game.is_game_failed {
                    *color = Color::srgba(0.890, 0.458, 0.443, 1.0).into();  // 빨간색 (실패)
                }
            }
            Interaction::Hovered if !is_connected => {
                *color = STAR_COLORS[1].into();  // 흰색 (호버)
            }
            _ if !is_connected => {
                *color = STAR_COLORS[0].into();  // 회색 (기본)
            }
            _ => {}
        }
    }
}

pub fn update_constellation_display(
    game: Res<ConstellationGame>,
    mut round_query: Query<&mut Text, With<ConstellationRoundDisplay>>,
    mut name_query: Query<&mut Text, (With<ConstellationNameDisplay>, Without<ConstellationRoundDisplay>)>,
) {
    if !game.is_changed() { return; }

    let constellation = game.get_current_constellation();

    for mut text in name_query.iter_mut() {
        text.sections[0].value = constellation.name.to_string();
    }

    for mut text in round_query.iter_mut() {
        text.sections[0].value = format!("{}/3", game.current_round);
    }
}

// Success 이미지 표시
pub fn show_constellation_success_image(
    game: Res<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    success_query: Query<Entity, With<SuccessImage>>,
    constellation_ui_query: Query<Entity, With<ConstellationUI>>
) {
    if game.is_changed() && game.show_round_complete_message && success_query.is_empty() {
        if let Ok(ui_entity) = constellation_ui_query.get_single() {
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
    } else if !game.show_round_complete_message && !success_query.is_empty() {
        for entity in success_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Fail 이미지 표시
pub fn show_constellation_fail_image(
    game: Res<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fail_query: Query<Entity, With<FailImage>>,
    constellation_ui_query: Query<Entity, With<ConstellationUI>>
) {
    if game.is_changed() && game.show_fail_message && fail_query.is_empty() {
        if let Ok(ui_entity) = constellation_ui_query.get_single() {
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
    } else if !game.show_fail_message && !fail_query.is_empty() {
        for entity in fail_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Retry 오버레이 표시
pub fn show_constellation_retry_overlay(
    game: Res<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    retry_query: Query<Entity, With<FailOverlay>>,
    constellation_ui_query: Query<Entity, With<ConstellationUI>>
) {
    if game.is_game_failed && !game.show_fail_message && retry_query.is_empty() {
        if let Ok(ui_entity) = constellation_ui_query.get_single() {
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
                                background_color: Color::srgba(0.933, 0.914, 0.945, 1.0).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(100.0)),
                                ..default()
                            },
                            ConstellationRetryButton
                        )).with_children(|retry_btn| {
                            retry_btn.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(120.0),
                                    margin: UiRect::left(Val::Px(8.0)),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("images/minigame/retry2.png")),
                                ..default()
                            });
                        });

                        content.spawn(node!(
                            style: Style {
                                width: Val::Px(530.0),
                                height: Val::Px(145.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect {
                                    left: Val::Px(25.0),
                                    right: Val::Px(25.0),
                                    top: Val::Px(25.0),
                                    bottom: Val::Px(35.0),
                                },
                                ..default()
                            },
                            background_color: Color::srgba(0.686, 0.631, 0.753, 1.0) .into(),
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

pub fn handle_constellation_retry_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ConstellationRetryButton>)>,
    mut game: ResMut<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    overlay_query: Query<Entity, With<FailOverlay>>,
    mut status_query: Query<&mut Style, With<StatusContainer>>,
) {
    for (interaction, mut color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                
                if let Ok(entity) = overlay_query.get_single() {
                    commands.entity(entity).despawn_recursive();
                }
                
                game.hard_reset_for_entry();
                game.game_started = true;

                for mut style in status_query.iter_mut() {
                    style.display = Display::Flex;
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

// 키 팝업 표시
pub fn handle_constellation_key_popup(
    game: Res<ConstellationGame>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    popup_query: Query<Entity, With<KeyPopup>>,
    constellation_ui_query: Query<Entity, With<ConstellationUI>>
) {
    if game.key_obtained 
        && game.is_game_complete >= 3 
        && !game.show_round_complete_message
        && popup_query.is_empty() 
    {
        if let Ok(ue) = constellation_ui_query.get_single() {
            let encrypted_key = "5dcb92dab920abafae1b";
            
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
                                background_color: Color::srgba(0.784, 0.753, 0.843, 1.0).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(30.0))
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
                                background_color: Color::srgba(0.784, 0.753, 0.843, 1.0).into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
                            )).with_children(|body| {
                                body.spawn(text!(
                                    "별자리의 열쇠를 얻었습니다! 축하드립니다!",
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    25.0,
                                    Color::srgba(0.561, 0.424, 0.686, 1.0)
                                ));

                                body.spawn(ImageBundle {
                                    style: Style {
                                        width: Val::Px(70.0),
                                        height: Val::Px(70.0),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("images/minigame/star2.png")),
                                    ..default()
                                });

                                body.spawn(text!(
                                    encrypted_key,
                                    asset_server.load("fonts/Galmuri11-Bold.ttf"),
                                    20.0,
                                    Color::srgba(0.627, 0.459, 0.761, 1.0)
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
                                background_color: Color::srgba(0.784, 0.753, 0.843, 1.0).into(),
                                ..default()
                            },
                            ConstellationCloseButton
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

pub fn handle_constellation_next_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ConstellationCloseButton>)>,  // ConstellationNextButton -> ConstellationCloseButton
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
                *color = Color::srgba(0.724, 0.693, 0.803, 1.0).into();  // 버튼 색상에 맞게 조정
            }
            Interaction::Hovered => {
                *color = Color::srgba(0.684, 0.653, 0.763, 1.0).into();
            }
            _ => {
                *color = Color::srgba(0.784, 0.753, 0.843, 1.0).into();
            }
        }
    }
}

pub fn update_constellation_timers(
    time: Res<Time>,
    mut game: ResMut<ConstellationGame>,
) {
    if game.show_round_complete_message {
        game.success_timer.tick(time.delta());
        
        if game.success_timer.just_finished() {
            game.show_round_complete_message = false;
            
            // 마지막 라운드가 아닐 때만 다음 라운드로 진행
            if game.current_round < 3 {
                game.advance_to_next_round();
            }
        }
    }
    
    if game.show_fail_message {
        game.fail_timer.tick(time.delta());
        
        if game.fail_timer.just_finished() {
            game.show_fail_message = false;
        }
    }
}

pub fn cleanup_constellation_minigame(
    mut commands: Commands,
    ui_query: Query<Entity, With<ConstellationUI>>,
) {
    ui_query.iter().for_each(|e| commands.entity(e).despawn_recursive());
}