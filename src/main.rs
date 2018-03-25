extern crate rayon;
#[macro_use]
extern crate structopt;

use args::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs::{metadata, read_dir};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

mod args;

fn main() {
    let opt: Opt = Opt::from_args();

    if opt.files.is_empty() {
        println!("No files to work on!");
        std::process::exit(0);
    }

    let target = opt.target.clone();
    let replacement = opt.replacement.clone();

    let mut files = Vec::new();

    for path in opt.files {
        let metadata_result = metadata(&path);

        if metadata_result.is_err() {
            println!("Couldn't read file {:?}", &path);
            continue;
        }

        let mdata = metadata_result.unwrap();

        if mdata.is_dir() {
            process_dir(path, &mut files).unwrap();
        } else {
            files.push(path);
        }
    }

    let _: Vec<()> = files
        .into_par_iter()
        .map(|file: PathBuf| {
            process_file(&target, &replacement, file).unwrap();
        })
        .collect();
}

fn process_dir(path: PathBuf, files: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    let dir = read_dir(path)?;

    for item in dir {
        let result = item?.path();
        let mdata = result.metadata()?;

        if mdata.is_dir() {
            process_dir(result, files)?;
        } else {
            files.push(result);
        }
    }

    Ok(())
}

fn process_file(target: &str, replacement: &str, path: PathBuf) -> Result<(), std::io::Error> {
    println!("Processing file {:?}", &path);

    let mut file = File::open(&path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    drop(file);

    let new_contents = contents.replace(target, replacement);

    let mut dest = File::create(&path)?;
    dest.write_all(&new_contents.as_bytes())?;

    Ok(())
}
