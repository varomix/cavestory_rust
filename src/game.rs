use crate::prelude::*;

pub struct Game {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub scr_width: i32,
    pub scr_height: i32,
    fps: u32,
    gfx: Graphics,
    char: Sprite,
}

impl Game {
    pub fn new(scr_width: i32, scr_height: i32) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(scr_width, scr_height)
            .title("CaveStory Raylib Rust")
            .build();

        let gfx = Graphics::new(&rl);

        Self {
            rl,
            thread,
            scr_width,
            scr_height,
            fps: 50,
            gfx,
            char: Sprite::new(String::from("filepath")),
        }
    }

    pub fn run(&mut self) {
        self.rl.set_target_fps(self.fps);
        while !self.rl.window_should_close() {
            // UPDATE

            // DRAW
            self.draw();
        }
    }

    pub fn draw(&mut self) {
        // Draw //
        // let mut d = Graphics::new(self.rl.begin_drawing(&self.thread));
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::GRAY);
        d.draw_text("CaveStory", 20, 20, 20, Color::RAYWHITE);

        // draw char
        self.char.draw(&mut d, 400, 200);
    }

    pub fn update(&self) {
        unimplemented!()
    }
}
