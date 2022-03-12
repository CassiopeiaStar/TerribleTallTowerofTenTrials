
use hecs::*;
use macroquad::prelude::*;
use crate::prelude::*;

pub struct ScreenMessage {
    pub pos: (f32,f32),
    pub msg: String,
    pub color: Color,
}

pub fn draw_animations(world: &World) {
    for (_,message) in world.query::<&ScreenMessage>().iter() {
        draw_text(
            message.msg.as_str(),
            message.pos.0,message.pos.1,
            30.,
            message.color
        );
    }

    for (_,animation) in world.query::<&RangedAttackAnimation>().iter() {
        draw_line(
            animation.start.0,
            animation.start.1,
            animation.finish.0,
            animation.finish.1,
            3.,animation.color
        );
    }
}

pub fn animation_system(world: &mut World){
    let mut messages_to_despawn = Vec::new();
    for (ent,message) in world.query::<&mut ScreenMessage>().iter() {
        message.pos.1 -= 0.3;
        message.color.a -= 0.01;
        if message.color.a <= 0. {
            messages_to_despawn.push(ent);
        }
    }

    for (ent,ranged_animation) in world.query::<&mut RangedAttackAnimation>().iter() {
        ranged_animation.color.a -= 0.03;
        if ranged_animation.color.a <= 0. {
            messages_to_despawn.push(ent);
        }
    }

    messages_to_despawn.iter().for_each(|&e|{world.despawn(e).ok();});

}

pub fn emit_message(world: &mut World, tile: (i32,i32), msg: String,color: Color) {
    let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
    world.spawn((
        ScreenMessage {
            pos: ((tile.0+1) as f32*tile_size,(tile.1+2) as f32*tile_size),
            msg: msg.clone(),
            color,
        },
    ));

}

pub struct RangedAttackAnimation {
    pub start: (f32,f32),
    pub finish: (f32,f32),
    pub color: Color,
}

pub fn emit_ranged_attack_animation(
    world: &mut World, 
    start_tile:(i32,i32),
    finish_tile:(i32,i32),
    color: Color
) {
    let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
    let start = ((start_tile.0 as f32+1.5) *tile_size,(start_tile.1 as f32+1.5) *tile_size);
    let finish = ((finish_tile.0 as f32+1.5) *tile_size,(finish_tile.1 as f32+1.5) *tile_size);
    world.spawn((RangedAttackAnimation{start,finish,color},));
}
