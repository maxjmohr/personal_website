mod components {
    pub mod bullet;
    pub mod icons;
    pub mod navbar;
    pub mod project;
    pub mod timeline;
}
mod pages {
    pub mod home;
    pub mod projects {
        pub mod automobile_segmentation;
        pub mod fynd;
    }
}
mod router;
mod safehtml;

use crate::components::navbar::Navbar;
use router::{switch, Route};
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    use_effect(|| {
        let window = window().unwrap();
        let location = window.location();
        let hash = location.hash().unwrap_or_default();
        if !hash.is_empty() {
            let document = window.document().unwrap();
            let element = document.query_selector(&hash).unwrap();
            if let Some(element) = element {
                element.scroll_into_view();
            }
        }
        || ()
    });

    html! {
        <>
        // Navbar
        <Navbar />

        <BrowserRouter>
            <Switch<Route> render={switch} /> // Routes to different pages
        </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
