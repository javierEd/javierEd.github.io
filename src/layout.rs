use dioxus::prelude::*;

use crate::Routes;
use crate::components::JobTitlesCarousel;
use crate::constants::{COPYRIGHT, LANGUAGE_NAMES, LOGO_PNG};
use crate::hooks::use_language;
use crate::icons::{CheckMini, ChevronUpMini, GitHubIcon, LinkedInIcon};

#[component]
pub fn Layout() -> Element {
    let mut language = use_language();
    let language_name = use_memo(move || *LANGUAGE_NAMES.get(&language()).unwrap());

    rsx! {
        div { class: "flex flex-col min-h-screen",
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

            main { class: "main grow", Outlet::<Routes> {} }

            footer { class: "footer",
                aside { class: "opacity-75",
                    p {
                        "Version: "
                        {env!("CARGO_PKG_VERSION")}
                        " ("
                        {env!("GIT_REV_SHORT")}
                        ")"
                    }

                    p {
                        "Built on: "
                        {env!("BUILD_DATETIME")}
                    }

                    p { {COPYRIGHT} }
                }

                nav {
                    div { class: "dropdown dropdown-top",
                        button {
                            class: "btn btn-outline btn-accent w-30 justify-between",
                            tabindex: "0",
                            {language_name()}
                            ChevronUpMini {}
                        }

                        ul {
                            class: "dropdown-content menu bg-base-100 rounded-box w-30 p-2 shadow",
                            tabindex: "0",
                            for (id , name) in LANGUAGE_NAMES.iter() {
                                li { class: if language() == *id { "menu-active" },
                                    a {
                                        class: "flex justify-between",
                                        onclick: move |_| language.set((*id).to_owned()),
                                        {*name}
                                        if language() == *id {
                                            CheckMini {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
