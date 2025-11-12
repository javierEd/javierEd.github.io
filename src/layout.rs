use dioxus::prelude::*;

use crate::{LOGO_PNG, Routes};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "navbar bg-base-300",
            Link { class: "flex gap-3 p-2 font-bold text-lg", to: Routes::home(),
                img { class: "h-8 rounded", src: LOGO_PNG }

                span { "Javier E. - Software Developer" }
            }
        }

        Outlet::<Routes> {}
    }
}
