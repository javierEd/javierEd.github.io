use chrono::{Locale, NaiveDate};
use convert_case::{Case, Casing};
use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;
use dioxus_i18n::t;

use crate::components::{JobTitlesCarousel, PageTitle};
use crate::constants::EXPERIENCE;

fn date_to_text(value: NaiveDate, locale: Locale) -> String {
    value.format_localized("%B %Y", locale).to_string().to_case(Case::Title)
}

#[component]
pub fn AboutPage() -> Element {
    let i18n = i18n();
    let chrono_locale = use_memo(move || match i18n.language().to_string().as_str() {
        "es" => Locale::es_VE,
        "pt" => Locale::pt_BR,
        _ => Locale::en_US,
    });

    rsx! {
        PageTitle { {t!("about-me")} }

        h1 { class: "h1", {t!("about-me")} }

        section {
            h2 { class: "h2", {t!("professional-experience")} }

            for exp in EXPERIENCE {
                div { class: "mb-5",
                    h3 { class: "h3 opacity-90", {exp.company_name} }

                    div { class: "opacity-80 grid max-sm:grid-cols-1 grid-cols-4 gap-2",
                        div { class: "font-bold", {exp.position} }
                        div { {exp.job_type} }
                        div {
                            {date_to_text(exp.started_at, chrono_locale())}
                            " - "
                            {date_to_text(exp.ended_at, chrono_locale())}
                        }
                        div {
                            if exp.remote {
                                {t!("remote")}
                            } else {
                                {exp.location}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FakeHomePage() -> Element {
    HomePage()
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        PageTitle { {t!("home")} }

        section { class: "hero p-4",
            div { class: "hero-content text-center",
                div {
                    h1 { class: "h1", {t!("hello-there")} }

                    h2 { class: "h2", {t!("im-javier")} }

                    h3 { class: "h3",
                        {t!("a")}
                        " "
                        JobTitlesCarousel {}
                        " "
                        {t!("with-some-years-of-experience")}
                    }
                }
            }
        }
    }
}
