use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::*;
use crate::preload::{HanaFaces, UnhaFaces, HjinFaces, BackgroundHandles, EventImageHandles};

#[derive(Resource, Default)]
pub struct CharacterImageManager {
    pub character_entities: HashMap<String, Entity>,
    pub current_speaker: Option<String>,
    pub visible_characters: Vec<String>,
}

#[derive(Resource)]
pub struct BackgroundManager {
    pub current_background: Option<String>,
    pub background_layers: Vec<Entity>,
    pub next_z_index: f32,
}

#[derive(Resource, Default)]
pub struct EventImageManager {
    pub current_event_image: Option<Entity>,
    pub current_event_name: Option<String>,
}

#[derive(Resource, Default)]
pub struct SoundEffectManager {
    pub last_played_sound: Option<String>,
}

impl Default for BackgroundManager {
    fn default() -> Self {
        Self { current_background: None, background_layers: Vec::new(), next_z_index: -15.0 }
    }
}

impl CharacterImageManager {
    pub fn reset(&mut self) {
        self.character_entities.clear();
        self.current_speaker = None;
        self.visible_characters.clear();
    }
}

impl BackgroundManager {
    pub fn reset(&mut self) {
        self.current_background = None;
        self.background_layers.clear();
        self.next_z_index = -15.0;
    }
}

impl EventImageManager {
    pub fn reset(&mut self) {
        self.current_event_image = None;
        self.current_event_name = None;
    }
}

const BASE_RESOLUTION: Vec2 = Vec2::new(1280.0, 720.0);
const CHARACTER_SIZE_BASE: Vec2 = Vec2::new(864.0 * 1.08, 1135.125 * 1.08);
const CHARACTER_Y_BASE: f32 = -275.0;
const SUPPORTED_CHARACTERS: &[&str] = &["하나", "은하", "혜진"];

fn calculate_scale_factor(current_resolution: Vec2) -> f32 {
    (current_resolution.x / BASE_RESOLUTION.x).min(current_resolution.y / BASE_RESOLUTION.y)
}

fn calculate_character_positions(count: usize, width: f32) -> Vec<f32> {
    match count {
        1 => vec![0.0],
        2 => vec![-width * 0.2, width * 0.2], // Increased spacing
        3 => {
            let spacing = width * 0.3; // Increased spacing for consistency
            vec![-spacing, 0.0, spacing]
        },
        4 => {
            let spacing = width * 0.3; // Increased spacing for consistency
            vec![-spacing * 1.5, -spacing * 0.5, spacing * 0.5, spacing * 1.5]
        },
        _ => vec![],
    }
}

fn get_character_texture(name: &str, emotion: Option<&str>, hana: Option<&HanaFaces>, unha: Option<&UnhaFaces>, hjin: Option<&HjinFaces>) -> Option<Handle<Image>> {
    match name {
        "하나" => {
            let faces = hana?;
            Some(match emotion.unwrap_or("def") {
                "buggu" => faces.buggu.clone(),
                "musub" => faces.musub.clone(),
                "musi" => faces.musi.clone(),
                "hwana" => faces.hwana.clone(),
                "hansim" => faces.hansim.clone(),
                _ => faces.def.clone(),
            })
        },
        "은하" => {
            let faces = unha?;
            Some(match emotion.unwrap_or("def") {
                "banjjack" => faces.banjjack.clone(),
                "happy" => faces.happy.clone(),
                "buggu" => faces.buggu.clone(),
                "musub" => faces.musub.clone(),
                "musubbutsinna" => faces.musubbutsinna.clone(),
                "pigim" => faces.pigim.clone(),
                _ => faces.def.clone(),
            })
        },
        "혜진" => {
            let faces = hjin?;
            Some(match emotion.unwrap_or("def") {
                "happy" => faces.happy.clone(),
                "jinji" => faces.jinji.clone(),
                "gomin" => faces.gomin.clone(),
                "ariyon" => faces.ariyon.clone(),
                _ => faces.def.clone(),
            })
        },
        _ => None,
    }
}

