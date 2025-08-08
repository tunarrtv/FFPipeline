mod model;
mod output_format;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Args::parse();
    println!("Hello, world! {}, {}", cli.verbose, *model::HIDE_BANNER_OPTION)
}
