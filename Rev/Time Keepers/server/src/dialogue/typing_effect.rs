use bevy::prelude::*;

#[derive(Component)]
pub struct TypingEffect {
    pub is_typing: bool,
    pub typing_timer: Timer,
    pub current_char_index: usize,
    pub full_text: String,
    pub displayed_text: String,
}

impl Default for TypingEffect {
    fn default() -> Self {
        Self {
            is_typing: false,
            typing_timer: Timer::from_seconds(0.03, TimerMode::Repeating),
            current_char_index: 0,
            full_text: String::new(),
            displayed_text: String::new(),
        }
    }
}

impl TypingEffect {
    pub fn start_typing(&mut self, text: String) {
        self.full_text = text;
        self.displayed_text = String::new();
        self.current_char_index = 0;
        self.is_typing = true;
        self.typing_timer.reset();
    }

    pub fn complete_typing(&mut self) {
        if self.is_typing {
            self.displayed_text = self.full_text.clone();
            self.current_char_index = self.full_text.chars().count();
            self.is_typing = false;
        }
    }

    pub fn update_typing(&mut self, delta_time: std::time::Duration) {
        if !self.is_typing {
            return;
        }

        self.typing_timer.tick(delta_time);
        
        if self.typing_timer.just_finished() {
            let chars: Vec<char> = self.full_text.chars().collect();
            
            if self.current_char_index < chars.len() {
                self.current_char_index += 1;
                self.displayed_text = chars[..self.current_char_index].iter().collect();
            } else {
                self.is_typing = false;
            }
        }
    }

    pub fn get_displayed_text(&self) -> &String {
        if self.is_typing {
            &self.displayed_text
        } else {
            &self.full_text
        }
    }
}