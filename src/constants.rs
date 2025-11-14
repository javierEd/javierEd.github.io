use std::{collections::HashMap, sync::LazyLock};

use dioxus::prelude::{Asset, asset, manganis};
use unic_langid::{LanguageIdentifier, langid};

pub const COPYRIGHT: &str = "© 2025 Javier E.";

pub const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
pub const LOGO_PNG: Asset = asset!("assets/logo.png");
pub const STYLE_CSS: Asset = asset!("assets/style.css");

pub static LANGUAGE_NAMES: LazyLock<HashMap<LanguageIdentifier, &str>> = LazyLock::new(|| {
    let mut language_names = HashMap::new();
    language_names.insert(langid!("en"), "English");
    language_names.insert(langid!("es"), "Español");
    language_names
});

pub const JOB_TITLES: [&str; 27] = [
    "Software Developer",
    "Backend Developer",
    "Frontend Developer",
    "Fullstack Developer",
    "Ruby Developer",
    "Rust Developer",
    "Javascript Developer",
    "React Developer",
    "Mobile Developer",
    "Flutter Developer",
    "Software Engineer",
    "Backend Engineer",
    "Frontend Engineer",
    "Fullstack Engineer",
    "Ruby Engineer",
    "Rust Engineer",
    "Javascript Engineer",
    "React Engineer",
    "Mobile Engineer",
    "Flutter Engineer",
    "Programmer",
    "Coder",
    "Loving Husband",
    "Amateur Gardener",
    "Aikidoka",
    "Iaijutsuka",
    "Human Being",
];
