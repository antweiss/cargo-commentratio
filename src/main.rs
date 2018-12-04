#[macro_use]
extern crate quicli;
extern crate console;
extern crate regex;
extern crate indicatif;
extern crate walkdir;

//mod cargo;
mod emoji;
mod progressbar;
mod calculate;

use console::style;
use quicli::prelude::*;
use std::env;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Cli {
    #[structopt(name = "commentratio")]
    Commentratio(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "threshold", short = "t", default_value = "1")]
    threshold: usize
}

main!(|_cli: Cli| {
    let args: Args = match Cli::from_args() {
        Cli::Commentratio(args) => args,
    };

    let threshold = args.threshold;

    println!(
        "{} {} `{}`{}",
        emoji::WRENCH,
        style("Calculating comment rate").bold(),
        style(&threshold).bold().yellow(),
        style("...").bold()
    );

    let project_dir = env::current_dir().unwrap();

    let pbar = progressbar::new();
    pbar.tick();
    //calc ratio
    let ratio = calculate::calculate(&project_dir, pbar)?;

    println!(
        "{} {} {} {}",
        emoji::SPARKLE,
        style("Done!").bold().green(),
        style("Comment to code ratio is").bold(),
        style(&format!("{:.2}",ratio)).underlined()
    );
});
