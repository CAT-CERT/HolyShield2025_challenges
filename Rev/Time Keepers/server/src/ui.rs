use bevy::prelude::*;
use crate::components::*;
use crate::dialogue::{DialogueManager, VisitedLocations};
use crate::audio_system::{AudioManager, play_click_sound, play_dialogue_sound, play_choice_sound, play_chapter_bgm};
use crate::minigames::first_minigame::{MinigameState, ButtonSequenceGame};
use crate::minigames::second_minigame::ConstellationGame;
use crate::minigames::third_minigame::{PianoMinigameState, PianoTileGame};
use crate::minigames::fourth_minigame::QuizGame;
use crate::minigames::fifth_minigame::MathGame;

const CHOICE_COLORS: [Color; 3] = [Color::srgba(0.1, 0.1, 0.3, 0.9), Color::srgba(0.2, 0.2, 0.5, 0.95), Color::srgba(0.3, 0.3, 0.7, 1.0)];

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text {($t:expr, $font:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $font, font_size: $s, color: $c }) }; }
macro_rules! btn { ($($f:ident: $v:expr),*) => { ButtonBundle { $($f: $v,)* border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)), ..default() } }; }

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let window_size = windows.get_single().map(|w| Vec2::new(w.width(), w.height())).unwrap_or(Vec2::new(1280.0, 720.0));
    
    commands.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.0, 0.0, 0.0), custom_size: Some(window_size), ..default() },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -15.0)), ..default()
    }, BackgroundSprite, GameUI));
    
    commands.spawn((node!(style: Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(0.0),
        width: Val::Percent(100.0),
        height: Val::Vh(27.8),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Vw(1.6)), ..default() },
        background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
        visibility: Visibility::Visible), DialogueBox, GameUI)).with_children(|p| {
        p.spawn(node!(style: Style {
            width: Val::Percent(100.0),
            height: Val::Auto,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Vh(1.4)), ..default() })).with_children(|h| {
            h.spawn((text!("화자", asset_server.load("fonts/NotoSansKR-Regular.ttf"), 28.0, Color::srgb(1.0, 0.8, 0.0)), SpeakerNameText));
        });
        p.spawn((text!("대사가 여기에 표시됩니다.", asset_server.load("fonts/NotoSansKR-Regular.ttf"), 25.0, Color::WHITE), DialogueText));
    });
    
    commands.spawn((text!("스페이스바: 계속하기 | ESC: 메뉴", asset_server.load("fonts/NotoSansKR-Regular.ttf"), 16.0, Color::srgb(0.7, 0.7, 0.7)).with_style(Style { position_type: PositionType::Absolute, bottom: Val::Vh(1.4), right: Val::Vw(1.6), ..default() }), GameUI));
    
    // 설정 버튼
    commands.spawn((btn!(style: Style {
        position_type: PositionType::Absolute,
        top: Val::Vh(1.4),
        right: Val::Vw(0.8),
        width: Val::Vw(6.3),
        height: Val::Vh(5.6),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, ..default() },
        border_color: BorderColor(Color::srgb(1.0, 0.6, 0.7)),
        background_color: Color::srgba(1.0, 1.0, 1.0, 1.0).into()), crate::components::InGameSettingsButton, GameUI)).with_children(|p| {
        p.spawn(text!("설정", asset_server.load("fonts/YOnepick-Bold.ttf"), 18.0, Color::srgb(1.0, 0.6, 0.7)));
    });
    
    // 스킵 버튼
    commands.spawn((btn!(style: Style {
        position_type: PositionType::Absolute,
        top: Val::Vh(1.4), right: Val::Vw(8.5),
        width: Val::Vw(6.3), height: Val::Vh(5.6),
        border: UiRect::all(Val::Px(2.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center, ..default() },
        border_color: BorderColor(Color::srgb(1.0, 0.6, 0.7)),
        background_color: Color::srgba(1.0, 1.0, 1.0, 1.0).into()), SkipButton, GameUI)).with_children(|p| {
        p.spawn(text!("스킵", asset_server.load("fonts/YOnepick-Bold.ttf"), 18.0, Color::srgb(1.0, 0.6, 0.7)));
    });
}

fn is_any_minigame_blocking(
    minigame_state: &MinigameState, 
    button_game: Option<&ButtonSequenceGame>,
    constellation_game: Option<&ConstellationGame>,
    quiz_game: Option<&QuizGame>,
    math_game: Option<&MathGame>,
    piano_state: &PianoMinigameState,
    piano_tile_game: Option<&PianoTileGame>
) -> bool {
    let button_blocking = match minigame_state {
        MinigameState::ButtonSequence => {
            !button_game.map_or(false, |g| g.is_game_complete >= 4 && g.key_obtained)
        },
        MinigameState::Constellation => {
            !constellation_game.map_or(false, |g| g.is_game_complete >= 3 && g.key_obtained)
        },
        MinigameState::Quiz => {  // 새로 추가
            !quiz_game.map_or(false, |g| g.is_game_complete >= 4 && g.key_obtained)
        },
        MinigameState::Math => {
            // 기존 코드 삭제하고 아래로 교체
            !math_game.map_or(false, |g| g.is_game_complete >= 4 && g.key_obtained)
        },
        MinigameState::KeyCheck => true,
        MinigameState::None => false,
    };

    let piano_blocking = match piano_state {
        PianoMinigameState::Active => {
            // is_completed가 usize라면 0이 아닌 값을 완료로 간주
            !piano_tile_game.map_or(false, |g| g.is_completed > 0 && g.key_obtained)
        },
        PianoMinigameState::Inactive => false,
    };

    button_blocking || piano_blocking
}

pub fn handle_skip_button(
    mut q: Query<(&Interaction, &mut BackgroundColor, &Children), (Changed<Interaction>, With<SkipButton>)>,
    mut text_q: Query<&mut Text>,
    mut dm: ResMut<DialogueManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    mg_state: Res<State<MinigameState>>,
    mg_game: Option<Res<ButtonSequenceGame>>,
    constellation_game: Option<Res<ConstellationGame>>,
    quiz_game: Option<Res<QuizGame>>,
    math_game: Option<Res<MathGame>>,
    piano_state: Res<State<PianoMinigameState>>,
    piano_game: Option<Res<PianoTileGame>>
) {
    // 통합된 미니게임 차단 체크
    let is_minigame_blocking = is_any_minigame_blocking(
        mg_state.get(), 
        mg_game.as_deref(),
        constellation_game.as_deref(),
        quiz_game.as_deref(),
        math_game.as_deref(),
        piano_state.get(),
        piano_game.as_deref()
    );
    
    let has_minigame_trigger = dm.get_current_line()
        .map_or(false, |line| line.minigame_trigger.is_some());
    
    let is_skip_disabled = is_minigame_blocking || has_minigame_trigger;
    
    // 나머지 코드는 동일...
    for (interaction, mut bg, children) in &mut q {
        for &child in children {
            if let Ok(mut text) = text_q.get_mut(child) {
                (*bg, text.sections[0].style.color) = match *interaction {
                    Interaction::Pressed => {
                        if !is_skip_disabled {
                            play_click_sound(&mut commands, &asset_server, &audio_manager);
                            skip_to_next_event(&mut dm);
                        }
                        (Color::srgba(0.3, 0.3, 0.7, 1.0).into(), Color::srgb(1.0, 1.0, 1.0))
                    }
                    Interaction::Hovered => (Color::srgba(0.2, 0.2, 0.5, 0.9).into(), Color::srgb(1.0, 0.9, 0.6)),
                    _ => (Color::srgba(0.1, 0.1, 0.3, 0.8).into(), Color::srgb(0.9, 0.9, 1.0)),
                };
            }
        }
    }
}

fn skip_to_next_event(dm: &mut DialogueManager) {
    if let Some(scene) = dm.current_scene.as_ref() {
        let scene = scene.clone();
        let skip_target = scene.lines.iter().enumerate().skip(dm.current_line_index + 1).find(|(_, line)| line.event_image.is_some() || line.minigame_trigger.is_some() || line.choices.is_some()).map(|(i, _)| i);
        
        dm.current_line_index = skip_target.unwrap_or(scene.lines.len().saturating_sub(1));
        (dm.typing_effect.is_typing, dm.typing_effect.current_char_index, dm.choice_system.is_choice_active, dm.pending_minigame) = (false, 0, false, None);
        dm.typing_effect.full_text.clear(); dm.typing_effect.displayed_text.clear(); dm.choice_system.current_choices.clear(); dm.typing_effect.typing_timer.reset();
        
        if let Some(line) = scene.lines.get(dm.current_line_index) {
            if let Some(mt) = &line.minigame_trigger { dm.pending_minigame = Some(mt.clone()); }
            if line.event_image.is_none() && !line.text.is_empty() { dm.start_typing(line.text.clone()); }
            dm.set_changed();
        }
    }
}

pub fn update_typing_effect(mut dm: ResMut<DialogueManager>, time: Res<Time>) { dm.update_typing(time.delta()); }

pub fn handle_input(
    kb: Res<ButtonInput<KeyCode>>, 
    mut dm: ResMut<DialogueManager>, 
    mut state: ResMut<NextState<crate::game_state::GameState>>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    audio_manager: Res<AudioManager>, 
    mg_state: Res<State<MinigameState>>, 
    mg_game: Option<Res<ButtonSequenceGame>>,
    constellation_game: Option<Res<ConstellationGame>>,
    quiz_game: Option<Res<QuizGame>>,
    math_game: Option<Res<MathGame>>,
    piano_state: Res<State<PianoMinigameState>>,
    piano_game: Option<Res<PianoTileGame>>,
    // 키 팝업 체크 추가
    key_popup_query: Query<Entity, With<KeyPopup>>,
) {
    if kb.just_pressed(KeyCode::Space) {
        // 키 팝업이 떠 있으면 스페이스바 입력 무시
        if !key_popup_query.is_empty() {
            return;
        }
        
        if is_any_minigame_blocking(mg_state.get(), mg_game.as_deref(), constellation_game.as_deref(), quiz_game.as_deref(), math_game.as_deref(), piano_state.get(), piano_game.as_deref()) || dm.choice_system.is_choice_active { 
            return; 
        }
        let is_event = dm.get_current_line().map_or(false, |l| l.event_image.is_some());
        if is_event { dm.next_line(); return; }
        if dm.typing_effect.is_typing { dm.complete_typing(); return; }
        play_dialogue_sound(&mut commands, &asset_server, &audio_manager);
        dm.next_line();
    }
    if kb.just_pressed(KeyCode::Escape) { state.set(crate::game_state::GameState::MainMenu); }
}

pub fn update_dialogue_ui(dm: Res<DialogueManager>, mut speaker_q: Query<&mut Text, (With<SpeakerNameText>, Without<DialogueText>)>, mut dialogue_q: Query<&mut Text, (With<DialogueText>, Without<SpeakerNameText>)>, mut box_q: Query<&mut Visibility, With<DialogueBox>>) {
    for mut vis in box_q.iter_mut() {
        *vis = if dm.get_current_line().map_or(false, |l| l.event_image.is_some() || dm.choice_system.is_choice_active) { Visibility::Hidden } else { Visibility::Visible };
    }
    if let Some(line) = dm.get_current_line() {
        if line.event_image.is_some() { return; }
        for mut text in speaker_q.iter_mut() { text.sections[0].value = line.speaker.clone(); }
        for mut text in dialogue_q.iter_mut() { text.sections[0].value = dm.get_displayed_text().clone(); }
    }
}

pub fn start_initial_typing(mut dm: ResMut<DialogueManager>, mut started: Local<bool>) {
    if !*started {
        let should_start_typing = dm.get_current_line()
            .map(|line| line.event_image.is_none() && !line.text.is_empty())
            .unwrap_or(false);
        
        if should_start_typing {
            if let Some(line) = dm.get_current_line() {
                let text = line.text.clone();
                dm.start_typing(text);
            }
        }
        *started = true;
    }
}

pub fn manage_choice_ui(
    mut commands: Commands, 
    mut dm: ResMut<DialogueManager>, 
    visited_locations: Res<VisitedLocations>,
    asset_server: Res<AssetServer>, 
    choice_q: Query<Entity, With<ChoiceMenu>>
) {
    if dm.is_changed() {
        choice_q.iter().for_each(|e| commands.entity(e).despawn_recursive());
        if dm.choice_system.is_choice_active {
            // 필터링된 선택지 사용
            let filtered_choices = dm.get_filtered_current_choices(&visited_locations);
            
            if filtered_choices.is_empty() {
                
                // chapter4로 자동 이동
                dm.move_to_scene("chapter4");
                dm.choice_system.is_choice_active = false;
                return;
            }

            commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center, 
                        ..default() 
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                    ..default()
                }, 
                ChoiceMenu, 
                GameUI
            )).with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style { 
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Vh(2.1), 
                        ..default() 
                    },
                    ..default()
                }).with_children(|container| {

                    for (i, choice) in filtered_choices.iter().enumerate() {
                        container.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Vw(39.1),
                                    height: Val::Vh(8.3),
                                    border: UiRect::all(Val::Px(3.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Vw(0.8)), 
                                    ..default() 
                                },
                                border_color: BorderColor(Color::srgb(0.8, 0.7, 1.0)),
                                background_color: CHOICE_COLORS[0].into(),
                                border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)),
                                ..default()
                            },
                            ChoiceButton { 
                                choice_index: i,
                            }
                        )).with_children(|button| {
                            button.spawn((
                                TextBundle::from_section(
                                    format!("{}. {}", i + 1, choice.text), 
                                    TextStyle {
                                        font: asset_server.load("fonts/NotoSansKR-Regular.ttf"), 
                                        font_size: 20.0, 
                                        color: Color::WHITE
                                    }
                                ), 
                                ChoiceText
                            ));
                        });
                    }
                });
            });
        }
    }
}

