use dioxus::core::{DynamicNode, Template, TemplateNode};
use dioxus::prelude::*;

#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use web_sys::wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{HtmlDivElement, HtmlSpanElement};

use crate::hooks::use_job_title;

#[component]
pub fn JobTitlesCarousel() -> Element {
    let job_title = use_job_title();
    let mut job_titles = use_signal(|| vec![job_title.value()]);

    #[cfg(feature = "web")]
    let mut element: Signal<Option<HtmlSpanElement>> = use_signal(|| None);

    use_effect(move || {
        job_titles.push(job_title.value());

        if job_titles.peek().len() > 2 {
            let _ = job_titles.remove(0);
        }
    });

    #[cfg(feature = "web")]
    use_effect(move || {
        let _ = job_titles.read();

        if let Some(element) = &*element.peek() {
            let _ = element.remove_attribute("style");
            element.set_scroll_top(0);

            let child_nodes = element.child_nodes();
            let last_child = child_nodes
                .get(child_nodes.length() - 1)
                .and_then(|node| node.dyn_into::<HtmlDivElement>().ok());

            if let Some(last_child) = last_child {
                let last_child_width = last_child.client_width();
                let last_child_height = last_child.client_height();
                let _ = element.set_attribute(
                    "style",
                    &format!(
                        "scroll-behavior: smooth; max-width: {}px; max-height: {}px",
                        last_child_width, last_child_height
                    ),
                );
            }

            element.set_scroll_top(element.scroll_height());
        }
    });

    rsx! {
        span {
            class: "job-titles-carousel",
            onmounted: move |event| {
                #[cfg(feature = "web")]
                element.set(event.data().as_web_event().dyn_into::<HtmlSpanElement>().ok());
            },
            for job_title in job_titles() {
                div { {job_title} }
            }
        }
    }
}

#[component]
pub fn PageTitle(children: Element) -> Element {
    let job_title = use_job_title();

    let vnode = children?;
    let page_title = match vnode.template {
        Template {
            roots: &[TemplateNode::Text { text }],
            node_paths: &[],
            attr_paths: &[],
            ..
        } => text.to_string(),
        Template {
            roots: &[TemplateNode::Dynamic { id }],
            node_paths: &[&[0]],
            attr_paths: &[],
            ..
        } => {
            let node = &vnode.dynamic_nodes[id];
            match node {
                DynamicNode::Text(text) => text.value.clone(),
                _ => {
                    return rsx!();
                }
            }
        }
        _ => {
            return rsx!();
        }
    };

    rsx! {
        document::Title { "{page_title} | Javier E. [{job_title}]" }
    }
}
