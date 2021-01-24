use crate::prelude::*;

pub struct Player {
    pub position: Vector2,
}

impl Player {
    pub fn new(position: Vector2) -> Self {
        Self {
            position,
        }
    }

    pub fn render(&self) {
        // Graphics::load_image()
    }
}