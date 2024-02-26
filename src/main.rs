use crate::safehtml::SafeHtml;
use comrak::{markdown_to_html, Options};
use yew::prelude::*;

mod safehtml;

#[function_component(App)]
fn app() -> Html {
    // Get content
    let content_aboutme = markdown_to_html( include_str!("./../res/content/aboutme.md"), &Options::default() );
    let content_profexper = markdown_to_html( include_str!("./../res/content/profexper.md"), &Options::default() );
    let content_educ = markdown_to_html( include_str!("./../res/content/educ.md"), &Options::default() );
    let content_projects = markdown_to_html( include_str!("./../res/content/projects.md"), &Options::default() );
    let content_techskills = markdown_to_html( include_str!("./../res/content/techskills.md"), &Options::default() );
    let content_langs = markdown_to_html( include_str!("./../res/content/langs.md"), &Options::default() );

    html! {
        <>
        // Sidebar
        <aside class="sidebar">
            <ul class="links">
            <li>
                <img loading="lazy" src="res/images/icons/aboutme.png" />
                <a href="#aboutme">{"About me"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/profexper.png" />
                <a href="#profexper">{"Professional experience"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/educ.png" />
                <a href="#educ">{"Education"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/projects.png" />
                <a href="#projects">{"Projects"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/techskills.png" />
                <a href="#techskills">{"Technical skills"}</a>
            </li>
            <li>
                <img loading="lazy" src="res/images/icons/langs.png" />
                <a href="#langs">{"Languages"}</a>
            </li>
            </ul>
        </aside>

        // Main content
        <div id="aboutme" class="flex flex-col justify-center bg-gradient-to-b from-stone-300 to-gray-300 dark:from-stone-600 dark:to-gray-700
        pl-44 w-full h-full">
            <h1 class="text-8xl manual_h1">{"Max Mohr"}</h1>
            <SafeHtml html={content_aboutme} class="animate-typing overflow-hidden whitespace-nowrap border-r-4 border-r-white manual_p" />
        </div>
        <div id="profexper" class="bg-gradient-to-b from-gray-300 to-green-100 dark:from-gray-700 dark:to-green-900
       pt-28 pb-10 pl-44 w-full">
            <h1 class="text-7xl manual_h1">{"Professional experience"}</h1>
            <SafeHtml html={content_profexper} class="manual_p" />
        </div>
        <div id="educ" class="bg-gradient-to-b from-green-100 to-teal-200 dark:from-green-900 dark:to-teal-900
        pt-28 pb-14 pl-44 w-full">
        <h1 class="text-7xl manual_h1">{"Education"}</h1>
            <SafeHtml html={content_educ} class="manual_p" />
        </div>
        <div id="projects" class="bg-gradient-to-b from-teal-200 to-cyan-200 dark:from-teal-900 dark:to-cyan-900
        pt-28 pb-14 pl-44 w-full">
            <h1 class="text-7xl manual_h1">{"Projects"}</h1>
            <SafeHtml html={content_projects} class="manual_p" />
        </div>
        <div id="techskills" class="bg-gradient-to-b from-cyan-200 to-indigo-300 dark:from-cyan-900 dark:to-indigo-900
        pt-28 pb-14 pl-44 w-full">
            <h1 class="text-7xl manual_h1">{"Technical skills"}</h1>
            <SafeHtml html={content_techskills} class="manual_p" />
        </div>
        <div id="langs" class="bg-gradient-to-b from-indigo-300 to-stone-300 dark:from-indigo-900 dark:to-stone-600
        pt-28 pb-14 pl-44 w-full">
            <h1 class="text-7xl manual_h1">{"Languages"}</h1>
            <SafeHtml html={content_langs} class="manual_p" />
        </div>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
