pub(crate) use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

use crate::orwritekey::KeywordReader;
use crate::parts::{KeyCell, LsCfg, ParamCfg};

pub fn ls_run(cfg: LsCfg, para: ParamCfg) {
    if !cfg!(target_os = "windows") {
        panic!("run dyna only works on windows pc now")
    }
    let LsCfg {
        env_path,
        bin_path,
        job_path,
    } = cfg;
    // canonicalize is not working properly for this job, cmd call pushd related
    let mut work = Path::new(&job_path).to_path_buf();
    dbg!(&work);
    let stream = fs::read(&work).unwrap();
    let mut file = File::options().write(true).open(job_path.clone()).unwrap();
    let mut para_read = KeywordReader::new(stream);
    work.pop();
    let mut left = work.clone();
    left.push("left\\");
    fs::DirBuilder::new().recursive(true).create(&left).unwrap();
    let mut right = work.clone();
    right.push("right\\");
    fs::DirBuilder::new()
        .recursive(true)
        .create(&right)
        .unwrap();
    dbg!(&left);
    // work.push()
    let output = |dir| {
        Command::new("cmd")
            .args(["/C", "pushd"])
            .arg(&dir)
            .args([
                "&&",
                "call",
                &env_path,
                "&&",
                "mpiexec",
                "-c",
                "10",
                "-aa",
                &bin_path,
                "i=",
                &job_path,
                "memory=44m",
            ])
            .output()
            .unwrap();
    };
    // output(&work);
    //TODO: modify file content
    loop {
        para_read.find_kwd_a(crate::orwritekey::Keyword::Parameter);
        para_read.consume_comment_line();
        if ('R', para.name.clone()) == para_read.read_keycell_a().parse_para() {
            let para_cell = para_read.read_keycell_a();
            let old_val = para_cell.to_float();
            let cursor = para_read.seek_head();
            let new_val = para.left;
            let new_cell = KeyCell::from(new_val);
            new_cell.replace(cursor, &mut file);
            dbg!(old_val);
            break;
        };
    }
    output(&left);
    //TODO: modify file content
    output(&right);
}
