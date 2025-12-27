use bevy::prelude::*;

// 대화 관련 컴포넌트
#[derive(Component)]
pub struct DialogueBox;

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct SpeakerNameText;

#[derive(Component)]
pub struct BackgroundSprite;

// 선택지 관련 컴포넌트
#[derive(Component)]
pub struct ChoiceMenu;

#[derive(Component, Clone)]
pub struct ChoiceButton {
    pub choice_index: usize,
}

#[derive(Component)]
pub struct ChoiceText;

// 메인 메뉴 관련 컴포넌트
#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct ExitButton;
// 게임 UI 관련 컴포넌트
#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct GameButtonGrid;

// 인게임 설정 관련 컴포넌트
#[derive(Component)]
pub struct InGameSettingsButton;

#[derive(Component)]
pub struct InGameSettingsPopup;

#[derive(Component)]
pub struct PopupBackToGameButton;

#[derive(Component)]
pub struct PopupExitGameButton;

#[derive(Component)]
pub struct EventImage;

// 스킵 관련 컴포넌트
#[derive(Component)]
pub struct SkipButton;

// 미니게임 관련 컴포넌트

#[derive(Component)]
pub struct RoundDisplay;

#[derive(Component)]
pub struct GameCard;

#[derive(Component)]
pub struct ReadyOverlay;

#[derive(Component)]
pub struct RetryOverlay;

#[derive(Component)]
pub struct SuccessImage;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct MinigameUI;

#[derive(Component)]
pub struct FailImage;

#[derive(Component)]
pub struct RetryButton;

#[derive(Component)]
pub struct FailOverlay;

#[derive(Component)]
pub struct NextButton;

#[derive(Component)]
pub struct MinigameButton {
    pub button_id: usize,
}

#[derive(Component)]
pub struct MinigameDisplay;

#[derive(Component)]
pub struct MinigameCloseButton;

#[derive(Component)]
pub struct KeyIcon {
    pub fade_timer: Timer,
    pub is_fading_in: bool,
}

#[derive(Component)]
pub struct KeyPopup;

#[derive(Component)]
pub struct StatusContainer;

#[derive(Component)]
pub struct ButtonContainer;

// 캐릭터 시스템 관련 컴포넌트
#[derive(Component)]
pub struct CharacterImage { 
    pub character_name: String 
}

#[derive(Component)]
pub struct BackgroundLayer { 
    pub background_name: String, 
    pub z_index: f32 
}

#[derive(Component)]
pub struct CharacterSlideAnimation { 
    pub target_x: f32, 
    pub start_x: f32, 
    pub progress: f32, 
    pub speed: f32 
}

#[derive(Component)]
pub struct CharacterFocusAnimation { 
    pub is_focused: bool, 
    pub target_brightness: f32, 
    pub current_brightness: f32, 
    pub speed: f32 
}

#[derive(Component)]
pub struct CharacterFadeInAnimation { 
    pub current_alpha: f32, 
    pub target_alpha: f32, 
    pub speed: f32 
}

#[derive(Component)]
pub struct MenuBackgroundFade {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SettingsBackgroundFade {
    pub timer: Timer,
}

// 오디오 관련 컴포넌트들
#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component)]
pub struct ClickSound;

#[derive(Component)]
pub struct DialogueSound;

#[derive(Component)]
pub struct ChoiceSound;

// 설정 메뉴 관련 컴포넌트
#[derive(Component)]
pub struct SettingsMenu;

#[derive(Component)]
pub struct VolumeSlider { pub volume_type: VolumeType, pub slider_id: u32, pub is_dragging: bool }

#[derive(Component)]
pub struct VolumeText { pub volume_type: VolumeType, pub slider_id: u32 }

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct SliderTrack {}

#[derive(Clone, Copy, PartialEq)]
pub enum VolumeType { 
    Master, 
    Bgm, 
    Sfx, 
    Click 
}

// 별자리 미니게임 관련 컴포넌트 (기존 미니게임 컴포넌트 섹션에 추가)
// components.rs에 추가할 Math 미니게임 관련 컴포넌트들
#[derive(Component)]
pub struct ConstellationRoundDisplay;

#[derive(Component)]
pub struct ConstellationRetryButton;

#[derive(Component)]
pub struct MathUI;

#[derive(Component)]
pub struct MathDisplay;

#[derive(Component)]
pub struct MathStartButton;

#[derive(Component)]
pub struct MathCloseButton;

#[derive(Component)]
pub struct MathAnswerButton { 
    pub answer_index: usize 
}

#[derive(Component)]
pub struct MathQuestionText;

#[derive(Component)]
pub struct MathAnswerArea;

// 퀴즈 미니게임 관련 컴포넌트
#[derive(Component)]
pub struct QuizAnswerButton { pub answer_index: usize }

#[derive(Component)]
pub struct QuizStartButton;

#[derive(Component)]
pub struct QuizCloseButton;

#[derive(Component)]
pub struct QuizDisplay;

#[derive(Component)]
pub struct QuizQuestion;

// second_ui.rs
#[derive(Component)]
pub struct ConstellationLine;

#[derive(Component)]
pub struct ConstellationUI;

#[derive(Component)]
pub struct ConstellationStartButton;

#[derive(Component)]
pub struct ConstellationCloseButton;

#[derive(Component)]
pub struct ConstellationGameArea;

#[derive(Component)]
pub struct ConstellationNameDisplay;

// third_ui.rs
#[derive(Component)]
pub struct PianoRetryButton;

#[derive(Component)]
pub struct PianoMinigameUI;

#[derive(Component)]
pub struct PianoStartButton;

#[derive(Component)]
pub struct PianoCloseButton;

#[derive(Component)]
pub struct PianoListenButton;

#[derive(Component)]
pub struct PianoDisplay;

#[derive(Component)]
pub struct PianoKeyboard;

// fourth_ui.rs
#[derive(Component)]
pub struct AnswerContainer;

#[derive(Component)]
pub struct QuizGameArea;

#[derive(Component)]
pub struct QuestionContainer;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct QuizAnswerText;

// fifth_ui.rs
#[derive(Component)]
pub struct MathAnswerText;

#[derive(Component)]
pub struct MathGameArea;

#[derive(Component)]
pub struct MathQuestionContainer;

