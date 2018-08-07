extern crate pacbone;
extern crate config;
extern crate clap;


use pacbone::{Game, ConfigBuilder};
use config::*;
use clap::{Arg, App};
use std::error::Error;
use std::str::FromStr;

fn apply_command_line_arguments(config: &mut ConfigBuilder) -> Result<(), Box<Error>> {
    let matches = App::new("Pacbone game")
        .about("This is a programming/puzzle/adventure game")
        .arg(Arg::with_name("debug_general")
            .long("debug-general")
            .value_name("debug_general")
            .help("Enable or disable the general debugging")
            .takes_value(true))
        .arg(Arg::with_name("debug_piston")
            .long("debug-piston")
            .value_name("debug_piston")
            .help("Enable or disable the piston debugging")
            .takes_value(true))
        .arg(Arg::with_name("debug_graphics")
            .long("debug-graphics")
            .value_name("debug_graphics")
            .help("Enable or disable the graphics debugging")
            .takes_value(true))
        .arg(Arg::with_name("debug_input")
            .long("debug-input")
            .value_name("debug_input")
            .help("Enable or disable the input debugging")
            .takes_value(true))
        .arg(Arg::with_name("fullscreen")
            .long("fullscreen")
            .value_name("fullscreen")
            .help("Set or unset if run the game in fullscreen")
            .takes_value(true))
        .arg(Arg::with_name("max_fps")
            .long("max-fps")
            .value_name("max_fps")
            .help("Set the maximium fps")
            .takes_value(true))
        .arg(Arg::with_name("max_ups")
            .long("max-ups")
            .value_name("max_ups")
            .help("Set the maximium ups")
            .takes_value(true))
        .arg(Arg::with_name("vsync")
            .long("vsync")
            .value_name("vsync")
            .help("Enable or disable the vsync")
            .takes_value(true))
        .arg(Arg::with_name("samples")
            .long("samples")
            .value_name("samples")
            .help("Set the number of samples for antialiasing")
            .takes_value(true))
        .arg(Arg::with_name("player_save_path")
            .long("player-save-path")
            .value_name("player_save_path")
            .help("Set the path of the save files")
            .takes_value(true))    
        .arg(Arg::with_name("show_dev_info")
            .long("show-dev-info")
            .value_name("show_dev_info")
            .help("Enable or disable the show of development infos")
            .takes_value(true))    
        .get_matches();

    match matches.value_of("debug_general") {
        Some(val) => { config.debug_general(bool::from_str(val)?); },
        None => { }
    }
    match matches.value_of("debug_piston") {
        Some(val) => { config.debug_piston(bool::from_str(val)?); },
        None => { }
    }
    match matches.value_of("debug_graphics") {
        Some(val) => { config.debug_graphics(bool::from_str(val)?); },
        None => { }
    }
    match matches.value_of("debug_input") {
        Some(val) => { config.debug_input(bool::from_str(val)?); },
        None => { }
    }
    match matches.value_of("fullscreen") {
        Some(val) => { config.fullscreen(bool::from_str(val)?); },
        None => { }
    }
    match matches.value_of("max_fps") {
        Some(val) => { config.max_fps(u16::from_str(val)?); },
        None => { }
    }
    match matches.value_of("max_ups") {
        Some(val) => { config.max_ups(u16::from_str(val)?); },
        None => { }
    }
    match matches.value_of("vsync") {
        Some(val) => { config.vsync(bool::from_str(val)?); },
        None => { }
    }
    match matches.value_of("samples") {
        Some(val) => { config.samples(u8::from_str(val)?); },
        None => { }
    }
    match matches.value_of("player_save_path") {
        Some(val) => { config.player_save_path(String::from_str(val)?); },
        None => { }
    }
    match matches.value_of("show_dev_info") {
        Some(val) => { config.show_dev_info(bool::from_str(val)?); },
        None => { }
    }
    Ok(())
}

fn load_settings(cfg: &mut ConfigBuilder) -> Result<(), ConfigError> {
    let mut settings = config::Config::new(); 

    settings.set_default("debug_general", "false")?
        .set_default("debug_piston", "false")?
        .set_default("debug_graphics", "false")?
        .set_default("debug_input", "false")?
        .set_default("fullscreen", "true")?
        .set_default("max_fps", "120")?
        .set_default("max_ups", "120")?
        .set_default("vsync", "true")?
        .set_default("samples", "8")?
        .set_default("player_save_path", "~/.pacbone/saves")?
        .set_default("show_dev_info", "false")?
        .merge(config::File::with_name("~/.pacbone/conf").required(false))?;

    cfg.debug_general(settings.get::<bool>("debug_general")?)
        .debug_piston(settings.get::<bool>("debug_piston")?)
        .debug_graphics(settings.get::<bool>("debug_graphics")?)
        .debug_input(settings.get::<bool>("debug_input")?)
        .fullscreen(settings.get::<bool>("fullscreen")?)
        .max_fps(settings.get::<u16>("max_fps")?)
        .max_ups(settings.get::<u16>("max_ups")?)
        .vsync(settings.get::<bool>("vsync")?)
        .samples(settings.get::<u8>("samples")?)
        .player_save_path(settings.get::<String>("player_save_path")?)
        .show_dev_info(settings.get::<bool>("show_dev_info")?);

    Ok(())
}

fn main() {
    let mut cfg_builder = ConfigBuilder::default();
    load_settings(&mut cfg_builder).unwrap();
    apply_command_line_arguments(&mut cfg_builder).unwrap();
    let cfg = cfg_builder.build().unwrap();

    if *cfg.debug_general() {
        println!("{:?}", cfg);
    }     
    let mut game = Game::setup(cfg.clone());
    if *cfg.debug_general() {
        println!("[GENERAL] setup completed");
    }
    game.run();
}
