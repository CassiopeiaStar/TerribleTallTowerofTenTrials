
use hecs::*;
use crate::prelude::*;

pub fn memory_system(world: &mut World,_resources: &mut Resources) {
    for (_,monster_memory) in world.query::<&mut MonsterMemory>().iter(){
        if monster_memory.time_to_remember > 0 {
            monster_memory.time_to_remember -=1;
        }
    }
    for (_,(appearance,monster_memory)) in 
        world.query::<(&Appearance,&mut MonsterMemory)>().iter() {
            if appearance.in_fov {
                monster_memory.time_to_remember = monster_memory.strength;
            }

    }
}
