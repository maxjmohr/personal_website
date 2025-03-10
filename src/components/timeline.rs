use crate::components::bullet::Bullet;
use crate::router::Route;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;

// Define the struct to represent a single timeline entry
#[derive(Debug, PartialEq, Deserialize)]
pub struct TimelineEntry {
    pub title: String,
    pub institution: String,
    pub institution_url: String,
    pub institution_image: String,
    pub location: String,
    pub time: String,
    pub skills: Vec<String>,
    pub projects: Vec<String>,
    pub content: String,
}

// Properties to get content and return as bullet for list
#[derive(Debug, PartialEq, Properties)]
pub struct TimelineProps {
    pub title: String,
    pub institution: String,
    pub institution_url: String,
    pub institution_image: String,
    pub location: String,
    pub time: String,
    pub skills: Vec<String>,
    pub projects: Vec<String>,
    pub content: String,
}

// Timeline entry formatting
#[function_component]
pub fn Timeline(props: &TimelineProps) -> Html {
    html! {
        <div class="relative pl-6 sm:pl-12 py-6 sm:py-8 rounded-3xl bg-stone-200/50 dark:bg-slate-800/50" data-aos="fade-up">
            <div class="flex max-xl:flex-col xl:flew-row">
                // Title & logo (mobile)
                <div class="xl:hidden flex max-sm:flex-col sm:flex-row">
                    // Logo (above for true mobile)
                    <a href={props.institution_url.clone()} target="_blank" class="sm:hidden h-full w-full flex-1 flex justify-center items-center">
                        <img class="basis-10/10 max-h-16 px-10 pt-2 pb-4 object-scale-down" src = {format!("./../../res/images/logos/{}", {props.institution_image.clone()})}/>
                    </a>
                    // Title
                    <p class="basis-10/10 sm:basis-4/10 flex-1 flex justify-start items-center text-2xl/10 sm:text-4xl/12 font-medium text-slate-700 dark:text-slate-300">{&props.title}</p>
                    // Logo (right)
                    <a href={props.institution_url.clone()} target="_blank" class="hidden sm:block h-full w-full flex-1 flex justify-center items-center">
                        <img class="basis-6/10 max-h-20 xl:max-h-40 px-6 xl:px-12 pt-2 pb-4 xl:pb-8 object-scale-down" src = {format!("./../../res/images/logos/{}", {props.institution_image.clone()})}/>
                    </a>
                </div>
                // Title, tags and text
                <div class="xl:basis-7/10">
                    // Title
                    <p class="hidden xl:block text-4xl/12 font-medium text-slate-700 dark:text-slate-300">{&props.title}</p>

                    <div class="flex max-lg:flex-col lg:flex-row justify-left gap-1 lg:gap-4 mr-4">
                        // Institution
                        <a href={props.institution_url.clone()} target="_blank" rel="noopener noreferrer" class="py-1 lg:py-2 group flex items-center justify-left">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" class="h-6 w-6 lg:h-10 lg:w-10 mr-2 fill-slate-600 2xl:group-hover:fill-slate-700 dark:fill-slate-600 dark:2xl:group-hover:fill-slate-300 transition">
                                <path d="M48 0C21.5 0 0 21.5 0 48L0 464c0 26.5 21.5 48 48 48l96 0 0-80c0-26.5 21.5-48 48-48s48 21.5 48 48l0 80 96 0c26.5 0 48-21.5 48-48l0-416c0-26.5-21.5-48-48-48L48 0zM64 240c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zm112-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zM80 96l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zM272 96l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16z"/>
                            </svg>
                            <div class="text-lg/8 sm:text-xl/9 2xl:group-hover:font-bold text-slate-600 2xl:group-hover:text-slate-600 dark:text-slate-400 dark:2xl:group-hover:text-slate-300 transition"> {props.institution.clone()} </div>
                        </a>

                        // Location
                        <div class="pb-1 lg:pb-0 lg:py-2 flex items-center justify-left">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" class="h-6 w-6 lg:h-10 lg:w-10 mr-2 fill-slate-600 dark:fill-slate-600">
                                <path d="M215.7 499.2C267 435 384 279.4 384 192C384 86 298 0 192 0S0 86 0 192c0 87.4 117 243 168.3 307.2c12.3 15.3 35.1 15.3 47.4 0zM192 128a64 64 0 1 1 0 128 64 64 0 1 1 0-128z"/>
                            </svg>
                            <div class="text-lg/8 sm:text-xl/9 text-slate-600 dark:text-slate-400"> {props.location.clone()} </div>
                        </div>

                        // Time
                        <div class="pb-1 lg:pb-0 lg:py-2 flex items-center justify-left">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="h-6 w-6 lg:h-10 lg:w-10 mr-2 fill-slate-600 dark:fill-slate-600">
                                <path d="M128 0c17.7 0 32 14.3 32 32l0 32 128 0 0-32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 32 48 0c26.5 0 48 21.5 48 48l0 48L0 160l0-48C0 85.5 21.5 64 48 64l48 0 0-32c0-17.7 14.3-32 32-32zM0 192l448 0 0 272c0 26.5-21.5 48-48 48L48 512c-26.5 0-48-21.5-48-48L0 192zm64 80l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm128 0l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM64 400l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zm112 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16z"/>
                            </svg>
                            <div class="text-lg/8 sm:text-xl/9 text-slate-600 dark:text-slate-400"> {props.time.clone()} </div>
                        </div>
                    </div>

                    //Skills
                    <div class="pt-1 pb-2 flex flex-wrap gap-2">
                        {for props.skills.iter().map(|skill| html! { <div class="px-2 sm:px-3 py-1 rounded-full text-base sm:text-lg text-center font-medium text-slate-500 dark:text-slate-500 bg-stone-50/25 dark:bg-slate-700/25">{skill}</div> })}
                    </div>

                    // Linked projects
                    <div class="pl-2 pt-1 pb-2 flex flex-wrap gap-x-4 gap-y-1 sm:gap-y-2 items-center">
                        {for props.projects.iter().map(|project| {
                            let to_route = match project.as_str() {
                                "Image Segmentation & Classification" => Route::AutomobileSegmentation,
                                "Data Extraction from PDF and XML" => Route::BankingKPIs,
                                "Analytical Campaign Management" => Route::CampaignManagement,
                                "Cognitive Biases in LLMs" => Route::CognitiveBiasesLLMs,
                                "fynd" => Route::Fynd,
                                "Machine Learning Mini Project" => Route::MLeCommerce,
                                "My Personal Website" => Route::WebsiteDevelopment,
                                _ => panic!("Invalid slug: {}", project),
                            };

                            html! {
                                <Link<Route> to={to_route}>
                                    <div class="flex items-center justify-center group">
                                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" class="h-4 w-4 sm:h-6 sm:w-6 mr-2 fill-slate-500 dark:fill-slate-500 2xl:group-hover:fill-slate-600 dark:2xl:group-hover:dark:fill-slate-400">
                                            <path d="M320 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l82.7 0L201.4 265.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L448 109.3l0 82.7c0 17.7 14.3 32 32 32s32-14.3 32-32l0-160c0-17.7-14.3-32-32-32L320 0zM80 32C35.8 32 0 67.8 0 112L0 432c0 44.2 35.8 80 80 80l320 0c44.2 0 80-35.8 80-80l0-112c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 112c0 8.8-7.2 16-16 16L80 448c-8.8 0-16-7.2-16-16l0-320c0-8.8 7.2-16 16-16l112 0c17.7 0 32-14.3 32-32s-14.3-32-32-32L80 32z"/>
                                        </svg>
                                        <div class="py-1 text-base sm:text-lg text-center font-medium text-slate-500 dark:text-slate-500 2xl:group-hover:font-bold 2xl:group-hover:text-slate-600 dark:2xl:group-hover:dark:text-slate-400 transition">
                                            {project}
                                        </div>
                                    </div>
                                </Link<Route>>
                            }
                        })}
                    </div>
                </div>
                // Logo (desktop)
                <div class="hidden xl:block basis-3/10">
                    <a href={props.institution_url.clone()} target="_blank" class="h-full w-full flex-1 flex justify-center items-center 2xl:hover:scale-105 transition">
                        <img class="max-h-40 px-12 pt-2 pb-8 object-scale-down" src = {format!("./../../res/images/logos/{}", {props.institution_image.clone()})}/>
                    </a>
                </div>
            </div>
            {if !props.content.is_empty() {
                html! {
                    <div class="mt-2 mr-4">
                        <Bullet content={props.content.clone()} />
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
