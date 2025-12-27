use bevy::prelude::*;

mod minigames;
mod components;
mod dialogue;
mod game_state;
mod menu;
pub mod audio_system;
pub mod settings;
pub mod character_system;
mod ui;
mod preload;

use dialogue::DialogueManager;
use game_state::GameState;
use audio_system::*;
use character_system::*;
use ui::*;
use minigames::first_minigame::{MinigameState, ButtonSequenceGame};
use minigames::second_minigame::ConstellationGame;
use minigames::third_minigame::{PianoMinigameState, PianoTileGame};
use minigames::fourth_minigame::QuizGame;
use minigames::fifth_minigame::MathGame;
use crate::preload::PreloadPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { 
                title: "시간을 여는 동아리".into(),
                resolution: (1280., 720.).into(),
                resizable: false,
                decorations: true, 
                ..default() 
            }), 
            ..default() 
        }))
        
        .add_plugins(PreloadPlugin)
        
        // 상태 초기화
        .init_state::<GameState>()
        .init_state::<MinigameState>()
        .init_state::<PianoMinigameState>()
        
        // 리소스 초기화
        .init_resource::<DialogueManager>()
        .init_resource::<AudioManager>()
        .init_resource::<CharacterImageManager>()
        .init_resource::<BackgroundManager>()
        .init_resource::<EventImageManager>()
        .init_resource::<SoundEffectManager>()
        .init_resource::<ButtonSequenceGame>()
        .init_resource::<ConstellationGame>()
        .init_resource::<PianoTileGame>()
        .init_resource::<QuizGame>()
        .init_resource::<MathGame>()
        .init_resource::<minigames::first_minigame::CompletedMinigames>()
        .init_resource::<minigames::third_minigame::CompletedPianoMinigames>()
        .init_resource::<crate::dialogue::VisitedLocations>()
        .init_resource::<minigames::check_keys::KeyCheckState>()

        // 시작 시스템
        .add_systems(Startup, (setup_camera, setup_audio_system))
        
        // 미니게임 트리거 체크
        .add_systems(Update, (
            minigames::first_minigame::check_minigame_trigger,
            minigames::second_minigame::check_constellation_minigame_trigger,
            minigames::third_minigame::check_piano_minigame_trigger,
            minigames::fourth_minigame::check_quiz_trigger,
            minigames::fifth_minigame::check_math_minigame_trigger,
            minigames::check_keys::check_key_trigger,
        ).run_if(in_state(GameState::Playing)))
        
        // 첫 번째 미니게임 (버튼 시퀀스)
        .add_systems(OnEnter(MinigameState::ButtonSequence), minigames::uis::first_ui::setup_button_sequence_minigame)
        .add_systems(OnExit(MinigameState::ButtonSequence), minigames::uis::first_ui::cleanup_minigame)
        .add_systems(Update,
            (minigames::uis::first_ui::handle_minigame_start_button,
                minigames::uis::first_ui::handle_minigame_close_button,
                minigames::uis::first_ui::handle_minigame_buttons,
                minigames::uis::first_ui::update_sequence_display,
                minigames::uis::first_ui::update_minigame_display,
                minigames::uis::first_ui::update_key_icon_fade,
                minigames::uis::first_ui::handle_key_popup,
                minigames::uis::first_ui::show_success_image,
                minigames::uis::first_ui::handle_next_button,
                minigames::uis::first_ui::show_fail_image,
                minigames::uis::first_ui::show_retry_overlay,
                minigames::uis::first_ui::handle_retry_button).run_if(in_state(MinigameState::ButtonSequence)))

        // 두 번째 미니게임 (별자리)
        .add_systems(OnEnter(MinigameState::Constellation), minigames::uis::second_ui::setup_constellation_minigame)
        .add_systems(OnExit(MinigameState::Constellation), minigames::uis::second_ui::cleanup_constellation_minigame)
        .add_systems(Update,
            (minigames::uis::second_ui::handle_constellation_start_button,  
            minigames::uis::second_ui::handle_constellation_stars,
            minigames::uis::second_ui::update_constellation_stars,
            minigames::uis::second_ui::update_constellation_lines,
            minigames::uis::second_ui::update_constellation_display,
            minigames::uis::second_ui::show_constellation_success_image,
            minigames::uis::second_ui::show_constellation_fail_image,
            minigames::uis::second_ui::show_constellation_retry_overlay,
            minigames::uis::second_ui::handle_constellation_retry_button,
            minigames::uis::second_ui::handle_constellation_next_button,
            minigames::uis::second_ui::update_constellation_timers,
            minigames::uis::second_ui::handle_constellation_key_popup).run_if(in_state(MinigameState::Constellation)))
        
        // 세 번째 미니게임 (피아노)
        .add_systems(OnEnter(PianoMinigameState::Active), (
            minigames::uis::third_ui::setup_piano_minigame,
        minigames::third_minigame::reset_piano_on_game_restart).chain())
        .add_systems(OnExit(PianoMinigameState::Active), minigames::uis::third_ui::cleanup_piano_minigame)
        .add_systems(Update, (
            minigames::uis::third_ui::handle_piano_start_button,
            minigames::uis::third_ui::handle_piano_keys,
            minigames::uis::third_ui::update_piano_sequence_display,
            minigames::uis::third_ui::show_piano_success_image,
            minigames::uis::third_ui::show_piano_fail_image,
            minigames::uis::third_ui::show_piano_retry_overlay,
            minigames::uis::third_ui::handle_piano_retry_button,
            minigames::uis::third_ui::update_piano_minigame_display,
            minigames::uis::third_ui::handle_piano_key_popup,
            minigames::uis::third_ui::handle_piano_listen_button,
            minigames::uis::third_ui::handle_piano_close_button,
            minigames::uis::third_ui::update_speaker_image,
            minigames::uis::third_ui::update_key_icon_fade,
            minigames::uis::third_ui::update_piano_bgm_control  
        ).run_if(in_state(PianoMinigameState::Active)))

        // 네 번째 미니게임 (퀴즈)
        .add_systems(OnEnter(MinigameState::Quiz), minigames::uis::fourth_ui::setup_quiz_minigame)
        .add_systems(OnExit(MinigameState::Quiz), minigames::uis::fourth_ui::cleanup_quiz_minigame)
        .add_systems(Update,
            (minigames::uis::fourth_ui::handle_quiz_start_button,
                minigames::uis::fourth_ui::handle_quiz_close_button,
                minigames::uis::fourth_ui::handle_quiz_answer_buttons,
                minigames::uis::fourth_ui::update_quiz_display,
                minigames::uis::fourth_ui::handle_quiz_key_popup,
                minigames::uis::fourth_ui::show_quiz_success_image,
                minigames::uis::fourth_ui::show_quiz_fail_image,
                minigames::fourth_minigame::update_quiz_feedback_timer)
                .run_if(in_state(MinigameState::Quiz)))

        // 다섯 번째 미니게임 (수학)
        .add_systems(OnEnter(MinigameState::Math), minigames::uis::fifth_ui::setup_math_minigame)
        .add_systems(OnExit(MinigameState::Math), minigames::uis::fifth_ui::cleanup_math_minigame)
        .add_systems(Update,
            (minigames::uis::fifth_ui::handle_math_start_button,
                minigames::uis::fifth_ui::handle_math_close_button,
                minigames::uis::fifth_ui::handle_math_answer_buttons,
                minigames::uis::fifth_ui::update_math_display,
                minigames::uis::fifth_ui::handle_math_key_popup,
                minigames::uis::fifth_ui::show_math_success_image,
                minigames::uis::fifth_ui::show_math_fail_image,
                crate::minigames::uis::fifth_ui::update_math_start_button_visibility,
                minigames::uis::fifth_ui::update_key_icon_fade).run_if(in_state(MinigameState::Math)))

        // 키 검증 미니게임
        .add_systems(OnEnter(MinigameState::KeyCheck), minigames::check_keys::setup_key_check_ui)
        .add_systems(OnExit(MinigameState::KeyCheck), minigames::check_keys::cleanup_key_check)
        .add_systems(Update, (
            minigames::check_keys::handle_key_input,
            minigames::check_keys::handle_key_check_button,
            minigames::check_keys::show_result_image,
            minigames::check_keys::update_cursor_blink,
            minigames::check_keys::update_result_display,
        ).run_if(in_state(MinigameState::KeyCheck)))

        // 기본 시스템들
        .add_systems(Update, (update_audio_volumes, tick_despawn_after, handle_window_resize))
        .add_systems(Update, menu::update_main_menu_background_fade.run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, settings::update_settings_background_fade.run_if(in_state(GameState::Settings)))
        .add_systems(Update, character_system::animate_fade)
        .add_systems(OnEnter(GameState::MainMenu), (menu::setup_main_menu, play_background_music))
        .add_systems(OnExit(GameState::MainMenu), menu::cleanup_main_menu)
        .add_systems(Update, menu::handle_menu_buttons)
        .add_systems(OnEnter(GameState::Settings), settings::setup_settings_menu)
        .add_systems(OnExit(GameState::Settings), settings::cleanup_settings_menu)
        .add_systems(Update, (settings::handle_settings_input, settings::handle_volume_sliders).run_if(in_state(GameState::Settings)))
        .add_systems(OnEnter(GameState::Playing), (setup_game_ui, reset_character_manager, menu::start_rain_on_dialogue,
            minigames::first_minigame::reset_minigame_on_game_state_change,
            minigames::third_minigame::reset_piano_on_game_state_change))
        .add_systems(OnExit(GameState::Playing), menu::stop_rain_ambience)
        .add_systems(Update, menu::stop_rain_on_gym.run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_game_ui)
        .add_systems(Update, (update_typing_effect, start_initial_typing).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (update_event_image, animate_event_image_fade_in, play_sound_effect).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (handle_input, handle_choice_keyboard, handle_skip_button).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (update_dialogue_ui, manage_choice_ui, handle_choice_buttons, handle_dialogue_progression).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (settings::handle_ingame_settings_button, settings::handle_ingame_popup_buttons, settings::handle_volume_sliders).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (update_background_image, animate_background_fade_in).run_if(in_state(GameState::Playing)))
        .add_systems(Update, (update_character_images, update_character_emotion, animate_character_slides, animate_character_fade_in, animate_character_focus).run_if(in_state(GameState::Playing)))
        
        .run();
}

fn setup_camera(mut commands: Commands) { 
    commands.spawn(Camera2dBundle::default()); 
}
