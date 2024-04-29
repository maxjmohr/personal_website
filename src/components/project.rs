use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;
use yew::prelude::*;
//use yew_router::prelude::*;

// Coauthors
#[derive(Debug, PartialEq, Deserialize)]
pub struct Coauthor {
	pub name: String,
	pub url: Option<String>,
}

// Metadata of a project
#[derive(Debug, PartialEq, Deserialize)]
pub struct ProjectMetadata {
    //pub name: String,
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
        //name,
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

    html! {
        <div class="flex flex-col w-[41rem] h-[48rem] lg:w-96 lg:h-96 flex-shrink-0 mr-10 bg-white dark:bg-gray-800 rounded-2xl lg:hover:scale-105 shadow-md lg:shadow-none lg:hover:shadow-md transition lg:hover:duration-300 ease-in-out">
            <p class="text-left w-full pl-8 pr-4 mt-8 mb-6 lg:pl-6 lg:pr-2 lg:mt-6 lg:mb-4 text-stone-600 dark:text-neutral-400 text-4xl lg:text-xl">{&title}</p>
            <p class="text-left w-full pl-8 pr-4 lg:pl-6 lg:pr-2 mb-3 antialiased font-extrabold text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-6xl lg:text-3xl leading-[4.2rem]">{&subtitle}</p>
            <div class="flex-1 flex justify-center items-center mb-8 lg:mb-6 mx-8 lg:mx-6">
                <img class="object-scale-down" src = {format!("./../../res/images/projects/{}", &image)}/>
            </div>
        </div>
    }
}
