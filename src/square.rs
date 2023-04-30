use macroquad::prelude::*;

#[derive(Clone)]
pub struct Square {
    pub mass: f32,
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
    pub vel: f32,
}

#[derive(Debug)]
pub struct SquaresCollision{
    pub time: f32,
    pub x: f32,
    pub vel1: f32,
    pub vel2: f32,
}

#[derive(Debug)]
pub struct WallCollision{
    pub time: f32,
    pub vel: f32,
}

#[derive(Debug)]
pub enum Collision{
    WallCollision(WallCollision),
    SquaresCollision(SquaresCollision),
}

impl Square{
    pub fn draw(&self){
        draw_rectangle_lines(self.pos.x, self.pos.y, self.width, self.height, 6.0, WHITE);
        draw_text(&self.mass.to_string(), self.pos.x, self.pos.y - 10.0, 40.0, WHITE);
    }
}

pub fn update_squares(left: &mut Square, right: &mut Square, wall_x: &f32, coll_list: &Vec<Collision>, current_time: &f64, index: &mut usize){
    let tick_time: f64 = 1.0/60.0;

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

        if list_time as f64 >= current_time + tick_time {
            break;
        }


        *index += 1;

    }

    if *index == 0 as usize {
        left.pos.x += left.vel * tick_time as f32;
        right.pos.x += right.vel * tick_time as f32;
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

    if ( list_time as f64 ) < *current_time {
        left.pos.x += left.vel * tick_time as f32;
        right.pos.x += right.vel * tick_time as f32;
        return;
    }

    let time_left_in_tick = current_time + tick_time - list_time as f64;
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

}

pub fn simulate_everything(left: &mut Square, right: &mut Square, wall_x: f32) -> Vec<Collision>{
    // loop {
    // - find collision that happens first
    // - calculate the result of that 
    // }
    let mut time = 0.0;

    let mut coll_list: Vec<Collision> = Vec::new();


    loop{
        let squares_col_time = find_collision_time(left.pos.x + left.width, left.vel, right.pos.x, right.vel);
        let wall_col_time = find_collision_time(wall_x, 0.0, left.pos.x, left.vel);

        if !squares_col_time.0 && !wall_col_time.0 { // there is no collision at all
            break;
        }

        if !squares_col_time.0 { // there is no collision between the squares
            left.pos.x += left.vel * squares_col_time.1;
            right.pos.x += right.vel * squares_col_time.1;

            left.vel = -left.vel;

            time += wall_col_time.1;


            let c = WallCollision {
                time,
                vel: left.vel,
            };

            coll_list.push(Collision::WallCollision(c));

        } else if !wall_col_time.0 || squares_col_time.1 < wall_col_time.1{ // there is no collision with the wall or the squares collider sooner
            left.pos.x += left.vel * squares_col_time.1;
            right.pos.x += right.vel * squares_col_time.1 + 0.001;

            let (v1, v2) = ellastic_collision(left.mass, left.vel, right.mass, right.vel);
            left.vel = v1;
            right.vel = v2;

            time += squares_col_time.1;

            let c = SquaresCollision {
                time,
                x: left.pos.x,
                vel1: left.vel,
                vel2: right.vel,
            };

            coll_list.push(Collision::SquaresCollision(c));
        } else { // the collision with the wall happens first
            left.pos.x += left.vel * squares_col_time.1;
            right.pos.x += right.vel * squares_col_time.1;

            left.vel = -left.vel;

            time += wall_col_time.1;


            let c = WallCollision {
                time,
                vel: left.vel,
            };

            coll_list.push(Collision::WallCollision(c));
        }

    }
    coll_list
}

fn ellastic_collision(mass1: f32, vel1: f32, mass2: f32, vel2: f32) -> (f32, f32){
    let v1 = (mass1 - mass2)/(mass1 + mass2)*vel1 + (2.0*mass2)/(mass1+mass2)*vel2;
    let v2 = (2.0*mass1)/(mass1+mass2)*vel1+(mass2-mass1)/(mass1+mass2)*vel2;
    (v1, v2)
}

fn find_collision_time(p1: f32, v1: f32, p2: f32, v2: f32) -> (bool, f32) {
    // p1 + v1 * t = p2 + v2 * t
    // ( p2 - p1 ) / ( v1 - v2 ) = t
    let v_diff = v1 - v2;
    if v_diff == 0.0{
        return (false, 10000000000.0);
    }

    let p_diff = p2 - p1;

    let result = p_diff / v_diff;

    if result == 0.0{
        println!("p1 {} v1 {} p2 {} v2 {}", p1, v1, p2, v2);
        println!("oh no");
    }

    if result < 0.0 {
        return (false, result);
    }

    return (true, result);
}

