#[macro_use] extern crate derive_builder;
#[macro_use] extern crate getset;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

pub use game::Game;
pub mod game;

pub use config::Config;
pub use config::ConfigBuilder;
pub mod config;