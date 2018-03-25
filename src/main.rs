#[macro_use]
extern crate structopt;

use args::*;
use std::fs::{metadata, read_dir};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;
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

    // Channel to receive files to work on
    let (send, recv) = channel();
    let sender = Arc::new(Mutex::new(send));

    // Start a thread to search directories and add files to the channel
    spawn(move || {
        for path in opt.files {
            let metadata_result = metadata(&path);

            if metadata_result.is_err() {
                println!("Couldn't read file {:?}", &path);
                continue;
            }

            let mdata = metadata_result.unwrap();

            if mdata.is_dir() {
                process_dir(path, sender.clone()).unwrap();
            } else {
                let x = sender.lock().unwrap();
                x.send(path).unwrap();
            }
        }
    });

    for file in recv {
        process_file(&target, &replacement, file).unwrap();
    }
}

fn process_dir(path: PathBuf, send: Arc<Mutex<Sender<PathBuf>>>) -> Result<(), std::io::Error> {
    let dir = read_dir(path)?;

    for item in dir {
        let result = item?.path();
        let mdata = result.metadata()?;

        if mdata.is_dir() {
            process_dir(result, send.clone())?;
        } else {
            let x = send.lock().unwrap();
            x.send(result).unwrap();
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