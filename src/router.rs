use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
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
        //Route::NotFound => html! { <PageNotFound /> },
    }
}
