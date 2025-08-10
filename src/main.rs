mod model;
mod output_format;

use clap::Parser;
use crate::model::{StaticPixelFormat, Yuv420P};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Args::parse();
    let pix = StaticPixelFormat {
        name: "".to_string(),
        ffmpeg_name: "".to_string(),
        bit_depth: 0,
    };
    println!("Hello, world! {}, {}", cli.verbose, *Yuv420P)
}
