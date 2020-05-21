use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opts {

    #[structopt(parse(from_os_str))]
    pub object: Option<PathBuf>,

    #[structopt(short = "a")]
    pub hidden: bool,

    #[structopt(short = "l")]
    pub long: bool,

    #[structopt(short = "h")]
    pub human: bool,
}
