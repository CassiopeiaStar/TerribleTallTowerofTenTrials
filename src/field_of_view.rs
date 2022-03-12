use std::collections::HashSet;
use hecs::*;
use crate::prelude::*;
use bresenham::Bresenham;

pub fn update_fov(world: &mut World,resources: &mut Resources,magic_mapping: bool) {
    let player = player(world).unwrap();
    let positions_in_fov = if let Ok(pos) = world.get::<Pos>(player) {
        fov(world,(pos.x,pos.y),FOV_DISTANCE,magic_mapping)
    } else {
        HashSet::new()
    };

    resources.fov_set = positions_in_fov.clone();

    let mut new_memories = Vec::new();
    let mut dead_memories = Vec::new();
    for (ent,(pos,mut appearance,memory)) in world.query::<(&Pos,&mut Appearance,Option<&PlayerMemory>)>().iter() {
        if positions_in_fov.contains(&(pos.x,pos.y)) {
            if memory.is_some() {
                dead_memories.push(ent.clone());
            }
            appearance.in_fov = true;
        } else {
            if appearance.in_fov {
                //create memory
                let mut appearance = appearance.clone();
                appearance.in_fov = false;
                
                new_memories.push((
                    PlayerMemory,
                    pos.clone(),
                    appearance,
                    OnLevel,

                ));
            }
            appearance.in_fov = false;
        }
    }
    dead_memories.iter().for_each(|&e|{world.despawn(e).ok();});
    world.spawn_batch(new_memories);
}


fn block_map(world: &World) -> HashSet<(i32,i32)> {
    let mut set = HashSet::new();
    for (_,(pos,_)) in world.query::<(&Pos,&BlocksSight)>().iter() {
        set.insert((pos.x,pos.y));
    }
    set
}

fn line_blocked(block_map: &HashSet<(i32,i32)>, a: (i32,i32), b: (i32,i32)) -> bool {
    let a = (a.0 as isize,a.1 as isize);
    let b = (b.0 as isize,b.1 as isize);

    for (x,y) in Bresenham::new(a,b) {
        if (x,y) != a && (x,y) != b && block_map.contains(&(x as i32,y as i32)) {
            return true;
        }
    }

    false
}

fn fov(world: &World, origin: (i32,i32), distance: u32, magic_mapping: bool ) -> HashSet<(i32,i32)> {
    let debug = magic_mapping || DEBUG_FOV;
    let mut set = HashSet::new();
    let distance = distance as i32;
    let left = origin.0-distance;
    let right = origin.0+distance+1;
    let top = origin.1-distance;
    let bot = origin.1+distance+1;
    let block_map = block_map(world);
    for x in left..right {
        for y in top..bot {
            if debug || !line_blocked(&block_map,origin,(x,y)) {
                set.insert((x,y));
            }
        }
    }
    set
}
