use dioxus::prelude::*;

#[component]
pub fn AuthLayout(children: Element) -> Element {
    rsx! {
        div {
            class: "auth-layout",

            img {
                class: "auth-logo",
                src: asset!("/assets/icons/chat.png"),
            }

            div {
                class: "auth-card",

                {children}
            }
        }
    }
}