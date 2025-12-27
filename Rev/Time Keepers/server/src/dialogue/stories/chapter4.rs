use super::{DialogueLine, DialogueScene};
use crate::dialogue::Choice;

pub fn create_chapter4() -> DialogueScene {
    DialogueScene {
        scene_id: "chapter4".to_string(),
        background: Some("room".to_string()),
        lines: vec![
            DialogueLine {
                speaker: "나레이션".to_string(),
                text: "그날 저녁, 집".to_string(),
                character_emotion: Some("def".to_string()),
                background: Some("room".to_string()),
                choices: None,
                next_line_index: None,
                displayed_character: Some("성헌".to_string()),
                visible_characters: Some(vec!["성헌".to_string()]),
                event_image: None,
                minigame_trigger: None,
                sound_effect: None,
            },
            DialogueLine {
                speaker: "성헌".to_string(),
                text: "하.. 이제 과학실만 남았네. 누구랑 같이 갈까?".to_string(),
                character_emotion: Some("".to_string()),
                background: Some("room".to_string()), 
                choices: Some(vec![  // 선택지 추가
                    Choice {
                        text: "은하와 함께 과학실을 간다.".to_string(),
                        next_scene_id: Some("s_unha".to_string()),
                        next_line_index: None,
                        choice_id: "s_unha".to_string(),
                        affection_changes: None,
                    },
                    Choice {
                        text: "하나와 함께 과학실을 간다.".to_string(),
                        next_scene_id: Some("s_hana".to_string()),
                        next_line_index: None,
                        choice_id: "s_hana".to_string(),
                        affection_changes: None,
                    },
                    Choice {
                        text: "혜진과 함께 과학실을 간다.".to_string(),
                        next_scene_id: Some("s_hjin".to_string()),
                        next_line_index: None,
                        choice_id: "s_hjin".to_string(),
                        affection_changes: None,
                    },
                ]),
                next_line_index: None,
                displayed_character: Some("".to_string()),
                visible_characters: Some(vec!["".to_string()]), 
                event_image: None,
                minigame_trigger: None,
                sound_effect: None,
            },
            // 추가 대화 계속...
        ],
    }
}