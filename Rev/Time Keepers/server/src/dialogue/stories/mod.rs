// src/dialogue/stories/mod.rs

pub mod intro_scene;
pub mod chapter1;
pub mod chapter2;
pub mod chapter3;
pub mod chapter4;
pub mod s_hana;
pub mod s_unha;
pub mod s_hjin;
pub mod end_0;
pub mod end_1;
pub mod end_2;

// Choice는 부모 모듈에서 import
use super::Choice;  // 또는 use crate::dialogue::Choice;

#[derive(Clone, Debug)]
pub struct DialogueLine {
    pub speaker: String,
    pub text: String,
    pub character_emotion: Option<String>,
    pub displayed_character: Option<String>,
    pub visible_characters: Option<Vec<String>>,
    pub background: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub next_line_index: Option<usize>,
    pub event_image: Option<String>,
    pub minigame_trigger: Option<String>,
    pub sound_effect: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DialogueScene {
    pub scene_id: String,
    pub background: Option<String>,
    pub lines: Vec<DialogueLine>,
}

pub fn load_all_stories() -> Vec<DialogueScene> {
    vec![
        intro_scene::create_intro_scene(),
        chapter1::create_chapter1(),
        chapter2::create_chapter2(),
        chapter3::create_chapter3(),
        chapter4::create_chapter4(),
        s_hana::create_s_hana(),
        s_unha::create_s_unha(),
        s_hjin::create_s_hjin(),
        end_0::create_end_0(),
        end_1::create_end_1(),
        end_2::create_end_2(),
    ]
}