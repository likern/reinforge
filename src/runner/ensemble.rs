use crate::runner::run::Run;

#[derive(Debug)]
pub struct Ensemble {
    pub runs: Vec<Run>,
}

impl Ensemble {
    pub fn new() -> Self {
        Self { runs: Vec::new() }
    }
}
