use serde_derive::Deserialize;
use std::{fs, path::PathBuf, u8};

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
    pub secid: Option<KeyCell>,
    pub mat: Option<String>,
    pub mid: Option<KeyCell>,
}

// e.g. the first cell of a keyword input is `id`
#[derive(Debug, Default, Deserialize, Clone, Copy)]
pub struct KeyCell(pub [u8; 10]);

impl Part {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }
    fn first_name(&self) -> String {
        let mut ast = self.name.split(|c| c == '-' || c == '_');
        ast.next().unwrap().to_owned()
    }
    // if not specified, default name goes to part's first name, e.g. soil-1-2 goes to soil
    pub fn sec(&self) -> String {
        match self.sec {
            Some(ref s) => s.clone(),
            None => self.first_name(),
        }
    }
    pub fn mat(&self) -> String {
        match self.mat {
            Some(ref m) => m.clone(),
            None => self.first_name(),
        }
    }
    pub fn mid_allo(&mut self, lord: &mut KeyId) {
        if self.mid.is_none() {
            lord.next().unwrap();
            self.mid = Some(lord.0);
        }
    }
    pub fn sid_allo(&mut self, lord: &mut KeyId) {
        if self.secid.is_none() {
            lord.next().unwrap();
            self.secid = Some(lord.0);
        }
    }
}

pub struct KeyId(KeyCell);

impl KeyId {
    pub fn new() -> Self {
        let mut kcell = [b'9'; 10];
        kcell[0] = b'1';
        KeyId(KeyCell(kcell))
    }
}

impl Iterator for KeyId {
    type Item = u64;

    // FIXME: I don't think this is a smart move.
    fn next(&mut self) -> Option<Self::Item> {
        // parse cell to u64
        let zro_str = String::from_utf8_lossy(&self.0 .0);
        let zro: u64 = zro_str.parse().expect("KeyId parse to u64");
        // do calculation
        let next = zro - 1;
        // convert back to string
        let next_str = next.to_string();
        // turn str into u8 iter
        let mut iter = next_str.bytes();
        for i in 0..10 {
            self.0 .0[i] = iter.next().unwrap();
        }
        Some(next)
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
