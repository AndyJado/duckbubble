use duckbubble::{
    orwritekey::{DirInRepo, KeywordReader},
    parts::{Config, KeyId},
};
use std::{env, fs, io};

fn main() -> io::Result<()> {
    //read toml, where is the description of the calculation
    let args: Vec<String> = env::args().collect();
    let toml_path = &args[1];
    let mut cfg = dbg!(Config::read(toml_path));
    let mut id_gen = KeyId::new();
    // extract parts
    for par in &mut cfg.parts {
        par.alloc(&mut id_gen);
        let path = par.path_to(DirInRepo::Secs);
        let sec_stream = fs::read(path)?;
        let mut kdar = KeywordReader::new(sec_stream);
        kdar.read_keyword_a();
    }
    //read material
    //read sections
    // keep models id intact
    //read & write to model
    //write to main.k
    Ok(())
}
