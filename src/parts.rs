use serde_derive::Deserialize;
use std::{
    fs::{self, File},
    io::{Seek, Write},
    u8,
};

type Opvec<T> = Option<Vec<T>>;

#[derive(Debug, Default, Deserialize)]
pub struct LsCfg {
    pub env_path: String,
    pub bin_path: String,
    pub job_path: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct ParamCfg {
    pub name: String,
    pub left: f64,
    pub right: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct DynaConfig {
    pub lsrun: LsCfg,
    pub param: ParamCfg,
    pub parts: Opvec<Part>,
    pub secs: Opvec<Section>,
    pub mats: Opvec<Material>,
    pub node_sets: Opvec<NodeSet>,
    pub bcs: Opvec<Bc>,
    pub inits: Opvec<Init>,
}

pub enum ConfigErr {
    Undefine,
}

impl DynaConfig {
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

impl From<u64> for KeyCell {
    fn from(value: u64) -> Self {
        let mut cell_buf = [b' '; 10];
        let val = value.to_string();
        let val_str = val.as_str().as_bytes().iter().take(9);
        for (i, xr) in val_str.enumerate() {
            cell_buf[i] = *xr;
        }
        KeyCell(cell_buf)
    }
}

impl From<f64> for KeyCell {
    fn from(value: f64) -> Self {
        let mut cell_buf = [b' '; 10];
        let val = value.to_string();
        let val_str = val.as_str().as_bytes().iter().take(9);
        for (i, xr) in val_str.enumerate() {
            cell_buf[i] = *xr;
        }
        KeyCell(cell_buf)
    }
}

impl KeyCell {
    // parse cell to u64
    pub fn to_num(&mut self) -> u64 {
        let zro_str = String::from_utf8_lossy(&self.0);
        zro_str.trim().parse().expect("a cell should parse to u64")
    }
    // to f64
    pub fn to_float(&self) -> f64 {
        let zro_str = String::from_utf8_lossy(&self.0);
        eprintln!("parsing Keycell float number {zro_str}");
        zro_str.trim().parse().expect("a cell should parse to f64")
    }
    pub fn parse_para(&mut self) -> (char, String) {
        let cell = self.0;
        let ty = cell[0] as char;
        let name = String::from_utf8(cell[1..].to_vec()).unwrap();
        (ty, name.trim().to_owned())
    }
    /// replace a cell after reading it
    pub fn replace(&self, cursor: u64, file: &mut File) {
        file.seek(std::io::SeekFrom::Start(cursor - 10)).unwrap();
        file.write_all(&self.0).unwrap();
    }
}

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
        kcell[0] = b' ';
        KeyId(KeyCell(kcell))
    }
}

impl Iterator for KeyId {
    type Item = u64;

    // FIXME: I don't think this is a smart move.
    fn next(&mut self) -> Option<Self::Item> {
        // parse cell to u64
        let zro_str = String::from_utf8_lossy(&self.0 .0);
        let zro: u64 = zro_str.trim().parse().expect("KeyId parse to u64");
        // do calculation
        let next = zro - 1;
        // convert back to string
        self.0 = KeyCell::from(next);
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
