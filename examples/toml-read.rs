use std::io;

use duckbubble::parts::Config;

fn main() -> io::Result<()> {
    let cfg = Config::read("dry.toml");
    dbg!(cfg);
    Ok(())
}
