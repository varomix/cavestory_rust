mod graphics;
mod sprite;

mod prelude {
    pub use raylib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 640;
    pub const SCREEN_HEIGHT: i32 = 450;

    pub const SPRITE_SCALE: f32 = 2.0;

    pub use crate::graphics::*;
    pub use crate::sprite::*;
}

use prelude::*;

fn main() {
    // INIT //
    let mut gfx = Graphics::new();

    let mut player = Sprite::new(&mut gfx, "content/sprites/MyChar.png");

    gfx.rl.set_target_fps(60);
    while !gfx.rl.window_should_close() {
        // Draw //
        let mut d = gfx.rl.begin_drawing(&gfx.thread);
        d.clear_background(Color::BLACK);

        player.draw(&mut d, 100.0, 100.0);

        // d.draw_text("CaveStory", 20, 20, 16, Color::GRAY);
    }
}
