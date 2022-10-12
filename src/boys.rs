use std::{
    env::Args,
    fs::{self, File},
    io::{self, LineWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::orwritekey::{self, KeywordReader};

/// read argment return command
pub enum Argommand {
    Init,
    Link,
}

#[derive(Debug)]
pub enum ArgoErr {
    Duh,
}

impl FromStr for Argommand {
    type Err = ArgoErr;
    // so je eventually need to `match a string` between env and a hand coded one
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "init" {
            Ok(Argommand::Init)
        } else {
            Err(ArgoErr::Duh)
        }
    }
}

/// Deal with arguments from command line
pub struct ArgBoy(Args);

impl ArgBoy {
    pub fn new() -> Self {
        let args = std::env::args();
        ArgBoy(args)
    }
    pub fn errand(&mut self) -> Argommand {
        dbg!(self.0.next());
        match self.0.next() {
            Some(ref s) => s.parse().expect("now only has `init`"),
            None => Argommand::Link,
        }
    }
}

/// deal with files entries relate
#[derive(Debug)]
pub struct RepoBoy {
    pub src: PathBuf,
    pub models: PathBuf,
    pub sections: PathBuf,
    pub materials: PathBuf,
}

impl RepoBoy {
    /// create RepoBoy in current entry, create dirs if not there
    pub fn new() -> Self {
        let paf = |s: &str| PathBuf::from(s);
        let src = paf("src");
        let models = paf("src/models");
        let sections = paf("src/sections");
        let materials = paf("src/materials");
        RepoBoy {
            src,
            models,
            sections,
            materials,
        }
    }
    pub fn init(self) -> io::Result<()> {
        let cre = |paf: PathBuf| {
            if !paf.exists() {
                fs::create_dir(paf)
            } else {
                Ok(())
            }
        };
        File::create("dry.toml")?;
        let mut f = File::create("main.key")?;
        f.write_all(b"*END")?;
        cre(self.src)?;
        cre(self.models)?;
        cre(self.materials)?;
        cre(self.sections)
    }
    /// link all cards in repo to main.k
    pub fn main_key_compo(&self, ddar: &DirWalker) -> io::Result<()> {
        // assume in `repo/`
        let main_k_path = Path::new("main.k");
        if !main_k_path.is_file() {
            panic!("current repo no main.k")
        };
        // cursor main.k
        let stream = fs::read(main_k_path).unwrap();
        let mut kdar = KeywordReader::new(stream);
        let head = kdar.find_kwd_a(orwritekey::Keyword::End);
        let mut file = File::options()
            .write(true)
            .open(main_k_path)
            .expect("open main.k for write");
        file.seek(SeekFrom::Start(head))
            .expect("should seek `*END` in main.k");
        // *INCLUDE
        let mut ln_wtr = LineWriter::new(file);
        ln_wtr.write_all(b"*INCLUDE\n").expect("write *INCLUDE");
        // read file names from `secs` `mats` and write to main
        let mut mats_k_p = ddar.key_paf_vec(self.materials.clone());
        let mut secs_k_p = ddar.key_paf_vec(self.sections.clone());
        let itr_secs = secs_k_p.iter_mut().filter_map(|c| c.as_os_str().to_str());
        let itr_mats = mats_k_p.iter_mut().filter_map(|c| c.as_os_str().to_str());
        let itr = itr_secs.chain(itr_mats);
        for i in itr {
            ln_wtr.write_all(i.as_bytes()).expect("writing to main.k");
            ln_wtr.write_all(b"\n").expect(r"writing `\n`");
        }
        // *INCLUDE_OFFSET..
        ln_wtr
            .write_all(b"*INCLUDE_AUTO_OFFSET\n")
            .expect("write *INCLUDE");
        // read models and write to main
        let mut modls_k_p = ddar.key_paf_vec(self.models.clone());
        let itr_modls = modls_k_p.iter_mut().filter_map(|c| c.as_os_str().to_str());
        for i in itr_modls {
            ln_wtr.write_all(i.as_bytes()).expect("writing to main.k");
            ln_wtr.write_all(b"\n").expect(r"writing `\n`");
        }
        //*END
        ln_wtr.write_all(b"*END\n").expect("write *END");
        Ok(())
    }
}

pub struct KeyPath<T: AsRef<Path>>(pub T);

impl<T: AsRef<Path>> KeyPath<T> {
    //FIXME: this is so ugly, sosososososo ugly
    pub fn is_k(&self) -> bool {
        let sufx = self.0.as_ref().extension();
        match sufx {
            Some(ref k) => *k == "k",
            None => false,
        }
    }
}

type WalkDir = Box<dyn Fn(PathBuf) -> std::fs::ReadDir>;

pub struct DirWalker {
    iter: WalkDir,
}

impl DirWalker {
    pub fn new() -> Self {
        let iter = Box::new(|p| fs::read_dir(p).expect("dir should exists"));
        DirWalker { iter }
    }
    pub fn key_paf_vec(&self, p: PathBuf) -> Vec<PathBuf> {
        let a = &self.iter;
        let b = a(p);
        b.filter_map(|c| c.ok()).map(|e| e.path()).collect()
    }
}
