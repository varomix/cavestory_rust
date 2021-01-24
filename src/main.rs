// mod game;
mod graphics;
mod sprite;
// mod player;

mod prelude {
    pub use raylib::prelude::*;

    pub const SCREEN_WIDTH: i32 = 800;
    pub const SCREEN_HEIGHT: i32 = 450;

    // pub use crate::game::*;
    pub use crate::graphics::*;
    pub use crate::sprite::*;
    // pub use crate::player::*;
}

use prelude::*;

fn main() {
    // let mut game = Game::new(800, 450);
    // game.run();

    // INIT //
    let mut gfx = Graphics::new();
    let mut spr = Sprite::new(&mut gfx, "content/sprites/MyChar.png");
    // let tex = gfx.load_image("content/sprites/MyChar.png");

    gfx.rl.set_target_fps(60);

    while !gfx.rl.window_should_close() {
        // Draw //
        let mut d = gfx.rl.begin_drawing(&gfx.thread);
        d.clear_background(Color::WHITE);

        spr.draw(10, 10);
        // d.draw_texture(&spr._sprite_sheet, 20, 20, Color::WHITE);

        d.draw_text("CaveStory", 20, 20, 16, Color::GRAY);
    }
}
