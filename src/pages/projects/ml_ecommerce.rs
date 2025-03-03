use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn MLeCommerce() -> Html {
    let md = include_str!("./../../../res/content/projects/2023-12_ml_ecommerce.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
