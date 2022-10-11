use std::{
    env::Args,
    fs::{self, File},
    io::{self, Seek, SeekFrom},
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
        cre(self.src)?;
        cre(self.models)?;
        cre(self.materials)?;
        cre(self.sections)
    }
    pub fn main_key_compo(&self) -> io::Result<()> {
        let main_k_path = Path::new("main.k");
        if !main_k_path.is_file() {
            panic!("current repo no main.k")
        };
        let stream = fs::read(main_k_path).expect("read main.k");
        let mut kdar = KeywordReader::new(stream);
        let head = kdar.find_kwd_a(orwritekey::Keyword::End);
        let mut main_k = File::options()
            .write(true)
            .open(main_k_path)
            .expect("open main.k for write");
        main_k
            .seek(SeekFrom::Start(head))
            .expect("seek *END in main.k");
        Ok(())
    }
    fn walk_key(&self) {
        // dir reader, didar, an iterator next() `DirEntry`
        let didar = |p: &PathBuf| fs::read_dir(p).expect("dir exists").filter_map(|c| c.ok());
        let m_d_dar = didar(&self.materials);
        let f_paths = m_d_dar
            .map(|c| c.path())
            .filter(|p| p.extension().is_some())
            .filter(|p| p.extension().unwrap() == "k")
            .collect::<Vec<_>>();
    }
}

trait DirWalk {}
