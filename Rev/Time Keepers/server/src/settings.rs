use bevy::prelude::*;
use bevy::time::TimerMode;
use crate::{game_state::GameState, audio_system::{AudioManager, play_click_sound}, components::*};

const SLIDER_WIDTH: f32 = 300.0;
const SLIDER_HEIGHT: f32 = 20.0;
const NORMAL_BUTTON: Color = Color::srgb(0.1, 0.1, 0.3);
const HOVERED_BUTTON: Color = Color::srgb(0.2, 0.2, 0.5);
const PRESSED_BUTTON: Color = Color::srgb(0.3, 0.3, 0.7);
const SETTINGS_BACKGROUND_FADE_DURATION: f32 = 1.1;

macro_rules! node { ($($f:ident: $v:expr),*) => { NodeBundle { $($f: $v,)* ..default() } }; }
macro_rules! text { ($t:expr, $font:expr, $s:expr, $c:expr) => { TextBundle::from_section($t, TextStyle { font: $font, font_size: $s, color: $c }) }; }
macro_rules! btn { ($style:expr, $bg:expr, $border:expr) => { ButtonBundle { style: $style, background_color: $bg.into(), border_color: BorderColor($border), border_radius: bevy::ui::BorderRadius::all(Val::Px(15.0)), ..default() } }; }

pub fn setup_ingame_settings_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        btn!(
            Style {
                position_type: PositionType::Absolute,
                top: Val::Vh(1.4),
                right: Val::Vw(0.8),
                width: Val::Vw(6.3),
                height: Val::Vh(5.6),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Color::srgba(0.1, 0.1, 0.3, 0.8),
            Color::srgb(0.8, 0.7, 1.0)
        ),
        InGameSettingsButton,
        GameUI
    )).with_children(|p| {
        p.spawn(text!("설정", asset_server.load("fonts/YOnepick-Bold.ttf"), 18.0, Color::srgb(0.9, 0.9, 1.0)));
    });
}

pub fn handle_ingame_settings_button(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<InGameSettingsButton>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: Res<AudioManager>,
    popup_q: Query<Entity, With<InGameSettingsPopup>>
) {
    for (interaction, mut color) in &mut q {
        *color = match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                if popup_q.is_empty() {
                    setup_ingame_settings_popup(&mut commands, &asset_server, &audio_manager);
                } else {
                    popup_q.iter().for_each(|e| commands.entity(e).despawn_recursive());
                }
                PRESSED_BUTTON
            }
            Interaction::Hovered => HOVERED_BUTTON,
            _ => Color::srgba(0.1, 0.1, 0.3, 0.8),
        }.into();
    }
}

fn setup_ingame_settings_popup(commands: &mut Commands, asset_server: &Res<AssetServer>, audio_manager: &Res<AudioManager>) {
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
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into()
        ),
        InGameSettingsPopup,
        GameUI
    )).with_children(|p| {
        p.spawn(node!(
            style: Style {
                width: Val::Vw(46.9),
                height: Val::Vh(69.4),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Vw(2.3)),
                border: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.05, 0.2).into(),
            border_color: BorderColor(Color::srgb(0.8, 0.7, 1.0)),
            border_radius: bevy::ui::BorderRadius::all(Val::Px(20.0))
        )).with_children(|popup| {
            popup.spawn(text!("설정", asset_server.load("fonts/YOnepick-Bold.ttf"), 36.0, Color::srgb(0.9, 0.9, 1.0))
                .with_style(Style { margin: UiRect::bottom(Val::Vh(4.2)), ..default() }));

            for (label, vol_type, vol, id) in [
                ("마스터 볼륨", VolumeType::Master, audio_manager.settings.master_volume, 10),
                ("배경음악", VolumeType::Bgm, audio_manager.settings.bgm_volume, 11),
                ("효과음", VolumeType::Sfx, audio_manager.settings.sfx_volume, 12),
                ("클릭음", VolumeType::Click, audio_manager.settings.click_volume, 13)
            ] {
                create_volume_slider(popup, asset_server, label, vol_type, vol, id);
            }

            popup.spawn(node!(
                style: Style {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Vw(1.6),
                    margin: UiRect::top(Val::Vh(2.0)),
                    ..default()
                }
            )).with_children(|btns| {
                btns.spawn((
                    btn!(
                        Style {
                            width: Val::Vw(11.7),
                            height: Val::Vh(6.9),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Color::srgb(0.1, 0.3, 0.1),
                        Color::srgb(0.7, 1.0, 0.7)
                    ),
                    PopupBackToGameButton
                )).with_children(|p| {
                    p.spawn(text!("게임으로", asset_server.load("fonts/YOnepick-Bold.ttf"), 18.0, Color::srgb(0.9, 1.0, 0.9)));
                });

                btns.spawn((
                    btn!(
                        Style {
                            width: Val::Vw(11.7),
                            height: Val::Vh(6.9),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Color::srgb(0.3, 0.1, 0.1),
                        Color::srgb(1.0, 0.7, 0.7)
                    ),
                    PopupExitGameButton
                )).with_children(|p| {
                    p.spawn(text!("게임 종료", asset_server.load("fonts/YOnepick-Bold.ttf"), 18.0, Color::srgb(1.0, 0.9, 0.9)));
                });
            });
        });
    });
}

