// Ref : https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html
use structopt::StructOpt;

// Import the lib module
mod lib;

// Bring the Opts struct into scope
use lib::opts::Opts;
use lib::list;

// Grabs the command line arguments from the user
// executes the ls command.
fn main() {
    let args = Opts::from_args();
    list::exec_ls(args);
}
