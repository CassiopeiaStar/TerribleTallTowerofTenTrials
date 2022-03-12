use crate::constants::*;
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

        let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);

        for (i,msg) in vec![
            "Press C at any time to view controls",
            "WASD or left mouse click to move",
            "Space to wait",
            "E or right click self to pickup items",
            "Tab or I to open inventory",
            "Items can be equiped or used from the inventory",
            "Walk into enemies to attack them",
        ].into_iter().enumerate() {
            draw_text_ex(
                msg,
                tile_size*2.,tile_size*i as f32+ 100.,
                TextParams {
                    font_size: tile_size as u16,
                    color: LIGHTGRAY,
                    ..Default::default()
                }
            );
        }

        next_frame().await;
    }

    next_frame().await
}
