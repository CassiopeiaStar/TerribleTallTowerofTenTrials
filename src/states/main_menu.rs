
use hecs::*;
use macroquad::prelude::*;
use crate::prelude::*;
use crate::resources::*;

pub async fn main_menu(world: &mut World, resources: &mut Resources) -> StateChange {
    world.clear();
    *resources = load_resources().await;
    loop {
        let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
        
        draw_text_ex(
            "Welcome to Ended",
            tile_size*2.,tile_size*5.,
            TextParams {
                font_size: tile_size as u16* 3,
                color: LIGHTGRAY,
                font: resources.font,
                ..Default::default()
            }
        );

        draw_text_ex(
            "Press space to begin",
            tile_size*2.,tile_size*15.,
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
    macroquad::rand::srand((get_time()*100000.) as u64);
    start_with_health_pot(world,resources);
    StateChange::Replace(GameState::Game)
}


fn start_with_health_pot(world:&mut World,resources:&mut Resources) {
    let ent = spawn(world,EntityKind::HealthPotion);
    world.remove_one::<OnLevel>(ent).ok();
    resources.player.inventory.push(ent);
}

