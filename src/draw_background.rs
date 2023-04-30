use macroquad::prelude::*;

pub fn draw_grid(color: Color, gap: f32, bold_every: i32, thick_every: f32, thick: f32, alpha: f32){
    let color = Color::new(color.r, color.g, color.b, alpha); 

    let x_center = screen_width() / 2.0;
    let y_center = screen_height() / 2.0;
    
    let mut x = 0.0;
    let mut y = 0.0;
    let mut counter = 0;


    while x_center + x < screen_width(){
        let current_thick = if counter % bold_every == 0 {
            thick_every
        } else {
            thick
        };

        draw_line(x_center + x,screen_height(), x_center + x, 0.0, current_thick, color);
        draw_line(x_center - x,screen_height(), x_center - x, 0.0, current_thick, color);

        draw_line(0.0 ,y_center + y, screen_width(), y_center + y, current_thick, color);
        draw_line(0.0 ,y_center - y, screen_width(), y_center - y, current_thick, color);

        x += gap;
        y += gap;
        counter += 1;
    }
}

pub fn draw_enviornment(color: Color, wall_x: f32, wall_y: f32, grid: f32){
    draw_line(wall_x - 1.0, wall_y, screen_width(), wall_y, 3.0, color);
    draw_line(wall_x, wall_y + 1.0, wall_x, wall_y - 4.0 * grid, 3.0, color);
}

pub fn draw_vingette(vingette: Texture2D){
    draw_texture_ex(vingette, 0.0, 0.0, WHITE,
    DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    }
    );
}