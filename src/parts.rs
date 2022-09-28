use serde_derive::Deserialize;
use std::{fs, io, path::PathBuf, u8};

use crate::orwritekey::DirInRepo;

type Opvec<T> = Option<Vec<T>>;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub parts: Vec<Part>,
    pub secs: Opvec<Section>,
    pub mats: Opvec<Material>,
    pub node_sets: Opvec<NodeSet>,
    pub bcs: Opvec<Bc>,
    pub inits: Opvec<Init>,
}

pub enum ConfigErr {
    Undefine,
}

impl Config {
    pub fn read(path: &str) -> Self {
        let ctn = fs::read_to_string(path).expect("!config read from toml file!");
        toml::from_str(&ctn).expect("!read toml file from current dir!")
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Part {
    pub name: String,
    pub id: Option<u8>,
    pub sec: Option<String>,
    pub secid: Option<[u8; 10]>,
    pub mat: Option<String>,
    pub mid: Option<[u8; 10]>,
}

impl Part {
    pub fn sec(&self) -> &str {
        match self.sec {
            Some(ref s) => s,
            None => &self.name,
        }
    }

    pub fn mat(&self) -> &str {
        match self.mat {
            Some(ref m) => m,
            None => &self.name,
        }
    }
    pub fn path_to(&self, dir: DirInRepo) -> PathBuf {
        let mut path = dir.path();
        match dir {
            DirInRepo::Secs => path.push(self.sec()),
            DirInRepo::Mats => path.push(self.mat()),
            _ => panic!("part_path to that not defined yet"),
        }
        path
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Section {
    pub name: String,
    pub id: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Material {
    pub name: String,
    pub id: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Bc {
    pub name: String,
    pub id: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Init {
    pub name: String,
    pub id: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct NodeSet {
    pub name: String,
    pub id: Option<u8>,
}
