use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn AutomobileSegmentation() -> Html {
    let md = include_str!("./../../../res/content/projects/2023-07_automobile_segmentation.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
