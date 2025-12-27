// fourth_minigame.rs - 퀴즈 미니게임 로직
use bevy::prelude::*;
use crate::dialogue::DialogueManager;

#[derive(Resource)]
pub struct QuizGame {
    pub current_round: usize,
    pub total_rounds: usize,
    pub score: usize,
    pub selected_answer: Option<usize>,
    pub is_answered: bool,
    pub is_correct: Option<bool>,
    pub game_started: bool,
    pub is_game_complete: usize,
    pub key_obtained: bool,
    pub show_feedback_timer: Timer,
    pub is_showing_feedback: bool,
    pub encrypted_key: String,
}

impl Default for QuizGame {
    fn default() -> Self {

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
            current_round: 1,
            total_rounds: 4,
            score: 0,
            selected_answer: None,
            is_answered: false,
            is_correct: None,
            game_started: false,
            is_game_complete: 0,
            key_obtained: false,
            show_feedback_timer: Timer::from_seconds(1.5, TimerMode::Once),
            is_showing_feedback: false,
            encrypted_key,
        }
    }
}

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


// 퀴즈 데이터: (질문, [선택지들], 정답 인덱스)
pub const QUIZ_DATA: [(&str, [&str; 4], usize); 4] = [
    (
        "가톨릭대학교 성심교정이 위치한 곳은?",
        ["서울특별시", "경기도 부천시", "경기도 성남시", "경기도 용인시"],
        1
    ),
    (
        "가톨릭대학교의 교훈은?",
        ["진리와 사랑", "믿음과 희망", "정의와 평화", "지혜와 용기"],
        0
    ),
    (
        "가톨릭대학교가 설립된 년도는?",
        ["1845년", "1855년", "1865년", "1875년"],
        1
    ),
    (
        "가톨릭대학교의 상징 동물은?",
        ["독수리", "사자", "비둘기", "말"],
        2
    ),
];

pub fn check_quiz_trigger(
    dm: Res<DialogueManager>,
    state: Res<State<crate::minigames::first_minigame::MinigameState>>,
    mut next_state: ResMut<NextState<crate::minigames::first_minigame::MinigameState>>,
    mut game: ResMut<QuizGame>,
    mut completed: ResMut<crate::minigames::first_minigame::CompletedMinigames>
) {
    if *state.get() != crate::minigames::first_minigame::MinigameState::None 
        || dm.is_typing() { 
        return; 
    }

    let Some(scene_id) = dm.current_scene.as_ref().map(|scene| scene.scene_id.clone()) else {
        return;
    };
    let key = (scene_id, dm.current_line_index);
    if completed.0.contains(&key) {
        return;
    }
    
    let trigger = dm.pending_minigame.as_ref().or_else(|| 
        dm.get_current_line().and_then(|l| l.minigame_trigger.as_ref()).filter(|_| !dm.is_choice_active())
    );
    
    if trigger.map_or(false, |t| t == "quiz") {
        *game = QuizGame::default();
        next_state.set(crate::minigames::first_minigame::MinigameState::Quiz);
        completed.0.insert(key);
    }
}

pub fn update_quiz_feedback_timer(
    time: Res<Time>,
    mut game: ResMut<QuizGame>,
) {
    if game.is_showing_feedback {
        game.show_feedback_timer.tick(time.delta());
        
        if game.show_feedback_timer.just_finished() {
            game.is_showing_feedback = false;
            
            // 다음 라운드로 진행
            if game.current_round < game.total_rounds {
                game.current_round += 1;
                game.selected_answer = None;
                game.is_answered = false;
                game.is_correct = None;
            } else {
                // 게임 완료
                game.is_game_complete = 4;
                if game.score >= 3 {
                    game.key_obtained = true;
                }
            }
        }
    }
}
