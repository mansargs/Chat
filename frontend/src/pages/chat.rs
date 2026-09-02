use dioxus::prelude::*;
use crate::components::{CreateGroupModal, UserListModal};

#[component]
pub fn Chat(on_logout: EventHandler<()>) -> Element {
    let mut active_conversations = use_signal(|| vec![("John", "● online")]);
    let mut active_chat_user = use_signal(|| "John".to_string());
    let mut message_input = use_signal(|| String::new());
    let mut show_create_group = use_signal(|| false);
    let mut show_user_list = use_signal(|| false);

    let groups = vec![("Rust Developers", ""), ("Friends", "")];

    let messages = vec![
        ("John", "Hello!", true),
        ("You", "Hi!", false),
        ("John", "How are you?", true),
    ];

    let conversations = active_conversations().iter().cloned().collect::<Vec<_>>();

    rsx! {
        div {
            class: "chat-container",

            // Sidebar
            div {
                class: "chat-sidebar",

                div {
                    class: "sidebar-header",

                    h2 { "Chats" }

                    button {
                        class: "logout-btn",
                        title: "Logout",
                        onclick: move |_| on_logout(()),
                        "🚪"
                    }
                }

                // Search bar
                div {
                    class: "search-bar",

                    input {
                        r#type: "text",
                        placeholder: "🔍 Search",
                        class: "search-input",
                    }
                }

                // Direct Messages
                div {
                    class: "chat-section",

                    div {
                        class: "groups-header",

                        h3 { "DIRECT MESSAGES" }

                        button {
                            class: "add-group-btn",
                            title: "Add direct message",
                            onclick: move |_| show_user_list.set(true),
                            "+"
                        }
                    }

                    for (name, status) in conversations.iter().cloned().collect::<Vec<_>>() {
                        div {
                            class: if active_chat_user() == name.to_string() {
                                "chat-item active"
                            } else {
                                "chat-item"
                            },
                            onclick: move |_| active_chat_user.set(name.to_string()),

                            span {
                                class: "chat-name",
                                "{name}"
                            }

                            span {
                                class: "chat-status",
                                "{status}"
                            }
                        }
                    }
                }

                // Groups
                div {
                    class: "chat-section",

                    div {
                        class: "groups-header",

                        h3 { "GROUPS" }

                        button {
                            class: "add-group-btn",
                            title: "Create new group",
                            onclick: move |_| show_create_group.set(true),
                            "+"
                        }
                    }

                    for (name, _) in groups.iter() {
                        div {
                            class: "chat-item",

                            span {
                                class: "chat-name",
                                "{name}"
                            }
                        }
                    }
                }
            }

            // Main Chat Area
            div {
                class: "chat-main",

                // Chat Header
                div {
                    class: "chat-header",

                    div {
                        class: "chat-header-info",

                        h2 { "{active_chat_user()}" }

                        p { "● online" }
                    }
                }

                // Messages
                div {
                    class: "chat-messages",

                    for (sender, text, is_other) in messages.iter() {
                        div {
                            class: if *is_other { "message other" } else { "message own" },

                            span {
                                class: "message-sender",
                                "{sender}"
                            }

                            p {
                                class: "message-text",
                                "{text}"
                            }
                        }
                    }
                }

                // Message Input
                div {
                    class: "chat-input-area",

                    div {
                        class: "chat-input-wrapper",

                        input {
                            class: "chat-input",
                            r#type: "text",
                            placeholder: "Write a message...",
                            value: "{message_input}",
                            oninput: move |evt| message_input.set(evt.value()),
                        }

                        button {
                            class: "send-btn",
                            onclick: move |_| {
                                if !message_input().is_empty() {
                                    message_input.set(String::new());
                                }
                            },
                            "Send"
                        }
                    }
                }
            }

            // Create Group Modal
            if show_create_group() {
                CreateGroupModal {
                    on_close: move |_| show_create_group.set(false),
                    on_create: move |group_name: String| {
                        println!("Creating group: {}", group_name);
                        show_create_group.set(false);
                    },
                }
            }

            // User List Modal
            if show_user_list() {
                UserListModal {
                    on_close: move |_| show_user_list.set(false),
                    on_select: move |user_name: String| {
                        let user_status = match user_name.as_str() {
                            "Diana" => "● away".to_string(),
                            "Eve" | "Henry" => "● offline".to_string(),
                            _ => "● online".to_string(),
                        };
                        
                        // Check if user is already in conversations
                        let mut convos = active_conversations();
                        if !convos.iter().any(|(name, _)| *name == user_name) {
                            convos.push((user_name.clone().leak(), user_status.leak()));
                        }
                        active_conversations.set(convos);
                        active_chat_user.set(user_name);
                        show_user_list.set(false);
                    },
                }
            }
        }
    }
}
