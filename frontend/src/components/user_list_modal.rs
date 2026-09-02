use dioxus::prelude::*;

#[component]
pub fn UserListModal(
    on_close: EventHandler<()>,
    on_select: EventHandler<String>,
) -> Element {
    let available_users = vec![
        ("Alice", "● online"),
        ("Bob", "● online"),
        ("Charlie", "● online"),
        ("Diana", "● away"),
        ("Eve", "● offline"),
        ("Frank", "● online"),
        ("Grace", "● online"),
        ("Henry", "● offline"),
    ];

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| on_close(()),

            div {
                class: "modal-content",
                onclick: move |evt: MouseEvent| {
                    evt.stop_propagation();
                },

                div {
                    class: "modal-header",

                    h2 { "Start Direct Message" }

                    button {
                        class: "modal-close",
                        onclick: move |_| on_close(()),
                        "✕"
                    }
                }

                div {
                    class: "modal-body user-list-modal",

                    div {
                        class: "user-list",

                        for (name, status) in available_users.iter().cloned().collect::<Vec<_>>() {
                            div {
                                class: "user-list-item",
                                ondoubleclick: move |_| {
                                    on_select(name.to_string());
                                },

                                div {
                                    class: "user-info",

                                    span {
                                        class: "user-name",
                                        "{name}"
                                    }

                                    span {
                                        class: "user-status",
                                        "{status}"
                                    }
                                }

                                span {
                                    class: "hint-text",
                                    "Double-click to message"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
