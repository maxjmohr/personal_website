use crate::components::bullet::Bullet;
use serde::Deserialize;
use yew::prelude::*;

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
    pub content: String,
}

// Timeline entry formatting
#[function_component]
pub fn Timeline(props: &TimelineProps) -> Html {
    html! {
        <div class="relative pl-12 py-8 rounded-3xl bg-stone-200/50 dark:bg-slate-800/50" data-aos="fade-up">
            <div class="flex flew-row">
                <div class="basis-7/10">
                    // Title
                    <p class="text-4xl/12 font-medium text-slate-700 dark:text-slate-300">{&props.title}</p>

                    <div class="flex flex-row justify-left gap-4">
                        // Institution
                        <a href={props.institution_url.clone()} target="_blank" rel="noopener noreferrer" class="py-2 group flex items-center justify-left">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" class="h-10 w-10 mr-2 fill-slate-700 group-hover:fill-slate-600 dark:fill-slate-600 dark:group-hover:fill-slate-300">
                                <path d="M48 0C21.5 0 0 21.5 0 48L0 464c0 26.5 21.5 48 48 48l96 0 0-80c0-26.5 21.5-48 48-48s48 21.5 48 48l0 80 96 0c26.5 0 48-21.5 48-48l0-416c0-26.5-21.5-48-48-48L48 0zM64 240c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zm112-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zM80 96l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16zm80 16c0-8.8 7.2-16 16-16l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32zM272 96l32 0c8.8 0 16 7.2 16 16l0 32c0 8.8-7.2 16-16 16l-32 0c-8.8 0-16-7.2-16-16l0-32c0-8.8 7.2-16 16-16z"/>
                            </svg>
                            <div class="text-xl/9 group-hover:font-bold text-slate-600 group-hover:text-slate-600 dark:text-slate-400 dark:group-hover:text-slate-300"> {props.institution.clone()} </div>
                        </a>

                        // Location
                        <div class="py-2 flex items-center justify-left">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" class="h-10 w-10 mr-2 fill-slate-700 dark:fill-slate-600">
                                <path d="M215.7 499.2C267 435 384 279.4 384 192C384 86 298 0 192 0S0 86 0 192c0 87.4 117 243 168.3 307.2c12.3 15.3 35.1 15.3 47.4 0zM192 128a64 64 0 1 1 0 128 64 64 0 1 1 0-128z"/>
                            </svg>
                            <div class="text-xl/9 text-slate-600 dark:text-slate-400"> {props.location.clone()} </div>
                        </div>

                        // Time
                        <div class="py-2 flex items-center justify-left">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="h-10 w-10 mr-2 fill-slate-700 dark:fill-slate-600">
                                <path d="M128 0c17.7 0 32 14.3 32 32l0 32 128 0 0-32c0-17.7 14.3-32 32-32s32 14.3 32 32l0 32 48 0c26.5 0 48 21.5 48 48l0 48L0 160l0-48C0 85.5 21.5 64 48 64l48 0 0-32c0-17.7 14.3-32 32-32zM0 192l448 0 0 272c0 26.5-21.5 48-48 48L48 512c-26.5 0-48-21.5-48-48L0 192zm64 80l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm128 0l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM64 400l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm144-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zm112 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16z"/>
                            </svg>
                            <div class="text-xl/9 text-slate-600 dark:text-slate-400"> {props.time.clone()} </div>
                        </div>
                    </div>

                    //Skills
                    <div class="pt-1 pb-2 flex flex-wrap gap-2">
                        {for props.skills.iter().map(|skill| html! { <div class="px-3 py-1 rounded-full text-lg text-center font-medium text-slate-700 dark:text-slate-500 bg-stone-100/50 dark:bg-slate-700/25">{skill}</div> })}
                    </div>
                </div>

                <div class="basis-3/10">
                    // Image
                    <a href={props.institution_url.clone()} target="_blank" class="h-full w-full flex-1 flex justify-center items-center">
                        <img class="max-h-40 px-12 pt-2 pb-8 object-scale-down" src = {format!("./../../res/images/logos/{}", {props.institution_image.clone()})}/>
                    </a>
                </div>
            </div>
            {if !props.content.is_empty() {
                html! {
                    <div class="mt-4 lg:mt-2">
                        <Bullet content={props.content.clone()} />
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
