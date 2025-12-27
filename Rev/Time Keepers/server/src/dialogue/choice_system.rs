use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Choice {
    pub text: String,
    pub next_scene_id: Option<String>, // 다른 씬으로 이동할 경우
    pub next_line_index: Option<usize>, // 같은 씬 내에서 이동할 경우
    pub choice_id: String,
    pub affection_changes: Option<HashMap<String, i32>>,
}

pub struct ChoiceSystem {
    pub is_choice_active: bool,
    pub current_choices: Vec<Choice>,
}

impl Default for ChoiceSystem {
    fn default() -> Self {
        Self {
            is_choice_active: false,
            current_choices: Vec::new(),
        }
    }
}

impl ChoiceSystem {
    pub fn activate_choices(&mut self, choices: Vec<Choice>) {
        self.is_choice_active = true;
        self.current_choices = choices;
    }

    pub fn clear_choices(&mut self) {
        self.is_choice_active = false;
        self.current_choices.clear();
    }

    pub fn get_choice(&self, index: usize) -> Option<&Choice> {
        self.current_choices.get(index)
    }

    pub fn get_all_choices(&self) -> &Vec<Choice> {
        &self.current_choices
    }
}