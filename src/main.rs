use argh::FromArgs;
use hashbrown::HashSet;
use rayon::prelude::*;
use regex::Regex;
// use std::collections::HashSet;
use std::error::Error;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{path::PathBuf, str::FromStr};

#[derive(FromArgs)]
/// identify the ABI extentions in use in a binary.
struct Arghs {
    /// path to the binary object.
    #[argh(positional, from_str_fn(into_pathbuf))]
    path: PathBuf,
}

// `Argh` parse helper function to parse the argument into a `PathBuf`.
fn into_pathbuf(s: &str) -> Result<PathBuf, String> {
    // `PathBuf::from_str` is infallible, therefore unwrap is fine and we will never return `String`.
    Ok(PathBuf::from_str(s).unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let bin: Arghs = argh::from_env();

    let od_args = [
        "--disassemble",
        " --wide",
        "--prefix-addresses",
        "--show-raw-insn",
    ];

    let bin_str = bin.path.as_path().to_str().expect("valid path to binary");

    let stdout = Command::new("objdump")
        .args(od_args)
        .arg(bin_str)
        .output()?
        .stdout;

    let stdout_utf8: String = String::from_utf8(stdout)?;
    let mut stdout_vec: Vec<String> = Vec::new();
    for line in stdout_utf8.lines() {
        stdout_vec.push(line.to_string());
    }

    // capture groups: 'Address', 'Label', 'Hex', 'Instruction
    let rgx =
        Regex::new(r"^([0-9a-f]+)\s+<(.*)>\s+([0-9a-f][0-9a-f]( [0-9a-f][0-9a-f])*)\s+(.*?)$")?;

    let extensions: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::default()));

    stdout_vec.into_par_iter().for_each(|line| {
        for cap in rgx.captures_iter(&line) {
            let hx = cap.get(3).expect("could not get third match group");
            let ext = get_ext_zydisinfo(hx.as_str());
            let mut guard = extensions.lock().expect("lock extension collection set");
            let _ = guard.insert(ext);
        }
    });

    println!("Extensions found: ");
    let guard = extensions.lock().expect("unable to obtain lock");
    guard.iter().for_each(|ext| print!("{ext} "));
    println!();
    Ok(())
}

fn get_ext_zydisinfo(s: &str) -> String {
    let stdout = Command::new("ZydisInfo")
        .arg("-64")
        .arg(s)
        .output()
        .expect("zydisinfo failed")
        .stdout;

    let stdout_utf8: String = String::from_utf8(stdout).expect("utf8 expected");
    let mut ext = String::new();

    for line in stdout_utf8.lines() {
        if line.contains("ISA-EXT") {
            let (_, e) = line.split_once(':').expect("split failed");
            let e = e.trim();
            ext.push_str(e);
        }
    }
    ext
}
