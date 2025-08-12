pub enum StepType {
    Global,
    Input,
    Output,
}

pub trait PipelineStep {
    fn get_type(&self) -> StepType;
    
    // Caller takes ownership of the returned vector
    fn get_options(&self) -> Vec<String>;
}