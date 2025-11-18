use dioxus::prelude::*;

mod backend;
mod components;

use components::MainLayout;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Router {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/characters")]
    CharactersList {}
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const HERO_BG: Asset = asset!("/assets/hero_bg.jpg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {}
        Router::<Router> {}
    }
}

fn Home() -> Element {
    rsx!(
        div { class: "absolute h-screen w-full z-10 flex items-center justify-center h-full",
            div { class: "flex flex-col w-1/2 h-1/2 items-center justify-center",
                h1 { class: "text-white font-semibold text-5xl mb-2", "Genshin that Impacted" }
                h2 { class: "text-white-500 text-3xl mb-2", "This is a test" }
                div { class: "flex flex-row transition-all duration-200",
                    button { class: "bg-black/25 text-white py-2 px-4 m-1 rounded hover:bg-black/50",
                        "Characters"
                    }
                    button { class: "bg-black/25 text-white py-2 px-4 m-1 rounded hover:bg-black/50",
                        "I am feeling Lucky"
                    }
                }
            }

        }
    )
}

#[derive(Clone, PartialEq)]
struct Character {
    id: usize,
    name: String,
    image: String,
    level: u32,
    class: String,
    bio: String,
}
fn CharactersList() -> Element {
    let mut characters = vec![
        Character {
            id: 1,
            name: "Albedo".to_string(),
            image: "https://genshin.jmp.blue/characters/albedo/card".to_string(),
            level: 85,
            class: "Alchmist".to_string(),
            bio: "Master of the blade, defender of realms.".to_string(),
        },
        Character {
            id: 2,
            name: "Lira the Mage".to_string(),
            image: "https://via.placeholder.com/400x500/7C3AED/FFFFFF?text=Mage".to_string(),
            level: 92,
            class: "Mage".to_string(),
            bio: "Wields arcane powers beyond mortal comprehension.".to_string(),
        },
        Character {
            id: 3,
            name: "Kael the Rogue".to_string(),
            image: "https://via.placeholder.com/400x500/059669/FFFFFF?text=Rogue".to_string(),
            level: 78,
            class: "Rogue".to_string(),
            bio: "Shadow assassin, strikes from the darkness.".to_string(),
        },
        Character {
            id: 4,
            name: "Elara the Healer".to_string(),
            image: "https://via.placeholder.com/400x500/10B981/FFFFFF?text=Healer".to_string(),
            level: 89,
            class: "Healer".to_string(),
            bio: "Bringer of light, mender of the broken.".to_string(),
        },
    ];
    let char_elements = characters.iter().map(|char| {
        rsx! {
            div {
                class: "group relative bg-gray-900 rounded-2xl overflow-hidden shadow-2xl hover:shadow-purple-500/50 transition-all duration-300 hover:scale-105 cursor-pointer border border-gray-700 hover:border-purple-500/50",
                key: "{char.id}",
                div {
                    class: "relative h-96 bg-cover bg-center",
                    style: "background-image: url('{char.image}');",
                }
                div { class: "absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col justify-end p-6",
                    div { class: "text-white",
                        h3 { class: "text-2xl font-bold mb-1", "{char.name}" }
                        div { class: "text-sm opacity-90 mb-2", "Level {char.level} | {char.class}" }
                        p { class: "text-sm opacity-80 line-clamp-2", "{char.bio}" }
                    }
                }
                div { class: "absolute top-4 right-4 bg-black/50 text-white px-3 py-1 rounded-full text-sm opacity-0 group-hover:opacity-100 transition-opacity",
                    Link { to: "/characters/{char.id}", "Open" }
                }
            }
        }
    });

    rsx! {
        div { class: "container mx-auto px-4 py-12",
            h1 { class: "text-4xl font-bold text-center text-white mb-12 bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent drop-shadow-lg",
                "Characters"
            }
            div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8", {char_elements} }
        }
    }
}
