use dioxus::prelude::*;

#[component]
pub fn Register(on_login: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "auth-form",

            h1 { "Register" }

            input {
                placeholder: "Username",
                autocomplete: "off",
            }

            input {
                placeholder: "Email",
                autocomplete: "off",
            }

            input {
                r#type: "password",
                placeholder: "Password",
                autocomplete: "off",
            }

            input {
                r#type: "password",
                placeholder: "Confirm password",
                autocomplete: "off",
            }

            button {
                "Register"
            }

            p {
                "Already have an account? "

                button {
                    onclick: move |_| on_login(()),
                    "Login"
                }
            }
        }
    }
}