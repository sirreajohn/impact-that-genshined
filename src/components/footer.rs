use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "text-white p-4 text-center",
            p { "© 2025 Mahesh Patapalli. All rights reserved." }
        }
    }
}