pub fn handle_ingame_popup_buttons(
    mut q: Query<(&Interaction, &mut BackgroundColor, Option<&PopupBackToGameButton>, Option<&PopupExitGameButton>), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_manager: ResMut<AudioManager>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
    popup_q: Query<Entity, With<InGameSettingsPopup>>
) {
    for (interaction, mut color, back, exit) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                play_click_sound(&mut commands, &asset_server, &audio_manager);
                let _ = audio_manager.save_settings();
                if back.is_some() {
                    *color = Color::srgb(0.2, 0.5, 0.2).into();
                    popup_q.iter().for_each(|e| commands.entity(e).despawn_recursive());
                } else if exit.is_some() {
                    *color = Color::srgb(0.5, 0.2, 0.2).into();
                    app_exit.send(bevy::app::AppExit::Success);
                }
            }
            Interaction::Hovered => {
                *color = if back.is_some() {
                    Color::srgb(0.15, 0.4, 0.15)
                } else if exit.is_some() {
                    Color::srgb(0.4, 0.15, 0.15)
                } else {
                    return;
                }.into()
            }
            _ => {
                *color = if back.is_some() {
                    Color::srgb(0.1, 0.3, 0.1)
                } else if exit.is_some() {
                    Color::srgb(0.3, 0.1, 0.1)
                } else {
                    return;
                }.into()
            }
        }
    }
}

pub fn setup_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>, audio_manager: Res<AudioManager>, windows: Query<&Window>) {

    let window_size = windows.get_single()
        .map(|w| Vec2::new(w.width(), w.height()))
        .unwrap_or(Vec2::new(1280.0, 720.0));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/background_o.png"),
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                custom_size: Some(window_size),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            ..default()
        },
        SettingsMenu,
        SettingsBackgroundFade {
            timer: Timer::from_seconds(SETTINGS_BACKGROUND_FADE_DURATION, TimerMode::Once),
        },
    ));

    commands.spawn((
        node!(
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            }
        ),
        SettingsMenu
    )).with_children(|p| {
        p.spawn(text!("오디오 설정", asset_server.load("fonts/YOnepick-Bold.ttf"), 48.0, Color::srgb(0.9, 0.9, 1.0))
            .with_style(Style { margin: UiRect::bottom(Val::Vh(5.6)), ..default() }));

        for (label, vol_type, vol, id) in [
            ("마스터 볼륨", VolumeType::Master, audio_manager.settings.master_volume, 0),
            ("배경음악", VolumeType::Bgm, audio_manager.settings.bgm_volume, 1),
            ("효과음", VolumeType::Sfx, audio_manager.settings.sfx_volume, 2),
            ("클릭음", VolumeType::Click, audio_manager.settings.click_volume, 3)
        ] {
            create_volume_slider(p, &asset_server, label, vol_type, vol, id);
        }

        p.spawn((
            btn!(
                Style {
                    width: Val::Vw(15.6),
                    height: Val::Vh(8.3),
                    border: UiRect::all(Val::Px(3.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Vh(5.6)),
                    ..default()
                },
                NORMAL_BUTTON,
                Color::srgb(0.8, 0.7, 1.0)
            ),
            BackToMenuButton
        )).with_children(|p| {
            p.spawn(text!("돌아가기", asset_server.load("fonts/YOnepick-Bold.ttf"), 24.0, Color::srgb(0.9, 0.9, 1.0)));
        });
    });
}

