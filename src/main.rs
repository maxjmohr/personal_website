use crate::safehtml::SafeHtml;
use comrak::{markdown_to_html, Options};
use include_dir::{include_dir, Dir};
use yew::prelude::*;

mod safehtml;

#[function_component(App)]
fn app() -> Html {
    // Get content
    let content_aboutme = markdown_to_html( &include_str!("./../res/content/aboutme/aboutme.md"), &Options::default() );

    html! {
        <>
        // Sidebar
        <aside class="sidebar">
            <ul class="links">
            <li>
                <img loading="lazy" src="res/images/icons/aboutme.png" />
                <a href="#">{"About me"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/profexper.png" />
                <a href="#">{"Professional experience"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/educ.png" />
                <a href="#">{"Education"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/projects.png" />
                <a href="#">{"Projects"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/techskills.png" />
                <a href="#">{"Technical skills"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/langs.png" />
                <a href="#">{"Languages"}</a>
            </li>
            </ul>
        </aside>

        // Main content
        <div id="aboutme" class="bg-blue-500 text-white p-4 mt-4 float-right" style="width: 100%; height: 100vh">
            <div style="padding-left: 10%;">
                <SafeHtml html={content_aboutme} />
            </div>
        </div>
        <div id="profexper" class="bg-green-500 text-white p-4 mt-4 float-right" style="width: 90%; height: 100vh">
            <h2>{"Professional experience"}</h2>
            <p>{"Career steps go here"}</p>
        </div>
        <div class="bg-red-500 text-white p-4 mt-4 float-right" style="width: 90%; height: 100vh">
            {"This div uses Tailwind CSS classes!"}
        </div>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
