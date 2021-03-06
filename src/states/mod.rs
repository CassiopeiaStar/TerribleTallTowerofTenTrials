use hecs::World;
use crate::resources::Resources;

pub mod game;
pub mod inventory;
pub mod game_over;
pub mod main_menu;
pub mod aiming;
pub mod game_controls;
pub mod win;

pub enum GameState {
    Game,
    GameOver,
    MainMenu,
    Win,
}

pub enum StateChange {
    Replace(GameState),
    Quit,
}

impl GameState {
    pub async fn run(&self, world: &mut World,resources: &mut Resources) -> StateChange {
        match self {
            GameState::Game => {game::game(world,resources).await}
            GameState::GameOver => {game_over::game_over(world,resources).await}
            GameState::MainMenu => {main_menu::main_menu(world,resources).await}
            GameState::Win => {win::win_state(world,resources).await}
        }
    }
}
