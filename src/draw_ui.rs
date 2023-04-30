use egui::*;

use crate::{square::Square, simulate};

pub fn draw_ui(index: &mut usize, simulation_time: &mut f64, left: &mut Square, right: &mut Square, left_sim: &mut Square, right_sim: &mut Square, grid: &f32, tick_time: &mut f64, active: &mut bool, wall_x : &f32, coll_list: &mut Vec<simulate::Collision>){

    egui_macroquad::ui(|ctx| {

        egui::Window::new("Settings")
            .anchor(Align2::RIGHT_TOP, egui::emath::vec2(-10.0, 10.0))
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                
                manipulate_square(ui, left, "Left square", active);
                manipulate_square(ui, right, "Right square", active);

                ui.separator();

                time_slider(ui, tick_time);
                simulation_toggle(ui, active, left, right, left_sim, right_sim, wall_x, coll_list, index, simulation_time);
            });

    });
    egui_macroquad::draw();
}

fn manipulate_square(ui: &mut Ui, square: &mut Square, name: &str, active: &mut bool){
    egui::CollapsingHeader::new(name)
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_enabled(!*active, 
                DragValue::new(&mut square.mass)
                    .speed(1),
            );
            ui.label("Mass");
        });

        ui.horizontal(|ui| {
            ui.add_enabled(!*active,
                DragValue::new(&mut square.vel)
                    .speed(1),
            );
            ui.label("Velocity");
        });
    
        ui.horizontal(|ui| {
            ui.add_enabled(!*active,
                DragValue::new(&mut square.width)
                    .speed(1),
            );
            ui.label("Width");
        });

        ui.horizontal(|ui| {
            ui.add_enabled(!*active,
                DragValue::new(&mut square.height)
                    .speed(1),
            );
            ui.label("Heigth");
        });

        ui.horizontal(|ui| {
            ui.add_enabled(!*active,
                DragValue::new(&mut square.pos.x)
                    .speed(1),
            );
            ui.label("Position");
        });
    

});
}

fn time_slider(ui: &mut Ui, tick_time: &mut f64){
    ui.add(
        Slider::new(tick_time, 0.0..=1.0/10.0)
            .logarithmic(true)
            .text("Time per tick")
    );
}

fn simulation_toggle(ui: &mut Ui, active: &mut bool, left: &mut Square, right: &mut Square, left_sim: &mut Square, right_sim: &mut Square
    , wall_x : &f32, coll_list: &mut Vec<simulate::Collision>, index: &mut usize, simulation_time: &mut f64){
    ui.horizontal(|ui| {
        if ui.add(egui::Button::new("Run simulation")).clicked() {
            change_sim_state(active, left, right, left_sim, right_sim, wall_x, coll_list, index, simulation_time);
        }
        ui.add_enabled(false, egui::Checkbox::new(active, ""));
    });
}

fn change_sim_state(active: &mut bool, left: &mut Square, right: &mut Square, left_sim: &mut Square, right_sim: &mut Square, wall_x : &f32, coll_list: &mut Vec<simulate::Collision>, index: &mut usize, simulation_time: &mut f64){
    if !*active {
        *left_sim = (*left).clone();
        *right_sim = (*right).clone();
        *coll_list = simulate::simulate_everything(&mut left_sim.clone(), &mut right_sim.clone(), *wall_x);

        *index = 0;
        *simulation_time = 0.0;
    }


    *active = !*active;
}

