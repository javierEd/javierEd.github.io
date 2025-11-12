use dioxus::prelude::*;

#[component]
pub fn FakeHomePage() -> Element {
    HomePage()
}

#[component]
pub fn HomePage() -> Element {
    rsx! {}
}
