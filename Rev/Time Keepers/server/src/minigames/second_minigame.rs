// second_minigame.rs - 별자리 잇기 미니게임 로직
use bevy::prelude::*;
use crate::dialogue::DialogueManager;
use crate::minigames::first_minigame::{MinigameState, CompletedMinigames};

/// 별자리 데이터
#[derive(Debug, Clone)]
pub struct ConstellationData {
    pub name: &'static str,
    pub stars: &'static [(f32, f32)],   // 별들의 위치(게임 영역 내 상대 좌표)
    pub connections: &'static [usize],  // 반드시 눌러야 하는 순서 (별 인덱스)
}

// ───────────── 별 좌표 샘플 ─────────────
pub const CASSIOPEIA_STARS: [(f32, f32); 5] = [
    (100.0, 60.0),
    (210.0, 150.0),
    (330.0, 130.0),
    (450.0, 230.0),
    (550.0, 140.0),
];

pub const URSA_MAJOR_STARS: [(f32, f32); 7] = [
    ( 55.0, 60.0),
    (170.0, 90.0),
    (230.0, 150.0),
    (300.0, 195.0),
    (335.0, 290.0),
    (520.0, 300.0),
    (550.0, 200.0),
];

pub const ORION_STARS: [(f32, f32); 9] = [
    ( 80.0,  80.0),
    (140.0,  30.0),
    (220.0, 110.0),
    (220.0, 180.0),
    (110.0, 240.0),
    (105.0, 300.0),
    (400.0, 270.0),
    (590.0, 285.0),
    (480.0, 200.0),
];

// ───────────── 정답 순서 ─────────────
pub const CASSIOPEIA_SEQUENCE: [usize; 5] = [0, 1, 2, 3, 4];
pub const URSA_MAJOR_SEQUENCE: [usize; 7] = [0, 1, 2, 3, 4, 5, 6];
pub const ORION_SEQUENCE:      [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

// ───────────── 전체 목록 ─────────────
pub const CONSTELLATIONS: [ConstellationData; 3] = [
    ConstellationData { name: "카시오페이아", stars: &CASSIOPEIA_STARS, connections: &CASSIOPEIA_SEQUENCE },
    ConstellationData { name: "큰곰자리",   stars: &URSA_MAJOR_STARS,  connections: &URSA_MAJOR_SEQUENCE },
    ConstellationData { name: "사자자리", stars: &ORION_STARS,       connections: &ORION_SEQUENCE },
];

/// 게임 상태
#[derive(Resource)]
pub struct ConstellationGame {
    pub current_constellation: usize,
    pub stars_connected: Vec<usize>,
    pub connection_lines: Vec<(Vec2, Vec2)>,
    pub completed: Vec<bool>,
    pub is_game_complete: usize,
    pub is_game_failed: bool,
    pub key_obtained: bool,
    pub game_started: bool,
    
    // 새로 추가할 필드들
    pub current_round: usize,  // 현재 라운드 번호 (1부터 시작)
    pub show_round_complete_message: bool,  // 라운드 완료 메시지 표시 여부
    pub show_fail_message: bool,  // 실패 메시지 표시 여부

    pub success_timer: Timer,  // success 이미지 표시 타이머
    pub fail_timer: Timer,
}

impl Default for ConstellationGame {
    fn default() -> Self {
        Self {
            current_constellation: 0,
            stars_connected: Vec::new(),
            connection_lines: Vec::new(),
            completed: vec![false; CONSTELLATIONS.len()],
            is_game_complete: 0,
            is_game_failed: false,
            key_obtained: false,
            game_started: false,
            current_round: 1,
            show_round_complete_message: false,
            show_fail_message: false,
            success_timer: Timer::from_seconds(1.5, TimerMode::Once),
            fail_timer: Timer::from_seconds(1.5, TimerMode::Once),
        }
    }
}

impl ConstellationGame {
    pub fn get_current_constellation(&self) -> &ConstellationData {
        &CONSTELLATIONS[self.current_constellation]
    }

    pub fn hard_reset_for_entry(&mut self) {
        self.current_constellation = 0;
        self.stars_connected.clear();
        self.connection_lines.clear();
        self.completed = vec![false; CONSTELLATIONS.len()];
        self.is_game_complete = 0;
        self.is_game_failed = false;
        self.key_obtained = false;
        self.game_started = false;
        self.current_round = 1;
        self.show_round_complete_message = false;
        self.show_fail_message = false;
        self.success_timer.reset();
        self.fail_timer.reset();
    }

    pub fn connect_star(&mut self, star_id: usize) -> bool {
        if self.stars_connected.contains(&star_id) {
            return false;
        }

        let idx = self.current_constellation;
        let constellation = &CONSTELLATIONS[idx];

        let expected_opt = constellation.connections.get(self.stars_connected.len());
        if let Some(&expected) = expected_opt {
            if star_id == expected {
                if let Some(&prev) = self.stars_connected.last() {
                    let p = Vec2::new(constellation.stars[prev].0, constellation.stars[prev].1);
                    let c = Vec2::new(constellation.stars[star_id].0, constellation.stars[star_id].1);
                    self.connection_lines.push((p, c));
                }

                self.stars_connected.push(star_id);

                if self.stars_connected.len() >= constellation.connections.len() {
                    self.is_game_complete += 1;
                    self.show_round_complete_message = true;
                    self.success_timer.reset();  // 타이머 시작

                    if self.completed.len() != CONSTELLATIONS.len() {
                        self.completed = vec![false; CONSTELLATIONS.len()];
                    }
                    self.completed[self.current_constellation] = true;

                    let all_done = self.completed.iter().all(|&c| c);
                    if all_done {
                        self.key_obtained = true;
                    }
                    // advance_to_next_round()는 타이머 후에 호출됨
                }

                true
            } else {
                self.is_game_failed = true;
                self.show_fail_message = true;
                self.fail_timer.reset();  // 타이머 시작
                false
            }
        } else {
            false
        }
    }

    pub fn advance_to_next_round(&mut self) {
        self.current_constellation = (self.current_constellation + 1) % CONSTELLATIONS.len();
        self.current_round += 1;
        self.stars_connected.clear();
        self.connection_lines.clear();
        self.is_game_failed = false;
        self.game_started = true;
        self.show_round_complete_message = false;
        self.show_fail_message = false;
    }
}


/// 대사 진행 중 미니게임 트리거
pub fn check_constellation_minigame_trigger(
    dm: Res<DialogueManager>,
    state: Res<State<MinigameState>>,
    mut next_state: ResMut<NextState<MinigameState>>,
    mut constellation_game: ResMut<ConstellationGame>,
    mut completed: ResMut<CompletedMinigames>,
) {
    // 이미 다른 미니게임 중이거나 타이핑/선택지 상황이거나, 이 줄을 이미 처리했다면 패스
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

    let trigger = dm
        .pending_minigame
        .as_ref()
        .or_else(|| dm.get_current_line().and_then(|l| l.minigame_trigger.as_ref()))
        .filter(|_| !dm.is_choice_active());

    if trigger.map_or(false, |t| t == "constellation") {
        constellation_game.hard_reset_for_entry(); // ★ 항상 카시오부터/완전 초기화
        next_state.set(MinigameState::Constellation);
        completed.0.insert(key);
    }
}
