use dioxus::prelude::*;
use crate::components::CreateGroupModal;

#[derive(Clone, PartialEq)]
struct Conversation {
    name: String,
    status: String,
    is_group: bool,
}

#[component]
pub fn Chat(on_logout: EventHandler<()>) -> Element {
    let mut active_conversations = use_signal(|| {
        vec![
            Conversation {
                name: "John".to_string(),
                status: "● online".to_string(),
                is_group: false,
            }
        ]
    });

    let mut active_chat_user = use_signal(|| "John".to_string());
    let mut message_input = use_signal(|| String::new());
    let mut show_create_group = use_signal(|| false);
    let mut sidebar_tab = use_signal(|| "directs");

    let mut chats_search = use_signal(|| String::new());
    let mut groups_search = use_signal(|| String::new());
    let mut users_search = use_signal(|| String::new());

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

    let messages = vec![
        ("John", "Hello!", true),
        ("You", "Hi!", false),
        ("John", "How are you?", true),
    ];

    // ============================================================
    // CHATS
    // Only conversations where is_group == false
    // ============================================================

    let chats = active_conversations()
        .into_iter()
        .filter(|conversation| !conversation.is_group)
        .filter(|conversation| {
            conversation
                .name
                .to_lowercase()
                .contains(&chats_search().to_lowercase())
        })
        .collect::<Vec<_>>();

    // ============================================================
    // GROUPS
    // Only conversations where is_group == true
    // ============================================================

    let groups = active_conversations()
        .into_iter()
        .filter(|conversation| conversation.is_group)
        .filter(|conversation| {
            conversation
                .name
                .to_lowercase()
                .contains(&groups_search().to_lowercase())
        })
        .collect::<Vec<_>>();

    // ============================================================
    // USERS
    // ============================================================

    let users = available_users
        .iter()
        .cloned()
        .filter(|(name, _)| {
            name.to_lowercase()
                .contains(&users_search().to_lowercase())
        })
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "chat-container",

            // ====================================================
            // SIDEBAR
            // ====================================================

            div {
                class: "chat-sidebar",

                // ------------------------------------------------
                // Sidebar Header
                // ------------------------------------------------

                div {
                    class: "sidebar-header",

                    h2 {
                        "Messages"
                    }

                    button {
                        class: "logout-btn",
                        title: "Logout",

                        onclick: move |_| {
                            on_logout(());
                        },

                        img {
                            src: asset!("/assets/icons/logout.png"),
                            alt: "Logout",
                        }
                    }
                }

                // ------------------------------------------------
                // Sidebar Tabs
                // ------------------------------------------------

                div {
                    class: "sidebar-tabs",

                    button {
                        class: if sidebar_tab() == "directs" {
                            "sidebar-tab active"
                        } else {
                            "sidebar-tab"
                        },

                        onclick: move |_| {
                            sidebar_tab.set("directs");
                        },

                        "Directs"
                    }

                    button {
                        class: if sidebar_tab() == "groups" {
                            "sidebar-tab active"
                        } else {
                            "sidebar-tab"
                        },

                        onclick: move |_| {
                            sidebar_tab.set("groups");
                        },

                        "Groups"
                    }

                    button {
                        class: if sidebar_tab() == "users" {
                            "sidebar-tab active"
                        } else {
                            "sidebar-tab"
                        },

                        onclick: move |_| {
                            sidebar_tab.set("users");
                        },

                        "Users"
                    }
                }

                // =================================================
                // CHATS TAB
                // =================================================

                if sidebar_tab() == "directs" {
                    div {
                        class: "search-bar",

                        input {
                            r#type: "text",
                            placeholder: "Search chats",
                            class: "search-input",
                            value: "{chats_search}",

                            oninput: move |evt| {
                                chats_search.set(evt.value());
                            },
                        }
                    }

                    div {
                        class: "chat-section chat-list-section",

                        div {
                            class: "groups-header",

                            h3 {
                                "ACTIVE CHATS"
                            }
                        }

                        for conversation in chats.iter() {
                            div {
                                class: if active_chat_user() == conversation.name {
                                    "chat-item active"
                                } else {
                                    "chat-item"
                                },

                                onclick: {
                                    let name = conversation.name.clone();

                                    move |_| {
                                        active_chat_user.set(name.clone());
                                    }
                                },

                                span {
                                    class: "chat-avatar",
                                    ""
                                }

                                span {
                                    class: "chat-name",
                                    "{conversation.name}"
                                }

                                span {
                                    class: "chat-status",
                                    "{conversation.status}"
                                }

                                button {
                                    class: "delete-chat-btn",
                                    title: "Remove chat",

                                    onclick: {
                                        let name = conversation.name.clone();

                                        move |evt| {
                                            evt.stop_propagation();

                                            active_conversations.set(
                                                active_conversations()
                                                    .into_iter()
                                                    .filter(|item| item.name != name)
                                                    .collect()
                                            );

                                            if active_chat_user() == name {
                                                active_chat_user.set(String::new());
                                            }
                                        }
                                    },

                                    "×"
                                }
                            }
                        }
                    }
                }

                // =================================================
                // GROUPS TAB
                // =================================================

                else if sidebar_tab() == "groups" {
                    div {
                        class: "search-bar",

                        input {
                            r#type: "text",
                            placeholder: "Search groups",
                            class: "search-input",
                            value: "{groups_search}",

                            oninput: move |evt| {
                                groups_search.set(evt.value());
                            },
                        }
                    }

                    div {
                        class: "chat-section chat-list-section",

                        div {
                            class: "groups-header",

                            h3 {
                                "GROUPS"
                            }

                            button {
                                class: "add-group-btn",
                                title: "Create new group",

                                onclick: move |_| {
                                    show_create_group.set(true);
                                },

                                "+"
                            }
                        }

                        for conversation in groups.iter() {
                            div {
                                class: if active_chat_user() == conversation.name {
                                    "chat-item active"
                                } else {
                                    "chat-item"
                                },

                                onclick: {
                                    let name = conversation.name.clone();

                                    move |_| {
                                        active_chat_user.set(name.clone());
                                    }
                                },

                                span {
                                    class: "chat-avatar",
                                    "#"
                                }

                                span {
                                    class: "chat-name",
                                    "{conversation.name}"
                                }

                                span {
                                    class: "chat-status",
                                    "{conversation.status}"
                                }

                                button {
                                    class: "delete-chat-btn",
                                    title: "Remove group",

                                    onclick: {
                                        let name = conversation.name.clone();

                                        move |evt| {
                                            evt.stop_propagation();

                                            active_conversations.set(
                                                active_conversations()
                                                    .into_iter()
                                                    .filter(|item| item.name != name)
                                                    .collect()
                                            );

                                            if active_chat_user() == name {
                                                active_chat_user.set(String::new());
                                            }
                                        }
                                    },

                                    "×"
                                }
                            }
                        }
                    }
                }

                // =================================================
                // USERS TAB
                // =================================================

                else {
                    div {
                        class: "search-bar",

                        input {
                            r#type: "text",
                            placeholder: "Search users",
                            class: "search-input",
                            value: "{users_search}",

                            oninput: move |evt| {
                                users_search.set(evt.value());
                            },
                        }
                    }

                    div {
                        class: "chat-section user-directory",

                        h3 {
                            "ALL USERS"
                        }

                        for (name, status) in users.iter().cloned() {
                            div {
                                class: "chat-item",
                                title: "Double-click to open chat",

                                ondoubleclick: {
                                    let user_name = name.to_string();
                                    let user_status = status.to_string();

                                    move |_| {
                                        let mut convos = active_conversations();

                                        if !convos.iter().any(|item| {
                                            item.name == user_name
                                        }) {
                                            convos.push(
                                                Conversation {
                                                    name: user_name.clone(),
                                                    status: user_status.clone(),
                                                    is_group: false,
                                                }
                                            );

                                            active_conversations.set(convos);
                                        }

                                        active_chat_user.set(user_name.clone());
                                        sidebar_tab.set("chats");
                                    }
                                },

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
                }
            }

            // ====================================================
            // MAIN CHAT AREA
            // ====================================================

            div {
                class: "chat-main",

                // ------------------------------------------------
                // Chat Header
                // ------------------------------------------------

                div {
                    class: "chat-header",

                    div {
                        class: "chat-header-info",

                        h2 {
                            "{active_chat_user()}"
                        }

                        p {
                            "● online"
                        }
                    }
                }

                // ------------------------------------------------
                // Messages
                // ------------------------------------------------

                div {
                    class: "chat-messages",

                    for (sender, text, is_other) in messages.iter() {
                        div {
                            class: if *is_other {
                                "message other"
                            } else {
                                "message own"
                            },

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

                // ------------------------------------------------
                // Message Input
                // ------------------------------------------------

                div {
                    class: "chat-input-area",

                    div {
                        class: "chat-input-wrapper",

                        input {
                            class: "chat-input",
                            r#type: "text",
                            placeholder: "Write a message...",
                            value: "{message_input}",

                            oninput: move |evt| {
                                message_input.set(evt.value());
                            },
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

            // ====================================================
            // CREATE GROUP MODAL
            // ====================================================

            if show_create_group() {
                CreateGroupModal {
                    on_close: move |_| {
                        show_create_group.set(false);
                    },

                    on_create: move |group_name: String| {
                        active_conversations.write().push(
                            Conversation {
                                name: group_name.clone(),
                                status: "group".to_string(),
                                is_group: true,
                            }
                        );

                        active_chat_user.set(group_name);
                        show_create_group.set(false);
                    },
                }
            }
        }
    }
}

