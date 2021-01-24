use crate::prelude::*;

pub struct Graphics {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
}

impl Graphics {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("CaveStory Rust")
            .build();

        Self {
            rl,
            thread,
        }
    }

    pub fn load_image(&mut self, file_path: &str) -> Texture2D {
        let texture = self.rl
            .load_texture(&self.thread, file_path)
            .expect("Could not load Texture from Image");
        texture
    }
}
