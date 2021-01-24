use crate::prelude::*;

pub struct Sprite<'a> {
    gfx: &'a mut Graphics,
    pub _sprite_sheet: Texture2D,
}

impl<'a> Sprite<'a> {
    pub fn new(gfx: &'a mut Graphics, file_path: &str) -> Self {
        let _texture = gfx.load_image(file_path);
        Self {
            gfx,
            _sprite_sheet: _texture,
        }
    }

    pub fn draw(&mut self, x: i32, y: i32) {
        let mut d = self.gfx.rl.begin_drawing(&self.gfx.thread);
        d.draw_circle(x, y, 35., Color::RED);
    }
}
