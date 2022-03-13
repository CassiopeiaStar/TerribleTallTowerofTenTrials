use hecs::*;
use macroquad::rand::*;
use crate::prelude::*;
use std::collections::HashSet;

pub struct DungeonMap {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<EntityKind>,
    pub items: Vec<Vec<EntityKind>>,
    pub actors: Vec<Option<EntityKind>>,
    pub player_pos: Option<(usize,usize)>,
}

impl DungeonMap {
    pub fn new(width: usize, height: usize, default_terrain: EntityKind) -> Self {
        Self {
            width,height,
            terrain: vec![default_terrain; width*height],
            items: vec![vec![];width*height],
            actors: vec![None;width*height],
            player_pos: None,
        }
    }

    pub fn load_to_world(&self,world: &mut World) -> Option<(usize,usize)> {
        let mut x = 0;
        let mut y = 0;
        for kind in self.terrain.iter() {
            spawn_at(world,*kind,(x,y).into());
            x+=1;
            if x >= self.width as i32 {
                x = 0;
                y+=1;
            }
        }

        let mut x = 0;
        let mut y = 0;
        for items in self.items.iter() {
            for kind in items {
                spawn_at(world,*kind,(x,y).into());
            }
            x+=1;
            if x >= self.width as i32 {
                x = 0;
                y+=1;
            }
        }

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for kind in self.actors.iter() {
            let mut at_player = false;
            if let Some(player_pos) = self.player_pos {
                if player_pos.0 as i32== x && player_pos.1 as i32== y {
                    at_player = true;
                }
            }
            if !at_player {
                if let Some(kind) = kind {
                    spawn_at(world,*kind,(x,y).into());
                }
            }
            x+=1;
            if x >= self.width as i32 {
                x = 0;
                y+=1;
            }
        }

        self.player_pos
    }

    pub fn index_as_pos(&self, index: usize) -> Result<(usize,usize),String> {
        if index > self.terrain.len() {
            return Err(format!("index out of range: {:?}",index));
        }
        let y = index / self.width;
        let x = index % self.width;
        Ok((x,y))
    }

    pub fn get_index(&self, pos: (usize,usize)) -> Result<usize,String> {
        if pos.0>=self.width||pos.1>=self.height {
            return Err(format!("OOB index at: {:?}",pos));
        }
        Ok(pos.0+(self.width*pos.1))
    }

