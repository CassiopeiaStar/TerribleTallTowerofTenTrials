use macroquad::prelude::*;
use hecs::*;


pub mod resources;
pub mod console;
pub mod components;
pub mod factory;
pub mod dungeon_gen;
pub mod states;
pub mod movement;
pub mod combat;
pub mod behavior;
pub mod field_of_view;
pub mod monster_memory;
pub mod hud;
pub mod lists;
pub mod screen_messages;

pub mod prelude {
    pub use crate::resources::Resources;
    pub use crate::factory::*;
    pub use crate::components::*;
    pub use crate::{
        world_to_console,
        player,
        draw_map_and_hud,
        mouse_to_map,
        get_entities_at,
        //build_dungeon,
        clear_hightlights,
        get_cloned,
    };
    pub use crate::hud::draw_hud;
    pub use crate::movement::{Dir,movement_map,stepping_tiles};
    pub use crate::constants::*;
    pub use crate::behavior::act;
    pub use crate::states::*;
    pub use crate::field_of_view::update_fov;
    pub use crate::lists::*;
    pub use crate::screen_messages::*;
}

pub mod constants {
    pub const TILE_WIDTH: f32 = 32.;
    pub const TILE_HEIGHT: f32 = 32.;
    pub const SPRITE_WIDTH: f32 = 32.;
    pub const SPRITE_HEIGHT: f32 = 32.;
    pub const SPRITE_SHEET_COLUMNS: u32 = 10;
    pub const ARENA_WIDTH: usize = 22;
    pub const ARENA_HEIGHT: usize = 22;
    pub const HUD_WIDTH: usize = 320;
    pub const WINDOW_WIDTH: i32 = (TILE_WIDTH as usize * (ARENA_WIDTH+2) + HUD_WIDTH) as i32;
    pub const WINDOW_HEIGHT: i32 = (TILE_HEIGHT as usize * (ARENA_HEIGHT+2)) as i32;
    pub const FOV_DISTANCE: u32 = 20;
    pub const DEBUG_FOV: bool = false;
}


use prelude::*;
use resources::load_resources;
use dungeon_gen::*;
use console::AsciiConsole;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Ended".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        //window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");
    dbg!(&WINDOW_WIDTH);
    dbg!(&WINDOW_HEIGHT);


    let mut world = World::new();
    let mut resources = load_resources().await;


    let mut state = GameState::MainMenu;

    loop {
        let state_change = state.run(&mut world,&mut resources).await;
        match state_change {
            StateChange::Quit => {
                break;
            }
            StateChange::Replace(new_state) => {
                state = new_state;
            }
        }
    }
}

