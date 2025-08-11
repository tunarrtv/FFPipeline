mod model;
mod output_format;
mod capabilities;

use clap::Parser;
use crate::model::{StaticPixelFormat, YUV420P};

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
    println!("Hello, world! {}, {}", cli.verbose, *YUV420P)
}