    pub fn is_blocked(&self,pos: (usize,usize)) -> bool {
        if let Ok(index) = self.get_index(pos) {
            if self.terrain[index] != EntityKind::Floor {
                return false;
            } else if self.actors[index] != None {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }

    pub fn fill_rect(&mut self, kind: EntityKind, pos:(usize,usize), size:(usize,usize)) -> Result<(),String> {
        for index in self.rect(pos,size)? {
            self.terrain[index] = kind;
        }
        Ok(())
    }

    pub fn rect(&self, pos: (usize,usize), size: (usize,usize)) -> Result<Vec<usize>,String>{
        let mut indexes = Vec::new();
        for x in pos.0..(pos.0+size.0) {
            for y in pos.1..(pos.1+size.1) {
                let index = self.get_index((x,y))?;
                indexes.push(index);
            }
        }
        Ok(indexes)
    }

    pub fn dig_room(&mut self,pos:(usize,usize),size:(usize,usize))->Result<(),String>{
        self.fill_rect(EntityKind::Wall,pos,size)?;

        let inner_pos = (pos.0+1,pos.1+1);
        let inner_size = (size.0-2,size.1-2);
        self.fill_rect(EntityKind::Floor,inner_pos,inner_size)?;
        
        let actors = vec![
            EntityKind::Player,
            EntityKind::Rat,
        ];
        self.fill_room_with_actors(inner_pos,inner_size,actors)?;
        self.fill_room_with_items(inner_pos,inner_size,vec![
            EntityKind::Sword,
            EntityKind::Sword,
            EntityKind::Sword,
        ])?;

        Ok(())
    }

    pub fn fill_room_with_actors(
        &mut self,
        pos:(usize,usize),
        size:(usize,usize),
        mut actors: Vec<EntityKind>
    ) -> Result<(),String> {
        let mut rect_indexes = self.rect(pos,size)?;
        rect_indexes.shuffle();
        for actor_pos in rect_indexes.choose_multiple(actors.len()) {
            self.actors[*actor_pos].replace(actors.pop().unwrap());
        }
        Ok(())
    }

    pub fn fill_room_with_items(
        &mut self,
        pos:(usize,usize),
        size:(usize,usize),
        mut items: Vec<EntityKind>
    ) -> Result<(),String> {
        let mut rect_indexes = self.rect(pos,size)?;
        rect_indexes.shuffle();
        for item_pos in rect_indexes.choose_multiple(items.len()) {
            self.items[*item_pos].push(items.pop().unwrap());
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Room {
    pub pos: (usize,usize),
    pub size: (usize,usize),
    pub danger: bool,
    pub loot: bool,
    pub player: bool,
    pub exit: bool,
}

impl Room {
    fn rand_tile_within(&self) -> (usize,usize) {
        let (x,y) = self.pos;
        let (w,h) = self.size;
        (gen_range(x,x+w),gen_range(y,y+h))
    }

    fn tiles_within(&self) -> Vec<(usize,usize)> {
        let (room_x,room_y) = self.pos;
        let (w,h) = self.size;
        let mut vec = Vec::new();
        for x in room_x..(room_x+w) {
            for y in room_y..(room_y+h) {
                vec.push((x,y));
            }
        }
        vec
    }

    fn is_border(&self,tile: (usize,usize)) -> bool {
        //top
        if tile.0 < self.pos.0 + self.size.0 &&
            tile.0 >= self.pos.0 &&
            tile.1 == self.pos.1 - 1 {
                return true;
            }

        //bot
        if tile.0 < self.pos.0 + self.size.0 &&
            tile.0 >= self.pos.0 &&
            tile.1 == self.pos.1 + self.size.1 {
                return true;
            }

        //right
        if tile.1 < self.pos.1 + self. size.1 &&
            tile.1 >= self. pos.1 &&
            tile.0 == self.pos.0 + self.size.0 {
                return true;
            }

        //left
        if tile.1 < self.pos.1 + self.size.1 &&
            tile.1 >= self.pos.1 &&
            tile.0 == self.pos.0 - 1 {
                return true;
            }

        return false;
    }
}

pub struct QuadrantMap {
    doors: [bool;12],
    enter: usize,
    exit: usize,
    difficulty: u32,
    loot: u32,
    danger_rooms: Vec<usize>,
    treasure_rooms: Vec<usize>,
    level: u32,
}


impl QuadrantMap {
    pub fn new(level: u32, template: DungeonTemplate )-> Self {
        let enter = template.entrance;
        let exit = template.exit;
        let mut doors = [false;12];
        for (i,val) in template.doors.iter().enumerate() {
            doors[i] = *val > 0;
        }
        Self {
            doors,
            exit,
            enter,
            difficulty: level,
            loot: level,
            danger_rooms: template.danger_rooms,
            treasure_rooms: vec![gen_range(0,8)],
            level,
        }
    }

    pub fn build(&self) -> DungeonMap {
        let mut map = DungeonMap::new(22,22,EntityKind::Wall);
        
        let mut rooms: Vec<Room> = Vec::new();

        for quadrant_id in 0..9 {
            let (quad_x,quad_y) = quadrant_xy(quadrant_id).unwrap();
            let w = gen_range(3,6);
            let h = gen_range(3,6);
            let x = quad_x+ 1 + gen_range(1,6-w);
            let y = quad_y+ 1 + gen_range(1,6-h);
            rooms.push(Room{
                pos: (x,y),
                size: (w,h),
                danger: self.danger_rooms.contains(&quadrant_id),
                loot: self.treasure_rooms.contains(&quadrant_id),
                player: self.enter == quadrant_id,
                exit: self.exit == quadrant_id,
            });
        }
        for room in rooms.iter() {
            map.fill_rect(EntityKind::Floor,room.pos,room.size).ok();
            let mut tiles_within = room.tiles_within();
            tiles_within.shuffle();

            if room.player {
                let player_pos = tiles_within.pop().unwrap();
                map.player_pos.replace(player_pos);
            }

            if room.exit {
                let coordinates = tiles_within.pop().unwrap();
                let tile_index = map.get_index(coordinates).unwrap();

                if self.level == 10 {
                    map.terrain[tile_index] = EntityKind::Exit;
                } else {
                    map.terrain[tile_index] = EntityKind::Stairs;
                }

            }

            if room.loot {
                let coordinates = tiles_within.pop().unwrap();
                let tile_index = map.get_index(coordinates).unwrap();

                let items: Vec<EntityKind> = ITEMS.iter().map(|k|k.clone()).collect();

                map.items[tile_index].push(items.choose().unwrap().clone());
            }

            let enemy_list: Vec<EntityKind> = ENEMIES.iter()
                .take(self.difficulty as usize + 1)
                .map(|k|k.clone())
                .collect();


            for _ in 0..gen_range(0,3) as i32 {
                let enemy = enemy_list.choose().unwrap();
                if let Some(tile) = tiles_within.pop() {
                    if let Ok(index) = map.get_index(tile) {
                        map.actors[index].replace(*enemy);
                    }
                }
            }


        }

        let mut hallways: Vec<(usize,(usize,usize),usize,(usize,usize))> = Vec::new();
        for (i,&door) in self.doors.iter().enumerate() {
            if door {
                if let Some((q1,q2)) = connected_quadrants(i) {
                    let room1 = &rooms[q1];
                    let room2 = &rooms[q2];
                    hallways.push((q1,room1.rand_tile_within(),q2,room2.rand_tile_within()));
                }
            }
        }

        for (q1,start,q2,finish) in hallways {
            let mut start = start; 
            let mut finish = finish;
            let room1 = &rooms[q1];
            let room2 = &rooms[q2];
            if start.0 > finish.0 {
                let placeholder = start;
                start = finish;
                finish = placeholder;
            }

            let mut tiles: HashSet<(usize,usize)> = HashSet::new();
            for x in start.0..=finish.0 {
                tiles.insert((x,start.1));
            }
            if start.1 > finish.1 {
                for y in finish.1..=start.1 {
                    tiles.insert((finish.0,y));
                }
            } else {
                for y in start.1..=finish.1 {
                    tiles.insert((finish.0,y));
                }
            }

            for tile in tiles {
                if let Ok(index) = map.get_index(tile) {
                    if map.terrain[index] == EntityKind::Wall {
                        if room1.is_border(tile)||room2.is_border(tile){
                            map.terrain[index] = EntityKind::Door;
                        } else {
                            map.terrain[index] = EntityKind::Floor;
                        }
                    }
                }
            }
        }

        map
    }

}

fn quadrant_xy(quadrant: usize) -> Option<(usize,usize)> {
    match quadrant {
        0 => Some((0,  0 )),
        1 => Some((7,  0 )),
        2 => Some((14, 0 )),
        3 => Some((0,  7 )),
        4 => Some((7,  7 )),
        5 => Some((14, 7 )),
        6 => Some((0,  14)),
        7 => Some((7,  14)),
        8 => Some((14, 14)),
        _ => {None}
    }
}

fn connected_quadrants(border_index:usize) -> Option<(usize,usize)> {
    match border_index {
        0  => Some((0,1)),
        1  => Some((1,2)),
        2  => Some((0,3)),
        3  => Some((1,4)),
        4  => Some((2,5)),
        5  => Some((3,4)),
        6  => Some((4,5)),
        7  => Some((3,6)),
        8  => Some((4,7)),
        9  => Some((5,8)),
        10 => Some((6,7)),
        11 => Some((7,8)),
        _ => {None}
    }
}


//pub type DungeonTemplate = (usize,usize,[u8;12]);
#[derive(Clone)]
pub struct DungeonTemplate {
    pub entrance: usize,
    pub exit: usize,
    pub doors: [u8;12],
    pub treasure_rooms: Vec<usize>,
    pub danger_rooms: Vec<usize>,
}

impl DungeonTemplate {
    pub fn rotate(&mut self) {
        let rotate_index = |i| -> Option<usize> {
            match i {
                0 => Some(6),
                1 => Some(3),
                2 => Some(0),
                3 => Some(7),
                4 => Some(4),
                5 => Some(1),
                6 => Some(8),
                7 => Some(5),
                8 => Some(2),
                _ => None
            }
        };

        self.entrance = rotate_index(self.entrance).unwrap();
        self.exit = rotate_index(self.exit).unwrap();
        
        self.treasure_rooms = self.treasure_rooms.iter()
            .map(|i|rotate_index(*i).unwrap()).collect();

        self.danger_rooms = self.danger_rooms.iter()
            .map(|i|rotate_index(*i).unwrap()).collect();

        self.doors = [
            self.doors[4],
            self.doors[9],
            self.doors[1],
            self.doors[6],
            self.doors[11],
            self.doors[3],
            self.doors[8],
            self.doors[0],
            self.doors[5],
            self.doors[10],
            self.doors[2],
            self.doors[7],
        ];
    }
    
    pub fn transpose(&mut self) {
        let transpose_index = |i| -> Option<usize> {
            match i {
                0 => Some(0),
                1 => Some(3),
                2 => Some(6),
                3 => Some(1),
                4 => Some(4),
                5 => Some(7),
                6 => Some(2),
                7 => Some(5),
                8 => Some(8),
                _ => None
            }
        };

        self.entrance = transpose_index(self.entrance).unwrap();
        self.exit = transpose_index(self.exit).unwrap();
        
        self.treasure_rooms = self.treasure_rooms.iter()
            .map(|i|transpose_index(*i).unwrap()).collect();

        self.danger_rooms = self.danger_rooms.iter()
            .map(|i|transpose_index(*i).unwrap()).collect();

        self.doors = [
            self.doors[2],
            self.doors[7],
            self.doors[0],
            self.doors[5],
            self.doors[10],
            self.doors[3],
            self.doors[8],
            self.doors[1],
            self.doors[6],
            self.doors[11],
            self.doors[4],
            self.doors[9],
        ];
    }

}
