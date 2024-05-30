use crate::pages::home::Home;
use crate::pages::projects::fynd::Fynd;
use crate::pages::projects::automobile_segmentation::AutomobileSegmentation;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/projects/fynd")]
    Fynd,
    #[at("/projects/automobile_segmentation")]
    AutomobileSegmentation
    //#[not_found]
    //#[at("/404")]
    //NotFound,
}

fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        let mut options = web_sys::ScrollToOptions::new();
        options.top(0.0);
        window.scroll_with_scroll_to_options(&options);
    }
}

pub fn switch(route: Route) -> Html {
    scroll_to_top();

    match route {
        Route::Home => html! { <Home /> },
        Route::Projects => html! { <Home scroll_to={"projects".to_string()} /> },
        Route::Fynd => html! { <Fynd /> },
        Route::AutomobileSegmentation => html! { <AutomobileSegmentation /> },
        //Route::NotFound => html! { <PageNotFound /> },
    }
}
