use dioxus::prelude::*;

mod components;
mod constants;
mod hooks;
mod icons;
mod layout;
mod pages;

use constants::JOB_TITLES;
use layout::Layout;
use pages::{FakeHomePage, HomePage};

use crate::hooks::JobTitle;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Routes {
    #[layout(Layout)]
        #[route("/")]
        FakeHomePage,
        #[route("/home")]
        HomePage,
}

impl Routes {
    fn home() -> Self {
        Self::FakeHomePage
    }
}

const FAVICON_ICO: Asset = asset!("assets/favicon.ico");
const LOGO_PNG: Asset = asset!("assets/logo.png");
const STYLE_CSS: Asset = asset!("assets/style.css");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Routes::static_routes().iter().map(ToString::to_string).collect())
}

#[component]
fn App() -> Element {
    let mut job_title_index = use_signal(|| 0);
    let job_title = use_memo(move || JOB_TITLES[job_title_index()].to_owned());

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
    }
}
