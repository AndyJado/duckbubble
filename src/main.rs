use duckbubble::{orwritekey::print_dir, parts::Config};
use std::{env, path::Path};

fn main() {
    //read toml
    let args: Vec<String> = env::args().collect();
    let mut cfg = dbg!(Config::read(&args[1]));
    //read model
    //read material
    //read sections
    //write to main.k
}
