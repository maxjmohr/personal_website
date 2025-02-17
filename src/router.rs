use crate::pages::home::Home;
use crate::pages::projects::automobile_segmentation::AutomobileSegmentation;
use crate::pages::projects::cognitive_biases_llms::CognitiveBiasesLLMs;
use crate::pages::projects::fynd::Fynd;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/automobile_segmentation")]
    AutomobileSegmentation,
    #[at("/cognitive_biases_llms")]
    CognitiveBiasesLLMs,
    #[at("/fynd")]
    Fynd,
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
    match route {
        Route::Home => html! { <Home /> },
        Route::AutomobileSegmentation => {scroll_to_top(); html! { <AutomobileSegmentation /> }},
        Route::CognitiveBiasesLLMs => {scroll_to_top(); html! { <CognitiveBiasesLLMs /> }},
        Route::Fynd => {scroll_to_top(); html! { <Fynd /> }},
        //Route::NotFound => html! { <PageNotFound /> },
    }
}
