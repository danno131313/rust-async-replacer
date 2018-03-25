use super::*;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name="replacer", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct Opt {
    /// String that will be replaced.
    #[structopt(name="TARGET")]
    pub target: String,

    /// String to replace with.
    #[structopt(name="REPLACEMENT")]
    pub replacement: String,

    /// File(s) and/or directory(s) to search in.
    #[structopt(name="FILES", parse(from_os_str))]
    pub files: Vec<PathBuf>
}