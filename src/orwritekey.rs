use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

pub struct KeywordReader<R: AsRef<[u8]>>(Cursor<R>);

// There is no boundary check!!
impl<R: AsRef<[u8]>> Iterator for KeywordReader<R> {
    type Item = Option<(String, u64)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_keyword_a() {
            Keyword::Part => Some(Some(self.process_part())),
            Keyword::End => None,
            _ => Some(None),
        }
    }
}

impl<R: AsRef<[u8]>> KeywordReader<R> {
    pub fn new(stream: R) -> Self {
        KeywordReader(Cursor::new(stream))
    }
    pub fn read_char(&mut self) -> u8 {
        let mut char_buf = [b' '; 1];
        self.0.read_exact(&mut char_buf).expect("!reading a char!");
        char_buf[0]
    }
    pub fn read_line(&mut self) -> String {
        let mut buf = String::new();
        self.0.read_line(&mut buf).expect("!reading a line!");
        buf
    }
    pub fn seek_head(&self) -> u64 {
        self.0.position()
    }
    pub fn read_keyword_a(&mut self) -> Keyword {
        while self.read_char() != b'*' {
            continue;
        }
        self.read_line()
            .parse::<Keyword>()
            .expect("parse readed keyword")
    }
    fn consume_comment_line(&mut self) {
        if self.read_char() == b'$' {
            //TODO: we can use comment line to help locating
            self.read_line();
        } else {
            self.0.set_position(self.seek_head() - 1)
        }
    }
    // below keyword, may have a comment line
    fn consume_title(&mut self) -> String {
        let ln = self.read_line();
        let trimed = trim_newline(&ln).trim_end();
        let v: Vec<&str> = trimed.split(|c| c == '-' || c == '_').collect();
        //FIXME: currently only the prefix of name is taken into consideration
        v.first().expect("keyword should has title").to_string()
    }
    // after located keyword, return position to be rewrite
    pub fn process_part(&mut self) -> (String, u64) {
        self.consume_comment_line();
        let name = self.consume_title();
        //TODO: write toml read info into model
        self.consume_comment_line();
        // now we at the beginning of line keycells
        dbg!(name, self.seek_head())
    }
}

// e.g. the first cell of a keyword input is `id`
pub struct KeyCell(SeekFrom, [u8; 10]);

impl KeyCell {
    pub fn write_to(self, f: &mut File) -> io::Result<()> {
        f.seek(self.0)?;
        f.write_all(&self.1)?;
        Ok(())
    }
}

// one calculation, one repo, dir structure should be conventional
pub enum DirInRepo {
    Secs,
    Mats,
    Models,
}

impl DirInRepo {
    //FIXME: assume we are in repo, so start with ./
    pub fn path(&self) -> PathBuf {
        //suffix
        let sfx = match self {
            DirInRepo::Secs => "sections",
            DirInRepo::Mats => "materials",
            DirInRepo::Models => "models",
        };
        let mut buf = PathBuf::from("./");
        buf.push(sfx);
        buf
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
        match trim_newline(s).trim() {
            "PART" => Ok(Self::Part),
            "SECTION_SHELL" => Ok(Self::Shell),
            "SECTION_SOLID" => Ok(Self::Solid),
            "SET_NODE_LIST" => Ok(Self::SetNode),
            "END" => Ok(Self::End),
            _ => Ok(Self::Undefined),
        }
    }
}

fn trim_newline(s: &str) -> &str {
    let patrn = |c: char| c == '\r' || c == '\n';
    s.trim_end_matches(patrn)
}

#[test]
fn test_newline_trim() {
    assert_eq!(trim_newline("!\r\n"), "!");
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
