use crate::prelude::*;

pub struct Sprite {
    pub _sprite_sheet: Texture2D,
}

impl Sprite {
    pub fn new(gfx: &mut Graphics, file_path: &str) -> Self {
        let _texture = gfx.load_image(file_path);
        Self {
            _sprite_sheet: _texture,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, x: f32, y: f32) {
        // d.draw_circle(x, y, 35., Color::RED);
        // d.draw_texture(&self._sprite_sheet, x, y, Color::WHITE);

        let src_rect = Rectangle::new(0.0, 0.0, 16.0, 16.0);
        let dest_rect = Rectangle::new(x, y,
                                       src_rect.width * SPRITE_SCALE,
                                       src_rect.height * SPRITE_SCALE);
        let origin = Vector2::new(8.0, 8.0);

        d.draw_texture_pro(&self._sprite_sheet, src_rect, dest_rect, origin, 0.0, Color::WHITE);
    }
}
