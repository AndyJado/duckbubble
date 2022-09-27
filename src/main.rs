use duckbubble::parts::Config;
use std::env;

fn main() {
    //read toml, where is the description of the calculation
    let args: Vec<String> = env::args().collect();
    let toml_path = &args[1];
    let mut cfg = dbg!(Config::read(toml_path));
    // extract parts
    for par in &mut cfg.parts {}
    //read material
    //read sections
    // keep models id intact
    //read & write to model
    //write to main.k
}
