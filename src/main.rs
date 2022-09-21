use duckbubble::parts::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::read(&args[1]);
}
