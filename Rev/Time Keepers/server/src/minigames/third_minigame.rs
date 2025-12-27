// src/minigames/third_minigame.rs
use bevy::prelude::*;
use crate::dialogue::DialogueManager;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PianoMinigameState { 
    #[default] 
    Inactive, 
    Active
}

#[derive(Resource, Default)]
pub struct CompletedPianoMinigames { 
    pub piano_tiles: bool 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PianoNote {
    Do, DoSharp, Re, ReSharp, Mi, Fa, FaSharp, Sol, SolSharp, La, LaSharp, Si,
}

impl PianoNote {
    pub fn to_korean(&self) -> &'static str {
        match self {
            PianoNote::Do => "도",
            PianoNote::DoSharp => "도#",
            PianoNote::Re => "레", 
            PianoNote::ReSharp => "레#",
            PianoNote::Mi => "미",
            PianoNote::Fa => "파",
            PianoNote::FaSharp => "파#",
            PianoNote::Sol => "솔",
            PianoNote::SolSharp => "솔#",
            PianoNote::La => "라",
            PianoNote::LaSharp => "라#",
            PianoNote::Si => "시",
        }
    }

    pub fn is_black_key(&self) -> bool {
        matches!(self, 
            PianoNote::DoSharp | PianoNote::ReSharp | PianoNote::FaSharp | 
            PianoNote::SolSharp | PianoNote::LaSharp
        )
    }

    pub fn to_audio_filename(&self) -> &'static str {
        match self {
            PianoNote::Do => "C.ogg",
            PianoNote::DoSharp => "Cs.ogg",
            PianoNote::Re => "D.ogg",
            PianoNote::ReSharp => "Ds.ogg",
            PianoNote::Mi => "E.ogg",
            PianoNote::Fa => "F.ogg",
            PianoNote::FaSharp => "Fs.ogg",
            PianoNote::Sol => "G.ogg",
            PianoNote::SolSharp => "Gs.ogg",
            PianoNote::La => "A.ogg",
            PianoNote::LaSharp => "As.ogg",
            PianoNote::Si => "B.ogg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PianoGamePhase {
    NotStarted,           // 게임 시작 전
    WaitingForListen,     // 스피커 버튼 클릭 대기
    PlayingSound,         // 사운드 재생 중 (2.4초)
    AcceptingInput,       // 입력 받는 중
    ShowingSuccess,       // 성공 이미지 표시 (1.5초)
    ShowingFail,          // 실패 이미지 표시 (1.5초)
    ShowingRetry,         // 재시도 오버레이
    ShowingKeyPopup,      // 키 획득 팝업
}

#[derive(Resource)]
pub struct PianoTileGame {
    pub target_sequence: Vec<PianoNote>,
    pub player_sequence: Vec<PianoNote>,
    pub is_completed: usize,
    pub is_failed: bool,
    pub key_obtained: bool,
    pub current_round: u32,
    pub total_rounds: u32,
    
    // 통합된 상태 관리
    pub phase: PianoGamePhase,
    pub phase_timer: Timer,
    
    // 더 이상 사용하지 않는 필드들 (하위 호환성을 위해 남겨둠)
    pub show_sequence: bool,
    pub sequence_index: usize,
    pub sequence_timer: Timer,
    pub game_started: bool,
    pub encrypted_key: String,
}

impl Default for PianoTileGame {
    fn default() -> Self {
        // XOR 단계용 키들 (10단계)
        let keys: [u8; 10] = [0x13, 0xA7, 0x5C, 0x09, 0x2E, 0xB0, 0x4D, 0x81, 0x3F, 0x72];

        let mut data: [u8; 10] = [
            0xBF, 0x58, 0x7A, 0x93, 0xB4,
            0x82, 0xC9, 0x19, 0x14, 0x04
        ];
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
        // 고정 2자리(hex)로 출력
        let encrypted_key = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        Self {
            target_sequence: vec![PianoNote::Re, PianoNote::Fa, PianoNote::Sol],
            player_sequence: Vec::with_capacity(8),
            is_completed: 0,
            is_failed: false,
            key_obtained: false,
            current_round: 1,
            total_rounds: 3,
            
            phase: PianoGamePhase::NotStarted,
            phase_timer: Timer::from_seconds(0.0, TimerMode::Once),
            
            show_sequence: false,
            sequence_index: 0,
            sequence_timer: Timer::from_seconds(0.6, TimerMode::Repeating),
            game_started: false,
            encrypted_key,
        }
    }
}

impl PianoTileGame {
    pub fn can_accept_input(&self) -> bool {
        self.phase == PianoGamePhase::AcceptingInput
    }
    
