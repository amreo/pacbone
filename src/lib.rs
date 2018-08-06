#[macro_use] extern crate derive_builder;
#[macro_use] extern crate getset;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate fps_counter;

pub use config::Config;
pub use config::ConfigBuilder;
pub mod config;
mod graphic_resources;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use fps_counter::FPSCounter;
use graphic_resources::GraphicResources;

pub struct Game<'a> {  
    config: Config,
    window: Window,
    gl: GlGraphics,
    graphic_resources: GraphicResources<'a>,
    fps_counter: FPSCounter,
    ups_counter: FPSCounter,
    last_fps: u16,
    last_ups: u16,
    
    rotation: f64,
}

impl<'a> Game<'a> {

    pub fn setup(config: Config) -> Game<'a> {
        let opengl = OpenGL::V4_5;
        let window = WindowSettings::new(
                "pacbone",
                [1920, 1080]
            )
            .fullscreen(*config.fullscreen())
            .opengl(opengl)
            .samples(*config.samples())
            .vsync(*config.vsync())
            .exit_on_esc(true)
            .build()
            .unwrap();


        // Create a new game and run it.
        Game {
            window: window,
            gl: GlGraphics::new(opengl),
            config: config,
            fps_counter: FPSCounter::new(),
            ups_counter: FPSCounter::new(),
            graphic_resources: GraphicResources::load(),
            last_fps: 0,
            last_ups: 0,
            rotation: 0.0
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new()
            .max_fps(*self.config.max_fps() as u64)
            .ups(*self.config.max_ups() as u64)
            .swap_buffers(true)
        );

        if *self.config.debug_general() {
            println!("[GENERAL] game start running");
        }
        while let Some(e) = events.next(&mut self.window) {
            if *self.config.debug_piston() {
                println!("[PISTON] event {:?}", e);
            }

            match e {
                Event::Loop(Loop::Render(args)) => { self.render(&args); }
                Event::Loop(Loop::Update(args)) => { self.update(&args); }
                Event::Loop(Loop::Idle(_)) => { }
                Event::Loop(Loop::AfterRender(_)) => { }
                Event::Input(Input::Button(ButtonArgs { state: ButtonState::Press, scancode: _, button: Button::Mouse(MouseButton::Left)})) => { 
                    self.rotation += 100.0; 
                }
                _ => {  eprintln!("{:?}", e); }
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);
        let roboto_light = &mut self.graphic_resources.roboto_glyph_cache;
        let show_dev_info = *self.config.show_dev_info();
        let fps = self.last_fps;
        let ups = self.last_ups;
        self.gl.draw(args.viewport(), |c, gl| {
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

        self.last_fps = self.fps_counter.tick() as u16;

        if *self.config.show_dev_info() {
            println!("[GENERAL] fps={}", self.last_fps);
        }    
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;

        self.last_ups = self.ups_counter.tick() as u16;

        if *self.config.show_dev_info() {
            println!("[GENERAL] ups={}", self.last_ups);
        }    
    }
}