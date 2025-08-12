use crate::capabilities;
use crate::model::input_file::InputFile;
use crate::model::pipeline_step;
use crate::model::pipeline_step::StepType;

pub trait InputOption {
    fn applies_to(&self, input: &InputFile) -> bool;
}

pub struct ReadrateInputOption<'a> {
    initial_burst_seconds: u16,
    binary_capabilities: &'a capabilities::BinaryCapabilities,
}

impl<'a> InputOption for ReadrateInputOption<'a> {
    fn applies_to(&self, input: &InputFile) -> bool {
        match input {
            InputFile::Video(_) => true,
            InputFile::Concat(_) => true,
        }
    }
}

impl<'a> pipeline_step::PipelineStep for ReadrateInputOption<'a> {
    fn get_type(&self) -> StepType {
        StepType::Input
    }

    fn get_options(&self) -> Vec<String> {
        let mut result = vec!["-readrate".to_string(), "1.0".to_string()];
        if self.initial_burst_seconds > 0
            && self.binary_capabilities.has_option("initial_burst_seconds")
        {
            result.extend(
                [
                    "-initial_burst_seconds".to_string(),
                    self.initial_burst_seconds.to_string(),
                ]
            );
        }

        result
    }
}
