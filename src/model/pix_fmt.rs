pub(crate) trait PixelFormat {
    fn get_name(&self) -> &str;
    fn get_ffmpeg_name(&self) -> &str;
    fn get_bit_depth(&self) -> u8;
}


