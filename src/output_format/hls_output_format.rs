use crate::model;

pub struct HlsOutputFormat {
    pub opts: Vec<String>,
}

impl HlsOutputFormat {

}

impl<'a> crate::model::pipeline_step::PipelineStep<'a> for HlsOutputFormat {
    fn get_type(&self) -> crate::model::pipeline_step::StepType {
        model::pipeline_step::StepType::Global
    }

    fn get_options(&'a self) -> impl Iterator<Item=&'a str> {
        self.opts.iter().map(|s| s.as_str())
    }
}
