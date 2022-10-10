use std::{
    io,
    path::{Path, PathBuf},
};

// Deal with arguments from command line
pub struct ArgBoy {
    args: Vec<String>,
}

#[derive(Debug)]
pub struct RepoBoy<'a> {
    src: &'a str,
    models: &'a str,
    sections: &'a str,
    materials: &'a str,
}

impl RepoBoy<'_> {
    fn new() -> Self {
        RepoBoy {
            src: "./src/",
            models: "./src/models/",
            sections: "./src/sections/",
            materials: "./src/materials/",
        }
    }
    fn init(&self) -> io::Result<()> {
        Ok(())
    }
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
