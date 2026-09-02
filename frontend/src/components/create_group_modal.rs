use dioxus::prelude::*;

#[component]
pub fn CreateGroupModal(
    on_close: EventHandler<()>,
    on_create: EventHandler<String>,
) -> Element {
    let mut group_name = use_signal(|| String::new());
    // let mut members = use_signal(|| String::new());

    let available_members = vec![
        "Alice",
        "Bob",
        "John",
        "Charlie",
        "Diana",
        "Eve",
    ];

    let mut selected_members: Signal<Vec<String>> = use_signal(Vec::new);

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

                    h2 { "Create New Group" }

                    button {
                        class: "modal-close",
                        onclick: move |_| on_close(()),
                        "✕"
                    }
                }

                div {
                    class: "modal-body",

                    // Group Name Input
                    div {
                        class: "form-group",

                        label { "Group Name" }

                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Enter group name...",
                            value: "{group_name}",
                            oninput: move |evt| group_name.set(evt.value()),
                        }
                    }

                    // Members Selection
                    div {
                        class: "form-group",

                        label { "Add Members" }

                        div {
                            class: "members-list",

                            for member in available_members.iter().cloned().collect::<Vec<_>>() {
                                div {
                                    class: "member-item",

                                    input {
                                        r#type: "checkbox",
                                        id: "member_{member}",
                                        onchange: move |evt| {
                                            if evt.checked() {
                                                selected_members.write().push(member.to_string());
                                            } else {
                                                selected_members.write().retain(|m| m != &member);
                                            }
                                        },
                                    }

                                    label {
                                        r#for: "member_{member}",
                                        "{member}"
                                    }
                                }
                            }
                        }
                    }

                    // Selected Members Display
                    if !selected_members().is_empty() {
                        div {
                            class: "selected-members",

                            p { "Selected: {selected_members().len()} members" }

                            div {
                                class: "members-tags",

                                for member in selected_members().iter().cloned().collect::<Vec<_>>() {
                                    div {
                                        class: "member-tag",

                                        span { "{member}" }

                                        button {
                                            class: "tag-remove",
                                            onclick: move |_| {
                                                selected_members.write().retain(|m| m != &member);
                                            },
                                            "✕"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "modal-footer",

                    button {
                        class: "btn-secondary",
                        onclick: move |_| on_close(()),
                        "Cancel"
                    }

                    button {
                        class: "btn-primary",
                        disabled: group_name().is_empty() || selected_members().is_empty(),
                        onclick: move |_| {
                            on_create(group_name());
                        },
                        "Create Group"
                    }
                }
            }
        }
    }
}
