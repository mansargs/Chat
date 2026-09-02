use dioxus::prelude::*;

#[component]
pub fn Login(on_register: EventHandler<()>, on_login: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "auth-form",

            h1 { "Login" }

            input {
                placeholder: "Email",
                autocomplete: "off",
            }

            input {
                r#type: "password",
                placeholder: "Password",
                autocomplete: "off",
            }

            button {
                onclick: move |_| on_login(()),
                "Login"
            }

            p {
                "Don't have an account? "

                button {
                    onclick: move |_| on_register(()),
                    "Register"
                }
            }
        }
    }
}