    pub fn can_click_speaker(&self) -> bool {
        // 스피커 버튼을 여러 번 들을 수 있도록
        // 대기 중이거나 입력 중일 때도 재생을 허용한다.
        matches!(self.phase, PianoGamePhase::WaitingForListen | PianoGamePhase::AcceptingInput)
    }
    
    pub fn set_round_targets(&mut self) {
        self.target_sequence = match self.current_round {
            1 => vec![PianoNote::Re, PianoNote::Fa, PianoNote::Sol],
            2 => vec![PianoNote::Do, PianoNote::Mi, PianoNote::Sol],
            3 => vec![PianoNote::FaSharp, PianoNote::La, PianoNote::Do, PianoNote::ReSharp ,PianoNote::FaSharp,
            PianoNote::ReSharp, PianoNote::Do, PianoNote::SolSharp],
            _ => vec![PianoNote::Re, PianoNote::Fa, PianoNote::Sol],
        };
        self.player_sequence.clear();
    }

    pub fn start_game(&mut self) {
    
        // ✅ 누적 승리 수 초기화
        self.is_completed = 0;
        self.is_failed = false;
        self.key_obtained = false;
    
        self.current_round = 1;
        self.player_sequence.clear();
        self.phase = PianoGamePhase::WaitingForListen;
        self.phase_timer = Timer::from_seconds(0.0, TimerMode::Once);
        self.game_started = true;
    
    }    
    
    pub fn start_playing_sound(&mut self) {
        self.phase = PianoGamePhase::PlayingSound;
        self.phase_timer = Timer::from_seconds(2.4, TimerMode::Once);
    }
    
    pub fn start_accepting_input(&mut self) {
        self.phase = PianoGamePhase::AcceptingInput;
        self.player_sequence.clear();
    }
    
    pub fn show_success(&mut self) {
        self.phase = PianoGamePhase::ShowingSuccess;
        self.phase_timer = Timer::from_seconds(1.5, TimerMode::Once);
    }
    
    pub fn show_fail(&mut self) {
        self.phase = PianoGamePhase::ShowingFail;
        self.phase_timer = Timer::from_seconds(1.5, TimerMode::Once);
    }
    
    pub fn show_retry(&mut self) {
        self.phase = PianoGamePhase::ShowingRetry;
        self.is_failed = true;
    }
    
    pub fn show_key_popup(&mut self) {
        self.phase = PianoGamePhase::ShowingKeyPopup;
    }

    pub fn add_note(&mut self, note: PianoNote) {
        if !self.can_accept_input() {
            return;
        }

        let expected_index = self.player_sequence.len();
        if let Some(&expected) = self.target_sequence.get(expected_index) {
            if expected == note {
                self.player_sequence.push(note);
                
                if self.player_sequence.len() == self.target_sequence.len() {
                    self.show_success();
                }
            } else {
                self.show_fail();
            }
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

pub fn check_piano_minigame_trigger(
    dm: Res<DialogueManager>,
    state: Res<State<PianoMinigameState>>,
    mut next_state: ResMut<NextState<PianoMinigameState>>,
    mut game: ResMut<PianoTileGame>,
    completed: Res<CompletedPianoMinigames>,
) {
    if *state.get() != PianoMinigameState::Inactive || dm.is_typing() || completed.piano_tiles { 
        return; 
    }

    let trigger = dm.pending_minigame.as_ref().or_else(|| {
        dm.get_current_line()
            .and_then(|l| l.minigame_trigger.as_ref())
            .filter(|_| !dm.is_choice_active())
    });

    if trigger.map_or(false, |t| t == "piano_tiles") {
        *game = PianoTileGame::default();
        next_state.set(PianoMinigameState::Active);
    }
}

pub fn reset_piano_on_game_restart(
    mut game: ResMut<PianoTileGame>,
    mut completed: ResMut<CompletedPianoMinigames>,
) {
    *game = PianoTileGame::default();
    completed.piano_tiles = false;
}

pub fn reset_piano_on_game_state_change(
    mut game: ResMut<PianoTileGame>,
    mut completed: ResMut<CompletedPianoMinigames>,
    mut piano_state: ResMut<NextState<PianoMinigameState>>,
) {
    *game = PianoTileGame::default();
    completed.piano_tiles = false;
    piano_state.set(PianoMinigameState::Inactive);
}

// XOR 단계 함수 (10단계)
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
