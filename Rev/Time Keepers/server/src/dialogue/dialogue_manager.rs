// src/dialogue/dialogue_manager.rs
use bevy::prelude::*;
use std::collections::HashMap;
use super::{TypingEffect, ChoiceSystem, DialogueScene, DialogueLine, Choice};
use super::stories;
use crate::dialogue::VisitedLocations;

#[derive(Resource)]
pub struct DialogueManager {
    pub current_scene: Option<DialogueScene>,
    pub current_line_index: usize,
    pub scenes: Vec<DialogueScene>,
    pub character_affection: HashMap<String, i32>,
    pub current_background: Option<String>,
    pub pending_minigame: Option<String>,
    
    // 분리된 컴포넌트들
    pub typing_effect: TypingEffect,
    pub choice_system: ChoiceSystem,
}

impl Default for DialogueManager {
    fn default() -> Self {
        // 캐릭터 호감도 초기화 (모두 0부터 시작)
        let mut character_affection = HashMap::new();
        character_affection.insert("혜진".to_string(), 0);
        character_affection.insert("은하".to_string(), 0);
        character_affection.insert("하나".to_string(), 0);
        character_affection.insert("성헌".to_string(), 0);

        let all_scenes = stories::load_all_stories();
        let default_scene = all_scenes.first().cloned();

        Self {
            current_scene: default_scene,
            current_line_index: 0,
            scenes: all_scenes,
            character_affection,
            current_background: Some("classroom".to_string()),
            pending_minigame: None,
            typing_effect: TypingEffect::default(),
            choice_system: ChoiceSystem::default(),
        }
    }
}

impl DialogueManager {
    pub fn get_current_line(&self) -> Option<&DialogueLine> {
        if let Some(scene) = &self.current_scene {
            scene.lines.get(self.current_line_index)
        } else {
            None
        }
    }

