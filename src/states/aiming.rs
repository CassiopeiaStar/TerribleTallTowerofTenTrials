use hecs::*;
use macroquad::prelude::*;
use crate::prelude::*;
use crate::combat::*;


pub async fn aiming_state(
    world: &mut World,
    resources: &mut Resources,
    _attack_data: &AttackData,
) -> Option<(i32,i32)> {
    next_frame().await;
    let player = player(world).unwrap();
    let player_position = get_cloned::<Pos>(world,player).unwrap();
    let draw = |world,resources| {
        draw_map_and_hud(world,resources);
    };
    loop {
        if is_key_pressed(macroquad::input::KeyCode::Escape) {
            break;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            break;
        }
        draw(world,resources);

        if let Some(mouse_tile) = mouse_to_map() {
            if let Some((path,distance)) = unblocked_attack_line(
                world,player_position.into(),mouse_tile
            ) {
                let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
                let start = ((player_position.x as f32+1.5) *tile_size,(player_position.y as f32+1.5) *tile_size);
                let finish = ((mouse_tile.0 as f32+1.5) *tile_size,(mouse_tile.1 as f32+1.5) *tile_size);
                draw_line(start.0,start.1,finish.0,finish.1,3.,WHITE);
                if is_mouse_button_pressed(MouseButton::Left) {
                    next_frame().await;
                    return Some(mouse_tile);
                }
            }
        }
        next_frame().await
    }
    dbg!();
    next_frame().await;
    None
}
