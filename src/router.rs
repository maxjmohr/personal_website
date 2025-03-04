use crate::pages::home::Home;
use crate::pages::projects::automobile_segmentation::AutomobileSegmentation;
use crate::pages::projects::banking_kpis::BankingKPIs;
use crate::pages::projects::campaign_management::CampaignManagement;
use crate::pages::projects::cognitive_biases_llms::CognitiveBiasesLLMs;
use crate::pages::projects::fynd::Fynd;
use crate::pages::projects::ml_ecommerce::MLeCommerce;
use crate::pages::projects::website_development::WebsiteDevelopment;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/automobile_segmentation")]
    AutomobileSegmentation,
    #[at("/banking_kpis")]
    BankingKPIs,
    #[at("/campaign_management")]
    CampaignManagement,
    #[at("/cognitive_biases_llms")]
    CognitiveBiasesLLMs,
    #[at("/fynd")]
    Fynd,
    #[at("/ml_ecommerce")]
    MLeCommerce,
    #[at("/website_development")]
    WebsiteDevelopment,
    //#[not_found]
    //#[at("/404")]
    //NotFound,
}

fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        let options = web_sys::ScrollToOptions::new();
        options.set_top(0.0);
        window.scroll_with_scroll_to_options(&options);
    }
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::AutomobileSegmentation => {
            scroll_to_top();
            html! { <AutomobileSegmentation /> }
        }
        Route::BankingKPIs => {
            scroll_to_top();
            html! { <BankingKPIs /> }
        }
        Route::CampaignManagement => {
            scroll_to_top();
            html! { <CampaignManagement /> }
        }
        Route::CognitiveBiasesLLMs => {
            scroll_to_top();
            html! { <CognitiveBiasesLLMs /> }
        }
        Route::Fynd => {
            scroll_to_top();
            html! { <Fynd /> }
        }
        Route::MLeCommerce => {
            scroll_to_top();
            html! { <MLeCommerce /> }
        }
        Route::WebsiteDevelopment => {
            scroll_to_top();
            html! { <WebsiteDevelopment /> }
        } //Route::NotFound => html! { <PageNotFound /> },
    }
}