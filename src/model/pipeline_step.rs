pub enum StepType {
    Global,
    Input,
    Output,
}

pub trait PipelineStep<'a> {
    fn get_type(&self) -> StepType;
    fn get_options(&'a self) -> &'a [String];
}