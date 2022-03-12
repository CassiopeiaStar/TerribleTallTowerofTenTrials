use macroquad::prelude::*;
use hecs::*;
use crate::prelude::*;
use crate::combat::*;
use crate::states::inventory::inventory_state;
use super::{GameState,StateChange};
use crate::monster_memory::memory_system;
use crate::dungeon_gen::*;

pub async fn game(
    world: &mut World,
    resources: &mut Resources,
) -> StateChange {
    loop {
        if resources.level == 0 ||
            resources.new_level_request
            //is_key_pressed(KeyCode::P) 
        {
            new_level(world,resources);
            resources.new_level_request = false;
        }

        /*
        if is_key_pressed(KeyCode::M) {
            emit_ranged_attack_animation(world,(0,0),(5,0),WHITE);
            dbg!();
        }
        */
        update_fov(world,resources);
        let actions = player_input(world,resources).await;
        if player_actions(world,resources,actions) {
            memory_system(world,resources);
            //moster actions
            let mut behaviors = Vec::new();
            for (ent,behavior) in world.query::<&Behavior>().iter() {
                behaviors.push((ent,behavior.clone()));
            }
            for (actor,behavior) in behaviors.iter() {
                update_fov(world,resources);
                act(world,resources,*actor,*behavior);
            }

        }

        let player = player(world).unwrap();
        if let Ok(health) = world.get::<Health>(player) {
            if health.current <= 0 {
                return StateChange::Replace(GameState::GameOver);
            }
        }



        update_fov(world,resources);
        highlight_mouse_movement(world,resources);
        draw_map_and_hud(world,resources);
        animation_system(world);
        draw_animations(world);
        clear_hightlights(resources);
        next_frame().await
    }
}

fn reset_game(world:&mut World,resources: &mut Resources) {
    world.clear();
    resources.level = 0;
    use crate::resources::PlayerData;
    resources.player = PlayerData::new();
}

fn new_level(world:&mut World,resources: &mut Resources) {
    resources.level += 1;
    use macroquad::rand::*;
    
    //remove old level
    let entities_to_despawn: Vec<Entity> = world.query::<&OnLevel>().iter()
        .map(|(e,_)|e.clone()).collect();

    entities_to_despawn.iter().for_each(|&e|{world.despawn(e).ok();});

    //create new level
    let mut template = DUNGEON_TEMPLATES.choose().unwrap().clone();
    if gen_range(0,2) as u32 == 0 {
        template.transpose();
    }
    for _ in 0..gen_range(0,4) as u32 {
        template.rotate();
    }
    let map = QuadrantMap::new(resources.level,template).build();

    //load new level to world
    let pos = map.load_to_world(world).unwrap_or((10,10));

    if let Ok(player_id) = player(world) {
        if let Ok(mut player_pos) = world.get_mut::<Pos>(player_id) {
            player_pos.x = pos.0 as i32;
            player_pos.y = pos.1 as i32;
        } 
    } else {
        spawn_at(world,EntityKind::Player,Pos::new(pos.0 as i32,pos.1 as i32));
    }
}


#[derive(Copy,Clone)]
pub enum PlayerAction {
    TryWalk(Dir),
    PickUpItem(Entity),
    DropItem(Entity),
    EquipItem(Entity),
    UseItem(Entity),
    Wait,
}

