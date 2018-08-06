#[derive(Builder, Getters, Debug, PartialEq, Default, Clone)]
pub struct Config {
    #[builder(default = "false")]
    #[get = "pub"]
    debug_general: bool,
    #[builder(default = "false")]
    #[get = "pub"]
    debug_piston: bool,
    #[builder(default = "false")]
    #[get = "pub"]
    debug_graphics: bool,
    #[builder(default = "true")]
    #[get = "pub"]
    fullscreen: bool,
    #[builder(default = "120")]
    #[get = "pub"]
    max_fps: u16,
    #[builder(default = "120")]
    #[get = "pub"]
    max_ups: u16,
    #[builder(default = "true")]
    #[get = "pub"]
    vsync: bool,
    #[builder(default = "8")]
    #[get = "pub"]
    samples: u8,
    #[builder(default = "String::from(\"~/.pacbone/saves\")")]
    #[get = "pub"]
    player_save_path: String,
    #[builder(default = "false")]
    #[get = "pub"]
    show_dev_info: bool
}