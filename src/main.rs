mod components {
    pub mod bullet;
    pub mod icons;
    pub mod navbar;
    pub mod timeline;
}
mod safehtml;

use crate::components::icons::Icons;
use crate::components::navbar::Navbar;
use crate::components::timeline::{Timeline, TimelineEntry};
use crate::safehtml::SafeHtml;
use comrak::{markdown_to_html, ComrakOptions};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // Render settings
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    // Get content
    let content_profexper = include_str!("./../res/content/profexper.yaml");
    let content_educ = include_str!("./../res/content/educ.yaml");
    let content_projects = markdown_to_html(include_str!("./../res/content/projects.md"), &options);

    // Iterate over each timeline entry and create TimelineProps
    // Professional experience
    let content_profexper_entries: Vec<TimelineEntry> =
        serde_yaml::from_str(content_profexper).unwrap();
    let content_profexper_timeline: Vec<Html> = content_profexper_entries
        .iter()
        .map(|entry| {
            html! {
                <Timeline
                    title={entry.title.clone()}
                    place={entry.place.clone()}
                    place_url={entry.place_url.clone()}
                    time={entry.time.clone()}
                    skills={entry.skills.clone()}
                    color={entry.color.clone()}
                    content={entry.content.clone()}
                />
            }
        })
        .collect();
    // Education
    let content_educ_entries: Vec<TimelineEntry> = serde_yaml::from_str(content_educ).unwrap();
    let content_educ_timeline: Vec<Html> = content_educ_entries
        .iter()
        .map(|entry| {
            html! {
                <Timeline
                    title={entry.title.clone()}
                    place={entry.place.clone()}
                    place_url={entry.place_url.clone()}
                    time={entry.time.clone()}
                    skills={entry.skills.clone()}
                    color={entry.color.clone()}
                    content={entry.content.clone()}
                />
            }
        })
        .collect();

    html! {
        <>
        // Navbar
        <Navbar />

        // Main content
        // About me
        <div id="aboutme" class="flex flex-col justify-center bg-gradient-to-b from-stone-300 to-gray-300 dark:from-stone-600 dark:to-gray-700
        pl-12 lg:pl-44 pr-20 w-full h-full">
            <h1 class="text-9xl lg:text-8xl manual_h1 mb-4">{"Max Mohr"}</h1>
            <div class="flex mb-6">
                <a href="https://www.linkedin.com/in/maxjmohr/" target="_blank" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="36" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                    </svg>
                </a>
                <a href="https://github.com/maxjmohr" target="_blank" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                </a>
                <a href="mailto:maxjmohr@gmail.com" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M12.042 23.648c-7.813 0-12.042-4.876-12.042-11.171 0-6.727 4.762-12.125 13.276-12.125 6.214 0 10.724 4.038 10.724 9.601 0 8.712-10.33 11.012-9.812 6.042-.71 1.108-1.854 2.354-4.053 2.354-2.516 0-4.08-1.842-4.08-4.807 0-4.444 2.921-8.199 6.379-8.199 1.659 0 2.8.876 3.277 2.221l.464-1.632h2.338c-.244.832-2.321 8.527-2.321 8.527-.648 2.666 1.35 2.713 3.122 1.297 3.329-2.58 3.501-9.327-.998-12.141-4.821-2.891-15.795-1.102-15.795 8.693 0 5.611 3.95 9.381 9.829 9.381 3.436 0 5.542-.93 7.295-1.948l1.177 1.698c-1.711.966-4.461 2.209-8.78 2.209zm-2.344-14.305c-.715 1.34-1.177 3.076-1.177 4.424 0 3.61 3.522 3.633 5.252.239.712-1.394 1.171-3.171 1.171-4.529 0-2.917-3.495-3.434-5.246-.134z"/>
                    </svg>
                </a>
                <a href="https://unsplash.com/@maxjmohr" target="_blank" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M448 230.2V480H0V230.2H141.1V355.1H306.9V230.2zM306.9 32H141.1V156.9H306.9z"/>
                    </svg>
                </a>
            </div>
            <span class="type-aboutme antialiased text-5xl lg:text-3xl font-medium leading-snug tracking-normal text-wrap text-stone-600 dark:text-neutral-300 leading-tight lg:leading-snug"></span>
        </div>

        // Professional experience
        <div id="profexper" class="bg-gradient-to-b from-gray-300 to-green-100 dark:from-gray-700 dark:to-emerald-900
        pt-28 pb-10 pl-12 lg:pl-44 pr-20">
            <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Professional experience"}</h1>
            { for content_profexper_timeline.iter().cloned() }
        </div>

        // Education
        <div id="educ" class="bg-gradient-to-b from-green-100 to-teal-200 dark:from-emerald-900 dark:to-teal-800
        pt-28 pb-14 pl-12 lg:pl-44 pr-20">
            <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Education"}</h1>
            { for content_educ_timeline.iter().cloned() }
        </div>

        // Projects
        <div id="projects" class="bg-gradient-to-b from-teal-200 to-cyan-200 dark:from-teal-800 dark:to-cyan-900
        pt-28 pb-14 pl-12 lg:pl-44 pr-20">
            <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Projects"}</h1>
            <SafeHtml html={content_projects}/>
        </div>

        // Technical skills
        <div id="techskills" class="bg-gradient-to-b from-cyan-200 to-indigo-300 dark:from-cyan-900 dark:to-indigo-900
        pt-28 pb-14 pl-12 lg:pl-44 pr-20">
            <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Technical skills"}</h1>
            <div class="flex flex-col items-center">
                <span class="text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-7xl lg:text-5xl font-semibold mb-6 lg:mb-4 text-center" data-aos="fade">{"Programming languages"}</span>
                <Icons icons={vec![
                    "matlab".to_string(),
                    "python".to_string(),
                    "r".to_string(),
                    "sql".to_string(),
                    "stata".to_string()
                ]}/>
                <span class="text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-7xl lg:text-5xl font-semibold mt-10 lg:mt-6 mb-6 lg:mb-4 text-center" data-aos="fade">{"Frameworks & libraries"}</span>
                <Icons icons={vec![
                    "arrow".to_string(),
                    "dash".to_string(),
                    "detectron".to_string(),
                    "gradio".to_string(),
                    "huggingface".to_string(),
                    "kafka".to_string(),
                    "llamaindex".to_string(),
                    "pytorch".to_string(),
                    "scikitlearn".to_string(),
                    "shiny".to_string(),
                    "spark".to_string(),
                    "yew".to_string()
                ]}/>
                <span class="text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-7xl lg:text-5xl font-semibold mt-10 lg:mt-6 mb-6 lg:mb-4 text-center" data-aos="fade">{"Tools"}</span>
                <Icons icons={vec![
                    "atlassian".to_string(),
                    "css".to_string(),
                    "earthengine".to_string(),
                    "git".to_string(),
                    "html".to_string(),
                    "informatica".to_string(),
                    "powerbi".to_string(),
                    "oracledb".to_string(),
                    "postgresql".to_string(),
                    "qgis".to_string(),
                    "tailwind".to_string()
                ]}/>
            </div>
        </div>

        // Languages
        <div id="langs" class="bg-gradient-to-b from-indigo-300 to-stone-300 dark:from-indigo-900 dark:to-stone-600
        pt-28 pb-14 pl-12 lg:pl-44 pr-20">
            <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Languages"}</h1>
            <div class="flex flex-wrap justify-center" data-aos="fade">
                <div class="flex flex-col items-center m-6 group">
                    <img src="res/images/langs/germany.png" alt="German Flag" class="langs_flag" />
                    <p class="langs_title">{"German"}</p>
                    <p class="langs_level">{"C2"}</p>
                </div>
                <div class="flex flex-col items-center m-6 group">
                    <img src="res/images/langs/uk.png" alt="UK Flag" class="langs_flag" />
                    <p class="langs_title">{"English"}</p>
                    <p class="langs_level">{"C1"}</p>
                </div>
            </div>
            <p class="type-langs antialiased text-4xl lg:text-3xl font-medium leading-snug tracking-normal text-wrap text-stone-600 dark:text-neutral-300 text-center" data-aos="fade"></p>
            <div class="flex flex-wrap justify-center" data-aos="fade">
                <div class="flex flex-col items-center m-6 group">
                    <img src="res/images/langs/france.png" alt="German Flag" class="langs_flag" />
                    <p class="langs_title">{"French"}</p>
                    <p class="langs_level">{"B1"}</p>
                </div>
                <div class="flex flex-col items-center m-6 group">
                    <img src="res/images/langs/italy.png" alt="German Flag" class="langs_flag" />
                    <p class="langs_title">{"Italian"}</p>
                    <p class="langs_level">{"A2"}</p>
                </div>
                <div class="flex flex-col items-center m-6 group">
                    <img src="res/images/langs/spain.png" alt="German Flag" class="langs_flag" />
                    <p class="langs_title">{"Spanish"}</p>
                    <p class="langs_level">{"A1"}</p>
                </div>
            </div>
        </div>

        // Footer
        <div id="footer" class="pb-52 lg:pb-6 bg-stone-300 dark:bg-stone-600">
            <div class="flex justify-center items-center h-20">
                <a href="https://www.linkedin.com/in/maxjmohr/" target="_blank" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                    </svg>
                </a>
                <a href="https://github.com/maxjmohr" target="_blank" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                </a>
                <a href="mailto:maxjmohr@gmail.com" class="mr-6 lg:mr-4">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M12.042 23.648c-7.813 0-12.042-4.876-12.042-11.171 0-6.727 4.762-12.125 13.276-12.125 6.214 0 10.724 4.038 10.724 9.601 0 8.712-10.33 11.012-9.812 6.042-.71 1.108-1.854 2.354-4.053 2.354-2.516 0-4.08-1.842-4.08-4.807 0-4.444 2.921-8.199 6.379-8.199 1.659 0 2.8.876 3.277 2.221l.464-1.632h2.338c-.244.832-2.321 8.527-2.321 8.527-.648 2.666 1.35 2.713 3.122 1.297 3.329-2.58 3.501-9.327-.998-12.141-4.821-2.891-15.795-1.102-15.795 8.693 0 5.611 3.95 9.381 9.829 9.381 3.436 0 5.542-.93 7.295-1.948l1.177 1.698c-1.711.966-4.461 2.209-8.78 2.209zm-2.344-14.305c-.715 1.34-1.177 3.076-1.177 4.424 0 3.61 3.522 3.633 5.252.239.712-1.394 1.171-3.171 1.171-4.529 0-2.917-3.495-3.434-5.246-.134z"/>
                    </svg>
                </a>
                <a href="https://unsplash.com/@maxjmohr" target="_blank" class="mr-6 lg:mr-4">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="w-16 h-16 lg:w-8 lg:h-8 fill-gray-700 dark:fill-stone-300">
                        <path d="M448 230.2V480H0V230.2H141.1V355.1H306.9V230.2zM306.9 32H141.1V156.9H306.9z"/>
                    </svg>
                </a>
            </div>
            <p class="antialiased text-xl lg:text-md text-gray-700 dark:text-stone-200 text-center font-lg lg:font-medium pt-4 lg:pt-0 px-20">{"I constructed this website on the basis of the "}
                <a href="https://yew.rs" target="_blank" class="italic lg:hover:font-bold">{"yew.rs"}</a>
                    {" framework. It is in constant development. For more technical insights feel free to access the "}
                        <a href="https://github.com/maxjmohr/personal_website" target="_blank" class="italic lg:hover:font-bold">{"GitHub repository"}</a>
                            {"."}</p>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
