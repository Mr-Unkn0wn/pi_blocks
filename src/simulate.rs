use crate::square::Square;

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
