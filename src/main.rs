use duckbubble::{
    boys::{ArgBoy, Argommand, DirWalker, KeyPath, RepoBoy},
    orwritekey::{KeywordReader, PartReader},
    param::ls_run,
    parts::{DynaConfig, KeyCell, KeyId, Part},
};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Seek, SeekFrom, Write},
};

fn main() -> io::Result<()> {
    let mut argboy = ArgBoy::new();
    let repoboy = RepoBoy::new();
    match argboy.errand() {
        Argommand::Init => repoboy.init(),
        Argommand::Link => link_dyna_repo(repoboy),
        Argommand::Para => run_para(),
    }
}

fn run_para() -> io::Result<()> {
    eprintln!("should be reading configuration from `dry.toml` in current dir");
    let DynaConfig { lsrun, param, .. } = DynaConfig::read("dry.toml");
    ls_run(lsrun, param);
    Ok(())
}

fn link_dyna_repo(repo_boy: RepoBoy) -> io::Result<()> {
    let dir_boy = DirWalker::new();
    //read toml, where is the description of the calculation
    let cfg = DynaConfig::read("dry.toml");
    //Now links in repo happens
    let mut id_gen = KeyId::new();
    let mut mid_map: HashMap<String, KeyCell> = HashMap::new();
    let mut sid_map: HashMap<String, KeyCell> = HashMap::new();
    let mut par_map: HashMap<String, &Part> = HashMap::new();
    // extract parts
    let mut parts = cfg.parts.unwrap();
    for par in &mut parts {
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
    //write to attri dirs,(key, value)
    for (k, v) in mid_map.iter() {
        let mut path = repo_boy.materials.clone();
        path.push(k);
        path.set_extension("k");
        dbg!(&path);
        let stream =
            fs::read(&path).expect("material file name should match description in `dry.toml`");
        let mut kdar = KeywordReader::new(stream);
        let seek_head = kdar.process_part_attri();
        let mut file = File::options().write(true).open(path)?;
        file.seek(SeekFrom::Start(seek_head))?;
        file.write_all(&v.0)?;
    }
    for (k, v) in sid_map.iter() {
        let mut path = repo_boy.sections.clone();
        path.push(k);
        path.set_extension("k");
        dbg!(&path);
        let stream =
            fs::read(&path).expect("section file name should match description in `dry.toml`");
        let mut kdar = KeywordReader::new(stream);
        let seek_head = kdar.process_part_attri();
        let mut file = File::options().write(true).open(path)?;
        file.seek(SeekFrom::Start(seek_head))?;
        file.write_all(&v.0)?;
    }
    for entry in fs::read_dir(&repo_boy.models)? {
        let entry = entry?;
        let k_path = entry.path();
        if KeyPath(&k_path).is_k() {
            // file read & write
            let stream = fs::read(&k_path)?;
            let mut file = File::options().write(true).open(k_path)?;
            let kdar = KeywordReader::new(stream);
            let pdar = PartReader(kdar);
            // read the whole file, return where and who to write
            let write_vec: Vec<_> = pdar.collect();
            for order in write_vec {
                if let Some((name, head)) = order {
                    // retrieve part from cache
                    let name = name.to_lowercase();
                    let fnm = name
                        .split(|c| c == '-' || c == '_')
                        .next()
                        .expect("first name of part form k");
                    let par = match par_map.get(&name) {
                        Some(n) => n,
                        None => par_map.get(fnm).expect("toml has matched first name"),
                    };
                    // get the duckbubble organized id
                    let mat_cell = match par.mid {
                        Some(ref mid) => mid,
                        None => mid_map.get(&par.mat()).expect("mat in book"),
                    };
                    let sec_cell = match par.secid {
                        Some(ref sid) => sid,
                        None => sid_map.get(&par.sec()).expect("sec in book"),
                    };
                    // move a cell forward
                    file.seek(SeekFrom::Start(head + 10))?;
                    file.write_all(&sec_cell.0)?;
                    file.write_all(&mat_cell.0)?;
                }
            }
        }
    }
    repo_boy.main_key_compo(&dir_boy)?;
    Ok(())
}
