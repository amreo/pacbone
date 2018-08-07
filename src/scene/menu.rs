use scene::GameScene;
use Game;
use piston::input::{Input, UpdateArgs, RenderArgs};

pub struct MenuScene {

}

impl GameScene for MenuScene {
    fn update(self,  game: &mut Game, args: &UpdateArgs) -> Option<Box<GameScene>> {
        None
    }
    fn render(self, game: &mut Game, args: &RenderArgs) -> Option<Box<GameScene>>{
        None
    }
    fn handle_input(self, game: &mut Game, args: &Input) -> Option<Box<GameScene>>{
        None
    }
}