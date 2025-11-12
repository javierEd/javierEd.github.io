use dioxus::prelude::*;

use crate::components::{JobTitlesCarousel, PageTitle};
use crate::hooks::use_job_title;

#[component]
pub fn FakeHomePage() -> Element {
    HomePage()
}

#[component]
pub fn HomePage() -> Element {
    let job_title = use_job_title();

    rsx! {
        PageTitle { "Home" }

        section { class: "hero p-4",
            div { class: "hero-content text-center",
                div {
                    h1 { class: "h1", "Hello there!" }

                    h2 { class: "h2", "I'm Javier E." }

                    h3 { class: "h3",
                        "A "

                        JobTitlesCarousel {}

                        " with some years of experience."
                    }
                }
            }
        }
    }
}
