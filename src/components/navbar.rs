use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "bg-black/25 p-4 flex items-center justify-end space-x-4",
            Link {
                to: "/",
                class: "px-4 py-2 text-white rounded-xl hover:bg-blue-700 transition-colors z-10",
                "Home"
            }
            Link {
                to: "/characters",
                class: "px-4 py-2 text-white rounded-xl hover:bg-blue-700 transition-colors z-10",
                "Characters"
            }
            Link {
                to: "/about",
                class: "px-4 py-2 text-white rounded-xl hover:bg-blue-700 transition-colors z-10",
                "About"
            }
        }
    }
}
