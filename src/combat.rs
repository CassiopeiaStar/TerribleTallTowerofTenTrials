use hecs::*;
use macroquad::rand::*;
use macroquad::color::*;

use crate::prelude::*;


#[derive(Clone,Debug)]
pub enum Range {
    Meele,
    Ranged(i32),
}

impl Range {
    pub fn in_range(&self,world: &World,actor:Entity,target:Entity)->Option<bool> {
        let actor_pos = get_cloned::<Pos>(world,actor)?;
        let target_pos = get_cloned::<Pos>(world,target)?;
        match self {
            Range::Meele => {
                use pathfinding::prelude::absdiff;
                Some(
                    absdiff(actor_pos.x,target_pos.x) +
                    absdiff(actor_pos.y,target_pos.y)
                    ==1
                )
            }
            Range::Ranged(range) => {
                let (_,dist) = unblocked_attack_line(world,
                    actor_pos.into(),target_pos.into())?;
                Some(dist as i32<= *range)
            }
        }
    }
}

#[derive(Clone,Debug)]
pub struct AttackData {
    pub range: Range,
    pub damage_low: i32,
    pub damage_high: i32,
    pub to_hit: i32,
    pub axe: bool,
}

impl Default for AttackData {
    fn default() -> Self {
        Self {
            range: Range::Meele,
            damage_low: 1,
            damage_high: 1,
            to_hit: 0,
            axe: false,
        }
    }
}

pub fn attack(
    world: &mut World,
    attacker: Entity,
    target:Entity,
    attack:AttackData,
) {
    //hackish way of getting a clone or default out of this convoluted world::get method
    let defense = get_defense(world,target);

    let hit: bool = {
        let hit_roll = gen_range(0,20)+ attack.to_hit;
        hit_roll > defense.dodging + 10
    };
    let target_pos = get_cloned::<Pos>(world,target).unwrap();
    let attacker_pos = get_cloned::<Pos>(world,attacker).unwrap();
    let attacker_appearance = get_cloned::<Appearance>(world,attacker).unwrap();

    if hit {
        let armor_roll = gen_range(0,defense.armor+1);
        let damage = (
            gen_range(attack.damage_low,attack.damage_high+1)-
            armor_roll
        ).max(0);
        if let Ok(mut health) = world.get_mut::<Health>(target) {
            health.current -= damage;
        }
        
        let player = player(world).unwrap();
        let message_color = if player == target {
            RED
        } else {
            GREEN
        };

        emit_message(world,target_pos.into(),format!("{}",damage),message_color);
        emit_ranged_attack_animation(world,attacker_pos.into(),target_pos.into(),attacker_appearance.color);

    } else {
        let player = player(world).unwrap();
        emit_message(world,target_pos.into(),"Miss".to_owned(),YELLOW);
        emit_ranged_attack_animation(world,attacker_pos.into(),target_pos.into(),YELLOW);
    }

    remove_dead_entities(world);
}

pub fn remove_dead_entities(
    world: &mut World
) {
    let player = player(world).unwrap();
    let mut entities_to_remove = Vec::new();
    for (ent,health) in world.query::<&Health>().iter() {
        if health.current <= 0 {
            entities_to_remove.push(ent);
        }
    }

    entities_to_remove.iter().for_each(|ent|{
        if *ent != player {
            world.despawn(*ent).ok();

        }
    });
}

fn get_defense(world: &World, entity:Entity) -> Defense {
    let mut base_defense = get_cloned::<Defense>(world,entity).unwrap_or_default();

    if let Some(equipment) = get_cloned::<Equipment>(world,entity) {
        if let Some(armor) = equipment.armor {
            if let Some(defense) = get_cloned::<Defense>(world,armor) {
                base_defense.dodging += defense.dodging;
                base_defense.armor += defense.armor;
            }
        }
    }
    base_defense
}

pub fn get_attack(world: &World, entity:Entity) -> Option<AttackData> {

    //if wielding a weapon use that
    if let Some(equipment) = get_cloned::<Equipment>(world,entity) {
        if let Some(weapon_ent) = equipment.weapon {
            if let Some(weapon) = get_cloned::<Weapon>(world,weapon_ent) {
                return Some(weapon.attack);
            }
        }
    }

    //if not, check for a natural weapon
    if let Some(weapon) = get_cloned::<Weapon>(world,entity) {
        return Some(weapon.attack);
    }

    None
}

pub fn attack_line(a:(i32,i32),b:(i32,i32))->(Vec<(i32,i32)>,u32) {
    use bresenham::Bresenham;
    let asize = (a.0 as isize,a.1 as isize);
    let bsize = (b.0 as isize,b.1 as isize);
    let path: Vec<(i32,i32)> = Bresenham::new(asize,bsize)
        .map(|(x,y)|(x as i32,y as i32)).skip(1).collect();

    let dist = path.len() as u32;

    (path,dist)
}

pub fn unblocked_attack_line(world:&World,a:(i32,i32),b:(i32,i32))
    ->Option<(Vec<(i32,i32)>,u32)> {
        use std::collections::HashSet;
        let (path,dist) = attack_line(a,b);

        if path.len() > 0 {
            let path_set: HashSet<(i32,i32)> = path.iter()
                .map(|p|*p).collect();

            for (_,(pos,bump)) in world.query::<(&Pos,&Bump)>().iter() {
                if path_set.contains(&(pos.x,pos.y)) {
                    match bump {
                        Bump::BlocksMovement => {
                            return None;
                        }
                        Bump::OpenDoor => {
                            return None;
                        }
                        Bump::Attack | Bump::NextLevel => {}
                    }
                }
            }
        }

        return Some((path,dist))
}
