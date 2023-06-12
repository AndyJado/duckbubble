pub(crate) use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;

use crate::orwritekey::KeywordReader;
use crate::parts::{KeyCell, LsCfg, ParamCfg};

pub fn ls_run(cfg: LsCfg, paras: ParamCfg) {
    if !cfg!(target_os = "windows") {
        panic!("run dyna only works on windows pc now")
    }
    let (tx, rx) = channel();
    let LsCfg {
        env_path,
        bin_path,
        job_path,
    } = cfg;
    // canonicalize is not working properly for this job, cmd call pushd related
    let mut work = Path::new(&job_path).to_path_buf();
    eprintln!("submitting job path is {work:#?}");
    let stream = fs::read(&work).expect("job path is wrong, check dry.toml");
    work.pop();
    // modify file content
    for para in paras.paras {
        let mut ws = work.clone(); // workspace
        let tx = tx.clone();
        let new_dir = format!("{para}\\");
        ws.push(new_dir);
        fs::remove_dir_all(&ws).ok();
        fs::DirBuilder::new().recursive(true).create(&ws).unwrap();
        let job = para_change(paras.name.clone(), para, &ws, &stream);
        let run_cfg = RunCfg {
            dir: ws,
            job: job.to_string_lossy().to_string(),
            env: env_path.to_string(),
            bin: bin_path.to_string(),
        };
        tx.send(thread::spawn(|| run_job(run_cfg))).unwrap();
    }
    rx.recv().unwrap().join().unwrap();
}

// return job name
fn para_change(para_name: String, para_val: f64, dir: &PathBuf, stream: &Vec<u8>) -> PathBuf {
    let mut para_read = KeywordReader::new(stream);
    let mut new_k = dir.clone();
    new_k.push("run.key");
    let mut file = File::create(&new_k).unwrap();
    eprintln!("creating new job file for each para");
    file.write_all(&stream).unwrap();
    loop {
        para_read.find_kwd_a(crate::orwritekey::Keyword::Parameter);
        para_read.consume_comment_line();
        if ('R', para_name.clone()) == para_read.read_keycell_a().parse_para() {
            let para_cell = para_read.read_keycell_a();
            let old_val = para_cell.to_float();
            eprintln!("the old val of specified para is {old_val}");
            let cursor = para_read.seek_head();
            let new_val = para_val;
            let new_cell = KeyCell::from(new_val);
            new_cell.replace(cursor, &mut file);
            return new_k;
        };
    }
}

struct RunCfg {
    dir: PathBuf,
    job: String,
    env: String,
    bin: String,
}

fn run_job(cfg: RunCfg) {
    let RunCfg { dir, job, env, bin } = cfg;
    Command::new("cmd")
        .args(["/C", "pushd"])
        .arg(&dir)
        .args([
            "&&",
            "call",
            &env,
            "&&",
            "mpiexec",
            "-c",
            "10",
            "-aa",
            &bin,
            "i=",
            &job,
            "memory=44m",
            "&&",
            // see lsdyna manual vol1 database option
            "l2a",
            "binout0000",
        ])
        .output()
        .unwrap();
}
