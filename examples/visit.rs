use std::{fs, io, path::Path};

use duckbubble::orwritekey::KeywordReader;

fn main() -> io::Result<()> {
    // let path = Path::new("./dyna-repo/src/models/shells.k");
    let path = Path::new("./dyna-repo/src/models/solids.k");
    let stream = fs::read(path)?;
    let kdar = KeywordReader::new(stream);
    kdar.for_each(|c| match c {
        Some(order) => {
            dbg!(order);
        }
        None => {}
    });
    Ok(())
}
