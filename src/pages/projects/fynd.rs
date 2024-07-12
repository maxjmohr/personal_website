use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn Fynd() -> Html {
    let md = include_str!("./../../../res/content/projects/2023-10_fynd.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
