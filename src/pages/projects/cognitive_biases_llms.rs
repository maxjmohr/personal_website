use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn CognitiveBiasesLLMs() -> Html {
    let md = include_str!("./../../../res/content/projects/2024-06_cognitive_biases_llms.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
