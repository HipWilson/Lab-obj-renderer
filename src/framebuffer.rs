use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub image: Image,
    pub color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background: Color) -> Self {
        let image = Image::gen_image_color(width, height, background);
        Framebuffer { width, height, image, color: Color::WHITE }
    }

    pub fn clear(&mut self, background: Color) {
        self.image.clear_background(background);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.image.draw_pixel(x, y, self.color);
        }
    }
}
