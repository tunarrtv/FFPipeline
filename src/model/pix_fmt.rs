use std::fmt;
use once_cell::sync::Lazy;
use macros::make_pixel_format;

#[derive(Debug)]
pub(crate) struct StaticPixelFormat {
    pub name: String,
    pub ffmpeg_name: String,
    pub bit_depth: u8,
}

impl fmt::Display for StaticPixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}), bit_depth = {}", self.name, self.ffmpeg_name, self.bit_depth)
    }
}

pub(crate) trait PixelFormat {
    fn name(&self) -> &str;
    fn ffmpeg_name(&self) -> &str;
    fn bit_depth(&self) -> u8;
}

pub static YUV420P: Lazy<StaticPixelFormat> = Lazy::new(|| make_pixel_format!("yuv420p", 8)) ;