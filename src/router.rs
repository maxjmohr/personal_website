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

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Projects => html! { <Home scroll_to={"projects".to_string()} /> },
        Route::Fynd => html! { <Fynd /> },
        Route::AutomobileSegmentation => html! { <AutomobileSegmentation /> },
        //Route::NotFound => html! { <PageNotFound /> },
    }
}
