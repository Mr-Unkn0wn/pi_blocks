use macroquad::prelude::*;
use crate::square::{Square};

mod square;
mod draw_game;
mod simulate;
mod playback;
mod draw_ui;


fn window_conf() -> Conf {
    Conf {
        window_title: "Pi Blocks".to_string(),
        fullscreen: false,
        window_resizable: false,
        window_width: 1300,
        window_height: 750,
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

    let mut time_per_tick: f64 = 1.0 / 50.0;
    let mut active: bool = false;


    let mut box_right = Square{
        mass: 1.0,
        pos: Vec2::new(wall_x + grid * 6.0, wall_y - grid * 2.0 - 1.0),
        width: grid * 2.0,
        height: grid * 2.0,
        vel: -300.0,
    };

    let mut box_left = Square{
        mass: 1.0,
        pos: Vec2::new(wall_x + grid * 3.0, wall_y - grid - 1.0),
        width: grid,
        height: grid,
        vel: 0.0,
    };

    let mut box_right_sim = Square{
        mass: 0.0,
        pos: Vec2::new(0.0, 0.0),
        width: 10.0,
        height: 10.0,
        vel: 0.0,
    };

    let mut box_left_sim = Square{
        mass: 0.0,
        pos: Vec2::new(0.0, 0.0),
        width: 10.0,
        height: 10.0,
        vel: 0.0,
    };


    let mut coll_list: Vec<simulate::Collision> = vec![];
    let mut index: usize = 0;
    let mut simulation_time: f64 = 0.0;

    loop {
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(Color::new(0.00, 0.37, 0.9, 1.00));
        draw_game::draw_grid(SKYBLUE, grid, 3, 3.0, 1.0, 0.3);
        draw_game::draw_enviornment(WHITE, wall_x, wall_y, grid);

        if active {
            box_right_sim.draw(&font);
            box_left_sim.draw(&font);
            playback::update_squares(&time_per_tick, &mut box_left_sim, &mut box_right_sim, &wall_x, &coll_list, &mut simulation_time, &mut index);
        } else {
            box_right.draw(&font);
            box_left.draw(&font);
        }

        draw_game::draw_collision_counter(&index, &font, &grid);

        draw_game::draw_vingette(vingette);
        draw_ui::draw_ui(&mut index, &mut simulation_time,&mut box_left, &mut box_right, &mut box_left_sim, &mut box_right_sim, &grid, &mut time_per_tick, &mut active, &wall_x, &mut coll_list);
        next_frame().await
    }
}
