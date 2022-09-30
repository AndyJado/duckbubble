use duckbubble::{
    orwritekey::{DirInRepo, KeywordReader},
    parts::{Config, KeyCell, KeyId, Part},
};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Seek, SeekFrom, Write},
};

fn main() -> io::Result<()> {
    //read toml, where is the description of the calculation
    let args: Vec<String> = env::args().collect();
    let toml_path = &args[1];
    let mut cfg = Config::read(toml_path);
    let mut id_gen = KeyId::new();
    let mut mid_map: HashMap<String, KeyCell> = HashMap::new();
    let mut sid_map: HashMap<String, KeyCell> = HashMap::new();
    let mut par_map: HashMap<String, &Part> = HashMap::new();
    let part_dir = DirInRepo::Models.path();
    // extract parts
    for par in &mut cfg.parts {
        //alloc material & section id
        if !mid_map.contains_key(&par.mat()) {
            par.mid_allo(&mut id_gen);
            mid_map.insert(par.mat(), par.mid.unwrap());
        }
        if !sid_map.contains_key(&par.sec()) {
            par.sid_allo(&mut id_gen);
            sid_map.insert(par.sec(), par.secid.unwrap());
        }
        par_map.insert(par.name(), par);
    }
    //write to attri dirs
    for (k, v) in mid_map.iter() {
        let mut path = DirInRepo::Mats.path();
        path.push(k);
        path.set_extension("k");
        dbg!(&path);
        let stream = fs::read(&path)?;
        let mut kdar = KeywordReader::new(stream);
        let seek_head = kdar.process_part_attri();
        let mut file = File::options().write(true).open(path)?;
        file.seek(SeekFrom::Start(seek_head))?;
        file.write_all(&v.0)?;
    }
    for (k, v) in sid_map.iter() {
        let mut path = DirInRepo::Secs.path();
        path.push(k);
        path.set_extension("k");
        dbg!(&path);
        let stream = fs::read(&path)?;
        let mut kdar = KeywordReader::new(stream);
        let seek_head = kdar.process_part_attri();
        let mut file = File::options().write(true).open(path)?;
        file.seek(SeekFrom::Start(seek_head))?;
        file.write_all(&v.0)?;
    }
    for entry in fs::read_dir(part_dir)? {
        let entry = entry?;
        let k_path = entry.path();
        if k_path.extension().unwrap() == "k" {
            // file read & write
            let stream = fs::read(&k_path)?;
            let mut file = File::options().write(true).open(k_path)?;
            let kdar = KeywordReader::new(stream);
            // read the whole file, return where & who to write
            let write_vec: Vec<_> = kdar.map(|c| c.expect("read part in loop")).collect();
            for (name, head) in write_vec {
                // retrieve part from cache
                let par = par_map.get(&name).expect("get part name from par_map");
                // get the duckbubble organized id
                let mat_cell = match par.mid {
                    Some(ref mid) => mid,
                    None => mid_map.get(&name).expect("mat in book"),
                };
                let sec_cell = match par.secid {
                    Some(ref sid) => sid,
                    None => sid_map.get(&name).expect("sec in book"),
                };
                file.seek(SeekFrom::Start(head + 10))?;
                file.write_all(&sec_cell.0)?;
                file.seek(SeekFrom::Current(10))?;
                file.write_all(&mat_cell.0)?;
            }
        }
    }

    //read & write to model
    //write to main.k
    Ok(())
}
