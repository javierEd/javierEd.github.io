use dioxus::prelude::*;

mod app;
mod components;
mod constants;
mod hooks;
mod icons;
mod layout;
mod pages;

use app::App;
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

fn main() {
    dioxus_sdk::storage::set_dir!();
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
