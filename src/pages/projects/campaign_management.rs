use crate::components::project::ProjectSite;
use yew::prelude::*;

#[function_component]
pub fn CampaignManagement() -> Html {
    let md = include_str!("./../../../res/content/projects/2021-06_campaign_management.md");
    html! {
        <ProjectSite markdown={md} />
    }
}
