pub mod typing_effect;
pub mod choice_system; 
pub mod dialogue_manager;
pub mod stories;

pub use typing_effect::*;
pub use choice_system::*;
pub use dialogue_manager::*;
pub use stories::*;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct VisitedLocations {
    pub completed_chapters: HashSet<String>,
}
