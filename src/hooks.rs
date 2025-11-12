use std::fmt::Display;

use dioxus::prelude::*;

#[derive(Clone)]
pub struct JobTitle(pub Memo<String>);

impl JobTitle {
    pub fn value(&self) -> String {
        self.0()
    }
}

impl Display for JobTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

pub fn use_job_title() -> JobTitle {
    use_context::<JobTitle>()
}