    pub fn get_bgm_for_scene(scene_id: &str) -> &'static str {
        match scene_id {
            "intro" => "rain.ogg",
            "chapter1" => "unha_theme.ogg",
            "chapter2" => "hana_theme.ogg",
            "chapter3" => "hyejin_theme.ogg",
            "s_unha" | "s_hana" | "s_hjin" | "chapter4" => "science.ogg",
            "end_0" | "end_1" | "end_2" => "last.ogg",
            _ => "bgm_main.ogg"
        }
    }

    pub fn mark_chapter_completed(&mut self, chapter_id: &str, visited_locations: &mut ResMut<VisitedLocations>) {
        visited_locations.completed_chapters.insert(chapter_id.to_string());
    }

    pub fn move_to_scene(&mut self, scene_id: &str) {
        let next_scene_index = self.scenes.iter().position(|scene| 
            scene.scene_id == *scene_id
        );
        
        if let Some(scene_index) = next_scene_index {
            self.current_scene = Some(self.scenes[scene_index].clone());
            self.current_line_index = 0;
            self.choice_system.clear_choices();
            
            if let Some(first_line) = self.current_scene.as_ref().and_then(|s| s.lines.get(0)) {
                let first_line = first_line.clone();
                self.update_all_ui_state(&first_line);
            }
            
        } else {
        }
    }

    pub fn filter_available_choices(&self, choices: &[Choice], visited_locations: &VisitedLocations) -> Vec<Choice> {
        choices.iter()
            .filter(|choice| {
                if let Some(next_scene_id) = &choice.next_scene_id {
                    !visited_locations.completed_chapters.contains(next_scene_id)
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }

    pub fn get_filtered_current_choices(&self, visited_locations: &VisitedLocations) -> Vec<Choice> {
        if let Some(line) = self.get_current_line() {
            if let Some(choices) = &line.choices {
                return self.filter_available_choices(choices, visited_locations);
            }
        }
        vec![]
    }
    
    pub fn complete_minigame(&mut self) {
        self.pending_minigame = None;
        self.next_line();
    }

    // 타이핑 관련 메서드들 (위임)
    pub fn start_typing(&mut self, text: String) {
        self.typing_effect.start_typing(text);
    }

    pub fn complete_typing(&mut self) {
        self.typing_effect.complete_typing();
    }

    pub fn update_typing(&mut self, delta_time: std::time::Duration) {
        self.typing_effect.update_typing(delta_time);
    }

    pub fn get_displayed_text(&self) -> &String {
        self.typing_effect.get_displayed_text()
    }

    pub fn is_typing(&self) -> bool {
        self.typing_effect.is_typing
    }

    // 선택지 관련 메서드들 (위임)
    pub fn is_choice_active(&self) -> bool {
        self.choice_system.is_choice_active
    }

    pub fn get_current_choices(&self) -> &Vec<Choice> {
        self.choice_system.get_all_choices()
    }

    pub fn set_changed(&mut self) {
        // &mut self를 받는 것만으로도 Bevy가 변경을 감지합니다
    }

    pub fn next_line(&mut self) -> bool {
        // 1. 타이핑 중이면 타이핑만 완료하고 종료
        if self.typing_effect.is_typing {
            self.complete_typing();
            return true;
        }

        // 2. 선택지 활성화 중이면 진행하지 않음
        if self.choice_system.is_choice_active {
            return false;
        }

        // 3. pending 미니게임이 있으면 제거 (스킵 처리)
        if self.pending_minigame.is_some() {
            self.pending_minigame = None;
        }

        // 4. 현재 씬 확인
        let scene = match &self.current_scene {
            Some(scene) => scene.clone(),
            None => return false,
        };

        // 5. 현재 라인 정보 확인
        if let Some(current_line) = scene.lines.get(self.current_line_index) {
            // 5-1. next_line_index가 있으면 해당 라인으로 점프
            if let Some(next_idx) = current_line.next_line_index {
                self.current_line_index = next_idx;
                if let Some(new_line) = scene.lines.get(self.current_line_index) {
                    let new_line_data = new_line.clone();
                    self.update_all_ui_state(&new_line_data);
                    return true;
                }
            }
            
            // 5-2. 선택지가 있으면 선택지 모드 활성화
            if let Some(choices) = &current_line.choices {
                self.choice_system.activate_choices(choices.clone());
                return true;
            }
        }

        // 6. 다음 라인으로 이동 가능한지 확인
        if self.current_line_index >= scene.lines.len() - 1 {
            return false;
        }

        // 7. 다음 라인으로 이동
        self.current_line_index += 1;
        
        // 8. 새 라인 정보 가져오기
        let new_line = match scene.lines.get(self.current_line_index) {
            Some(line) => line,
            None => {
                return false;
            }
        };

        // 9. 모든 UI 상태 업데이트
        let new_line_data = new_line.clone();
        self.update_all_ui_state(&new_line_data);

        true
    }

    // UI 상태를 완전히 업데이트하는 헬퍼 함수
    fn update_all_ui_state(&mut self, line: &DialogueLine) {
        // 텍스트 타이핑 시작
        self.start_typing(line.text.clone());
        
        // 배경 업데이트
        if let Some(background) = &line.background {
            self.current_background = Some(background.clone());
        }
        
        // 미니게임 트리거 설정
        if let Some(minigame_type) = &line.minigame_trigger {
            self.pending_minigame = Some(minigame_type.clone());
        }
        
        // 캐릭터 이미지 및 기타 UI 업데이트를 위해 변경 상태 설정
        self.set_changed();
        
        if let Some(bg) = &line.background {
        }
        if let Some(chars) = &line.visible_characters {
        }
        if let Some(displayed) = &line.displayed_character {
        }
    }

    pub fn make_choice(&mut self, choice_index: usize) -> bool {
        if !self.choice_system.is_choice_active || choice_index >= self.choice_system.current_choices.len() {
            return false;
        }
    
        // 선택지 정보를 미리 복사해서 borrow checker 문제 해결
        let choice = self.choice_system.current_choices[choice_index].clone();
    
        // 호감도 변화 적용
        if let Some(affection_changes) = &choice.affection_changes {
            self.apply_affection_changes(affection_changes);
        }
    
        // 선택지 모드 종료
        self.choice_system.clear_choices();
    
        // 선택에 따른 처리
        if let Some(next_scene_id) = &choice.next_scene_id {
            // 다른 씬으로 이동하는 로직
            
            // 지정된 scene_id를 가진 씬 찾기
            let next_scene_index = self.scenes.iter().position(|scene| 
                scene.scene_id == *next_scene_id
            );
            
            if let Some(scene_index) = next_scene_index {
                // 새 씬으로 교체
                self.current_scene = Some(self.scenes[scene_index].clone());
                self.current_line_index = 0; // 씬의 첫 번째 라인부터 시작
                
                // 새 씬의 첫 번째 라인 가져와서 복제
                let first_line = if let Some(scene) = &self.current_scene {
                    scene.lines.get(0).cloned()
                } else {
                    None
                };
                
                // 복제된 값 사용
                if let Some(new_line) = first_line {
                    self.update_all_ui_state(&new_line);
                }
                
                return true;
            } else {
            }
        } else if let Some(next_line_index) = choice.next_line_index {
            // 같은 씬 내에서 특정 라인으로 이동
            self.current_line_index = next_line_index;
        } else {
            // 기본적으로 다음 라인으로 이동
            self.current_line_index += 1;
        }
    
        // 현재 라인 인덱스 및 씬 복사
        let current_index = self.current_line_index;
        let current_line = if let Some(scene) = &self.current_scene {
            scene.lines.get(current_index).cloned()
        } else {
            None
        };
        
        // 복제된 값 사용
        if let Some(new_line) = current_line {
            self.update_all_ui_state(&new_line);
        }
    
        true
    }

    /// 호감도 변화를 적용하는 함수
    pub fn apply_affection_changes(&mut self, changes: &HashMap<String, i32>) {
        for (character_name, change_value) in changes {
            if let Some(current_affection) = self.character_affection.get_mut(character_name) {
                *current_affection += change_value;
                
                // 호감도 범위 제한 (0~100)
                *current_affection = (*current_affection).max(0).min(100);
            }
        }
    }

    /// 특정 캐릭터의 호감도를 직접 추가하는 함수
    pub fn add_affection(&mut self, character_name: &str, amount: i32) {
        if let Some(current_affection) = self.character_affection.get_mut(character_name) {
            *current_affection += amount;
            
            // 호감도 범위 제한 (0~100)
            *current_affection = (*current_affection).max(0).min(100);
        }
    }

    /// 특정 캐릭터의 호감도를 가져오는 함수
    pub fn get_affection(&self, character_name: &str) -> i32 {
        *self.character_affection.get(character_name).unwrap_or(&0)
    }

    /// 모든 캐릭터의 호감도를 가져오는 함수
    pub fn get_all_affection(&self) -> &HashMap<String, i32> {
        &self.character_affection
    }

    /// 캐릭터 호감도를 설정하는 함수
    pub fn set_affection(&mut self, character_name: &str, value: i32) {
        let clamped_value = value.max(0).min(100);
        self.character_affection.insert(character_name.to_string(), clamped_value);
    }

    pub fn is_scene_finished(&self) -> bool {
        if let Some(scene) = &self.current_scene {
            self.current_line_index >= scene.lines.len() - 1 && !self.choice_system.is_choice_active && !self.typing_effect.is_typing
        } else {
            true
        }
    }

    /// 현재 배경을 업데이트하는 함수
    pub fn update_background(&mut self, new_background: String) {
        self.current_background = Some(new_background);
    }

    /// 현재 배경을 가져오는 함수
    pub fn get_current_background(&self) -> Option<&String> {
        self.current_background.as_ref()
    }

    
}