async fn player_input (
    world: &World,
    resources: &Resources,
) -> Vec<PlayerAction> {
    let player = player(world).unwrap();
    let player_position = get_cloned::<Pos>(world,player).unwrap();

    let mut actions: Vec<PlayerAction> = Vec::new();
    if is_key_pressed(KeyCode::W) {
        actions.push(PlayerAction::TryWalk(Dir::N));
    }
    if is_key_pressed(KeyCode::A) {
        actions.push(PlayerAction::TryWalk(Dir::W));
    }
    if is_key_pressed(KeyCode::S) {
        actions.push(PlayerAction::TryWalk(Dir::S));
    }
    if is_key_pressed(KeyCode::D) {
        actions.push(PlayerAction::TryWalk(Dir::E));
    }
    if is_key_pressed(KeyCode::Period) {
        actions.push(PlayerAction::Wait);
    }
    if is_key_pressed(KeyCode::Comma) || is_key_pressed(KeyCode::G) {
        let player_position = {
            let player_position = world.get::<Pos>(player).unwrap();
            Pos::new(player_position.x,player_position.y)
        };
        //actions.push(PlayerAction::PickUp);
        //let mut items_under_player = Vec::new();
        for (ent,(pos,_)) in world.query::<(&Pos,&Item)>().iter(){
            if player_position.x == pos.x && player_position.y == pos.y {
                //items_under_player.push(ent);
                actions.push(PlayerAction::PickUpItem(ent));
            }
        }
    }

    if is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::Tab) {
        let mut returned_actions = inventory_state(world,resources).await;
        actions.append(&mut returned_actions);
    }

    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = mouse_to_map() {
            if resources.fov_set.contains(&mouse_pos) {
                use crate::movement::my_pathfinding::path;
                let player_position = {
                    let player_position = world.get::<Pos>(player).unwrap();
                    (player_position.x,player_position.y)
                };

                let mov_map = stepping_tiles(world);

                if let Some(p) = path(mov_map,player_position,mouse_pos) {
                    if p.0.len() > 1 {
                        let diff = (p.0[1].0-p.0[0].0,p.0[1].1-p.0[0].1);
                        if diff.0 == -1 {actions.push(PlayerAction::TryWalk(Dir::W))}
                        if diff.0 ==  1 {actions.push(PlayerAction::TryWalk(Dir::E))}
                        if diff.1 == -1 {actions.push(PlayerAction::TryWalk(Dir::N))}
                        if diff.1 ==  1 {actions.push(PlayerAction::TryWalk(Dir::S))}
                    }
                    if p.0.len() == 1 {
                        actions.push(PlayerAction::Wait);
                    }
                }
            }
        }
    }
    
    if is_mouse_button_pressed(MouseButton::Right) {
        if let Some(mouse_pos) = mouse_to_map() {

            if mouse_pos.0 == player_position.x && mouse_pos.1 == player_position.y {
                for (ent,(pos,_)) in world.query::<(&Pos,&Item)>().iter(){
                    if player_position.x == pos.x && player_position.y == pos.y {
                        //items_under_player.push(ent);
                        actions.push(PlayerAction::PickUpItem(ent));
                    }
                }
            }


        }
    }

    actions
}

fn highlight_mouse_movement(world: &mut World,resources: &mut Resources) {
    if let Some(mouse_pos) = mouse_to_map() {
        if resources.fov_set.contains(&mouse_pos) {
            use crate::movement::my_pathfinding::path;
            let player = player(world).unwrap();
            let player_position = {
                let player_position = world.get::<Pos>(player).unwrap();
                (player_position.x,player_position.y)
            };

            let mov_map = stepping_tiles(world);

            if let Some(p) = path(mov_map,player_position,mouse_pos) {
                
                
                if p.0.len() > 1 {
                    for (i,tile) in p.0.iter().enumerate() {
                        match i {
                            0 => {}
                            1 => {
                                let index = (tile.0+(tile.1*ARENA_WIDTH as i32)) as usize;
                                if index < resources.highlights.len() {
                                    resources.highlights[index] = Some(WHITE);
                                }
                            }
                            _ => {
                                //resources.highlights[(tile.0+(tile.1*ARENA_WIDTH as i32)) as usize] = Some(LIGHTGRAY);
                            }
                        }
                    }
                }
                if p.0.len() == 1 {
                    resources.highlights[(player_position.0 + (player_position.1*ARENA_WIDTH as i32)) as usize] = Some(WHITE);
                }
            }
        }
    }
}

