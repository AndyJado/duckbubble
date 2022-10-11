use std::fs;
use std::fs::DirEntry;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;
use std::str::FromStr;

pub struct KeywordReader<R: AsRef<[u8]>>(Cursor<R>);

pub struct PartReader<R: AsRef<[u8]>>(pub KeywordReader<R>);

// There is no boundary check!!
impl<R: AsRef<[u8]>> Iterator for PartReader<R> {
    type Item = Option<(String, u64)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.read_keyword_a().1 {
            Keyword::Part => Some(Some(self.0.process_part())),
            Keyword::End => None,
            _ => Some(None),
        }
    }
}

impl<R: AsRef<[u8]>> KeywordReader<R> {
    pub fn new(stream: R) -> Self {
        KeywordReader(Cursor::new(stream))
    }
    /// cursor move a char, return read char
    pub fn read_char(&mut self) -> u8 {
        let mut char_buf = [b' '; 1];
        self.0.read_exact(&mut char_buf).expect("!reading a char!");
        char_buf[0]
    }
    /// cursor move after \n, return read string
    pub fn read_line(&mut self) -> String {
        let mut buf = String::new();
        self.0.read_line(&mut buf).expect("!reading a line!");
        buf
    }
    /// seek head postion
    pub fn seek_head(&self) -> u64 {
        self.0.position()
    }
    pub fn seek_foward(&mut self, n: u64) {
        self.0.set_position(self.seek_head() + n)
    }
    pub fn seek_back(&mut self, n: u64) {
        self.0.set_position(self.seek_head() - n)
    }
    /// cursor after *
    fn find_keyword(&mut self) {
        while self.read_char() != b'*' {
            continue;
        }
    }
    pub fn read_keyword_a(&mut self) -> (u64, Keyword) {
        self.find_keyword();
        (
            self.seek_head(),
            self.read_line()
                .parse::<Keyword>()
                .expect("parse readed keyword"),
        )
    }
    fn consume_comment_line(&mut self) {
        loop {
            if self.read_char() == b'$' {
                //TODO: can use comment line to help locating and do hell more stuff
                self.read_line();
            } else {
                self.seek_back(1);
                break;
            }
        }
    }
    /// e.g. "MAT_ELASTIC" return "MAT"
    fn consume_prefix(&mut self) -> String {
        let ln = self.read_line();
        let v: Vec<&str> = ln.trim().split(|c| c == '-' || c == '_').collect();
        //FIXME: currently only the prefix of name is taken into consideration
        v.first().expect("keyword should has title").to_string()
    }
    /// `MAT` & `SECTION` id cursor position in key file
    pub fn process_part_attri(&mut self) -> u64 {
        self.find_keyword();
        let pref = self.consume_prefix();
        let s = pref.as_str();
        match s {
            "MAT" | "SECTION" => {
                self.consume_comment_line();
                self.seek_head()
            }
            _ => panic!("no mat or section in provided .key file!!"),
        }
    }
    /// after located keyword, return position to be rewrite
    pub fn process_part(&mut self) -> (String, u64) {
        // below keyword, may have a comment line
        self.consume_comment_line();
        let name = self.read_line().trim().to_string();
        //TODO: write toml read info into model
        self.consume_comment_line();
        // now we at the beginning of line keycells
        dbg!(name, self.seek_head())
    }
}

#[derive(Debug)]
pub enum Keyword {
    // *,&,name,$,cells
    Part,
    Shell,
    Solid,
    SetNode,
    End,
    Undefined,
}

#[derive(Debug)]
pub enum KwdErr {
    Undefined,
}

impl FromStr for Keyword {
    type Err = KwdErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "PART" => Ok(Self::Part),
            "SECTION_SHELL" => Ok(Self::Shell),
            "SECTION_SOLID" => Ok(Self::Solid),
            "SET_NODE_LIST" => Ok(Self::SetNode),
            "END" => Ok(Self::End),
            _ => Ok(Self::Undefined),
        }
    }
}

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub fn print_dir(dir: &Path) -> io::Result<()> {
    let print_path = |f: &DirEntry| println!("{}", f.path().display());
    visit_dirs(dir, &print_path)?;
    Ok(())
}
