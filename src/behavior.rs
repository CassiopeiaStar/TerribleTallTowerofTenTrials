use hecs::*;
use tree::*;
use crate::prelude::*;
use crate::combat::*;
use macroquad::rand::*;

pub fn act(world: &mut World, resources: &mut Resources, actor: Entity,behavior: Behavior) {
    let player = player(world).unwrap();

    let node = match behavior {
        Behavior::Erratic(percent) => BehaviorNode::Erratic(player,percent),
        Behavior::Slow(percent) => BehaviorNode::Slow(player,percent),
        Behavior::ApproachAndAttack => BehaviorNode::ApproachAndAttackOrWander(player),

    };
    let behavior_result = node.build(actor).tick(world,resources);

    match behavior_result {
        BehaviorResult::Fail => {
            dbg!();

        },
        BehaviorResult::Success => {
            dbg!();

        },
        BehaviorResult::Acting(action) => {
            match action {
                ActorAction::Wait => {
                }
                ActorAction::Move{origin:_,destination} => {
                    if let Ok(mut pos) = world.get_mut::<Pos>(actor) {
                        *pos = destination.into();
                    }
                }
                ActorAction::Attack{target,weapon} => {
                    if let Some(attack_data) = get_attack(world,actor) {
                        attack(world,actor,target,weapon.attack)
                    }
                }
                ActorAction::Wander => {
                    use macroquad::rand::*;
                    let block_map = stepping_tiles(world);
                    if let Ok(mut pos) = world.get_mut::<Pos>(actor) {
                        let successors: Vec<(i32,i32)>
                            = vec![(pos.x-1,pos.y),(pos.x+1,pos.y),
                                (pos.x,pos.y-1),(pos.x,pos.y+1)]
                            .into_iter()
                            .filter(|p|{!block_map.contains(&p)})
                            .collect();
                        
                        if let Some(dest) = successors.choose() {
                            pos.x = dest.0;
                            pos.y = dest.1;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ActorAction {
    Move{
        origin: (i32,i32),
        destination: (i32,i32)
    },
    Attack {
        target: Entity,
        weapon: Weapon,
    },
    Wander,
    Wait
}

#[allow(dead_code)]
pub enum BehaviorNode {
    ApproachAndAttack(Entity),
    Attack(Entity),
    ApproachAndAttackOrWait(Entity),
    ApproachAndAttackOrWander(Entity),
    ApproachEntity(Entity),
    ApproachIntoRange(Entity),
    InAttackRange(Entity),
    Erratic(Entity,u32),
    Slow(Entity,u32),
    Wander,
    InFOV,
    None,
    PercentChance(u32),


    //debug
    Debug(String),
    DebugRange(Entity),
}

impl BehaviorNode {
    pub fn build(&self, actor: Entity) -> Box<dyn Node> {
        match self {
            BehaviorNode::Erratic(target,wander_rate) => {
                let target = target.clone();
                Box::new(Selector::new(vec![
                    chance(actor,BehaviorNode::Wander,*wander_rate),
                    BehaviorNode::ApproachAndAttackOrWander(target).build(actor)
                ]))
            },
            &BehaviorNode::Slow(target,wait_rate) => {
                let target = target.clone();
                Box::new(Selector::new(vec![
                    chance(actor,BehaviorNode::None,wait_rate),
                    BehaviorNode::ApproachAndAttackOrWander(target).build(actor)
                ]))
            }
            BehaviorNode::Debug(message) => {
                let message = message.clone();
                action(move |_world: &World,_resources: &Resources| {
                    return BehaviorResult::Success;
                })
            }
            BehaviorNode::DebugRange(target) => {
                let target = target.clone();
                Box::new(Sequence::new(vec![
                    BehaviorNode::InAttackRange(target.clone()),
                    BehaviorNode::Debug("In attack range".to_owned())
                ].iter().map(|n|n.build(actor)).collect()))
            }

            BehaviorNode::ApproachAndAttackOrWander(target) => {
                let target = target.clone();
                Box::new(Selector::new(vec![
                    BehaviorNode::ApproachAndAttack(target.clone()),
                    BehaviorNode::Wander
                ].iter().map(|n|n.build(actor)).collect()))

            }
            BehaviorNode::ApproachAndAttackOrWait(target) => {
                let target = target.clone();
                Box::new(Selector::new(vec![
                    BehaviorNode::ApproachAndAttack(target.clone()),
                    BehaviorNode::None
                ].iter().map(|n|n.build(actor)).collect()))
            }
            BehaviorNode::Wander => {
                action(move |_world: &World,_resources: &Resources| {
                    return BehaviorResult::Acting(ActorAction::Wander);
                })
            }
            BehaviorNode::InFOV => {
                action(move |world: &World,_resources: &Resources| {
                    if let Ok(appearance) = world.get::<Appearance>(actor) {
                        if appearance.in_fov {
                            return BehaviorResult::Success;
                        }
                    }
                    return BehaviorResult::Fail;
                })
            }
            BehaviorNode::InAttackRange(target) => {
                let target = target.clone();
                action(move |world: &World,_resources: &Resources| {
                    if let Some(weapon) = get_attack(world,actor) {
                        if let Some(true) = weapon.range.in_range(world,actor,target) {
                            return BehaviorResult::Success;
                        }
                    }
                    BehaviorResult::Fail
                })
            }
            BehaviorNode::ApproachAndAttack(target) => {
                let target = target.clone();
                Box::new(Sequence::new(vec![
                    BehaviorNode::InFOV,
                    BehaviorNode::ApproachIntoRange(target.clone()),
                    BehaviorNode::Attack(target.clone())
                ].iter().map(|n|n.build(actor)).collect()))
            }

            BehaviorNode::Attack(target) => {
                let target = target.clone();
                action(move |world: &World, _resources: &Resources|{
                    if let Ok(weapon) = world.get::<Weapon>(actor) {
                        return BehaviorResult::Acting(ActorAction::Attack{
                            target,
                            weapon: (*weapon).clone()
                        });
                    }
                    return BehaviorResult::Fail;
                })
            },
            BehaviorNode::ApproachEntity(target) => {
                let target = target.clone();
                action(move |world: &World, _resources: &Resources|{
                    use crate::movement::my_pathfinding::basic_path;
                    if let Some((path,distance)) = basic_path(world,actor,target) {
                        if distance == 1 {
                            return BehaviorResult::Success;
                        } else {
                            return BehaviorResult::Acting(ActorAction::Move{
                                origin: path[0],
                                destination: path[1]
                            })
                            
                        }
                    }
                    //no path is found, so fail
                    return BehaviorResult::Fail;
                })
            },
            BehaviorNode::ApproachIntoRange(target) => {
                let target = target.clone();
                Box::new(Selector::new(vec![
                    BehaviorNode::InAttackRange(target.clone()),
                    BehaviorNode::ApproachEntity(target.clone()),
                ].iter().map(|n|n.build(actor)).collect()))
            }
            BehaviorNode::None => {
                action(move |_world: &World, _resources: &Resources| {
                    return BehaviorResult::Acting(ActorAction::Wait);
                })
            }
            BehaviorNode::PercentChance(percent) => {
                let percent = percent.clone();
                action(move |_world: &World, _resources: &Resources| {
                    let r: u32 = gen_range(0,100);
                    if percent > r {
                        BehaviorResult::Success
                    } else {
                        BehaviorResult::Fail
                    }
                })
            }
        }
    }
}

fn action<F: 'static>(action_function: F) -> Box::<Action> 
    where F: FnMut(&World, &Resources) -> BehaviorResult
{
    let action_function_box: Box<dyn FnMut(&World, &Resources) -> BehaviorResult> = Box::new(action_function);
    Box::new(
        Action{
            act: action_function_box,
        }
    )
}
fn chance(actor: Entity, node: BehaviorNode, percent: u32) -> Box::<dyn Node> {
    Box::new(Sequence::new(vec![
        BehaviorNode::PercentChance(percent),
        node
    ].iter().map(|n|n.build(actor)).collect()))
}

#[allow(dead_code)]
mod tree {
    use hecs::World;
    use crate::resources::Resources;
    use super::ActorAction;


    #[derive(Debug)]
    pub enum BehaviorResult {
        Success,
        Fail,
        Acting(ActorAction),
    }

    impl PartialEq for BehaviorResult {
        fn eq(&self, other: &Self) -> bool {
            match self {
                BehaviorResult::Fail => {
                    match other {
                        BehaviorResult::Fail => true,
                        _ => false
                    }
                }
                BehaviorResult::Acting(_) => {
                    match other {
                        BehaviorResult::Acting(_) => true,
                        _ => false
                    }
                }
                BehaviorResult::Success => {
                    match other {
                        BehaviorResult::Success => true,
                        _ => false
                    }
                }
            }
        }
    }

    pub trait Node {
        fn tick(&mut self, world: &World, resources: &Resources) -> BehaviorResult;
    }

    pub struct Action {
        pub act: Box<dyn FnMut(&World,&Resources) -> BehaviorResult>,
    }

    impl Node for Action {
        fn tick(&mut self,world: &World, resources: &Resources) -> BehaviorResult {
            (self.act)(world,resources)
        }
    }

    //returns the first non-failed result, doesn't complete the remaining nodes
    pub struct Selector {
        children: Vec< Box< dyn Node > >,
    }

    impl Selector {
        pub fn new(children: Vec< Box< dyn Node > >) -> Self {
            Self {
                children
            }
        }
    }

    impl Node for Selector {
        fn tick(&mut self,world: &World, resources: &Resources) -> BehaviorResult {
            for child in self.children.iter_mut() {
                let result = child.tick(world,resources);
                if result != BehaviorResult::Fail {
                    return result;
                }
            }
            BehaviorResult::Fail
        }
    }

    //returns the first non-success result
    pub struct Sequence {
        children: Vec< Box< dyn Node > >,
    }

    impl Sequence {
        pub fn new(children: Vec< Box< dyn Node > >) -> Self {
            Self{
                children
            }
        }
    }

    impl Node for Sequence {
        fn tick(&mut self,world: &World, resources: &Resources) -> BehaviorResult {
            for child in self.children.iter_mut() {
                let result = child.tick(world,resources);
                if result != BehaviorResult::Success {
                    return result;
                }
            }
            BehaviorResult::Success
        }
    }
}