fn update_character_focus(commands: &mut Commands, speaker: &str, visible: &[String], manager: &CharacterImageManager) {
    for name in visible {
        if let Some(entity) = manager.character_entities.get(name) {
            let target_brightness = if speaker == "성헌" { 0.5 } else if name == speaker { 1.0 } else if visible.len() >= 2 { 0.5 } else { 1.0 };
            commands.entity(*entity).insert(CharacterFocusAnimation {
                is_focused: speaker != "성헌" && name == speaker,
                target_brightness,
                current_brightness: 1.0,
                speed: 8.0,
            });
        }
    }
}

pub fn reset_character_manager(mut commands: Commands, mut cm: ResMut<CharacterImageManager>, mut bm: ResMut<BackgroundManager>, mut em: ResMut<EventImageManager>, mut sfx: ResMut<SoundEffectManager>, char_q: Query<Entity, With<CharacterImage>>, bg_q: Query<Entity, With<BackgroundLayer>>, event_q: Query<Entity, With<EventImage>>) {
    for entity in char_q.iter().chain(bg_q.iter()).chain(event_q.iter()) {
        commands.entity(entity).despawn();
    }
    cm.reset();
    bm.reset();
    em.reset();
    sfx.last_played_sound = None;
}

pub fn update_background_image(mut commands: Commands, dm: Res<crate::dialogue::DialogueManager>, mut bm: ResMut<BackgroundManager>, windows: Query<&Window>, bg_handles: Option<Res<BackgroundHandles>>) {
    if !dm.is_changed() { return; }
    let Some(line) = dm.get_current_line() else { return };
    let Some(new_bg) = &line.background else { return };
    if bm.current_background.as_ref() == Some(new_bg) { return; }

    let window_size = windows.get_single().map(|w| Vec2::new(w.width(), w.height())).unwrap_or(BASE_RESOLUTION);
    let new_z = bm.next_z_index + 1.0;
    let texture = bg_handles.and_then(|h| h.handles.get(new_bg).cloned()).unwrap_or_default();

    let entity = commands.spawn((
        SpriteBundle {
            texture,
            sprite: Sprite { custom_size: Some(window_size), color: Color::srgba(1.0, 1.0, 1.0, 0.0), ..default() },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, new_z)),
            ..default()
        },
        BackgroundLayer { background_name: new_bg.clone(), z_index: new_z },
        CharacterFadeInAnimation { current_alpha: 0.0, target_alpha: 1.0, speed: 2.0 },
        BackgroundSprite,
        GameUI,
    )).id();

    bm.background_layers.push(entity);
    bm.next_z_index = new_z;
    bm.current_background = Some(new_bg.clone());
}

