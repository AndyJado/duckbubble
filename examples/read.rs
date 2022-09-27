use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::SeekFrom;
use std::path::Path;
use std::str::FromStr;

fn main() -> io::Result<()> {
    rw_key_file("solids.k")
}

pub fn rw_key_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let stream = fs::read(&path)?;
    let mut file = File::options().write(true).open(path)?;
    let mut cursor = Cursor::new(stream);
    let nc = |c: &mut Cursor<Vec<u8>>| {
        let mut char_buf = [b' '; 1];
        c.read(&mut char_buf).unwrap();
        char_buf[0]
    };
    let nl = |c: &mut Cursor<Vec<u8>>| {
        let mut buf = String::new();
        c.read_line(&mut buf).unwrap();
        buf
    };
    loop {
        if nc(&mut cursor) == b'*' {
            let keyword = nl(&mut cursor).parse::<Keyword>().unwrap();
            dbg!(&keyword);
            match keyword {
                Keyword::Part => {
                    if nc(&mut cursor) == b'$' {
                        nl(&mut cursor);
                    }
                    // //FIXME: write toml memory to key file, position fixed!
                    let pos = SeekFrom::Start(cursor.position());
                    file.seek(pos)?;
                    file.write(b"LIOS")?;
                }
                Keyword::Shell => todo!(),
                Keyword::Solid => {}
                Keyword::SetNode => {}
                Keyword::Undefined => {
                    dbg!(&keyword);
                }
                Keyword::End => break,
            }
        }
    }
    Ok(())
}

fn write2part() {
    todo!()
}

#[derive(Debug)]
pub enum Keyword {
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

impl Keyword {
    pub fn some_new(from: &str) -> Option<Self> {
        match from {
            "PART" => Some(Self::Part),
            "SECTION_SHELL" => Some(Self::Shell),
            "SECTION_SOLID" => Some(Self::Solid),
            "SET_NODE_LIST" => Some(Self::SetNode),
            _ => None,
        }
    }
}
