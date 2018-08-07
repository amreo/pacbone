use Game;
use piston::input::{RenderArgs, UpdateArgs, Input};

pub mod menu;
pub use self::menu::MenuScene;

pub trait GameScene {
    fn update(&mut self, game: &mut Game, args: &UpdateArgs) -> Option<Box<GameScene>>;
    fn render(&mut self, game: &mut Game, args: &RenderArgs) -> Option<Box<GameScene>>;
    fn handle_input(&mut self, game: &mut Game, args: &Input) -> Option<Box<GameScene>>;
}