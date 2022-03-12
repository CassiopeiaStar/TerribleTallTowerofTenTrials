use macroquad::prelude::*;
use hecs::*;
use crate::prelude::*;
use crate::states::game::PlayerAction;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self,Group},
    Ui,
};

pub async fn inventory_state(
    world: &World,
    resources: &Resources
) -> Vec<PlayerAction>{
    let player = player(world).unwrap();
    let draw = |world,resources| {
        draw_map_and_hud(world,resources);
    };
    //next_frame to refresh any input buffers
    draw(world,resources);
    next_frame().await;

    //load state
    let items: Vec<(Entity,Option<Name>,Option<Equipable>,Option<Useable>)> = {
        let mut vec = Vec::new();
        for &item in resources.player.inventory.iter() {
            let mut entry = (item,None,None,None);
            if let Ok(name) = world.get::<Name>(item) {
                entry.1.replace((*name).clone());
            }
            if let Ok(equipable) = world.get::<Equipable>(item) {
                entry.2.replace((*equipable).clone());
            }
            if let Ok(useable) = world.get::<Useable>(item) {
                entry.3.replace((*useable).clone());
            }
            vec.push(entry);
        }
        vec
    };

    let equipment: Equipment = get_cloned(world,player).unwrap();
    let mut weapon_data: Option<(Entity,String)> = None;
    if let Some(weapon_id) = equipment.weapon {
        if let Some(name) = get_cloned::<Name>(world,weapon_id) {
            weapon_data.replace((weapon_id,name.name));
        }
    }

    //the vec of any actions taken
    let mut actions: Vec<PlayerAction> = Vec::new();
    loop {
        //input
        if is_key_pressed(macroquad::input::KeyCode::Escape) {
            break;
        }
        draw(world,resources);
        
        widgets::Window::new(hash!(), vec2(100.,100.),vec2(650.,600.))
            .titlebar(false)
            .movable(false)
            .ui(&mut *root_ui(), |ui| {
                Group::new(hash!("Equipment label"),Vec2::new(600.,100.)).ui(ui,|ui|{
                    ui.label(Vec2::new(10.,10.),&format!("{}","Equipment".to_owned()));
                    if let Some((ent,name)) = &weapon_data {
                        ui.label(Vec2::new(10.,40.),&format!("Weapon: {}",name));
                    }
                });

                Group::new(hash!("inventory label"), Vec2::new(600.,40.)).ui(ui, |ui| {
                    ui.label(Vec2::new(10.,10.),&format!("{}","Inventory".to_owned()));
                });
                for (i,(ent,name,equipable,useable)) in items.iter().enumerate() {
                    let name = {
                        if let Some(name) = name {
                            name.name.clone()
                        } else {
                            "No name found".to_owned()
                        }
                    };

                    Group::new(hash!("inventory",i), Vec2::new(600.,60.)).ui(ui, |ui| {
                        ui.label(Vec2::new(10.,10.),&format!("{}",name));
                        if ui.button(vec2(400.,10.),"Drop") {
                            actions.push(PlayerAction::DropItem(*ent));
                        }
                        if equipable.is_some() {
                            if ui.button(vec2(450.,10.),"Equip") {
                                actions.push(PlayerAction::EquipItem(*ent));
                            }
                        }
                        if useable.is_some() {
                            if ui.button(vec2(500.,10.),"Use") {
                                actions.push(PlayerAction::UseItem(*ent));
                            }
                        }
                    });

                }
            });


        if !actions.is_empty() {break;}
        next_frame().await;
    }

    //next_frame to refresh any input buffers
    draw(world,resources);
    next_frame().await;
    actions
}
