use hecs::*;
use macroquad::color::*;
use crate::components::*;
use crate::combat::*;

#[derive(Copy,Clone,PartialEq,Debug)]
pub enum EntityKind {
    Player,

    //Terrain
    Floor,
    Door,
    Wall,
    Stairs,
    Exit,

    //enemies
    Rat,
    Bat,
    Zombie,
    Wizard,
    Centipede,
    Goblin,
    Gnome,

    //items
    Sword,
    Axe,
    Armor,
    ThrowingSpear,
    HealthPotion,
    MagicMapping,
    //Scroll,

}

pub fn spawn_at(world:&mut World, kind: EntityKind, pos:Pos) -> Entity {
    let ent = spawn(world,kind);
    world.insert_one(ent,pos).ok();
    ent
}

pub fn spawn(world:&mut World, kind:EntityKind) -> Entity {
    match kind {
        EntityKind::Player => {world.spawn((
            Player::new(),
            Appearance{
                sprite: 15,
                color: BLUE,
                layer: 10,
                ..Default::default()
            },
            Health::new(15),
            Name{
                name: "Player".to_owned(),
                description: "This is you".to_owned()
            },
            Equipment{
                weapon: None,
                armor: None,
            },
            Weapon {
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 1,
                    damage_high: 2,
                    to_hit: 0,
                    ..Default::default()

                }
            }
        ))},


        // Enemies
        EntityKind::Zombie => {world.spawn((
            OnLevel,
            Name{
                name: "Zombie".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 10,
                color: DARKGREEN,
                layer: 9,
                ..Default::default()
            },
            Health::new(5),
            Bump::Attack,
            Behavior::Slow(50),
            Weapon{
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 2,
                    damage_high: 4,
                    to_hit: -3,
                    ..Default::default()
                }
            },
            Defense {
                dodging:-8,
                armor:2,
            }
        )) },
        EntityKind::Wizard => {world.spawn((
            OnLevel,
            Name{
                name: "Wizard".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 13,
                color: DARKGREEN,
                layer: 9,
                ..Default::default()
            },
            Health::new(2),
            Bump::Attack,
            Behavior::Erratic(10),
            Weapon{
                attack: AttackData {
                    range: Range::Ranged(3),
                    damage_low: 1,
                    damage_high: 3,
                    to_hit: -2,
                    ..Default::default()
                }
            },
            Defense {
                dodging: -2,
                armor: 0,
            }
        )) },
        EntityKind::Centipede => {world.spawn((
            OnLevel,
            Name{
                name: "Centipede".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 14,
                color: YELLOW,
                layer: 9,
                ..Default::default()
            },
            Health::new(2),
            Bump::Attack,
            Behavior::Erratic(30),
            Weapon{
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 1,
                    damage_high: 2,
                    to_hit: 0,
                    ..Default::default()
                }
            },
            Defense {
                dodging: 0,
                armor: 1,
            }
        )) },
        EntityKind::Goblin => {world.spawn((
            OnLevel,
            Name{
                name: "Goblin".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 16,
                color: GREEN,
                layer: 9,
                ..Default::default()
            },
            Health::new(5),
            Bump::Attack,
            Behavior::Erratic(10),
            Weapon{
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 1,
                    damage_high: 2,
                    to_hit: 0,
                    ..Default::default()
                }
            },
            Defense {
                dodging: -3,
                armor: 2,
            }
        )) },
        EntityKind::Gnome => {world.spawn((
            OnLevel,
            Name{
                name: "Gnome".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 17,
                color: RED,
                layer: 9,
                ..Default::default()
            },
            Health::new(2),
            Bump::Attack,
            Behavior::Erratic(50),
            Weapon{
                attack: AttackData {
                    range: Range::Ranged(3),
                    damage_low: 1,
                    damage_high: 1,
                    to_hit: 4,
                    ..Default::default()
                }
            },
            Defense {
                dodging: 0,
                armor: 0,
            }
        )) },
        EntityKind::Bat => { world.spawn((
            OnLevel,
            Name{
                name: "Bat".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 12,
                color: GRAY,
                layer: 9,
                ..Default::default()
            },
            Health::new(1),
            Bump::Attack,
            Behavior::Erratic(50),
            Weapon{
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 1,
                    damage_high: 1,
                    to_hit: -1,
                    ..Default::default()
                }
            },
            Defense {
                dodging: 1,
                armor: 0
            },
        )) },
        EntityKind::Rat => { world.spawn((
            OnLevel,
            Name{
                name: "Rat".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 11,
                color: DARKBROWN,
                layer: 9,
                ..Default::default()
            },
            Health::new(1),
            Bump::Attack,
            Behavior::Erratic(20),
            Weapon{
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 1,
                    damage_high: 2,
                    to_hit: 0,
                    ..Default::default()
                }
            },
        )) },



        //items
        EntityKind::Sword => { world.spawn((
            OnLevel,
            Name{
                name: "Sword".to_owned(),
                description: "".to_owned()
            },
            Item,
            Appearance{
                sprite: 20,
                color: LIGHTGRAY,
                layer: 5,
                ..Default::default()
            },
            Equipable::Weapon,
            Weapon {
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 2,
                    damage_high: 3,
                    to_hit: 5,
                    ..Default::default()
                }
            }
        )) }
        EntityKind::Axe => { world.spawn((
            OnLevel,
            Name{
                name: "Axe".to_owned(),
                description: "".to_owned()
            },
            Item,
            Appearance{
                sprite: 21,
                color: LIGHTGRAY,
                layer: 5,
                ..Default::default()
            },
            Equipable::Weapon,
            Weapon {
                attack: AttackData {
                    range: Range::Meele,
                    damage_low: 1,
                    damage_high: 3,
                    to_hit: 2,
                    axe: true,
                    ..Default::default()
                }
            }
        )) }
        EntityKind::Armor => { world.spawn((
            OnLevel,
            Name{
                name: "Armor".to_owned(),
                description: "".to_owned()
            },
            Item,
            Appearance{
                sprite: 23,
                color: LIGHTGRAY,
                layer: 5,
                ..Default::default()
            },
            Equipable::Armor,
            Defense{
                dodging: -3,
                armor: 3,
            }
        )) }


        EntityKind::HealthPotion => { world.spawn((
            OnLevel,
            Name{
                name: "Health Potion".to_owned(),
                description: "".to_owned()
            },
            Item,
            Appearance{
                sprite: 30,
                color: RED,
                layer: 5,
                ..Default::default()
            },
            Useable::Heal,
        )) }
        EntityKind::ThrowingSpear => { world.spawn((
            OnLevel,
            Name{
                name: "Throwing Spear".to_owned(),
                description: "".to_owned()
            },
            Item,
            Appearance{
                sprite: 22,
                color: WHITE,
                layer: 5,
                ..Default::default()
            },
            Useable::Throw(AttackData {
                range: Range::Ranged(10),
                damage_low: 1,
                damage_high: 2,
                to_hit: 4,
                axe: false,
                ..Default::default()
            },false),
        )) }
        EntityKind::MagicMapping => { world.spawn((
            OnLevel,
            Name{
                name: "Scroll of Mapping".to_owned(),
                description: "".to_owned()
            },
            Item,
            Appearance{
                sprite: 31,
                color: WHITE,
                layer: 5,
                ..Default::default()
            },
            Useable::MagicMapping,
        )) }

        //Terrain
        EntityKind::Floor => { world.spawn((
            OnLevel,
            Appearance{
                sprite: 1,
                color: GRAY,
                layer: 1,
                ignore_overlap: true,
                ..Default::default()
            },
        ))}
        EntityKind::Door => { world.spawn((
            OnLevel,
            Name{
                name: "Door".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite:6,
                color: BROWN,
                layer:5,
                ..Default::default()
            },
            Bump::OpenDoor,
            BlocksSight,
        ))}

        EntityKind::Wall => { world.spawn((
            OnLevel,
            Name{
                name: "Wall".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 2,
                color: GRAY,
                layer: 5,
                ..Default::default()
            },
            Bump::BlocksMovement,
            BlocksSight,
        ))}

        EntityKind::Stairs => { world.spawn((
            OnLevel,
            Name{
                name: "Stairs".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 4,
                color: WHITE,
                layer: 5,
                ..Default::default()
            },
            Bump::NextLevel,
        ))}

        EntityKind::Exit => { world.spawn((
            OnLevel,
            Name{
                name: "Exit".to_owned(),
                description: "".to_owned()
            },
            Appearance{
                sprite: 8,
                color: WHITE,
                layer: 5,
                ..Default::default()
            },
            Bump::NextLevel,
        ))}
    }
}



