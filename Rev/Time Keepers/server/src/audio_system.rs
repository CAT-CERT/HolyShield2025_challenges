use bevy::prelude::*;
use bevy::audio::Volume;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use crate::components::{BackgroundMusic, ClickSound, DialogueSound, ChoiceSound};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub click_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self { Self { master_volume: 1.0, bgm_volume: 0.8, sfx_volume: 0.8, click_volume: 0.5 } }
}

#[derive(Resource)]
pub struct AudioManager {
    pub settings: AudioSettings,
    pub settings_path: PathBuf,
    pub current_bgm: Option<Entity>,
}

impl Default for AudioManager {
    fn default() -> Self {
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        path.push("settings");
        let _ = fs::create_dir_all(&path);
        path.push("audio_settings.json");

        let mut am = Self {
            settings: AudioSettings::default(),
            settings_path: path,
            current_bgm: None
        };
        
        let _ = am.load_settings();
        am
    }
}

impl AudioManager {
    pub fn set_master_volume(&mut self, v: f32) {self.settings.master_volume = v.clamp(0.0, 1.0); }
    pub fn set_bgm_volume(&mut self, v: f32) { self.settings.bgm_volume = v.clamp(0.0, 1.0); }
    pub fn set_sfx_volume(&mut self, v: f32) { self.settings.sfx_volume = v.clamp(0.0, 1.0); }
    pub fn set_click_volume(&mut self, v: f32) { self.settings.click_volume = v.clamp(0.0, 1.0); }
    
    pub fn get_effective_volume(&self, t: &str) -> f32 {
        (match t {
            "bgm" => self.settings.bgm_volume,
            "sfx" => self.settings.sfx_volume,
            "click" => self.settings.click_volume,
            _ => 1.0 
        }) * self.settings.master_volume
    }

    pub fn save_settings(&self) -> Result<(), String> {
        fs::write(&self.settings_path, serde_json::to_string_pretty(&self.settings)
        .map_err(|e| format!("직렬화 오류: {}", e))?)
        .map_err(|e| format!("파일 쓰기 오류: {}", e))?;
        Ok(())
    }

    pub fn load_settings(&mut self) -> Result<(), String> {
        if !self.settings_path.exists() { return Ok(()); }
        self.settings = serde_json::from_str(&fs::read_to_string(&self.settings_path)
        .map_err(|e| format!("파일 읽기 오류: {}", e))?)
        .map_err(|e| format!("역직렬화 오류: {}", e))?;
        Ok(())
    }
}

#[derive(Component)]
pub struct DespawnAfter { pub remaining: f32 }

pub fn tick_despawn_after(time: Res<Time>, mut commands: Commands, mut q: Query<(Entity, &mut DespawnAfter)>) {
    for (e, mut d) in q.iter_mut() {
        d.remaining -= time.delta_seconds();
        if d.remaining <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

pub fn play_click_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, am: &AudioManager) {
    let vol = am.get_effective_volume("click");
    commands.spawn((AudioBundle {
        source: asset_server.load("sounds/click.ogg"),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new(vol))
    }, ClickSound, DespawnAfter { remaining: 3.0 }));
}

pub fn play_dialogue_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, am: &AudioManager) {
    let vol = am.get_effective_volume("sfx");
    commands.spawn((AudioBundle {
        source: asset_server.load("sounds/click.ogg"),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new(vol))
    }, DialogueSound, DespawnAfter { remaining: 3.0 }));
}

pub fn play_choice_sound(commands: &mut Commands, asset_server: &Res<AssetServer>, am: &AudioManager) {
    let vol = am.get_effective_volume("sfx");
    commands.spawn((AudioBundle {
        source: asset_server.load("sounds/click.ogg"),
        settings: PlaybackSettings::ONCE.with_volume(Volume::new(vol))
    }, ChoiceSound, DespawnAfter { remaining: 3.0 }));
}

pub fn play_background_music(mut commands: Commands, asset_server: Res<AssetServer>, mut am: ResMut<AudioManager>, bgm_q: Query<Entity, With<BackgroundMusic>>) {
    bgm_q.iter().for_each(|e| commands.entity(e).despawn());
    let vol = am.get_effective_volume("bgm");
    am.current_bgm = Some(commands.spawn((AudioBundle {
        source: asset_server.load("sounds/bgm_main.ogg"),
        settings: PlaybackSettings::LOOP.with_volume(Volume::new(vol))
    }, BackgroundMusic)).id());
}

pub fn play_chapter_bgm(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    am: &mut ResMut<AudioManager>,
    bgm_q: &Query<Entity, With<BackgroundMusic>>,
    bgm_file: &str
) {
    bgm_q.iter().for_each(|e| commands.entity(e).despawn());
    let vol = am.get_effective_volume("bgm");
    am.current_bgm = Some(commands.spawn((AudioBundle {
        source: asset_server.load(format!("sounds/bgms/{}", bgm_file)),
        settings: PlaybackSettings::LOOP.with_volume(Volume::new(vol))
    }, BackgroundMusic)).id());
}

pub fn update_audio_volumes(am: Res<AudioManager>, mut audio_q: Query<(&mut AudioSink, Option<&BackgroundMusic>, Option<&ClickSound>)>) {
    if am.is_changed() {
        for (audio_sink, bgm, click) in audio_q.iter_mut() {
            audio_sink.set_volume(am.get_effective_volume(
                if bgm.is_some() { "bgm" }
                else if click.is_some() { "click" }
                else { "sfx" }));
        }
    }
}

pub fn setup_audio_system() { }
pub fn cleanup_audio(
    mut commands: Commands,
    audio_q: Query<Entity, Or<(With<BackgroundMusic>, With<ClickSound>, With<DialogueSound>, With<ChoiceSound>)>>)
    { audio_q.iter().for_each(|e| commands.entity(e).despawn());}