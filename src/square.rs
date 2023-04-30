use macroquad::prelude::*;

#[derive(Clone)]
pub struct Square {
    pub mass: f32,
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub vel: f32,
}

impl Square{
    pub fn draw(&self, font: &Font){
        draw_rectangle_lines(self.pos.x, self.pos.y, self.width, self.height, 6.0, WHITE);


        let text_paras = TextParams{
            font: *font,
            font_size: 30,
            color: WHITE,
            ..Default::default()
        };

        draw_text_ex(&self.mass.to_string(), self.pos.x, self.pos.y - 10.0, text_paras);
    }
}

