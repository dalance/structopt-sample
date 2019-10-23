use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "App name")]
#[structopt(long_version(option_env!("LONG_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"))))]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Opt {
    #[structopt(name = "WORDS")]
    pub words: Vec<String>,

    #[structopt(short = "n")]
    pub count: Option<usize>,

    // before Rust 1.32
    //#[structopt(short = "p", long = "path", parse(from_os_str))]
    //pub path: PathBuf,
    #[structopt(short = "p", long = "path")]
    pub path: PathBuf,

    #[structopt(short = "r", long = "regex")]
    pub regex: Regex,

    #[structopt(
        short = "t",
        long = "threads",
        default_value(&THREADS),
        value_name = "NUM"
    )]
    pub threads: usize,

    #[structopt(
        short = "m",
        long = "method",
        possible_values(&Method::variants())
    )]
    pub method: Option<Method>,

    #[structopt(short = "a", conflicts_with_all(&["b", "c"]))]
    pub a: bool,

    #[structopt(short = "b", conflicts_with_all(&["a", "c"]))]
    pub b: bool,

    #[structopt(short = "c", conflicts_with_all(&["a", "b"]))]
    pub c: bool,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub sub: Sub,
}

#[derive(Debug, StructOpt)]
pub enum Sub {
    #[structopt(name = "sub1", about = "sub command1")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Sub1,
}

arg_enum! {
    #[derive(Debug)]
    pub enum Method {
        A,
        B,
        C,
    }
}

lazy_static! {
    static ref THREADS: String = format!("{}", num_cpus::get());
}

fn main() {
    let _ = Opt::from_args();
}
