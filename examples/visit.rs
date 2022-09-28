use std::{io, path::Path};

use duckbubble::orwritekey::{print_dir, rw_key_file};

fn main() -> io::Result<()> {
    let path = Path::new("./dyna-repo/src/models/shells.k");
    // print_dir(path)?;
    rw_key_file(path)?;
    Ok(())
}
