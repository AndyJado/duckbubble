use serde_derive::Deserialize;
use std::{fs, u8};

type Opvec<T> = Option<Vec<T>>;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub parts: Vec<Part>,
    pub secs: Opvec<Section>,
    pub mats: Opvec<Material>,
    pub bcs: Opvec<Bc>,
    pub inits: Opvec<Init>,
}

impl Config {
    pub fn read(path: &str) -> Self {
        let ctn = fs::read_to_string(path).expect("!config read from toml file!");
        toml::from_str(&ctn).expect("!toml rom str!")
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Part {
    pub name: String,
    pub id: Option<u8>,
    sec: Option<String>,
    mat: Option<String>,
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
