use std::io;

use crate::parts::{DynaConfig, Part};

// Deal with arguments from command line
pub struct ArgBoy {
    args: Vec<String>,
}

impl ArgBoy {
    pub fn new() -> Self {
        let args = std::env::args().collect();
        ArgBoy { args }
    }
    pub fn read_uh(&self) -> io::Result<()> {
        let uh: &str = &self.args[1];
        match uh {
            "init" => todo!(),
            "dry.toml" => todo!(),
            _ => panic!("not yet this arg!"),
        }
        Ok(())
    }
}