fn create_volume_slider(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: &str,
    volume_type: VolumeType,
    current_volume: f32,
    slider_id: u32
) {
    parent.spawn(node!(
        style: Style {
            width: Val::Vw(31.3),
            height: Val::Vh(11.1),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Vh(2.8)),
            ..default()
        }
    )).with_children(|p| {
        p.spawn(node!(
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Vh(4.2),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            }
        )).with_children(|p| {
            p.spawn(text!(label, asset_server.load("fonts/YOnepick-Bold.ttf"), 24.0, Color::srgb(0.9, 0.9, 1.0)));
            p.spawn((
                text!(format!("{}%", (current_volume * 100.0) as i32), asset_server.load("fonts/YOnepick-Bold.ttf"), 24.0, Color::srgb(0.8, 0.8, 1.0)),
                VolumeText { volume_type, slider_id }
            ));
        });

        p.spawn((
            node!(
                style: Style {
                    width: Val::Px(SLIDER_WIDTH),
                    height: Val::Px(SLIDER_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect::top(Val::Vh(1.4)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::srgb(0.8, 0.7, 1.0)),
                background_color: Color::srgb(0.1, 0.1, 0.2).into(),
                border_radius: bevy::ui::BorderRadius::all(Val::Px(10.0))
            ),
            SliderTrack{}
        )).with_children(|p| {
            p.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(20.0),
                        height: Val::Px(SLIDER_HEIGHT - 4.0),
                        left: Val::Px((SLIDER_WIDTH - 24.0) * current_volume),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.7, 0.6, 1.0).into(),
                    border_color: BorderColor(Color::srgb(1.0, 1.0, 1.0)),
                    border_radius: bevy::ui::BorderRadius::all(Val::Px(8.0)),
                    ..default()
                },
                VolumeSlider { volume_type, slider_id, is_dragging: false }
            ));
        });
    });
}

pub fn handle_settings_input(
    mut q: Query<(&Interaction, &mut BackgroundColor, Option<&BackToMenuButton>), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
    audio_manager: ResMut<AudioManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (interaction, mut color, back) in &mut q {
        if back.is_some() {
            *color = match *interaction {
                Interaction::Pressed => {
                    play_click_sound(&mut commands, &asset_server, &audio_manager);
                    let _ = audio_manager.save_settings();
                    state.set(GameState::MainMenu);
                    PRESSED_BUTTON
                }
                Interaction::Hovered => HOVERED_BUTTON,
                _ => NORMAL_BUTTON,
            }.into();
        }
    }
}

pub fn handle_volume_sliders(
    mut slider_q: Query<(&Interaction, &mut Style, &mut VolumeSlider, &mut BackgroundColor), With<Button>>,
    mut text_q: Query<(&mut Text, &VolumeText)>,
    windows: Query<&Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut audio: ResMut<AudioManager>
) {
    let window = match windows.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };
    if let Some(cursor) = window.cursor_position() {
        for (interaction, mut style, mut slider, mut color) in slider_q.iter_mut() {
            let pressed = *interaction == Interaction::Pressed;
            let mouse_down = mouse.pressed(MouseButton::Left);

            *color = match *interaction {
                Interaction::Hovered => Color::srgb(0.7, 0.7, 0.9),
                Interaction::None => Color::srgb(0.6, 0.6, 0.8),
                _ => Color::srgb(0.8, 0.8, 1.0),
            }.into();

            if pressed && mouse_down {
                slider.is_dragging = true;
            } else if !mouse_down {
                slider.is_dragging = false;
            }

            if slider.is_dragging || pressed {
                let rel_x = ((cursor.x - (window.width() / 2.0 - SLIDER_WIDTH / 2.0)) / SLIDER_WIDTH).clamp(0.0, 1.0);
                match slider.volume_type {
                    VolumeType::Master => audio.set_master_volume(rel_x),
                    VolumeType::Bgm => audio.set_bgm_volume(rel_x),
                    VolumeType::Sfx => audio.set_sfx_volume(rel_x),
                    VolumeType::Click => audio.set_click_volume(rel_x),
                }
                style.left = Val::Px((SLIDER_WIDTH - 24.0) * rel_x);
                
                for (mut text, vt) in text_q.iter_mut() {
                    if vt.slider_id == slider.slider_id {
                        text.sections[0].value = format!("{}%", (rel_x * 100.0) as i32);
                    }
                }
            }
        }
    }
}

pub fn cleanup_settings_menu(mut commands: Commands, q: Query<Entity, With<SettingsMenu>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_settings_background_fade(time: Res<Time>, mut query: Query<(&mut Sprite, &mut SettingsBackgroundFade)>) {
    for (mut sprite, mut fade) in query.iter_mut() {
        fade.timer.tick(time.delta());
        let duration = fade.timer.duration().as_secs_f32().max(f32::EPSILON);
        let alpha = (fade.timer.elapsed_secs() / duration).clamp(0.0, 1.0);
        let current = sprite.color.to_srgba();
        sprite.color = Color::srgba(current.red, current.green, current.blue, alpha);
    }
}
