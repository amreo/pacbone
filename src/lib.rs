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
mod scene;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use fps_counter::FPSCounter;
use graphic_resources::GraphicResources;
use scene::GameScene;
use scene::MenuScene;
pub struct Game<'a> {  
    config: Config,
    window: Window,
    gl: GlGraphics,
    graphic_resources: GraphicResources<'a>,
    fps_counter: FPSCounter,
    ups_counter: FPSCounter,
    last_fps: u16,
    last_ups: u16
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
            last_ups: 0
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

        let mut current_scene : Box<GameScene> = Box::new(MenuScene::new());

        while let Some(e) = events.next(&mut self.window) {
            if *self.config.debug_piston() {
                println!("[PISTON] event {:?}", e);
            }

            match e {
                Event::Loop(Loop::Render(args)) => { 
                    match current_scene.render(self, &args) {
                        Some(val) => current_scene = val,
                        None => {}
                    }
                    self.last_fps = self.fps_counter.tick() as u16;
                    if *self.config.show_dev_info() {
                        println!("[GENERAL] fps={}", self.last_fps);
                    }    
                }
                Event::Loop(Loop::Update(args)) => { 
                    match current_scene.update(self, &args) {
                        Some(val) => current_scene = val,
                        None => {}
                    }
                    self.last_ups = self.ups_counter.tick() as u16;
                    if *self.config.show_dev_info() {
                        println!("[GENERAL] ups={}", self.last_ups);
                    }    
                }
                Event::Loop(Loop::Idle(_)) => { }
                Event::Loop(Loop::AfterRender(_)) => { }
                Event::Input(args) => {
                    match current_scene.handle_input(self, &args) {
                        Some(val) => current_scene = val,
                        None => {}
                    }
                } 
                _ => {  eprintln!("{:?}", e); }
            }
        }
    }
}