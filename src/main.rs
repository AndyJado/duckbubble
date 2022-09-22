use duckbubble::parts::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cfg = Config::read(&args[1]);
    let parts = &mut cfg.parts;

    dbg!(parts.last().unwrap().sec());
}
