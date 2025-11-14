use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::{JobTitlesCarousel, PageTitle};

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