pub fn world_to_console(
    world:     &World,
    resources: &Resources
) -> AsciiConsole {
    //the map console (later to be blitted onto the fram console)
    let mut con = AsciiConsole::new(ARENA_WIDTH,ARENA_HEIGHT,None);

    //collect draw data for entities in fov
    let mut draw_data: Vec<(Pos,Appearance)> = world.query::<(&Pos,&Appearance)>().iter()
        .map(|(_,(pos,appearance))|{
            (pos.clone(),appearance.clone())
        }).collect();
    
    draw_data.sort_by(|a,b|{
        (a.1.layer).partial_cmp(&b.1.layer).unwrap()
    });

    //collect draw data for memories
    let mut memory_draw_data: Vec<(Pos,Appearance)> = world.query::<(&Pos,&Appearance,&PlayerMemory)>().iter()
        .map(|(_,(pos,appearance,_))|{
            let mut appearance = appearance.clone();
            appearance.color.a = 0.5;
            (pos.clone(),appearance)
        }).collect();

    memory_draw_data.sort_by(|a,b|{
        (a.1.layer).partial_cmp(&b.1.layer).unwrap()
    });

    //draw memories
    for (pos,appearance) in memory_draw_data.iter() {
        if let Some(tile) = con.get_mut(&(pos.x,pos.y)) {
            if let Some(current_fg) = tile.fg {
                if !tile.ignore_overlap {
                    tile.set_bg(current_fg);
                }
            }

            tile.set_fg(appearance.color)
                .set_c(appearance.sprite)
                .set_ignore_overlap(appearance.ignore_overlap)
                .set_layer(appearance.layer);
        }
    }

    //draw entities in fov
    for (pos,appearance) in draw_data.iter() {
        if appearance.in_fov {
            if let Some(tile) = con.get_mut(&(pos.x,pos.y)) {
                if let Some(current_fg) = tile.fg {
                    if !tile.ignore_overlap {
                        tile.set_bg(current_fg);
                    }
                }

                tile.set_fg(appearance.color)
                    .set_c(appearance.sprite)
                    .set_ignore_overlap(appearance.ignore_overlap)
                    .set_layer(appearance.layer);
            }
        }
    }

    //draw highlights
    let mut x = 0;
    let mut y = 0;
    for color in resources.highlights.iter() {
        if let Some(color) = color {
            if let Some(tile) = con.get_mut(&(x,y)) {
                tile.set_bg(*color);
            }
        }
        x+=1;
        if x >= ARENA_WIDTH as i32 {
            y += 1;
            x = 0;
        }
    }

    //draw frame on a separate console
    let mut con_with_frame = AsciiConsole::new(ARENA_WIDTH+2,ARENA_HEIGHT+2,None);
    let mut set_frame = |pos,sprite:u32| {
        if let Some(tile) = con_with_frame.get_mut(&pos) {
            tile.set_fg(DARKGRAY);
            tile.set_c(sprite);
        }
    };

    set_frame((0,0),40);
    set_frame((ARENA_WIDTH as i32+1,0),41);
    set_frame((ARENA_WIDTH as i32+1,ARENA_HEIGHT as i32+1),42);
    set_frame((0,ARENA_HEIGHT as i32+1),43);

    for x in 1..=ARENA_WIDTH as i32 {
        set_frame((x,0),44);
        set_frame((x,ARENA_HEIGHT as i32+1),46);
    }
    for y in 1..=ARENA_HEIGHT as i32 {
        set_frame((0,y),47);
        set_frame((ARENA_WIDTH as i32+1,y),45);
    }

    //blit map onto frame console
    con.blit(&(1,1),&mut con_with_frame);

    con_with_frame
}

pub fn player(world: &World) -> Result<Entity,String> {
    for (ent,_) in world.query::<&Player>().iter() {
        return Ok(ent);
    }
    Err("No player found".to_owned())
}

pub fn draw_map(world:&World,resources:&Resources) {
    let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
    let con = world_to_console(world,resources);
    con.draw(
        &resources.sprite_sheet,
        &(SPRITE_WIDTH,SPRITE_HEIGHT),
        SPRITE_SHEET_COLUMNS,
        &(tile_size,tile_size),
        &(0.,0.),
    );
}

pub fn mouse_to_map() -> Option<(i32,i32)> {
    let abs_mouse_position = mouse_position();
    let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
    let tile_mouse_position = (
        (abs_mouse_position.0 / tile_size).floor() as i32-1,
        (abs_mouse_position.1 / tile_size).floor() as i32-1
    );
    
    if tile_mouse_position.0 > ARENA_WIDTH as i32+1 ||
        tile_mouse_position.1 > ARENA_HEIGHT as i32+1 {
            return None;
        }

    if tile_mouse_position.0 <= 0 ||
        tile_mouse_position.1 <= 0 {
            return None;
        }

    return Some(tile_mouse_position);
}

pub fn get_entities_at(world: &World, pos: (i32,i32)) -> Vec<Entity> {
    let mut entities = Vec::new();

    for (ent,ent_pos) in world.query::<&Pos>().iter() {
        if pos.0 == ent_pos.x && pos.1 == ent_pos.y {
            entities.push(ent);
        }
    }

    entities
}

pub fn draw_map_and_hud(world: &World, resources: &Resources) {
    clear_background(BLACK);
    draw_map(world,resources);
    draw_hud(world,resources);
}

pub fn clear_hightlights(resources:&mut Resources) {
    resources.highlights = vec![None;ARENA_WIDTH*ARENA_HEIGHT];
}


pub fn get_cloned<T: Clone + Component>(world: &World, entity: Entity) -> Option<T> {
    if let Ok(component) = world.get::<T>(entity) {
        Some((*component).clone())
    } else {
        None
    }
}

