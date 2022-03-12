use macroquad::color::*;
use hecs::*;
use crate::combat::AttackData;

pub struct Armor {
    pub defense: i32,
}

#[derive(Clone)]
pub struct Appearance {
    pub sprite: u32,
    pub color: Color,
    pub layer: u32,
    pub ignore_overlap: bool,
    pub in_fov: bool,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            sprite: 0,
            color: WHITE,
            layer: 1,
            ignore_overlap: false,
            in_fov: false,
        }
    }
}

#[derive(Copy,Clone)]
pub enum Behavior {
    ApproachAndAttack,
    Erratic(u32),
    Slow(u32),
}

#[derive(Copy,Clone)]
pub struct BlocksSight;

#[derive(Copy,Clone,Debug)]
pub enum Bump {
    BlocksMovement,
    OpenDoor,
    Attack,
    NextLevel
}

#[derive(Debug,Default,Clone)]
pub struct Defense {
    pub dodging: i32,
    pub armor: i32,
}

#[derive(Debug,Clone)]
pub struct Equipment {
    pub weapon: Option<Entity>,
    pub armor: Option<Entity>
}

#[derive(Copy,Clone)]
pub enum Equipable {
    Weapon,
    Armor
}

#[derive(Copy,Clone)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Health {
            max,
            current: max,
        }
    }
}

#[derive(Copy,Clone)]
pub struct Item;


#[derive(Copy,Clone)]
pub struct OnLevel;

pub struct MonsterMemory {
    pub time_to_remember: i32,
    pub strength: i32,
}

#[derive(Clone)]
pub struct Name {
    pub name: String,
    pub description: String,
}

#[derive(Copy,Clone)]
pub struct PlayerMemory;

#[derive(Clone)]
pub struct Player {
    //pub inventory: Vec<Entity>,
}

impl Player {
    pub fn new() -> Self {
        Self{
            //inventory: Vec::new(),
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Pos{
    pub x: i32,
    pub y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self{
        Self {
            x,y
        }
    }
}

impl From<(i32,i32)> for Pos {
    fn from(i: (i32,i32)) -> Pos {
        Pos{
            x: i.0,
            y: i.1
        }
    }
}

impl Into<(i32,i32)> for Pos {
    fn into(self) -> (i32,i32) {
        (self.x,self.y)
    }
}

#[derive(Copy,Clone,Debug)]
pub enum Useable {
    //potions
    Heal,
    //Speed,
    //Strength,

    //scrolls
    //Fireball,
    //Magic Mapping

}

#[derive(Clone,Debug)]
pub struct Weapon {
    pub attack: AttackData,
}