pub fn update_character_images(mut commands: Commands, dm: Res<crate::dialogue::DialogueManager>, mut cm: ResMut<CharacterImageManager>, mut char_q: Query<(Entity, &mut Visibility, &mut Transform, &CharacterImage)>, windows: Query<&Window>, hana: Option<Res<HanaFaces>>, unha: Option<Res<UnhaFaces>>, hjin: Option<Res<HjinFaces>>) {
    if !dm.is_changed() { return; }
    let Some(line) = dm.get_current_line() else { return };

    let new_visible = line.visible_characters.as_ref().map_or_else(
        || {
            let display = line.displayed_character.as_ref().unwrap_or(&line.speaker);
            if !display.is_empty() && SUPPORTED_CHARACTERS.contains(&display.as_str()) {
                vec![display.clone()]
            } else { vec![] }
        },
        |chars| chars.iter().filter(|n| SUPPORTED_CHARACTERS.contains(&n.as_str())).cloned().collect(),
    );

    if cm.visible_characters != new_visible {
        cm.visible_characters = new_visible.clone();
        
        let (character_size, y_pos, window_width) = windows.get_single().map(|w| {
            let scale = calculate_scale_factor(Vec2::new(w.width(), w.height()));
            (CHARACTER_SIZE_BASE * scale, CHARACTER_Y_BASE * scale, w.width())
        }).unwrap_or((CHARACTER_SIZE_BASE, CHARACTER_Y_BASE, BASE_RESOLUTION.x));

        for (_, mut vis, _, _) in char_q.iter_mut() { *vis = Visibility::Hidden; }
        let positions = calculate_character_positions(new_visible.len(), window_width);

        for (i, name) in new_visible.iter().enumerate() {
            let Some(target_x) = positions.get(i) else { continue };

            if let Some(entity) = cm.character_entities.get(name) {
                if let Ok((eid, mut vis, mut transform, _)) = char_q.get_mut(*entity) {
                    *vis = Visibility::Visible;
                    if (transform.translation.x - target_x).abs() > 5.0 {
                        commands.entity(eid).insert(CharacterSlideAnimation {
                            target_x: *target_x, start_x: transform.translation.x, progress: 0.0, speed: 5.0,
                        });
                    } else {
                        (transform.translation.x, transform.translation.y) = (*target_x, y_pos);
                    }
                }
            } else if let Some(texture) = get_character_texture(name, None, hana.as_deref(), unha.as_deref(), hjin.as_deref()) {
                let entity = commands.spawn((
                    SpriteBundle {
                        texture,
                        sprite: Sprite { custom_size: Some(character_size), color: Color::srgba(1.0, 1.0, 1.0, 0.0), ..default() },
                        transform: Transform::from_translation(Vec3::new(0.0, y_pos, 1.0)),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    CharacterImage { character_name: name.clone() },
                    CharacterSlideAnimation { target_x: *target_x, start_x: 0.0, progress: 0.0, speed: 5.0 },
                    CharacterFadeInAnimation { current_alpha: 0.0, target_alpha: 1.0, speed: 5.0 },
                    GameUI,
                )).id();
                cm.character_entities.insert(name.clone(), entity);
            }
        }
    }

    if cm.current_speaker.as_ref() != Some(&line.speaker) {
        cm.current_speaker = Some(line.speaker.clone());
        for name in &cm.visible_characters {
            if let Some(entity) = cm.character_entities.get(name) {
                commands.entity(*entity).remove::<CharacterFocusAnimation>();
            }
        }
        update_character_focus(&mut commands, &line.speaker, &cm.visible_characters, &cm);
    }
}

pub fn update_character_emotion(dm: Res<crate::dialogue::DialogueManager>, mut q: Query<(&mut Handle<Image>, &CharacterImage)>, hana: Option<Res<HanaFaces>>, unha: Option<Res<UnhaFaces>>, hjin: Option<Res<HjinFaces>>) {
    if !dm.is_changed() { return; }
    let Some(line) = dm.get_current_line() else { return };
    let Some(emotion) = &line.character_emotion else { return };

    let display = line.displayed_character.as_ref().unwrap_or(&line.speaker);
    let Some(texture) = get_character_texture(display, Some(emotion), hana.as_deref(), unha.as_deref(), hjin.as_deref()) else { return };

    for (mut handle, char_img) in q.iter_mut() {
        if char_img.character_name == *display {
            *handle = texture;
            break;
        }
    }
}

pub fn update_event_image(mut commands: Commands, dm: Res<crate::dialogue::DialogueManager>, mut em: ResMut<EventImageManager>, event_q: Query<Entity, With<EventImage>>, windows: Query<&Window>, event_handles: Option<Res<EventImageHandles>>) {
    if !dm.is_changed() { return; }
    let Some(line) = dm.get_current_line() else { return };
    let current_event = line.event_image.as_ref();
    if em.current_event_name.as_ref() == current_event { return; }

    for entity in event_q.iter() { commands.entity(entity).despawn(); }
    (em.current_event_image, em.current_event_name) = (None, None);

    if let Some(event_name) = current_event {
        if let Some(texture) = event_handles.and_then(|h| h.handles.get(event_name).cloned()) {
            let window_size = windows.get_single().map(|w| Vec2::new(w.width(), w.height())).unwrap_or(BASE_RESOLUTION);
            let entity = commands.spawn((
                SpriteBundle {
                    texture,
                    sprite: Sprite { custom_size: Some(window_size), color: Color::srgba(1.0, 1.0, 1.0, 0.0), ..default() },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
                    ..default()
                },
                EventImage,
                GameUI,
                CharacterFadeInAnimation { current_alpha: 0.0, target_alpha: 1.0, speed: 3.0 },
            )).id();
            (em.current_event_image, em.current_event_name) = (Some(entity), Some(event_name.clone()));
        }
    }
}

pub fn play_sound_effect(
    mut commands: Commands,
    dm: Res<crate::dialogue::DialogueManager>,
    mut sfx_manager: ResMut<SoundEffectManager>,
    asset_server: Res<AssetServer>,
    audio_manager: Res<crate::audio_system::AudioManager>,
) {
    if !dm.is_changed() { return; }
    let Some(line) = dm.get_current_line() else { return };
    let current_sound = line.sound_effect.as_ref();

    if sfx_manager.last_played_sound.as_ref() == current_sound { return; }

    if let Some(sound_file) = current_sound {
        let vol = audio_manager.get_effective_volume("sfx");
        commands.spawn((
            AudioBundle {
                source: asset_server.load(format!("sounds/bgms/{}", sound_file)),
                settings: bevy::audio::PlaybackSettings::ONCE.with_volume(bevy::audio::Volume::new(vol))
            },
            crate::components::DialogueSound,
            crate::audio_system::DespawnAfter { remaining: 5.0 },
        ));
        sfx_manager.last_played_sound = Some(sound_file.clone());
    } else {
        sfx_manager.last_played_sound = None;
    }
}

pub fn animate_slides(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Transform, &mut CharacterSlideAnimation)>) {
    for (entity, mut transform, mut anim) in q.iter_mut() {
        anim.progress = (anim.progress + anim.speed * time.delta_seconds()).min(1.0);
        let eased = 1.0 - (1.0 - anim.progress).powi(3);
        transform.translation.x = anim.start_x + (anim.target_x - anim.start_x) * eased;
        if anim.progress >= 1.0 {
            transform.translation.x = anim.target_x;
            commands.entity(entity).remove::<CharacterSlideAnimation>();
        }
    }
}

