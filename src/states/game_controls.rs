use macroquad::prelude::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self,Group},
    Ui,
};

pub async fn control_screen() {

    next_frame().await;
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Space) ||
            is_key_pressed(KeyCode::Escape) ||
            is_key_pressed(KeyCode::C) ||
            is_key_pressed(KeyCode::Enter) {
                break;
            }

        let width = screen_width()-100.;
        let height = screen_height()-100.;
        widgets::Window::new(hash!(), vec2(10.,10.),vec2(width,height))
            .movable(false)
            .ui(&mut *root_ui(), |ui| {
                Group::new(hash!(),Vec2::new(width-50.,50.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),
                        "Press C at any time to view controls"
                    );
                });
                Group::new(hash!(),Vec2::new(width-50.,50.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),
                        "WASD or left mouse click to move"
                    );
                });
                Group::new(hash!(),Vec2::new(width-50.,50.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),
                        "Walk into enemies to attack them"
                    );
                });
                Group::new(hash!(),Vec2::new(width-50.,50.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),
                        "E or right click self to pickup items"
                    );
                });
                Group::new(hash!(),Vec2::new(width-50.,50.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),
                        "Tab or I to open inventory"
                    );
                });
                Group::new(hash!(),Vec2::new(width-50.,50.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),
                        "Items can be equiped or used from the inventory"
                    );
                });

            });


        next_frame().await;
    }

    next_frame().await
}
