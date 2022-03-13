use crate::factory::EntityKind;
use lazy_static::lazy_static;
use crate::dungeon_gen::DungeonTemplate;


lazy_static! {
    pub static ref ENEMIES: Vec<EntityKind> = vec![
        EntityKind::Bat,
        EntityKind::Rat,
        EntityKind::Centipede,
        EntityKind::Zombie,
        EntityKind::Gnome,
        EntityKind::Goblin,
        EntityKind::Wizard,
    ];

    pub static ref ITEMS: Vec<EntityKind> = vec![
        EntityKind::HealthPotion,
        EntityKind::Sword,
        EntityKind::Axe,
        EntityKind::Armor,
        EntityKind::ThrowingSpear,
        EntityKind::MagicMapping,
    ];

    pub static ref DUNGEON_TEMPLATES: Vec<DungeonTemplate> = vec![
        DungeonTemplate {
            entrance: 3,
            exit: 4,
            treasure_rooms: vec![2],
            danger_rooms: vec![8],
            doors: [
              1,1,
             1,0,1,
              0,1,
             1,0,1,
              1,1,
            ],
        },
        DungeonTemplate {
            entrance: 0,
            exit: 7,
            treasure_rooms: vec![8],
            danger_rooms: vec![8],
            doors: [
              1,1,
             0,0,1,
              1,1,
             1,0,0,
              1,1,
            ],
        },
        DungeonTemplate {
            entrance: 4,
            exit: 0,
            treasure_rooms: vec![8],
            danger_rooms: vec![],
            doors: [
              1,1,
             0,1,1,
              1,1,
             0,1,0,
              1,1,
            ],
        },
        DungeonTemplate {
            entrance: 4,
            exit: 5,
            treasure_rooms: vec![7],
            danger_rooms: vec![8],
            doors: [
              1,1,
             1,0,1,
              1,0,
             1,0,0,
              1,1,
            ],
        },
        DungeonTemplate {
            entrance: 4,
            exit: 5,
            treasure_rooms: vec![2],
            danger_rooms: vec![6],
            doors: [
              0,1,
             1,1,0,
              1,0,
             1,1,1,
              0,1,
            ],
        },
        DungeonTemplate {
            entrance: 0,
            exit: 2,
            treasure_rooms: vec![8],
            danger_rooms: vec![7],
            doors: [
              0,0,
             1,1,1,
              1,1,
             1,1,1,
              1,0,
            ],
        },


    ];

}
