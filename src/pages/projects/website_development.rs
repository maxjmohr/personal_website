use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn WebsiteDevelopment() -> Html {
    let md = include_str!("./../../../res/content/projects/2024-02_website_development.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
