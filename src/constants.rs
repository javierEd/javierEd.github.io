use std::collections::HashMap;
use std::sync::LazyLock;

use chrono::NaiveDate;
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
    language_names.insert(langid!("pt"), "Português");
    language_names
});

pub struct Experience<'a> {
    pub company_name: &'a str,
    pub position: &'a str,
    pub job_type: &'a str,
    pub location: &'a str,
    pub remote: bool,
    pub started_at: NaiveDate,
    pub ended_at: NaiveDate,
}

pub const EXPERIENCE: [Experience; 7] = [
    Experience {
        company_name: "ComunidadFeliz SpA",
        position: "Fullstack Developer",
        job_type: "Fulltime",
        location: "Buenos Aires, Argentina",
        remote: true,
        started_at: NaiveDate::from_ymd_opt(2022, 5, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2023, 2, 28).unwrap(),
    },
    Experience {
        company_name: "Vascar Solutions",
        position: "Backend Developer",
        job_type: "Freelance",
        location: "Cumaná, Venezuela",
        remote: true,
        started_at: NaiveDate::from_ymd_opt(2021, 4, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2021, 12, 31).unwrap(),
    },
    Experience {
        company_name: "SIM C.A.",
        position: "Backend Developer",
        job_type: "Freelance",
        location: "Cumaná, Venezuela",
        remote: false,
        started_at: NaiveDate::from_ymd_opt(2020, 11, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2021, 4, 30).unwrap(),
    },
    Experience {
        company_name: "SEAI Lab",
        position: "Mobile Developer",
        job_type: "Freelance",
        location: "Cumaná, Venezuela",
        remote: true,
        started_at: NaiveDate::from_ymd_opt(2019, 10, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2020, 4, 30).unwrap(),
    },
    Experience {
        company_name: "CESTICOM C.A.",
        position: "Software Developer",
        job_type: "Fulltime",
        location: "Cumaná, Venezuela",
        remote: false,
        started_at: NaiveDate::from_ymd_opt(2018, 5, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2019, 6, 30).unwrap(),
    },
    Experience {
        company_name: "Servicios VECA C.A.",
        position: "Backend Developer",
        job_type: "Fulltime",
        location: "Cumaná, Venezuela",
        remote: false,
        started_at: NaiveDate::from_ymd_opt(2017, 5, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2018, 5, 31).unwrap(),
    },
    Experience {
        company_name: "Sucre Municipality Town Hall",
        position: "Technical Support/Software Developer",
        job_type: "Fulltime",
        location: "Cumaná, Venezuela",
        remote: false,
        started_at: NaiveDate::from_ymd_opt(2010, 10, 1).unwrap(),
        ended_at: NaiveDate::from_ymd_opt(2017, 4, 30).unwrap(),
    },
];

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
