use bevy::prelude::*;

use super::draggable::Draggable;

#[derive(Component)]
#[require(Draggable)]
pub struct Slider {
    pub puzzle_id: u32,
    pub slider_id: usize,
}

impl Slider {
    pub fn new(puzzle_id: u32, slider_id: usize) -> Self {
        Self {
            puzzle_id,
            slider_id,
        }
    }
}
