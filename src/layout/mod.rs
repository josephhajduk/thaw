mod layout_header;
mod layout_sider;

use crate::utils::mount_style;
pub use layout_header::*;
pub use layout_sider::*;
use leptos::*;

#[derive(Default, PartialEq)]
pub enum LayoutPosition {
    #[default]
    Static,
    Absolute,
}

impl LayoutPosition {
    pub fn as_str(&self) -> &str {
        match self {
            LayoutPosition::Static => "static",
            LayoutPosition::Absolute => "absolute",
        }
    }
}

#[component]
pub fn Layout(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(optional)] position: LayoutPosition,
    #[prop(optional, into)] has_sider: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("layout", include_str!("./layout.css"));

    let style = create_memo(move |_| {
        let mut style = style.get();
        if has_sider.get() {
            style.push_str("display: flex; flex-wrap: nowrap; flex-direction: row; width: 100%;")
        }
        style
    });
    view! {
        <div
            class="thaw-layout"
            class=("thaw-layout--absolute-positioned", position == LayoutPosition::Absolute)
            style=move || style.get()
        >
            {children()}
        </div>
    }
}
