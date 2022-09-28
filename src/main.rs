use duckbubble::{
    orwritekey::{DirInRepo, KeywordReader},
    parts::Config,
};
use std::{env, fs, io};

fn main() -> io::Result<()> {
    //read toml, where is the description of the calculation
    let args: Vec<String> = env::args().collect();
    let toml_path = &args[1];
    let mut cfg = dbg!(Config::read(toml_path));
    // extract parts
    for par in &mut cfg.parts {
        let path = par.path_to(DirInRepo::Secs);
        let sec_stream = fs::read(path)?;
        let mut kdar = KeywordReader::new(sec_stream);
    }
    //read material
    //read sections
    // keep models id intact
    //read & write to model
    //write to main.k
    Ok(())
}
