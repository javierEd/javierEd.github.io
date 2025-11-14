use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_sdk::storage::{LocalStorage, use_storage};
use unic_langid::{LanguageIdentifier, langid};

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

pub fn use_language() -> Signal<LanguageIdentifier> {
    use_context()
}
