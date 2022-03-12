
use std::collections::{HashMap,HashSet};
use macroquad::prelude::*;
use hecs::*;
use crate::prelude::*;

#[derive(Copy,Clone)]
pub enum Dir {
    N,W,S,E
}

impl Dir {
    pub fn components(&self) -> (i32,i32) {
        match self {
            Dir::N => (0,-1),
            Dir::W => (-1,0),
            Dir::S => (0,1),
            Dir::E => (1,0),

        }
    }
}

pub fn distance_between_ents(world:&World,start:Entity,finish:Entity) -> Option<i32> {
    use pathfinding::prelude::absdiff;
    let start_pos = get_cloned::<Pos>(world,start)?;
    let finish_pos = get_cloned::<Pos>(world,finish)?;

    Some(absdiff(start_pos.x,finish_pos.y) + absdiff(start_pos.y,finish_pos.y))
}

pub fn movement_map(world: &World) -> HashMap<(i32,i32),(Entity,Bump)> {
    let mut map : HashMap<(i32,i32),(Entity,Bump)> = HashMap::new();

    for (entity,(
        pos,
        bump
    )) in world.query::<(
        &Pos,
        &Bump
    )>().iter() {
        map.insert((*pos).into(),(entity,bump.clone()));
    }
    map
}

pub fn stepping_tiles(world: &World) -> HashSet<(i32,i32)> {
    let mut map : HashSet<(i32,i32)> = HashSet::new();

    for (_,(
        pos,
        _
    )) in world.query::<(
        &Pos,
        &Bump
    )>().iter() {
        map.insert((*pos).into());
    }

    for (_,(
        pos,
        _
    )) in world.query::<(
        &Pos,
        &Player
    )>().iter() {
        map.insert((*pos).into());
    }
    map
}

pub mod my_pathfinding {
    use pathfinding::prelude::{absdiff,astar};
    use crate::components::Pos;
    use hecs::*;
    use crate::movement::stepping_tiles;
    use std::collections::HashSet;

    pub fn basic_path(world: &World, actor:Entity, target:Entity) -> Option<(Vec<(i32,i32)>,u32)> {
        let mut actor_pos = None;
        let mut target_pos = None;
        for (ent,pos) in world.query::<&Pos>().iter() {
            if ent == actor {
                actor_pos.replace((pos.x,pos.y));
            }
            if ent == target {
                target_pos.replace((pos.x,pos.y));
            }
        }

        let m_map = stepping_tiles(world);

        let actor_pos = actor_pos?;
        let target_pos = target_pos?;

        path(m_map,actor_pos,target_pos)
    }

    pub fn path(
        mut move_map: HashSet<(i32,i32)>, 
        start: (i32,i32), 
        goal: (i32,i32)
    ) -> Option<(Vec<(i32,i32)>,u32)>{
        move_map.remove(&start);
        move_map.remove(&goal);
        

        //allows for the player to move in a 
        let tall = {
            let dx = absdiff(start.0,goal.0);
            let dy = absdiff(start.1,goal.1);

            dy>dx
        };

        let successors= |pos: &(i32,i32)| -> Vec<((i32,i32),u32)> {
            let &(x,y) = pos;
            let vec = if tall {
                vec![(x,y-1),(x,y+1),(x-1,y),(x+1,y)]
            } else {
                vec![(x-1,y),(x+1,y),(x,y-1),(x,y+1)]
            };
            vec
                .into_iter()
                .filter(|p|{!move_map.contains(&p)})
                .map(|p|(p,1))
                .collect()

        };

        let distance = |pos: &(i32,i32)| -> u32 {
            (absdiff(pos.0,goal.0) + absdiff(pos.1,goal.1)) as u32
        };

        let success = |pos: &(i32,i32)| -> bool {
            *pos == goal
        };

        astar(&start,successors,distance,success)
    }
}
