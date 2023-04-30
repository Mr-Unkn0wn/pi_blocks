use egui::*;
use macroquad::ui;

use crate::square::Square;

pub fn draw_ui(left: &mut Square, right: &mut Square, grid: &f32){

    egui_macroquad::ui(|ctx| {

        egui::Window::new("Settings")
            .show(ctx, |ui| {
                
                manipulate_square(ui, left, "Left square");
                manipulate_square(ui, right, "Right square");

            });

    });
    egui_macroquad::draw();
}

fn manipulate_square(ui: &mut Ui, square: &mut Square, name: &str){
    egui::CollapsingHeader::new(name)
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut square.mass)
                    .speed(1),
            );
            ui.label("Mass");
        });

        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut square.vel)
                    .speed(1),
            );
            ui.label("Velocity");
        });
    
        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut square.width)
                    .speed(1),
            );
            ui.label("Width");
        });

        ui.horizontal(|ui| {
            ui.add(
                DragValue::new(&mut square.height)
                    .speed(1),
            );
            ui.label("Heigth");
        });
    

});
}