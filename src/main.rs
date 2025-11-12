use dioxus::prelude::*;

mod layout;
mod pages;

use layout::Layout;
use pages::{FakeHomePage, HomePage};

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
    rsx! {
        document::Link { rel: "icon", href: FAVICON_ICO }
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        Router::<Routes> {}
    }
}
