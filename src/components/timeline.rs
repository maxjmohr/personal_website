use crate::components::bullet::Bullet;
use serde::Deserialize;
use yew::prelude::*;

// Define the struct to represent a single timeline entry
#[derive(Debug, PartialEq, Deserialize)]
pub struct TimelineEntry {
    pub title: String,
    pub place: String,
    pub place_url: String,
    pub time: String,
    pub skills: Vec<String>,
    pub color: String,
    pub content: String,
}

// Properties to get content and return as bullet for list
#[derive(Debug, PartialEq, Properties)]
pub struct TimelineProps {
    pub title: String,
    pub place: String,
    pub place_url: String,
    pub time: String,
    pub skills: Vec<String>,
    pub color: String,
    pub content: String,
}

// Timeline entry formatting
#[function_component]
pub fn Timeline(props: &TimelineProps) -> Html {
    html! {
        <div class="relative pl-16 lg:pl-52 py-6 group" data-aos="fade-up">
            <a class="timeline_title">{&props.title}</a>
                <div class={format!("flex flex-col lg:flex-row items-start mb-1 group-last:before:hidden before:absolute before:left-2 lg:before:left-0 before:h-full before:px-px before:bg-slate-300 dark:before:bg-neutral-500 before:ml-3.5 lg:before:ml-[11.5rem] before:self-start before:-translate-x-1/2 before:translate-y-3 after:absolute after:left-2 lg:after:left-0 after:w-6 after:h-6 lg:after:w-4 lg:after:h-4 {} after:border-8 lg:after:border-4 after:box-content after:border-slate-50 dark:after:border-neutral-300 after:rounded-full after:ml-3.5 lg:after:ml-[11.5rem] after:-translate-x-1/2 after:translate-y-0.5", props.color)}>
                <time class="timeline_time">{&props.time}</time>
                <a href={props.place_url.clone()} target="_blank" class="timeline_desc italic pt-3 lg:pt-0">{&props.place}</a>
            </div>
            <div class="flex flex-wrap mt-3 lg:mt-0">
                {for props.skills.iter().map(|skill| html! { <div class="timeline_skills">{skill}</div> })}
            </div>
            {if !props.content.is_empty() {
                html! {
                    <div class="timeline_desc mt-4 lg:mt-2">
                        <Bullet content={props.content.clone()} />
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
