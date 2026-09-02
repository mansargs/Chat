use dioxus::prelude::*;

use crate::layout::AuthLayout;
use crate::pages::{Login, Register, Chat};

#[derive(Clone, Copy, PartialEq)]
enum AuthPage {
    Login,
    Register,
}

#[derive(Clone, Copy, PartialEq)]
enum AppPage {
    Auth(AuthPage),
    Chat,
}

pub fn app() -> Element {
    let mut page = use_signal(|| AppPage::Auth(AuthPage::Login));

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("assets/styles/app.css"),
        }

        document::Link {
            rel: "stylesheet",
            href: asset!("assets/styles/auth.css"),
        }

        document::Link {
            rel: "stylesheet",
            href: asset!("assets/styles/chat.css"),
        }

        match page() {
            AppPage::Auth(auth_page) => rsx! {
                AuthLayout {
                    match auth_page {
                        AuthPage::Login => rsx! {
                            Login {
                                on_register: move |_| page.set(AppPage::Auth(AuthPage::Register)),
                                on_login: move |_| page.set(AppPage::Chat),
                            }
                        },

                        AuthPage::Register => rsx! {
                            Register {
                                on_login: move |_| page.set(AppPage::Auth(AuthPage::Login))
                            }
                        },
                    }
                }
            },

            AppPage::Chat => rsx! {
                Chat {
                    on_logout: move |_| page.set(AppPage::Auth(AuthPage::Login))
                }
            },
        }
    }
}