pub fn animate_fade(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<
        (Entity, &mut Sprite, &mut CharacterFadeInAnimation),
        (
            Without<BackgroundLayer>,
            Without<CharacterImage>,
            Without<EventImage>,
        ),
    >,
) {
    for (entity, mut sprite, mut fade) in q.iter_mut() {
        fade.current_alpha = (fade.current_alpha + fade.speed * time.delta_seconds()).min(fade.target_alpha);
        let c = sprite.color.to_srgba();
        sprite.color = Color::srgba(c.red, c.green, c.blue, fade.current_alpha);
        if fade.current_alpha >= fade.target_alpha {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            commands.entity(entity).remove::<CharacterFadeInAnimation>();
        }
    }
}

pub fn animate_focus(time: Res<Time>, mut q: Query<(&mut Sprite, &mut CharacterFocusAnimation), (With<CharacterImage>, Without<CharacterFadeInAnimation>)>) {
    for (mut sprite, mut focus) in q.iter_mut() {
        let current = sprite.color.to_srgba();
        let avg = (current.red + current.green + current.blue) / 3.0;
        if focus.current_brightness == 1.0 && avg != 1.0 { focus.current_brightness = avg; }

        if (focus.current_brightness - focus.target_brightness).abs() > 0.01 {
            let delta = focus.speed * time.delta_seconds();
            focus.current_brightness = if focus.current_brightness < focus.target_brightness {
                (focus.current_brightness + delta).min(focus.target_brightness)
            } else {
                (focus.current_brightness - delta).max(focus.target_brightness)
            };
            sprite.color = Color::srgba(focus.current_brightness, focus.current_brightness, focus.current_brightness, 1.0);
        }
    }
}

