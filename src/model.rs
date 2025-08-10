pub(crate) mod pipeline_step;
mod global_option;
mod pix_fmt;

pub use global_option::*;
pub use pix_fmt::*;

#[derive(Debug)]
pub struct FrameSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub enum FrameDataLocation {
    Unknown,
    Software,
    Hardware,
}

#[derive(Debug)]
pub struct FrameState<PixFmt : PixelFormat> {
    realtime: bool,
    infinite_loop: bool,
    video_format: String,
    video_profile: Option<String>,
    video_preset: Option<String>,
    allow_b_frames: bool,
    pixel_format: Option<PixFmt>,
    scaled_size: FrameSize,
    padded_size: FrameSize,
    cropped_size: Option<FrameSize>,
    is_anamorphic: bool,
    frame_rate: Option<u32>,
    video_bit_rate: Option<u32>,
    video_buffer_size: Option<u32>,
    video_track_timescale: Option<u32>,
    deinterlaced: bool,
    frame_data_location: FrameDataLocation
}

impl<PixelFmt: PixelFormat> FrameState<PixelFmt> {
    pub fn ffmpeg_aspect_ratio(&self) -> &str {
        if self.padded_size.width == 640 {
            "4/3"
        } else {
            "16/9"
        }
    }

    pub fn bit_depth(&self) -> u8 {
        self.pixel_format.as_ref().map(|fmt| fmt.bit_depth()).unwrap_or(8)
    }
}

// #[derive(macros::PixelFormat, Debug)]
// pub struct Yuv420P {
//     name: String,
//     ffmpeg_name: String,
//     bit_depth: u8
// }