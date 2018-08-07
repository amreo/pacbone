use scene::GameScene;
use Game;
use piston::input::{Input, UpdateArgs, RenderArgs, ButtonArgs};
use piston::input::Input::Button;
use piston::input::Button::Keyboard;
use piston::input::ButtonState::Press;
use piston::input::keyboard::Key::{Left, Right};
pub struct MenuScene {
    rotation: f64,
    speed: f64,
}

impl MenuScene {
    pub fn new() -> Self {
        MenuScene {
            rotation: 0.0,
            speed: 2.0
        }
    }
}

impl GameScene for MenuScene {
    fn update(&mut self,  _: &mut Game, args: &UpdateArgs) -> Option<Box<GameScene>> {
        // Rotate 2 radians per second.
        self.rotation += self.speed * args.dt;
        None
    }
    fn render(&mut self, game: &mut Game, args: &RenderArgs) -> Option<Box<GameScene>>{
        use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
        let show_dev_info = *game.config.show_dev_info();
        let fps = game.last_fps;
        let ups = game.last_ups;
        let roboto_light = &mut game.graphic_resources.roboto_glyph_cache;


        game.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);    

            if show_dev_info {
                text(RED, 16, format!("FPS: {}", fps).as_str(), roboto_light,  c.transform.trans(0.0, 16.0), gl)
                    .expect("Text() has failed. Unbelievable!!!");
                text(RED, 16, format!("UPS: {}", ups).as_str(), roboto_light,  c.transform.trans(0.0, 32.0), gl)
                    .expect("Text() has failed. Unbelievable!!!");
            }
        });

        None
    }
    fn handle_input(&mut self, game: &mut Game, args: &Input) -> Option<Box<GameScene>>{
        if *game.config.debug_input() {
            println!("[INPUT] [MenuScene] Received input: {:?}", args);
        } 
        
        match args {
            Button(ButtonArgs { state: Press, button: Keyboard(Left), scancode: _}) => {
                self.speed -= 0.5;
            },
            Button(ButtonArgs { state: Press, button: Keyboard(Right), scancode: _}) => {
                self.speed += 0.5;
            },
            _ => {}
        }

        None
    }
}