pub fn handle_window_resize(
    mut events: EventReader<bevy::window::WindowResized>,
    mut sprites: (
        Query<&mut Sprite, (With<BackgroundLayer>, Without<CharacterImage>, Without<MainMenu>, Without<crate::components::SettingsMenu>, Without<EventImage>)>,
        Query<&mut Sprite, (With<MainMenu>, Without<CharacterImage>, Without<BackgroundLayer>, Without<crate::components::SettingsMenu>, Without<EventImage>)>,
        Query<&mut Sprite, (With<crate::components::SettingsMenu>, Without<CharacterImage>, Without<BackgroundLayer>, Without<MainMenu>, Without<EventImage>)>,
        Query<&mut Sprite, (With<EventImage>, Without<CharacterImage>, Without<BackgroundLayer>, Without<MainMenu>, Without<crate::components::SettingsMenu>)>
    ),
    mut char_q: Query<(&mut Sprite, &mut Transform, &CharacterImage), With<CharacterImage>>, 
    cm: Res<CharacterImageManager>
) {
    // 이벤트가 너무 많으면 마지막 이벤트만 처리 (과부하 방지)
    let events: Vec<_> = events.read().collect();
    if events.is_empty() { return; }
    
    // 마지막 이벤트만 사용하여 중복 처리 방지
    let event = events.last().unwrap();
    let resolution = Vec2::new(event.width, event.height);
    let scale = calculate_scale_factor(resolution);

    for mut sprite in sprites.0.iter_mut() {
        sprite.custom_size = Some(resolution);
    }
    for mut sprite in sprites.1.iter_mut() {
        sprite.custom_size = Some(resolution);
    }
    for mut sprite in sprites.2.iter_mut() {
        sprite.custom_size = Some(resolution);
    }
    for mut sprite in sprites.3.iter_mut() {
        sprite.custom_size = Some(resolution);
    }

    if !cm.visible_characters.is_empty() {
        let positions = calculate_character_positions(cm.visible_characters.len(), event.width);
        for (mut sprite, mut transform, char_img) in char_q.iter_mut() {
            if let Some(i) = cm.visible_characters.iter().position(|n| n == &char_img.character_name) {
                sprite.custom_size = Some(CHARACTER_SIZE_BASE * scale);
                transform.translation.y = CHARACTER_Y_BASE * scale;
                if let Some(x) = positions.get(i) { 
                    transform.translation.x = *x; 
                }
            }
        }
    }
}

// 호환성을 위한 별칭 함수들 (올바른 타입 시그니처로)
pub fn animate_character_slides(commands: Commands, time: Res<Time>, q: Query<(Entity, &mut Transform, &mut CharacterSlideAnimation)>) { 
    animate_slides(commands, time, q); 
}

pub fn animate_background_fade_in(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Sprite, &mut CharacterFadeInAnimation, &BackgroundLayer), (With<BackgroundLayer>, Without<CharacterImage>)>) {
    for (entity, mut sprite, mut fade, _bg) in q.iter_mut() {
        fade.current_alpha = (fade.current_alpha + fade.speed * time.delta_seconds()).min(fade.target_alpha);
        let c = sprite.color.to_srgba();
        sprite.color = Color::srgba(c.red, c.green, c.blue, fade.current_alpha);
        if fade.current_alpha >= fade.target_alpha {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            commands.entity(entity).remove::<CharacterFadeInAnimation>();
        }
    }
}

pub fn animate_character_fade_in(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Sprite, &mut CharacterFadeInAnimation), With<CharacterImage>>) {
    for (entity, mut sprite, mut fade) in q.iter_mut() {
        fade.current_alpha = (fade.current_alpha + fade.speed * time.delta_seconds()).min(fade.target_alpha);
        let c = sprite.color.to_srgba();
        sprite.color = Color::srgba(c.red, c.green, c.blue, fade.current_alpha);
        if fade.current_alpha >= fade.target_alpha {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            commands.entity(entity).remove::<CharacterFadeInAnimation>();
        }
    }
}

pub fn animate_event_image_fade_in(mut commands: Commands, time: Res<Time>, mut q: Query<(Entity, &mut Sprite, &mut CharacterFadeInAnimation), (With<EventImage>, Without<CharacterImage>, Without<BackgroundLayer>)>) {
    for (entity, mut sprite, mut fade) in q.iter_mut() {
        fade.current_alpha = (fade.current_alpha + fade.speed * time.delta_seconds()).min(fade.target_alpha);
        let c = sprite.color.to_srgba();
        sprite.color = Color::srgba(c.red, c.green, c.blue, fade.current_alpha);
        if fade.current_alpha >= fade.target_alpha {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            commands.entity(entity).remove::<CharacterFadeInAnimation>();
        }
    }
}

pub fn animate_character_focus(time: Res<Time>, q: Query<(&mut Sprite, &mut CharacterFocusAnimation), (With<CharacterImage>, Without<CharacterFadeInAnimation>)>) { 
    animate_focus(time, q); 
}
