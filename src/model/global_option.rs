use std::fmt;
use once_cell::sync::Lazy;
use super::pipeline_step;

pub struct GlobalOption {
    pub opts: Vec<String>,
}

impl<'a> pipeline_step::PipelineStep<'a> for GlobalOption {
    fn get_type(&self) -> pipeline_step::StepType {
        pipeline_step::StepType::Global
    }

    fn get_options(&'a self) -> impl Iterator<Item=&'a str> {
        self.opts.iter().map(|s| s.as_str())
    }
}

impl fmt::Display for GlobalOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.opts.join(" "))
    }
}

pub static HIDE_BANNER_OPTION: Lazy<GlobalOption> = Lazy::new(|| GlobalOption {
    opts: vec![String::from("-hide_banner")]
});
