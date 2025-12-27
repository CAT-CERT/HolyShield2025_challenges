use bevy::prelude::*;
use crate::dialogue::DialogueManager;
use crate::minigames::first_minigame::{MinigameState, CompletedMinigames};

#[derive(Resource)]
pub struct MathGame {
    pub current_round: usize,
    pub total_rounds: usize,
    pub score: usize,
    pub selected_answer: Option<usize>,
    pub is_answered: bool,
    pub is_correct: Option<bool>,
    pub is_showing_feedback: bool,
    pub show_feedback_timer: Timer,
    pub is_game_complete: usize,  // ✅ bool → usize로 변경
    pub key_obtained: bool,
    pub game_started: bool,
    pub game_timer: Timer,
    pub encrypted_key: String,
}

impl Default for MathGame {
    fn default() -> Self {
        let keys: [u8; 10] = [
            0x3A, 0xC7, 0x5D, 0x82, 0x1F,
            0xE4, 0x6B, 0x90, 0x2C, 0xF3
        ];

        let mut data: [u8; 8] = [
            0x82, 0xEC, 0x13, 0x82, 0x2E, 0x5A, 0x1A, 0x4D
        ];
        // 10단계 XOR 수행 (짝수번 XOR로 실효키 0x00 → 동일 값 유지)
        xor_stage1(&mut data, &keys[0]);
        xor_stage2(&mut data, &keys[1]);
        xor_stage3(&mut data, &keys[2]);
        xor_stage4(&mut data, &keys[3]);
        xor_stage5(&mut data, &keys[4]);
        xor_stage6(&mut data, &keys[5]);
        xor_stage7(&mut data, &keys[6]);
        xor_stage8(&mut data, &keys[7]);
        xor_stage9(&mut data, &keys[8]);
        xor_stage10(&mut data, &keys[9]);

        let encrypted_key = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        Self {
            current_round: 1,
            total_rounds: 4,
            score: 0,
            selected_answer: None,
            is_answered: false,
            is_correct: None,
            is_showing_feedback: false,
            show_feedback_timer: Timer::from_seconds(1.5, TimerMode::Once),
            is_game_complete: 0,
            key_obtained: false,
            game_started: false,
            game_timer: Timer::from_seconds(3.0, TimerMode::Once),
            encrypted_key,
        }
    }
}

// XOR 단계 함수 10개 (다른 미니게임과 동일한 형태)
fn xor_stage1(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage2(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage3(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage4(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage5(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage6(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage7(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage8(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage9(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }
fn xor_stage10(data: &mut [u8], key: &u8) { for b in data.iter_mut() { *b ^= *key; } }

// 수학 문제 데이터
pub const MATH_DATA: [(&str, [&str; 4], usize); 4] = [
    ("12 + 8 = ?", ["19", "18", "22", "20"], 3),
    ("15 × 3 = ?", ["35", "45", "55", "40"], 1),
    ("64 ÷ 8 = ?", ["6", "7", "8", "9"], 2),
    ("25 - 17 = ?", ["7", "9", "10", "6"], 99),
];

pub fn check_math_minigame_trigger(
    dm: Res<DialogueManager>,
    state: Res<State<MinigameState>>,
    mut next_state: ResMut<NextState<MinigameState>>,
    mut math_game: ResMut<MathGame>,
    completed: ResMut<CompletedMinigames>,
) {
    if *state.get() != MinigameState::None || dm.is_typing() {
        return;
    }

    let Some(scene_id) = dm.current_scene.as_ref().map(|scene| scene.scene_id.clone()) else {
        return;
    };
    let key = (scene_id, dm.current_line_index);
    
    if completed.0.contains(&key) && math_game.key_obtained {
        return;
    }

    let trigger = dm
        .pending_minigame
        .as_ref()
        .or_else(|| dm.get_current_line().and_then(|l| l.minigame_trigger.as_ref()))
        .filter(|_| !dm.is_choice_active());

    if trigger.map_or(false, |t| t == "math") {
        *math_game = MathGame::default();
        next_state.set(MinigameState::Math);
    }
}