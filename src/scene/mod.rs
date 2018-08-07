use Game;
use piston::input::{RenderArgs, UpdateArgs, Input};

mod menu;

pub trait GameScene {
    fn update(self, game: &mut Game, args: &UpdateArgs) -> Option<Box<GameScene>>;
    fn render(self, game: &mut Game, args: &RenderArgs) -> Option<Box<GameScene>>;
    fn handle_input(self, game: &mut Game, args: &Input) -> Option<Box<GameScene>>;
}