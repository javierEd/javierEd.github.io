use dioxus::prelude::*;
use dioxus_i18n::prelude::{I18nConfig, use_init_i18n};
use dioxus_sdk::storage::{LocalStorage, use_storage};
use unic_langid::{LanguageIdentifier, langid};

use crate::Routes;
use crate::constants::{FAVICON_ICO, JOB_TITLES, LOGO_PNG, STYLE_CSS};
use crate::hooks::JobTitle;

#[component]
pub fn App() -> Element {
    let mut is_starting = use_signal(|| true);
    let mut job_title_index = use_signal(|| 0);
    let job_title = use_memo(move || JOB_TITLES[job_title_index()].to_owned());
    let mut language_storage =
        use_storage::<LocalStorage, LanguageIdentifier>("_language".to_owned(), || langid!("en"));

    let i18n = use_init_i18n(|| {
        I18nConfig::new(language_storage())
            .with_fallback(langid!("es"))
            .with_locale((langid!("en"), include_str!("../locales/en.ftl")))
            .with_locale((langid!("es"), include_str!("../locales/es.ftl")))
            .with_locale((langid!("pt"), include_str!("../locales/pt.ftl")))
    });

    use_effect(move || {
        language_storage.set(i18n.language());
    });

    use_effect(move || {
        is_starting.set(false);
    });

    use_future(move || async move {
        loop {
            gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;

            job_title_index.set((job_title_index() + 1) % JOB_TITLES.len());
        }
    });

    use_context_provider(|| JobTitle(job_title));

    rsx! {
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }

        Router::<Routes> {}

        div { class: "splash", class: if !is_starting() { "splash-hidden" },
            figure {
                div { class: "splash-pulse" }

                img { src: LOGO_PNG }
            }
        }
    }
}
