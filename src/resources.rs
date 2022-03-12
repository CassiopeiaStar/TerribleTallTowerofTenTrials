use std::collections::HashSet;
use macroquad::prelude::*;
use hecs::*;
use crate::constants::*;
use macroquad::ui::*;

pub struct Resources{
    pub ascii: Texture2D,
    pub sprite_sheet: Texture2D,
    pub font: Font,
    pub player: PlayerData,
    pub fov_set: HashSet<(i32,i32)>,
    pub highlights: Vec<Option<Color>>,
    pub new_level_request: bool,
    pub level: u32,
}

pub async fn load_resources() -> Resources {
    let pressed_image = load_image("textures/button2-pressed.png").await.unwrap();
    let button_image = load_image("textures/button2.png").await.unwrap();

    let window_style = root_ui()
        .style_builder()
        .background(button_image.clone())
        .background_margin(RectOffset::new(10., 10.0, 10., 10.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .font_size(20)
        .background(button_image.clone())
        .background_clicked(pressed_image.clone())
        .background_margin(RectOffset::new(10., 10.0, 10., 10.0))
        .text_color(BLUE)
        .build();

    let label_style = root_ui()
        .style_builder()
        .font_size(30)
        .text_color(BLUE)
        .build();

    let skin = Skin {
        button_style,
        label_style,
        window_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin);



    Resources {
        ascii: load_and_filter("textures/ascii_8x8.png").await,
        sprite_sheet: load_and_filter("textures/sprite-sheet.png").await,
        font: load_ttf_font("fonts/FiraMono-Medium.ttf").await.unwrap(),
        player: PlayerData::new(),
        fov_set: HashSet::new(),
        highlights: vec![None;ARENA_WIDTH*ARENA_HEIGHT],
        new_level_request: false,
        level: 0,
    }
}


async fn load_and_filter(path: &str) -> Texture2D {
    let texture = load_texture(path).await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    texture
}

pub struct PlayerData {
    pub inventory: Vec<Entity>,
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            inventory: Vec::new(),
        }
    }
}
