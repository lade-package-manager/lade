use crate::paths::{lade_bin_path, lade_build_path};
use crate::{crash, err, log, write_log};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[macro_export]
macro_rules! exec_shellscript {
    ($word: expr) => {{
        if $word.first().unwrap() != &"__install_end__" {
            let mut child = std::process::Command::new($word.first().unwrap())
                .args($word.iter().skip(1))
                .current_dir(crate::paths::lade_build_path())
                .spawn()
                .unwrap();
            child.wait().unwrap();
        }
    }};
}

macro_rules! chmod {
    ($path: expr) => {
        if !cfg!(target_os = "windows") {
            let mut child = std::process::Command::new("chmod")
                .arg("+x")
                .arg($path)
                .spawn()
                .unwrap();
            child.wait().unwrap();
        }
    };
}

#[allow(unused)]
fn unzip_file_lade<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    let file = fs::File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).unwrap();
    let build_dir = lade_build_path();

    if build_dir.exists() {
        fs::remove_dir_all(&build_dir).unwrap();
    }
    fs::create_dir_all(&build_dir).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = build_dir.join(file.mangled_name());

        if file.is_dir() {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

pub fn unzip_and_install_lade<P: AsRef<Path>>(path: P, repo: &str) {
    unzip_file_lade(&path);

    let install_sh = lade_build_path().join("install.sh");

    if install_sh.exists() {
        chmod!(&install_sh);
        let file = fs::File::open(&install_sh).unwrap();
        let reader = BufReader::new(file);
        for read in reader.lines() {
            let line = read.unwrap();
            let words: Vec<&str> = line.split_whitespace().collect();
            exec_shellscript!(words);
        }
    } else {
        err!("Failed to find install script");
        log!(
            format!("Failed to find install script\n repository: {}", repo),
            "failed to find install script"
        );
        crash!(3);
    }

    let exec_name_path = lade_build_path().join("exec_name");

    let exec_name = fs::read_to_string(exec_name_path).unwrap_or_else(|e| {
        if e.kind() != std::io::ErrorKind::NotFound {
            err!("Failed to read exec_name file", e);
            log!("Failed to read exec_name file", e);
            crash!();
        } else {
            repo.to_string()
        }
    });

    match fs::rename(
        lade_build_path().join(exec_name.trim()),
        lade_bin_path().join(exec_name.trim()),
    ) {
        Ok(_) => {}
        Err(e) => {
            err!("Failed to move executable file", e);
            log!("Failed to move executable file", e);
            crash!(1);
        }
    }

    chmod!(lade_bin_path().join(exec_name.trim()));
}
