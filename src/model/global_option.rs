use std::fmt;
use once_cell::sync::Lazy;
use super::pipeline_step;

pub struct GlobalOption {
    pub opts: Vec<&'static str>,
}

impl pipeline_step::PipelineStep for GlobalOption {
    fn get_type(&self) -> pipeline_step::StepType {
        pipeline_step::StepType::Global
    }

    fn get_options(&self) -> Vec<String> {
        self.opts.iter().map(|s| s.to_string()).collect()
    }
}

impl fmt::Display for GlobalOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.opts.join(" "))
    }
}

pub static HIDE_BANNER_OPTION: Lazy<GlobalOption> = Lazy::new(|| GlobalOption {
    opts: vec!["-hide_banner"]
});
