use crate::components::footer::Footer;
use crate::components::icons::Icons;
use crate::router::Route;
use crate::safehtml::SafeHtml;
use comrak::{format_html, parse_document, Arena, ComrakOptions};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;
use yew::prelude::*;
use yew_router::prelude::*;

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
    pub url: Option<String>,
    pub url_git: Option<String>,
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
    } = document.metadata;

    let to_route = match name.as_str() {
        "AutomobileSegmentation" => Route::AutomobileSegmentation,
        "BankingKPIs" => Route::BankingKPIs,
        "CampaignManagement" => Route::CampaignManagement,
        "CognitiveBiasesLLMs" => Route::CognitiveBiasesLLMs,
        "Fynd" => Route::Fynd,
        "MLeCommerce" => Route::MLeCommerce,
        "WebsiteDevelopment" => Route::WebsiteDevelopment,
        _ => panic!("Invalid slug: {}", name),
    };

    html! {
        <Link<Route> to={to_route}>
            <div class="h-9/10 sm:h-full w-xs sm:w-lg shrink-0 mr-4 sm:mr-10 bg-stone-50/50 dark:bg-slate-700/50 rounded-4xl 2xl:hover:scale-105 2xl:hover:shadow-md transition 2xl:hover:duration-300 ease-in-out">
                <div class="h-full pt-6 sm:pt-10 pl-7 sm:pl-12 pr-2 sm:pr-4 flex flex-col">
                    <p class="w-full text-lg/9 sm:text-2xl/11 text-slate-600 dark:text-slate-400">{&title}</p>
                    <p class="w-full pt-3 sm:pt-6 text-2xl/10 sm:text-4xl/12 font-semibold text-slate-700 dark:text-slate-300">{&subtitle}</p>
                    <div class="pb-2 pr-4 sm:pb-6 sm:pr-8 flex-1 flex justify-center items-center overflow-y-hidden">
                        <img class="max-h-46 xl:max-h-64 2xl:max-h-80 object-scale-down rounded-xl" src = {format!("./../../res/images/projects/{}", &image)}/>
                    </div>
                </div>
            </div>
        </Link<Route>>
    }
}

// Project site formatting for individual page
#[function_component]
pub fn ProjectSite(props: &ProjectProps) -> Html {
    let md_str = props.markdown.clone();

    // Get yaml front matter
    let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
    let ProjectMetadata {
        name: _,
        title,
        subtitle,
        image: _,
        time,
        skills,
        url,
        url_git,
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
        // New
        <div class="relative z-20 h-screen mx-4 sm:mx-12 lg:mx-24">
            // Header
            <div class="h-fit w-full mt-10 rounded-4xl bg-stone-200/50 dark:bg-slate-800/50">
                <div class="px-4 sm:px-10 py-4 sm:py-8">
                    // Title & links
                    <div class="flex flex-row">
                        <div class="basis-4/5">
                            <p class="text-3xl/11 sm:text-5xl/11 font-bold text-slate-700 dark:text-slate-300">{&title}</p>
                        </div>
                        <div class="basis-1/5">
                            <div class="mt-1 flex flex-row justify-end">
                                if let Some(url_git) = &url_git {
                                    <a href={url_git.to_string()} target="_blank">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="h-10 w-10 fill-current text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                                        </svg>
                                    </a>
                                }
                                if let Some(url) = &url {
                                    <a href={url.to_string()} target="_blank" class="ml-4">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="h-10 w-10 fill-current text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                                            <path d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm10 12c0 .685-.07 1.354-.202 2h-3.853c.121-1.283.129-2.621 0-4h3.853c.132.646.202 1.315.202 2zm-.841-4h-3.5c-.383-1.96-1.052-3.751-1.948-5.278 2.435.977 4.397 2.882 5.448 5.278zm-5.554 0h-2.605v-5.658c1.215 1.46 2.117 3.41 2.605 5.658zm-4.605-5.658v5.658h-2.605c.488-2.248 1.39-4.198 2.605-5.658zm0 7.658v4h-2.93c-.146-1.421-.146-2.577 0-4h2.93zm0 6v5.658c-1.215-1.46-2.117-3.41-2.605-5.658h2.605zm2 5.658v-5.658h2.605c-.488 2.248-1.39 4.198-2.605 5.658zm0-7.658v-4h2.93c.146 1.421.146 2.577 0 4h-2.93zm-4.711-11.278c-.896 1.527-1.565 3.318-1.948 5.278h-3.5c1.051-2.396 3.013-4.301 5.448-5.278zm-6.087 7.278h3.853c-.121 1.283-.129 2.621 0 4h-3.853c-.132-.646-.202-1.315-.202-2s.07-1.354.202-2zm.639 6h3.5c.383 1.96 1.052 3.751 1.948 5.278-2.435-.977-4.397-2.882-5.448-5.278zm12.87 5.278c.896-1.527 1.565-3.318 1.948-5.278h3.5c-1.051 2.396-3.013 4.301-5.448 5.278z"/>
                                        </svg>
                                    </a>
                                }
                            </div>
                        </div>
                    </div>

                    // Subtitle and time (desktop: besides, mobile: below)
                    <div class="flex max-md:flex-col md:flew-row">
                        <div class="md:basis-4/5">
                            <p class="mt-4 text-2xl/9 sm:text-3xl/10 font-medium text-slate-600 dark:text-slate-400">{&subtitle}</p>
                        </div>
                        <div class="md:basis-1/5">
                            <p class="mt-2 mb-2 md:mb-0 md:mt-5 text-left md:text-right text-2xl/9 font-normal text-slate-600 dark:text-slate-400">{&time}</p>
                        </div>
                    </div>

                    // Icons
                    <Icons icons={skills.clone()} class="pt-2 flex flex-wrap justify-start gap-4" />
                </div>
            </div>

            // Rest of the content
            <div class="px-4 sm:px-10 pt-8">
                <article class="
                    prose max-w-none
                    prose-headings:text-slate-700 prose-headings:dark:text-slate-300
                    prose-h1:text-3xl/9 prose-h1:sm:text-4xl/10 prose-h1:font-medium prose-h1:italic prose-h1:text-center
                    prose-p:text-xl/8 prose-p:sm:text-2xl/9 prose-p:font-normal prose-p:text-justify prose-p:text-slate-600 prose-p:dark:text-slate-400
                    prose-strong:text-slate-700 prose-strong:dark:text-slate-300
                    prose-a:italic prose-a:font-normal prose-a:hover:font-semibold prose-a:text-slate-600 prose-a:dark:text-slate-400
                    prose-blockquote:border-slate-600 prose-blockquote:dark:text-slate-400
                    prose-img:w-9/10 prose-img:sm:w-7/10 prose-img:rounded-2xl prose-img:my-0
                    prose-ul:list-disc prose-ul:marker:text-slate-600 prose-ul:dark:marker:text-slate-400
                    prose-li:text-lg/8 prose-li:sm:text-xl/9 prose-li:font-normal prose-li:text-justify prose-li:text-slate-600 prose-li:dark:text-slate-400
                ">
                    <SafeHtml html={md_content.clone()} />
                </article>
            </div>

            <Footer/>
        </div>

        </>
    }
}
