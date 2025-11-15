use dioxus::prelude::*;
use dioxus_i18n::prelude::i18n;

use crate::Routes;
use crate::components::JobTitlesCarousel;
use crate::constants::{COPYRIGHT, LANGUAGE_NAMES, LOGO_PNG};
use crate::icons::{CheckMini, ChevronUpMini, GitHubIcon, LinkedInIcon};

#[component]
pub fn Layout() -> Element {
    let mut i18n = i18n();
    let language_name = use_memo(move || *LANGUAGE_NAMES.get(&i18n.language()).unwrap());

    let language_names = || {
        let mut names = LANGUAGE_NAMES.iter().collect::<Vec<_>>();
        names.sort_by_key(|(_, name)| *name);
        names
    };

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
                            class: "btn btn-outline btn-accent w-36 justify-between",
                            tabindex: "0",
                            {language_name()}
                            ChevronUpMini {}
                        }

                        ul {
                            class: "dropdown-content menu bg-base-100 rounded-box w-36 p-2 mb-2 border-solid border border-accent",
                            tabindex: "0",
                            for (id , name) in language_names() {
                                li { class: if i18n.language() == *id { "menu-active" },
                                    a {
                                        class: "flex justify-between",
                                        onclick: move |_| i18n.set_language(id.clone()),
                                        {*name}
                                        if i18n.language() == *id {
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
