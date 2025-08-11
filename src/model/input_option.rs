use crate::model::input_file::InputFile;
use crate::model::{pipeline_step, GlobalOption};
use crate::model::pipeline_step::PipelineStep;

pub trait InputOption {
    fn applies_to(&self, input: &InputFile) -> bool;
}

pub struct ReadrateInputOption {
    initial_burst_seconds: u16
}

impl InputOption for ReadrateInputOption {
    fn applies_to(&self, input: &InputFile) -> bool {
        match input {
            InputFile::Video(_) => true,
            InputFile::Concat(_) => true
        }
    }
}

impl<'a> pipeline_step::PipelineStep<'a> for ReadrateInputOption {
    fn get_options(&'a self) -> impl Iterator<Item=&'a str> {


    }
}