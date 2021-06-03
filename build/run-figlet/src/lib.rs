use std::process::Command;

include!(concat!(env!("OUT_DIR"), "/path.rs"));

pub struct RunFiglet;

impl RunFiglet {
    pub fn new() -> Command {
        Command::new(FIGLET_PATH)
    }
}
