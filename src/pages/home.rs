use crate::components::footer::Footer;
use crate::components::project::ProjectCard;
use crate::components::timeline::{Timeline, TimelineEntry};
use comrak::ComrakOptions;
use include_dir::{include_dir, Dir};
use yew::prelude::*;

static CONTENT_PROJECTS: Dir = include_dir!("res/content/projects/");

#[function_component]
pub fn Home() -> Html {
    // Render settings
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    // Get content
    let content_profexper = include_str!("./../../res/content/profexper.yaml");
    let content_educ = include_str!("./../../res/content/educ.yaml");
    let content_projects = &CONTENT_PROJECTS;

    // Get ASCII art from file
    let ascii_display = include_str!("./../../res/images/ascii.txt");

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
                    institution={entry.institution.clone()}
                    institution_url={entry.institution_url.clone()}
                    institution_image={entry.institution_image.clone()}
                    location={entry.location.clone()}
                    time={entry.time.clone()}
                    skills={entry.skills.clone()}
                    projects={entry.projects.clone()}
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
                    institution={entry.institution.clone()}
                    institution_url={entry.institution_url.clone()}
                    institution_image={entry.institution_image.clone()}
                    location={entry.location.clone()}
                    time={entry.time.clone()}
                    skills={entry.skills.clone()}
                    projects={entry.projects.clone()}
                    content={entry.content.clone()}
                />
            }
        })
        .collect();

    // Collect each project in the projects directory
    let mut content_projects_files = content_projects.files().collect::<Vec<_>>();
    content_projects_files.sort_by(|a, b| b.path().cmp(a.path()));
    let content_projects_html: Vec<Html> = content_projects_files
        .iter()
        .filter_map(|file| {
            file.contents_utf8().map(|content| {
                html! {
                    <ProjectCard markdown={content} />
                }
            })
        })
        .collect();

    html! {
        <>
        // Main content
        // About me
        <div id="aboutme" class="relative z-20 h-screen mx-8 sm:mx-14 lg:mx-20 min-[2000px]:mx-60 sm:flex sm:flex-col xl:flex-none xl:grid xl:grid-cols-16 xl:content-center" data-aos="zoom-in">
            // Ascii for mobile
            <div class="max-sm:hidden xl:hidden h-96 pt-32 flex items-start sm:items-start justify-center">
                <div class="origin-top scale-30">
                    <pre class="text-base text-mono leading-relaxed tracking-[.32em] text-slate-600 dark:text-slate-500">
                        {ascii_display}
                    </pre>
                </div>
            </div>
            // Text and links
            <div class="max-sm:absolute max-sm:bottom-0 pt-0 sm:pt-46 xl:pt-0 pb-32 sm:pb-0 flex flex-col justify-center xl:col-start-2 xl:col-end-11">
                <h1 class="w-full text-5xl xl:text-8xl font-medium text-slate-700 dark:text-slate-300">{"Max Mohr"}</h1>
                <div class="flex flex-row">
                    <p class="pt-3 text-xl/7 sm:text-2xl/9 xl:text-3xl/11 text-slate-600 dark:text-slate-400">
                        <a class="underline underline-offset-5">{"driving"}</a>
                        {" AI innovation @ BearingPoint. "}
                        <a class="underline underline-offset-5">{"developing & deploying"}</a>
                        {" along the entire processing stack from data engineering and governance, modelling and analysis to frontend visualization. "}
                        <a class="underline underline-offset-5">{"graduated"}</a>
                        {" data scientist."}
                    </p>
                </div>
                <div class="flex pt-4 xl:pt-6">
                    <a href="https://www.linkedin.com/in/maxjmohr/" target="_blank" class="mr-4">
                        <svg xmlns="http://www.w3.org/2000/svg" width="36" height="24" viewBox="0 0 24 24" class="h-6 w-6 sm:h-8 sm:w-8 fill-current text-slate-600 sm:text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                            <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                        </svg>
                    </a>
                    <a href="https://github.com/maxjmohr" target="_blank" class="mr-4">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="h-6 w-6 sm:h-8 sm:w-8 fill-current text-slate-600 sm:text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                    </a>
                    <a href="mailto:maxjmohr@gmail.com" class="mr-4">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="h-6 w-6 sm:h-8 sm:w-8 fill-current text-slate-600 sm:text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                            <path d="M12.042 23.648c-7.813 0-12.042-4.876-12.042-11.171 0-6.727 4.762-12.125 13.276-12.125 6.214 0 10.724 4.038 10.724 9.601 0 8.712-10.33 11.012-9.812 6.042-.71 1.108-1.854 2.354-4.053 2.354-2.516 0-4.08-1.842-4.08-4.807 0-4.444 2.921-8.199 6.379-8.199 1.659 0 2.8.876 3.277 2.221l.464-1.632h2.338c-.244.832-2.321 8.527-2.321 8.527-.648 2.666 1.35 2.713 3.122 1.297 3.329-2.58 3.501-9.327-.998-12.141-4.821-2.891-15.795-1.102-15.795 8.693 0 5.611 3.95 9.381 9.829 9.381 3.436 0 5.542-.93 7.295-1.948l1.177 1.698c-1.711.966-4.461 2.209-8.78 2.209zm-2.344-14.305c-.715 1.34-1.177 3.076-1.177 4.424 0 3.61 3.522 3.633 5.252.239.712-1.394 1.171-3.171 1.171-4.529 0-2.917-3.495-3.434-5.246-.134z"/>
                        </svg>
                    </a>
                    <a href="https://bsky.app/profile/maxmohr.net" target="_blank" class="mr-4">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -8 64 68.414" width="2232" height="2500" class="h-6 w-6 sm:h-8 sm:w-8 fill-current text-slate-600 sm:text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                            <path fill="#0085ff" d="M13.873 3.805C21.21 9.332 29.103 20.537 32 26.55v15.882c0-.338-.13.044-.41.867-1.512 4.456-7.418 21.847-20.923 7.944-7.111-7.32-3.819-14.64 9.125-16.85-7.405 1.264-15.73-.825-18.014-9.015C1.12 23.022 0 8.51 0 6.55 0-3.268 8.579-.182 13.873 3.805zm36.254 0C42.79 9.332 34.897 20.537 32 26.55v15.882c0-.338.13.044.41.867 1.512 4.456 7.418 21.847 20.923 7.944 7.111-7.32 3.819-14.64-9.125-16.85 7.405 1.264 15.73-.825 18.014-9.015C62.88 23.022 64 8.51 64 6.55c0-9.818-8.578-6.732-13.873-2.745z" fill="currentColor"/>
                        </svg>
                    </a>
                    <a href="https://unsplash.com/@maxjmohr" target="_blank" class="mr-4">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512" class="h-6 w-6 sm:h-8 sm:w-8 fill-current text-slate-600 sm:text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                            <path d="M448 230.2V480H0V230.2H141.1V355.1H306.9V230.2zM306.9 32H141.1V156.9H306.9z"/>
                        </svg>
                    </a>
                    <a href="https://lichess.org/@/maxjmohr" target="_blank" class="mr-4">
                        <svg viewBox="-2 -2 51.573 55.285" xmlns="http://www.w3.org/2000/svg" width="2385" height="2500" class="h-6 w-6 sm:h-8 sm:w-8 fill-current text-slate-600 sm:text-slate-700 2xl:hover:text-slate-600 dark:text-slate-600 dark:2xl:hover:text-slate-100">
                            <path d="M38.956.5c-3.53.418-6.452.902-9.286 2.984C5.534 1.786-.692 18.533.68 29.364 3.493 50.214 31.918 55.785 41.329 41.7c-7.444 7.696-19.276 8.752-28.323 3.084S-.506 27.392 4.683 17.567C9.873 7.742 18.996 4.535 29.03 6.405c2.43-1.418 5.225-3.22 7.655-3.187l-1.694 4.86 12.752 21.37c-.439 5.654-5.459 6.112-5.459 6.112-.574-1.47-1.634-2.942-4.842-6.036-3.207-3.094-17.465-10.177-15.788-16.207-2.001 6.967 10.311 14.152 14.04 17.663 3.73 3.51 5.426 6.04 5.795 6.756 0 0 9.392-2.504 7.838-8.927L37.4 7.171z" fill="currentColor"/>
                        </svg>
                    </a>
                </div>
            </div>
            // Ascii for desktop
            <div class="hidden xl:block xl:col-start-11 xl:col-end-14">
                <div class="scale-30">
                    <pre class="text-base text-mono leading-relaxed tracking-[.32em] text-slate-600 dark:text-slate-500">
                        {ascii_display}
                    </pre>
                </div>
            </div>
        </div>

        // Projects
        <div id="projects" class="relative z-20 h-9/10 mx-0 sm:mx-12 lg:mx-24 min-[2000px]:mx-72 my-8 rounded-4xl bg-stone-200/50 dark:bg-slate-800/50" data-aos="fade-up">
            <h1 class="pt-14 w-full text-center text-3xl sm:text-4xl font-medium italic text-slate-700 dark:text-slate-300">{"Some of my projects"}</h1>
            <div class="h-9/10 pt-8 pl-4 sm:p-16 flex flex-nowrap overflow-x-scroll no-scrollbar">
                { for content_projects_html.iter().cloned()}
            </div>
        </div>

        // Professional experience
        <div id="profexper" class="relative z-20 mx-6 sm:mx-12 lg:mx-24 min-[2000px]:mx-72">
            <h1 class="pt-40 sm:pt-18 w-full text-center text-3xl sm:text-4xl font-medium italic text-slate-700 dark:text-slate-300" data-aos="fade-up">{"Professional experience"}</h1>
            <div class="pt-10 flex flex-col gap-y-6 sm:gap-y-8">
                { for content_profexper_timeline.iter().cloned() }
            </div>
        </div>

        // Education
        <div id="educ" class="relative z-20 mx-6 sm:mx-12 lg:mx-24 min-[2000px]:mx-72 pb-10">
            <h1 class="pt-40 sm:pt-18 w-full text-center text-3xl sm:text-4xl font-medium italic text-slate-700 dark:text-slate-300" data-aos="fade-up">{"Education"}</h1>
            <div class="pt-10 flex flex-col gap-y-6 sm:gap-y-8">
                { for content_educ_timeline.iter().cloned() }
            </div>
        </div>

        <Footer/>

        </>
    }
}