fn player_actions(world: &mut World, resources: &mut Resources, actions: Vec<PlayerAction>) -> bool {
    let mut action_taken = false;
    let player = player(world).unwrap();
    let player_position = {
        let player_position = world.get::<Pos>(player).unwrap();
        Pos::new(player_position.x,player_position.y)
    };
    for action in actions {
        match action {
            PlayerAction::Wait => {
                action_taken = true;
            }
            PlayerAction::TryWalk(dir) => {
                let mut bumped: Option<(Entity,Bump)> = None;
                let map = movement_map(world);
                for (_,(_,mut pos)) in world.query::<(&Player,&mut Pos)>().iter() {
                    let (dx,dy) = dir.components();
                    let destination = (pos.x + dx,pos.y + dy);
                    if let Some((ent,bump)) = map.get(&destination) {
                        bumped.replace((*ent,*bump));
                    } else {
                        pos.x = destination.0;
                        pos.y = destination.1;
                        action_taken = true;
                    }
                }

                if let Some((ent,bump)) = bumped {
                    match bump {
                        Bump::OpenDoor => {
                            world.remove_one::<Bump>(ent).ok();
                            world.remove_one::<BlocksSight>(ent).ok();
                            if let Ok(mut appearance) = world.get_mut::<Appearance>(ent) {
                                appearance.sprite = 7;
                            }
                            action_taken = true;
                        }
                        Bump::Attack => {
                            let mut attacks_list: Vec<(Entity,AttackData)> = Vec::new();
                            if let Some(attack_data) = get_attack(world,player) {
                                if attack_data.axe {
                                    let adjacent_tiles: Vec<(i32,i32)> = 
                                    [(-1,-1),(0,-1,),(1,-1),
                                     (-1,0),         (1,0),
                                     (-1,1), (0,1),  (1,1)].iter().map(|diff|{
                                         (player_position.x+diff.0,
                                         player_position.y+diff.1)
                                    }).collect();

                                    for tile in adjacent_tiles {
                                        if let Some((ent,bump)) = map.get(&tile) {
                                            match bump {
                                                Bump::Attack => {
                                                    attacks_list.push((*ent,attack_data.clone()));
                                                },
                                                _ => {}
                                            }
                                        }
                                    }

                                } else {
                                    attacks_list.push((ent,attack_data));
                                }
                                action_taken = true;
                            }
                            for (ent,attack_data) in attacks_list {
                                attack(world,player,ent,attack_data);
                            }
                        }
                        Bump::NextLevel => {
                            resources.new_level_request = true;
                        }
                        _ => {}
                    }
                }
            }
            PlayerAction::PickUpItem(ent) => {
                world.remove_one::<Pos>(ent).ok();
                world.remove_one::<OnLevel>(ent).ok();
                resources.player.inventory.push(ent);
                action_taken = true;
            }
            PlayerAction::DropItem(ent) => {
                world.insert_one(ent,player_position.clone()).ok();
                world.insert_one(ent,OnLevel).ok();
                let mut items_to_remove = Vec::new();
                for (index,item) in resources.player.inventory.iter().enumerate() {
                    if *item == ent {
                        items_to_remove.push(index);
                    }
                }
                for index in items_to_remove {
                    resources.player.inventory.remove(index);
                }
                action_taken = true;
            }
            PlayerAction::EquipItem(ent) => {
                let mut equipable = None;
                if let Ok(item) = world.get::<Equipable>(ent) {
                    equipable.replace((*item).clone());
                }
                if let Some(equipable) = equipable {
                    if let Ok(mut player_equipment) = world.get_mut::<Equipment>(player) {
                        match equipable {
                            Equipable::Armor => {
                                if let Some(currently_equiped) = player_equipment.armor {
                                    resources.player.inventory.push(currently_equiped);
                                }
                                player_equipment.armor.replace(ent);
                            },
                            Equipable::Weapon => {
                                if let Some(currently_equiped) = player_equipment.weapon {
                                    resources.player.inventory.push(currently_equiped);
                                }
                                player_equipment.weapon.replace(ent);

                            }
                        }
                        let mut index_to_remove = Vec::new();
                        for (i,e) in resources.player.inventory.iter().enumerate() {
                            if *e == ent {
                                index_to_remove.push(i);
                            }
                        }
                        for i in index_to_remove {
                            resources.player.inventory.remove(i);
                        }
                    }
                }
                action_taken = true;
            }
            PlayerAction::UseItem(ent) => {
                let useable: Option<Useable> = get_cloned(world,ent);
                if let Some(useable) = useable {
                    match useable {
                        Useable::Heal => {
                            if let Ok(mut health) = world.get_mut::<Health>(player) {
                                health.current = health.max;
                                action_taken = true;
                            }
                        }
                    }
                    let mut index_to_remove = Vec::new();
                    for (i,e) in resources.player.inventory.iter().enumerate() {
                        if *e == ent {
                            index_to_remove.push(i);
                        }
                    }
                    for i in index_to_remove {
                        resources.player.inventory.remove(i);
                    }
                }
            }
        }
    }

    action_taken
}
