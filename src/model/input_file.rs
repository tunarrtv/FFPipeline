use crate::model::input_option::InputOption;
use crate::model::media_stream::{MediaStream, VideoStream};

// pub trait InputFile<'a> {
//     fn get_streams(&'a self) -> impl Iterator<Item = MediaStream<'a>>;
// 
//     fn add_option(&'a self, opt: &impl InputOption) {
//         if opt.applies_to() {
// 
//         }
//     }
// }

pub struct VideoInputFile {
    path: String,
    streams: Vec<VideoStream>
}

// impl <'a> InputFile<'a> for VideoInputFile {
//     fn get_streams(&'a self) -> impl Iterator<Item = MediaStream<'a>> {
//         self.streams.iter().map(MediaStream::Video)
//     }
// }

pub struct ConcatInputFile {

}

pub enum InputFile {
    Video(VideoInputFile),
    Concat(ConcatInputFile),
}