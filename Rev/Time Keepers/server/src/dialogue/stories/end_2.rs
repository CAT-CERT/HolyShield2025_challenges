use super::{DialogueLine, DialogueScene};

pub fn create_end_2() -> DialogueScene {
    DialogueScene {
        scene_id: "end_2".to_string(),
        background: Some("secert_room".to_string()),
        lines: vec![
            DialogueLine {
                speaker: "성헌".to_string(),
                text: "음? 이게 아니라는데? 다시 해보자.".to_string(),
                character_emotion: Some("".to_string()),
                background: Some("secret_room".to_string()),
                choices: None,
                next_line_index: None,
                sound_effect: None,
                displayed_character: Some(" ".to_string()),
                visible_characters: Some(vec!["은하".to_string(), "하나".to_string(), "혜진".to_string()]), 
                event_image: None,
                minigame_trigger: Some("check_key".to_string()),
            },
        ],
    }
}