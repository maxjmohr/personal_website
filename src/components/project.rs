use comrak::{format_html, parse_document, Arena, ComrakOptions};
use crate::router::Route;
use crate::safehtml::SafeHtml;
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;
use yew::prelude::*;
use yew_router::prelude::*;

// Coauthors
#[derive(Debug, PartialEq, Deserialize)]
pub struct Coauthor {
    pub name: String,
    pub url: Option<String>,
}

// Metadata of a project
#[derive(Debug, PartialEq, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub title: String,
    pub subtitle: String,
    pub image: String,
    pub time: String,
    pub skills: Vec<String>,
    #[allow(dead_code)]
    pub url: String,
    pub url_git: String,
    pub coauthors: Option<Vec<Coauthor>>,
    pub report: Option<String>,
}

// Properties to get markdown content
#[derive(Debug, PartialEq, Properties)]
pub struct ProjectProps {
    pub markdown: String,
}

// Project card formatting for home page
#[function_component]
pub fn ProjectCard(props: &ProjectProps) -> Html {
    let md_str = props.markdown.clone();

    // Get yaml front matter
    let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
    let ProjectMetadata {
        name,
        title,
        subtitle,
        image,
        time: _,
        skills: _,
        url: _,
        url_git: _,
        coauthors: _,
        report: _,
    } = document.metadata;

    let to_route = match name.as_str() {
        "Fynd" => Route::Fynd,
        "AutomobileSegmentation" => Route::AutomobileSegmentation,
        _ => panic!("Invalid slug: {}", name),
    };

    html! {
        <Link<Route> to={to_route}>
            <div class="flex flex-col w-[41rem] h-[48rem] lg:w-96 lg:h-96 flex-shrink-0 mr-10 bg-white dark:bg-gray-800 rounded-2xl lg:hover:scale-105 shadow-md lg:shadow-none lg:hover:shadow-md transition lg:hover:duration-300 ease-in-out">
                <p class="text-left w-full pl-8 pr-4 mt-8 mb-6 lg:pl-6 lg:pr-2 lg:mt-6 lg:mb-4 text-stone-600 dark:text-neutral-400 text-4xl lg:text-xl">{&title}</p>
                <p class="text-left w-full pl-8 pr-4 lg:pl-6 lg:pr-2 mb-3 antialiased font-extrabold text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-6xl lg:text-3xl leading-[4.2rem]">{&subtitle}</p>
                <div class="flex-1 flex justify-center items-center mb-8 lg:mb-6 mx-8 lg:mx-6">
                    <img class="object-scale-down" src = {format!("./../../res/images/projects/{}", &image)}/>
                </div>
            </div>
        </Link<Route>>
    }
}

// Project site formatting for home page
#[function_component]
pub fn ProjectSite(props: &ProjectProps) -> Html {
    let md_str = props.markdown.clone();

    // Get yaml front matter
    let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
    let ProjectMetadata {
        name,
        title,
        subtitle,
        image,
        time,
        skills,
        url,
        url_git,
        coauthors,
        report
    } = document.metadata;

    // Get actual content for project site
    let arena = Arena::new();
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_string());
    options.render.unsafe_ = true;
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    let root = parse_document(&arena, &md_str, &options);
    let mut md_content_vec = vec![];
    format_html(root, &options, &mut md_content_vec).unwrap();
    let md_content = String::from_utf8(md_content_vec).unwrap();

    html! {
        <>
        <div class="flex flex-col h-full">
        <div class="grow bg-gradient-to-b from-teal-200 to-cyan-200 dark:from-teal-800 dark:to-cyan-900
        flex justify-start pt-4 lg:pt-20 pb-4 lg:pb-20 pl-12 lg:pl-44 pr-4">
            // Key facts card
            //<div class=""

            <SafeHtml html={md_content.clone()} />
        </div>

        // Footer
        <div class="pb-52 lg:pb-6 bg-gradient-to-b from-cyan-200 to-stone-300 dark:from-cyan-900 dark:to-stone-600">
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
        </div>
        </>
    }
}