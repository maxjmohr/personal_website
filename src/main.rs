use crate::safehtml::SafeHtml;
use comrak::{markdown_to_html, ComrakOptions};
//use std::fs;
use yew::prelude::*;

mod safehtml;

#[function_component(App)]
fn app() -> Html {
    // Render settings
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;

    // Get content
    let content_profexper =
        markdown_to_html(include_str!("./../res/content/profexper.md"), &options);
    let content_educ = markdown_to_html(include_str!("./../res/content/educ.md"), &options);
    let content_projects = markdown_to_html(include_str!("./../res/content/projects.md"), &options);
    //let content_projects = fs::read_to_string("./../res/content/projects.html").unwrap();
    let content_techskills =
        markdown_to_html(include_str!("./../res/content/techskills.md"), &options);

    html! {
        <>
        // Sidebar
        <aside id="sidebar" class="group sidebar invisible lg:visible">
            <ul class="sidebar_links">
                <li>
                    <a href="#aboutme" class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="sidebar_svg">
                            <path d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm7.753 18.305c-.261-.586-.789-.991-1.871-1.241-2.293-.529-4.428-.993-3.393-2.945 3.145-5.942.833-9.119-2.489-9.119-3.388 0-5.644 3.299-2.489 9.119 1.066 1.964-1.148 2.427-3.393 2.945-1.084.25-1.608.658-1.867 1.246-1.405-1.723-2.251-3.919-2.251-6.31 0-5.514 4.486-10 10-10s10 4.486 10 10c0 2.389-.845 4.583-2.247 6.305z"/>
                        </svg>
                        <span class="sidebar_span hidden group-hover:block">{"About me"}</span>
                    </a>
                </li>
                <li>
                    <a href="#profexper" class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="sidebar_svg">
                            <path d="M12.23 15.5c-6.801 0-10.367-1.221-12.23-2.597v9.097h24v-8.949c-3.218 2.221-9.422 2.449-11.77 2.449zm1.77 2.532c0 1.087-.896 1.968-2 1.968s-2-.881-2-1.968v-1.032h4v1.032zm-14-8.541v-2.491h24v2.605c0 5.289-24 5.133-24-.114zm9-7.491c-1.104 0-2 .896-2 2v2h2v-1.5c0-.276.224-.5.5-.5h5c.276 0 .5.224.5.5v1.5h2v-2c0-1.104-.896-2-2-2h-6z"/>
                        </svg>
                        <span class="sidebar_span hidden group-hover:block">{"Professional experience"}</span>
                    </a>
                </li>
                <li>
                    <a href="#educ" class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="sidebar_svg">
                            <path d="M20 12.875v5.068c0 2.754-5.789 4.057-9 4.057-3.052 0-9-1.392-9-4.057v-6.294l9 4.863 9-3.637zm-8.083-10.875l-12.917 5.75 12 6.5 11-4.417v7.167h2v-8.25l-12.083-6.75zm13.083 20h-4c.578-1 1-2.5 1-4h2c0 1.516.391 2.859 1 4z"/>
                        </svg>
                        <span class="sidebar_span hidden group-hover:block">{"Education"}</span>
                    </a>
                </li>
                <li>
                    <a href="#projects" class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="sidebar_svg">
                            <path d="M8.566 17.842c-.945 2.462-3.678 4.012-6.563 4.161.139-2.772 1.684-5.608 4.209-6.563l.51.521c-1.534 1.523-2.061 2.765-2.144 3.461.704-.085 2.006-.608 3.483-2.096l.505.516zm-1.136-11.342c-1.778-.01-4.062.911-5.766 2.614-.65.649-1.222 1.408-1.664 2.258 1.538-1.163 3.228-1.485 5.147-.408.566-1.494 1.32-3.014 2.283-4.464zm5.204 17.5c.852-.44 1.61-1.013 2.261-1.664 1.708-1.706 2.622-4.001 2.604-5.782-1.575 1.03-3.125 1.772-4.466 2.296 1.077 1.92.764 3.614-.399 5.15zm11.312-23.956c-.428-.03-.848-.044-1.261-.044-9.338 0-14.465 7.426-16.101 13.009l4.428 4.428c5.78-1.855 12.988-6.777 12.988-15.993v-.059c-.002-.437-.019-.884-.054-1.341zm-5.946 7.956c-1.105 0-2-.895-2-2s.895-2 2-2 2 .895 2 2-.895 2-2 2z"/>
                        </svg>
                        <span class="sidebar_span hidden group-hover:block">{"Projects"}</span>
                    </a>
                </li>
                <li>
                    <a href="#techskills" class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="sidebar_svg">
                            <path d="M22 6v16h-20v-16h20zm2-6h-24v24h24v-24zm-11 11v1.649l3.229 1.351-3.229 1.347v1.653l5-2.201v-1.599l-5-2.2zm-7 2.201v1.599l5 2.2v-1.653l-3.229-1.347 3.229-1.351v-1.649l-5 2.201z"/>
                        </svg>
                        <span class="sidebar_span hidden group-hover:block">{"Technical skills"}</span>
                    </a>
                </li>
                <li>
                    <a href="#langs" class="flex items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="sidebar_svg">
                            <path d="M21 21h-1.713l-.658-1.846h-3l-.663 1.846h-1.659l3.04-8h1.603l3.05 8zm-2.814-3.12l-1.049-3.018-1.054 3.018h2.103zm-9.464-12.037l.125-.562-1.02-.199-.101.464c-.345-.05-.712-.057-1.083-.019.009-.249.023-.494.045-.728h1.141v-.966h-1.004c.049-.246.092-.394.134-.533l-.997-.3c-.072.245-.134.484-.195.833h-1.138v.966h1.014c-.027.312-.043.637-.048.964-1.119.411-1.595 1.195-1.595 1.905 0 .84.663 1.578 1.709 1.482 1.301-.118 2.169-1.1 2.679-2.308.525.303.746.814.548 1.286-.185.436-.725.852-1.757.831v1.041c1.146.018 2.272-.417 2.715-1.469.431-1.028-.062-2.151-1.172-2.688zm-1.342.71c-.162.36-.375.717-.648.998-.041-.3-.07-.628-.086-.978.249-.032.499-.038.734-.02zm-1.758.336c.028.44.078.844.148 1.205-.927.169-.963-.744-.148-1.205zm15.378 5.111c.552 0 1 .449 1 1v8c0 .551-.448 1-1 1h-8c-.552 0-1-.449-1-1v-8c0-.551.448-1 1-1h8zm0-2h-8c-1.656 0-3 1.343-3 3v8c0 1.657 1.344 3 3 3h8c1.657 0 3-1.343 3-3v-8c0-1.657-1.343-3-3-3zm-13 3c0-.342.035-.677.102-1h-5.102c-.552 0-1-.449-1-1v-8c0-.551.448-1 1-1h8c.552 0 1 .449 1 1v5.101c.323-.066.657-.101 1-.101h1v-5c0-1.657-1.343-3-3-3h-8c-1.656 0-3 1.343-3 3v8c0 1.657 1.344 3 3 3h5v-1z"/>
                        </svg>
                        <span class="sidebar_span hidden group-hover:block">{"Languages"}</span>
                    </a>
                </li>
            </ul>
        </aside>

        // Bottombar
        <aside id="bottombar" class="group bottombar visible lg:invisible">
            <ul class="flex flex-row h-max w-auto mx-2">
                <li class="w-28 mx-4">
                    <a href="#aboutme" class="flex flex-col items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="bottombar_svg">
                            <path d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm7.753 18.305c-.261-.586-.789-.991-1.871-1.241-2.293-.529-4.428-.993-3.393-2.945 3.145-5.942.833-9.119-2.489-9.119-3.388 0-5.644 3.299-2.489 9.119 1.066 1.964-1.148 2.427-3.393 2.945-1.084.25-1.608.658-1.867 1.246-1.405-1.723-2.251-3.919-2.251-6.31 0-5.514 4.486-10 10-10s10 4.486 10 10c0 2.389-.845 4.583-2.247 6.305z"/>
                        </svg>
                        <span class="bottombar_span">{"About me"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#profexper" class="flex flex-col items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="bottombar_svg">
                            <path d="M12.23 15.5c-6.801 0-10.367-1.221-12.23-2.597v9.097h24v-8.949c-3.218 2.221-9.422 2.449-11.77 2.449zm1.77 2.532c0 1.087-.896 1.968-2 1.968s-2-.881-2-1.968v-1.032h4v1.032zm-14-8.541v-2.491h24v2.605c0 5.289-24 5.133-24-.114zm9-7.491c-1.104 0-2 .896-2 2v2h2v-1.5c0-.276.224-.5.5-.5h5c.276 0 .5.224.5.5v1.5h2v-2c0-1.104-.896-2-2-2h-6z"/>
                        </svg>
                        <span class="bottombar_span">{"Experience"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#educ" class="flex flex-col items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="bottombar_svg">
                            <path d="M20 12.875v5.068c0 2.754-5.789 4.057-9 4.057-3.052 0-9-1.392-9-4.057v-6.294l9 4.863 9-3.637zm-8.083-10.875l-12.917 5.75 12 6.5 11-4.417v7.167h2v-8.25l-12.083-6.75zm13.083 20h-4c.578-1 1-2.5 1-4h2c0 1.516.391 2.859 1 4z"/>
                        </svg>
                        <span class="bottombar_span">{"Education"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#projects" class="flex flex-col items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="bottombar_svg">
                            <path d="M8.566 17.842c-.945 2.462-3.678 4.012-6.563 4.161.139-2.772 1.684-5.608 4.209-6.563l.51.521c-1.534 1.523-2.061 2.765-2.144 3.461.704-.085 2.006-.608 3.483-2.096l.505.516zm-1.136-11.342c-1.778-.01-4.062.911-5.766 2.614-.65.649-1.222 1.408-1.664 2.258 1.538-1.163 3.228-1.485 5.147-.408.566-1.494 1.32-3.014 2.283-4.464zm5.204 17.5c.852-.44 1.61-1.013 2.261-1.664 1.708-1.706 2.622-4.001 2.604-5.782-1.575 1.03-3.125 1.772-4.466 2.296 1.077 1.92.764 3.614-.399 5.15zm11.312-23.956c-.428-.03-.848-.044-1.261-.044-9.338 0-14.465 7.426-16.101 13.009l4.428 4.428c5.78-1.855 12.988-6.777 12.988-15.993v-.059c-.002-.437-.019-.884-.054-1.341zm-5.946 7.956c-1.105 0-2-.895-2-2s.895-2 2-2 2 .895 2 2-.895 2-2 2z"/>
                        </svg>
                        <span class="bottombar_span">{"Projects"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#techskills" class="flex flex-col items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="bottombar_svg">
                            <path d="M22 6v16h-20v-16h20zm2-6h-24v24h24v-24zm-11 11v1.649l3.229 1.351-3.229 1.347v1.653l5-2.201v-1.599l-5-2.2zm-7 2.201v1.599l5 2.2v-1.653l-3.229-1.347 3.229-1.351v-1.649l-5 2.201z"/>
                        </svg>
                        <span class="bottombar_span">{"Skills"}</span>
                    </a>
                </li>
                <li class="w-28 mx-4">
                    <a href="#langs" class="flex flex-col items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="bottombar_svg">
                            <path d="M21 21h-1.713l-.658-1.846h-3l-.663 1.846h-1.659l3.04-8h1.603l3.05 8zm-2.814-3.12l-1.049-3.018-1.054 3.018h2.103zm-9.464-12.037l.125-.562-1.02-.199-.101.464c-.345-.05-.712-.057-1.083-.019.009-.249.023-.494.045-.728h1.141v-.966h-1.004c.049-.246.092-.394.134-.533l-.997-.3c-.072.245-.134.484-.195.833h-1.138v.966h1.014c-.027.312-.043.637-.048.964-1.119.411-1.595 1.195-1.595 1.905 0 .84.663 1.578 1.709 1.482 1.301-.118 2.169-1.1 2.679-2.308.525.303.746.814.548 1.286-.185.436-.725.852-1.757.831v1.041c1.146.018 2.272-.417 2.715-1.469.431-1.028-.062-2.151-1.172-2.688zm-1.342.71c-.162.36-.375.717-.648.998-.041-.3-.07-.628-.086-.978.249-.032.499-.038.734-.02zm-1.758.336c.028.44.078.844.148 1.205-.927.169-.963-.744-.148-1.205zm15.378 5.111c.552 0 1 .449 1 1v8c0 .551-.448 1-1 1h-8c-.552 0-1-.449-1-1v-8c0-.551.448-1 1-1h8zm0-2h-8c-1.656 0-3 1.343-3 3v8c0 1.657 1.344 3 3 3h8c1.657 0 3-1.343 3-3v-8c0-1.657-1.343-3-3-3zm-13 3c0-.342.035-.677.102-1h-5.102c-.552 0-1-.449-1-1v-8c0-.551.448-1 1-1h8c.552 0 1 .449 1 1v5.101c.323-.066.657-.101 1-.101h1v-5c0-1.657-1.343-3-3-3h-8c-1.656 0-3 1.343-3 3v8c0 1.657 1.344 3 3 3h5v-1z"/>
                        </svg>
                        <span class="bottombar_span">{"Languages"}</span>
                    </a>
                </li>
            </ul>
        </aside>

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
            <SafeHtml html={content_profexper}/>
        </div>

        // Education
        <div id="educ" class="bg-gradient-to-b from-green-100 to-teal-200 dark:from-emerald-900 dark:to-teal-800
        pt-28 pb-14 pl-12 lg:pl-44 pr-20">
        <h1 class="text-8xl lg:text-7xl manual_h1" data-aos="fade">{"Education"}</h1>
            <SafeHtml html={content_educ}/>
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
            <SafeHtml html={content_techskills}/>
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
