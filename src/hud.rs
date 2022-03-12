use macroquad::prelude::*;
use crate::prelude::*;
use hecs::*;

pub fn draw_hud(world: &World, resources: &Resources) {
    let tile_size = screen_height()/(ARENA_HEIGHT as f32+2.);
    let hud_start = tile_size*(ARENA_WIDTH as f32+2.);
    let hud_width = tile_size*10.;
    let text_size = (tile_size*0.8) as u16;


    let player = player(world).unwrap();
    draw_text_ex(
        format!("Dungeon level: {}",resources.level).as_str(),
        hud_start+tile_size,0.8*tile_size,
        TextParams {
            font_size: text_size,
            color: LIGHTGRAY,
            font: resources.font,
            ..Default::default()
        });   
    draw_text_ex(
        "Player",
        hud_start+tile_size,1.8*tile_size,
        TextParams {
            font_size: text_size,
            color: LIGHTGRAY,
            font: resources.font,
            ..Default::default()
        });
    let bar_width = hud_width-(tile_size*2.);
    let bar_height = tile_size;

    //player health
    if let Ok(health) = world.get::<Health>(player) {
        draw_bar(Rect::new(
                hud_start+tile_size,
                tile_size*2.,
                bar_width,
                bar_height)
            ,RED,health.current as f32,health.max as f32);
    }

    
    //monster health
    let mut monster_health_y = tile_size*3.;
    for (_,(appearance,name,health)) in 
        world.query::<(&Appearance,&Name,&Health)>().without::<Player>().iter() {
            if appearance.in_fov {

                draw_text_ex(
                    name.name.as_str(),
                    hud_start+tile_size,monster_health_y+(tile_size*0.8),
                    TextParams {
                        font_size: text_size,
                        color: LIGHTGRAY,
                        font: resources.font,
                        ..Default::default()
                    }
                );

                draw_bar(Rect::new(
                        hud_start+tile_size,
                        monster_health_y+tile_size,
                        bar_width,bar_height),
                        appearance.color,
                        health.current as f32,health.max as f32);

                monster_health_y += tile_size*2.;
            }
    }

}

fn draw_bar(rect: Rect, color: Color, value: f32, max: f32) {
    let border_thickness = 2.;

    draw_rectangle_lines(rect.x,rect.y,rect.w,rect.h,border_thickness,WHITE);
    draw_rectangle(rect.x+border_thickness,rect.y+border_thickness,
                   (rect.w-(2.*border_thickness))*(value/max),
                   rect.h-(2.*border_thickness),
                   color);

}


