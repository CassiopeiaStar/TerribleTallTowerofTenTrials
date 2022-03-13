use hecs::*;
use macroquad::prelude::*;
use crate::prelude::*;

pub async fn win_state(world: &mut World,resources: &mut Resources) -> StateChange {
    loop {
        let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
        
        draw_text_ex(
            "Congratulations! You escaped!",
            tile_size*3.,tile_size*10.,
            TextParams {
                font_size: tile_size as u16,
                color: LIGHTGRAY,
                font: resources.font,
                ..Default::default()
            }
        );

        draw_text_ex(
            "Thank you for playing",
            tile_size*2.,tile_size*17.,
            TextParams {
                font_size: tile_size as u16,
                color: LIGHTGRAY,
                font: resources.font,
                ..Default::default()
            }
        );

        next_frame().await;
        if is_key_pressed(KeyCode::Space) ||
            is_key_pressed(KeyCode::Enter) ||
            is_key_pressed(KeyCode::Escape) {
                break;
            }

    }
    StateChange::Replace(GameState::MainMenu)
}
