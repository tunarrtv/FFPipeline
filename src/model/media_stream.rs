use crate::model::{FrameSize, PixelFormat, StaticPixelFormat};

pub enum VideoScanKind {
    Unknown,
    Progressive,
    Interlaced,
}

pub struct VideoStream {
    index: u8,
    codec: String,
    profile: String,
    pixel_format: Option<StaticPixelFormat>,
    // color params
    frame_size: FrameSize,
    sample_aspect_ratio: Option<String>,
    display_aspect_ratio: String,
    frame_rate: Option<String>,
    is_still_image: bool,
    scan_kind: VideoScanKind,
}

pub struct AudioStream {
    index: u8,
    codec: String,
    channels: u8,
}

pub struct SubtitleStream {
    index: u8,
    codec: String,
}

pub enum MediaStream<'a> {
    Video(&'a VideoStream),
    Audio(&'a AudioStream),
    Subtitle(&'a SubtitleStream)
}