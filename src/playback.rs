use crate::{square::Square, simulate::{WallCollision, Collision, SquaresCollision}, sound_player::SoundPlayer};


pub fn update_squares(sound: &SoundPlayer, time_per_tick : &f64, left: &mut Square, right: &mut Square, wall_x: &f32, coll_list: &Vec<Collision>, simulation_time: &mut f64, index: &mut usize){

    *simulation_time += time_per_tick;
    let old_index = *index;

    let mut list_time: f32;
    loop { // move through list until we reach last collision in current tick
        if *index == coll_list.len() {
            break;
        }

        match coll_list[*index] {
            Collision::WallCollision(WallCollision { time, .. }) => {
                list_time = time;
            },
            Collision::SquaresCollision(SquaresCollision { time, .. }) => {
                list_time = time;
            },
        }

        if list_time as f64 >= *simulation_time + time_per_tick {
            break;
        }


        *index += 1;

    }

    if *index == 0 as usize {
        left.pos.x += left.vel * *time_per_tick as f32;
        right.pos.x += right.vel * *time_per_tick as f32;
        return;
    }

    let relevant_collision = &coll_list[*index - 1];

    match relevant_collision {
        Collision::WallCollision(WallCollision { time,.. }) => {
            list_time = *time;
        },
        Collision::SquaresCollision(SquaresCollision { time,.. }) => {
            list_time = *time;
        },
    }

    if ( list_time as f64 ) < *simulation_time {
        left.pos.x += left.vel * *time_per_tick as f32;
        right.pos.x += right.vel * *time_per_tick as f32;
        return;
    }

    let time_left_in_tick = *simulation_time + time_per_tick - list_time as f64;
    match relevant_collision {
        Collision::WallCollision(WallCollision { vel, .. }) => {
            left.pos.x = *wall_x;
            left.vel = *vel;

            right.pos.x += right.vel * time_left_in_tick as f32;
            left.pos.x += left.vel * time_left_in_tick as f32;
        },
        Collision::SquaresCollision(SquaresCollision { x, vel1, vel2, .. }) => {
            right.pos.x = *x + left.width;
            left.pos.x = *x;

            left.vel = *vel1;
            right.vel = *vel2;

            right.pos.x += right.vel * time_left_in_tick as f32;
            left.pos.x += left.vel * time_left_in_tick as f32;
        },
    }
    sound.play(&((*index - old_index) as i32));

}
