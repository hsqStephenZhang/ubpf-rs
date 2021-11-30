use structopt::StructOpt;

mod error;
mod utils;
mod runtime;
pub use runtime::*;
pub use utils::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt.debug);
}
