// src/preload.rs
use bevy::{asset::LoadState, prelude::*};
use crate::game_state::GameState;
use std::collections::HashMap;

// ── 캐릭터 얼굴 핸들 리소스 ──
#[derive(Resource)]
pub struct HanaFaces {
    pub def: Handle<Image>,
    pub buggu: Handle<Image>,
    pub musi: Handle<Image>,
    pub musub: Handle<Image>,
    pub hwana: Handle<Image>,
    pub hansim: Handle<Image>,
}

#[derive(Resource)]
pub struct UnhaFaces {
    pub def: Handle<Image>,
    pub banjjack: Handle<Image>,
    pub happy: Handle<Image>,
    pub buggu: Handle<Image>,
    pub musub: Handle<Image>,
    pub musubbutsinna: Handle<Image>,
    pub pigim: Handle<Image>,
}

#[derive(Resource)]
pub struct HjinFaces {
    pub def: Handle<Image>,
    pub happy: Handle<Image>,
    pub jinji: Handle<Image>,
    pub gomin: Handle<Image>,
    pub ariyon: Handle<Image>,
}

// ── 배경 이미지 핸들 리소스 ──
#[derive(Resource)]
pub struct BackgroundHandles {
    pub handles: HashMap<String, Handle<Image>>,
}

// ── 이벤트 이미지 핸들 리소스 ──
#[derive(Resource)]
pub struct EventImageHandles {
    pub handles: HashMap<String, Handle<Image>>,
}

// ── 프리로드 대상 묶음 ──
#[derive(Resource, Default)]
pub struct PreloadGroup {
    pub images: Vec<Handle<Image>>,
}

pub struct PreloadPlugin;

impl Plugin for PreloadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Preload), start_preload)
           .add_systems(Update, check_preload.run_if(in_state(GameState::Preload)));
    }
}

// ── OnEnter: 에셋 로딩 착수 ──
fn start_preload(mut commands: Commands, assets: Res<AssetServer>) {
    // 캐릭터 이미지들
    let hana_def = assets.load("images/characters/하나_def.png");
    let hana_buggu = assets.load("images/characters/하나_buggu.png");
    let hana_musi = assets.load("images/characters/하나_musi.png");
    let hana_musub = assets.load("images/characters/하나_musub.png");
    let hana_hwana = assets.load("images/characters/하나_hwana.png");
    let hana_hansim = assets.load("images/characters/하나_hansim.png");
    
    let unha_def = assets.load("images/characters/은하_def.png");
    let unha_buggu = assets.load("images/characters/은하_buggu.png");
    let unha_banjjack = assets.load("images/characters/은하_banjjack.png");
    let unha_happy = assets.load("images/characters/은하_happy.png");
    let unha_musub = assets.load("images/characters/은하_musub.png");
    let unha_musubbutsinna = assets.load("images/characters/은하_musubbutsinna.png");
    let unha_pigim = assets.load("images/characters/은하_pigim.png");

    let hjin_def = assets.load("images/characters/혜진_def.png");
    let hjin_happy = assets.load("images/characters/혜진_happy.png");
    let hjin_jinji = assets.load("images/characters/혜진_jinji.png");
    let hjin_gomin = assets.load("images/characters/혜진_gomin.png");
    let hjin_ariyon = assets.load("images/characters/혜진_ariyon.png");

    // 배경 이미지들
    let mut bg_handles = HashMap::new();
    for (name, path) in [("classroom", "classroom"), ("bokdo", "bokdo"), ("library", "library"), ("library_bam", "library_bam"), 
    ("library_bam2", "library_bam2"), ("music_class", "music_class"), ("gu_bokdo", "gu_bokdo"), ("gym", "gym"), 
    ("oksang", "oksang"),("secret_room", "secret_room"), ("room", "room"), ("science_1", "science_1"), ("science_2", "science_2"), ("science_3", "science_3")] 
    {
        bg_handles.insert(name.to_string(), assets.load(&format!("images/backgrounds/{}.png", path)));
    }

    // 이벤트 이미지들
    let mut event_handles = HashMap::new();
    event_handles.insert("secret_door".to_string(), assets.load("images/events/secret_door.png"));
    event_handles.insert("win_minigame".to_string(), assets.load("images/events/win_minigame.png"));
    event_handles.insert("hana1".to_string(), assets.load("images/events/hana1.png"));
    event_handles.insert("Hjin1".to_string(), assets.load("images/events/Hjin1.png"));
    event_handles.insert("unha1".to_string(), assets.load("images/events/unha1.png"));
    event_handles.insert("unha2".to_string(), assets.load("images/events/unha2.png"));
    event_handles.insert("hana2".to_string(), assets.load("images/events/hana2.png"));
    event_handles.insert("Hjin2".to_string(), assets.load("images/events/Hjin2.png"));
    event_handles.insert("Hjin2".to_string(), assets.load("images/events/Hjin2.png"));
    event_handles.insert("pwd".to_string(), assets.load("images/events/pwd.png"));

    // 캐릭터 리소스 등록
    commands.insert_resource(HanaFaces { 
        def: hana_def.clone(), 
        buggu: hana_buggu.clone(), 
        musi: hana_musi.clone(), 
        musub: hana_musub.clone(),
        hwana: hana_hwana.clone(),
        hansim: hana_hansim.clone(),
    });
    commands.insert_resource(UnhaFaces {
        def: unha_def.clone(),
        banjjack: unha_banjjack.clone(),
        happy: unha_happy.clone(),
        buggu: unha_buggu.clone(),
        musub: unha_musub.clone(),
        musubbutsinna: unha_musubbutsinna.clone(),
        pigim: unha_pigim.clone(),
    });
    commands.insert_resource(HjinFaces {
        def: hjin_def.clone(),
        happy: hjin_happy.clone(),
        jinji: hjin_jinji.clone(),
        gomin: hjin_gomin.clone(),
        ariyon: hjin_ariyon.clone(),
    });

    // 배경 및 이벤트 리소스 등록
    commands.insert_resource(BackgroundHandles { handles: bg_handles.clone() });
    commands.insert_resource(EventImageHandles { handles: event_handles.clone() });

    // 전체 프리로드 그룹
    let mut all_handles = vec![
        hana_def, hana_buggu, hana_musi, hana_musub, 
        unha_def, unha_buggu, unha_banjjack, unha_happy, unha_musub, unha_musubbutsinna, unha_pigim, 
        hjin_def, hjin_happy, hjin_jinji, hjin_gomin
        ];
    all_handles.extend(bg_handles.values().cloned());
    all_handles.extend(event_handles.values().cloned());

    commands.insert_resource(PreloadGroup { images: all_handles });
}

// ── Update: 로드 완료/실패 감지 ──
fn check_preload(
    server: Res<AssetServer>,
    group: Option<Res<PreloadGroup>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let Some(group) = group else { return; };

    let mut all_loaded = true;
    let mut any_failed = false;

    for h in &group.images {
        match server.load_state(h.id()) {
            LoadState::Loaded => {}
            LoadState::Failed(_) => { any_failed = true; }
            _ => { all_loaded = false; }
        }
    }

    if any_failed || all_loaded {
        next.set(GameState::MainMenu);
    }
}