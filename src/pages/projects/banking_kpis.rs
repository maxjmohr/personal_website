use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn BankingKPIs() -> Html {
    let md = include_str!("./../../../res/content/projects/2024-10_bankingkpis.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
