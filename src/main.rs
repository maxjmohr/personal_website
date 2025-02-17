mod components {
    pub mod bullet;
    pub mod icons;
    pub mod project;
    pub mod timeline;
}
mod pages {
    pub mod home;
    pub mod projects {
        pub mod automobile_segmentation;
        pub mod cognitive_biases_llms;
        pub mod fynd;
    }
}
mod router;
mod safehtml;

use gloo_utils::document;
use router::{switch, Route};
use wasm_bindgen::prelude::*;
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

        // Call the function to handle mouse gradient behavior
        setup_mouse_gradient();

        // Return an empty cleanup closure
        || ()
    });

    html! {
        <>
        <div
            id="mouse-gradient"
            class="z-10 absolute w-80 h-80 rounded-full pointer-events-none bg-linear-to-r from-stone-200 to-transparent dark:from-sky-700 opacity-75 blur-3xl"
        ></div>

        <BrowserRouter>
            <Switch<Route> render={switch} /> // Routes to different pages
        </BrowserRouter>
        </>
    }
}

/// Function to handle the mouse gradient effect.
fn setup_mouse_gradient() {
    // Access the document using gloo_utils
    let mouse_gradient = document().get_element_by_id("mouse-gradient");

    if let Some(mouse_gradient) = mouse_gradient {
        // Create closure to handle mousemove event
        let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let x = e.client_x() as f64; // Convert to f64
            let y = e.client_y() as f64; // Convert to f64

            // Get the page scroll position
            let scroll_x = window().unwrap().scroll_x().unwrap_or(0.0);
            let scroll_y = window().unwrap().scroll_y().unwrap_or(0.0);

            // Adjust the mouse gradient position by subtracting the scroll offset
            let adjusted_x = x + scroll_x - 100.0;
            let adjusted_y = y + scroll_y - 120.0;

            // Apply the transform to the mouse gradient element
            mouse_gradient
                .set_attribute(
                    "style",
                    &format!("transform: translate({}px, {}px);", adjusted_x, adjusted_y),
                )
                .unwrap();
        }) as Box<dyn FnMut(_)>);

        // Attach the mousemove event listener
        let document = document(); // Ensure document is referenced here
        document
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();

        // Keep the closure alive
        closure.forget(); // Important: This prevents the closure from being dropped
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
