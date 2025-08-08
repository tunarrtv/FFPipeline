pub(crate) mod pipeline_step;
mod global_option;
mod pix_fmt;

pub use global_option::*;
pub use pix_fmt::*;

pub struct FrameState {
    realtime: bool,
    infinite_loop: bool,
    video_format: String,
    video_profile: Option<String>,
    video_preset: Option<String>,
    allow_b_frames: bool,

}

#[derive(macros::PixelFormat, Debug)]
pub struct Yuv420P { 
    name: String, 
    ffmpeg_name: String,
    bit_depth: u8 
}