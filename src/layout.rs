use dioxus::prelude::*;

use crate::components::JobTitlesCarousel;
use crate::icons::{GitHubIcon, LinkedInIcon};
use crate::{LOGO_PNG, Routes};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "navbar bg-base-300",
            div { class: "navbar-start flex-1",
                Link {
                    class: "flex gap-3 p-2 font-bold text-lg",
                    to: Routes::home(),
                    img { class: "h-8 rounded", src: LOGO_PNG }

                    div {
                        "Javier E."
                        span { class: "max-sm:hidden",
                            " - "
                            JobTitlesCarousel {}
                        }
                    }
                }
            }

            div { class: "navbar-end flex-0 gap-2",
                a {
                    class: "btn btn-circle btn-ghost",
                    href: "https://github.com/javierEd",
                    target: "_blank",
                    title: "My GitHub profile",
                    GitHubIcon {}
                }

                a {
                    class: "btn btn-square btn-ghost",
                    href: "https://linkedin.com/in/javiered",
                    target: "_blank",
                    title: "My LinkedIn profile",
                    LinkedInIcon {}
                }
            }
        }

        Outlet::<Routes> {}
    }
}
