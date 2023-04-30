use macroquad::prelude::*;
use crate::square::{Square};

mod square;
mod draw_everything;
mod simulate;
mod playback;


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
    let vingette = Texture2D::from_file_with_format(include_bytes!("../vingette.png"), None);
    let font = load_ttf_font_from_bytes(include_bytes!("../Monocraft.ttf")).unwrap();

    let cols = 13;
    let grid: f32 = screen_width() / cols as f32;
    let wall_x = screen_width() / 2.0 - grid * 5.0;
    let wall_y = screen_height() / 2.0 + grid * 2.0;



    let mut box_right = Square{
        mass: 10000000000.0,
        pos: Vec2::new(wall_x + 2000.0, wall_y - 302.0),
        width: 300.0,
        height: 300.0,
        vel: -300.0,
    };

    let mut box_left = Square{
        mass: 1.0,
        pos: Vec2::new(wall_x + 300.0, wall_y - 102.0),
        width: 100.0,
        height: 100.0,
        vel: 0.0,
    };


    let coll_list = simulate::simulate_everything(&mut box_left.clone(), &mut box_right.clone(), wall_x);
    let mut index: usize = 0;
    let mut simulation_time: f64 = 0.0;

    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            println!("Total : {}", coll_list.len());
            break;
        }

        clear_background(Color::new(0.00, 0.37, 0.9, 1.00));
        draw_everything::draw_grid(SKYBLUE, grid, 3, 3.0, 1.0, 0.3);
        draw_everything::draw_enviornment(WHITE, wall_x, wall_y, grid);

        box_right.draw(&font);
        box_left.draw(&font);
        playback::update_squares(&mut box_left, &mut box_right, &wall_x, &coll_list, &mut simulation_time, &mut index);

        draw_everything::draw_collision_counter(&index, &font, &grid);

        draw_everything::draw_vingette(vingette);
        next_frame().await
    }
}
