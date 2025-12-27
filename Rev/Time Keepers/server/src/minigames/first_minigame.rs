// minigame.rs - 극도로 압축된 미니게임 로직
use bevy::prelude::*;
use std::collections::VecDeque;
use crate::dialogue::DialogueManager;
use rand::Rng;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MinigameState { 
    #[default] None, 
    ButtonSequence,
    Constellation,
    Math,
    Quiz,
    KeyCheck,
}

#[derive(Resource, Default)]
pub struct CompletedMinigames(pub std::collections::HashSet<(String, usize)>);

const ROUND_DATA: [(usize, usize); 4] = [(0, 2), (2, 3), (5, 4), (9, 15)]; // (start, length)

#[derive(Resource)]
pub struct ButtonSequenceGame {
    pub sequence: VecDeque<usize>, pub player_input: Vec<usize>, pub current_showing_index: usize,
    pub is_showing_sequence: bool, pub is_accepting_input: bool, pub sequence_timer: Timer,
    pub gap_timer: Timer, pub end_delay_timer: Timer, pub round_delay_timer: Timer,
    pub is_in_end_delay: bool, pub is_in_gap: bool, pub is_in_round_delay: bool,
    pub current_round: usize, pub total_rounds: usize, pub is_game_complete: usize, // boolean에서 usize로 변경
    pub is_game_failed: bool, pub key_obtained: bool, pub game_started: bool,
    pub is_showing_success: bool,
    pub success_timer: Timer,
    pub is_showing_fail: bool,
    pub fail_timer: Timer,
    pub encrypted_key: String,
}

impl Default for ButtonSequenceGame {
    fn default() -> Self {
        let mut sequence = VecDeque::with_capacity(24);
        let mut rng = rand::thread_rng();
        for &(_, len) in &ROUND_DATA {
            for _ in 0..len {
                sequence.push_back(rng.gen_range(0..4));
            }
        }

        // XOR 단계용 키들
        let keys: [u8; 10] = [0x23, 0x57, 0x91, 0x11, 0xDE, 0xB4, 0x6A, 0xF0, 0x3C, 0x72];

        let mut data: [u8; 10] = [0xC6, 0x97, 0x71, 0x14, 0x9A, 0x75, 0xAA, 0x7F, 0xAD, 0x23];

        // 10단계 XOR 수행
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

        // hex 문자열로 변환
        let encrypted_key = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        Self {
            sequence,
            player_input: Vec::with_capacity(10),
            current_showing_index: 0,
            is_showing_sequence: false,
            is_accepting_input: false,
            sequence_timer: Timer::from_seconds(0.4, TimerMode::Repeating),
            gap_timer: Timer::from_seconds(0.1, TimerMode::Once),
            end_delay_timer: Timer::from_seconds(0.4, TimerMode::Once),
            round_delay_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_in_end_delay: false,
            is_in_gap: false,
            is_in_round_delay: false,
            current_round: 1,
            total_rounds: 4,
            is_game_complete: 0,
            is_game_failed: false,
            key_obtained: false,
            game_started: false,
            is_showing_success: false,
            success_timer: Timer::from_seconds(1.0, TimerMode::Once),
            is_showing_fail: false,
            fail_timer: Timer::from_seconds(1.5, TimerMode::Once),
            encrypted_key,
        }
    }
}

// XOR 단계 함수 10개
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

impl ButtonSequenceGame {}

pub fn reset_minigame_on_game_state_change(
    mut completed: ResMut<CompletedMinigames>, mut state: ResMut<NextState<MinigameState>>, mut dm: ResMut<DialogueManager>
) { completed.0.clear(); state.set(MinigameState::None); dm.pending_minigame = None; }

pub fn check_minigame_trigger(
    dm: Res<DialogueManager>,
    state: Res<State<MinigameState>>,
    mut next_state: ResMut<NextState<MinigameState>>,
    mut game: ResMut<ButtonSequenceGame>,
    mut completed: ResMut<CompletedMinigames>
) {
    if *state.get() != MinigameState::None || dm.is_typing() {
        return;
    }

    let Some(scene_id) = dm.current_scene.as_ref().map(|scene| scene.scene_id.clone()) else {
        return;
    };
    let key = (scene_id, dm.current_line_index);
    if completed.0.contains(&key) {
        return;
    }

    let trigger = dm.pending_minigame.as_ref().or_else(|| {
        dm.get_current_line()
            .and_then(|l| l.minigame_trigger.as_ref())
            .filter(|_| !dm.is_choice_active())
    });

    if trigger.map_or(false, |t| t == "button_sequence") {
        *game = ButtonSequenceGame::default();
        next_state.set(MinigameState::ButtonSequence);
        completed.0.insert(key);
    }
}
