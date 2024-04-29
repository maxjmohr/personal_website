mod components {
    pub mod bullet;
    pub mod icons;
    pub mod navbar;
    pub mod project;
    pub mod timeline;
}
mod pages {
    pub mod home;
}
mod router;
mod safehtml;

use crate::components::navbar::Navbar;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
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
