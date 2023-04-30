use macroquad::prelude::*;
use square::simulate_everything;
use crate::square::{Square, update_squares};

mod square;



fn window_conf() -> Conf {
    Conf {
        window_title: "Pi Blocks".to_string(),
        fullscreen: false,
        window_resizable: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {

    let cols = 13;
    let grid: f32 = screen_width() / cols as f32;
    let vingette = Texture2D::from_file_with_format(include_bytes!("../vingette.png"), None);
    let wall_x = screen_width() / 2.0 - grid * 5.0;
    let wall_y = screen_height() / 2.0 + grid * 2.0;

    let mut box_right = Square{
        mass: 1000000.0,
        pos: Vec2::new(wall_x + 600.0, wall_y - 302.0),
        width: 300.0,
        height: 300.0,
        vel: -90.0,
    };

    let mut box_left = Square{
        mass: 1.0,
        pos: Vec2::new(wall_x + 300.0, wall_y - 102.0),
        width: 100.0,
        height: 100.0,
        vel: 0.0,
    };


    let coll_list = simulate_everything(&mut box_left.clone(), &mut box_right.clone(), wall_x);
    let mut index: usize = 0;

    print!("List : {:#?}", coll_list);

    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            print!("Total : {}", coll_list.len());
            break;
        }

        clear_background(Color::new(0.00, 0.37, 0.9, 1.00));
        draw_grid(SKYBLUE, grid, 3, 3.0, 1.0, 0.3);
        draw_enviornment(WHITE, wall_x, wall_y, grid);

        box_right.draw();
        box_left.draw();
        update_squares(&mut box_left, &mut box_right, &wall_x, &coll_list, &get_time(), &mut index);

        draw_text(&get_fps().to_string(), 0.0 + 10.0, 0.0 + 30.0, 20.0, WHITE);
        draw_vingette(vingette);
        next_frame().await
    }
}

fn draw_grid(color: Color, gap: f32, bold_every: i32, thick_every: f32, thick: f32, alpha: f32){
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

fn draw_enviornment(color: Color, wall_x: f32, wall_y: f32, grid: f32){
    draw_line(wall_x - 1.0, wall_y, screen_width(), wall_y, 3.0, color);
    draw_line(wall_x, wall_y + 1.0, wall_x, wall_y - 4.0 * grid, 3.0, color);
}

fn draw_vingette(vingette: Texture2D){
    draw_texture_ex(vingette, 0.0, 0.0, WHITE,
    DrawTextureParams {
        dest_size: Some(vec2(screen_width(), screen_height())),
        ..Default::default()
    }
    );
}