// handle_choice_buttons 함수도 수정
pub fn handle_choice_buttons(
    mut q: Query<(&Interaction, &mut BackgroundColor, &ChoiceButton), (Changed<Interaction>, With<Button>)>,
    mut dm: ResMut<DialogueManager>,
    mut visited_locations: ResMut<VisitedLocations>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut audio_manager: ResMut<AudioManager>,
    bgm_q: Query<Entity, With<BackgroundMusic>>
) {
    for (interaction, mut color, btn) in q.iter_mut() {
        *color = match *interaction {
            Interaction::Pressed => {
                play_choice_sound(&mut commands, &asset_server, &audio_manager);

                // 필터링된 선택지에서 원래 선택지 찾기
                let filtered_choices = dm.get_filtered_current_choices(&visited_locations);
                if btn.choice_index < filtered_choices.len() {
                    let choice = &filtered_choices[btn.choice_index];

                    if let Some(next_scene_id) = &choice.next_scene_id {
                        // 챕터 완료 기록
                        visited_locations.completed_chapters.insert(next_scene_id.clone());

                        // BGM 변경
                        let bgm_file = DialogueManager::get_bgm_for_scene(next_scene_id);
                        play_chapter_bgm(&mut commands, &asset_server, &mut audio_manager, &bgm_q, bgm_file);

                        // 씬 이동
                        dm.move_to_scene(next_scene_id);
                    } else {
                        // next_line_index나 일반적인 선택지 처리
                        dm.make_choice(btn.choice_index);
                    }
                }

                CHOICE_COLORS[2]
            }
            Interaction::Hovered => CHOICE_COLORS[1],
            _ => CHOICE_COLORS[0],
        }.into();
    }
}

pub fn handle_choice_keyboard(kb: Res<ButtonInput<KeyCode>>, mut dm: ResMut<DialogueManager>, mut commands: Commands, asset_server: Res<AssetServer>, audio_manager: Res<AudioManager>) {
    if !dm.choice_system.is_choice_active { return; }
    for (i, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4].iter().enumerate().take(dm.get_current_choices().len().min(4)) {
        if kb.just_pressed(*key)
        {
            play_choice_sound(&mut commands, &asset_server, &audio_manager);
            break;
        }
    }
}

pub fn handle_dialogue_progression(dm: Res<DialogueManager>) { if dm.is_scene_finished() { /* 씬 종료 처리 */ } }
pub fn cleanup_game_ui(mut commands: Commands, ui_q: Query<Entity, With<GameUI>>) { ui_q.iter().for_each(|e| commands.entity(e).despawn_recursive()); }
