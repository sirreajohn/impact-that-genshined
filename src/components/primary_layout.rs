use crate::components::Footer;
use crate::components::NavBar;
use crate::Router;
use dioxus::prelude::*;

use crate::HERO_BG;

#[component]
pub fn MainLayout() -> Element {
    let bg = format!("background-image: url('{}');", HERO_BG);
    rsx!(
        div {
            class: "relative h-screen bg-cover bg-center bg-no-repeat overflow-hidden",
            style: bg,
            div { class: "absolute inset-0 bg-black opacity-25" }
            NavBar {}
            Outlet::<Router> {}
                // Footer {}
        }
    )
}
