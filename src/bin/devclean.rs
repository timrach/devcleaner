use clap::Parser;
use devclean::crossterm::run;
use std::{error::Error, path::Path, time::Duration};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let search_path = Path::new(&args.path);
    run(Duration::from_millis(250), search_path)
}
