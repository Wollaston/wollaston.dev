use leptos::*;

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let aratype: Project = Project {
        title: String::from("aratype"),
        path: String::from("projects/aratype"),
        github_link: String::from("https://github.com/Wollaston/aratype"),
        description: String::from("A simple WASM app using Rust and Leptos that converts English letters to Arabic equivalents according to the Buckwalter transliteration table."),
    };

    let wollaston_dev: Project = Project {
        title: String::from("wollaston.dev"),
        path: String::from("/"),
        github_link: String::from("https://github.com/Wollaston/wollaston.dev"),
        description: String::from("My personal website, built using Rust, Leptos, and axum."),
    };

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-stone-100">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 dark:after:border-r-[#fd8a04] after:border-r-8 after:bg-blue-700 dark:after:bg-[#fd8a04] after:animate-blink">"my_projects"</span></h1>
                        <ProjectCard project=aratype />
                        <ProjectCard project=wollaston_dev />
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg h-full">
                        <img class="object-scale-down drop-shadow-xl rounded-lg w-full min-h-0" src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/e6db900a-4de6-4026-a664-a8d9a8603d00/public" alt="An Astronaut Organizing his Projects in a Space Station."/>
                    </div>
                </div>
            </section>
        </div>
    }
}

struct Project {
    title: String,
    path: String,
    github_link: String,
    description: String,
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="max-w m-3 p-6 bg-stone-100 border border-gray-200 rounded-lg shadow dark:bg-indigo-900 dark:border-gray-700">
            <a href={project.path}>
                <h5 class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">{project.title}</h5>
            </a>
            <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">{project.description}</p>
            <a href={project.github_link} target="_blank" rel="noopener noreferrer" class="text-x inline-flex items-center text-blue-700 dark:text-stone-100 hover:underline">
                <img src="/assets/github-mark.svg" alt="The GitHub Logo" class="block dark:hidden h-6 w-6 m-2"/>
                <img src="/assets/github-mark-white.svg" alt="The GitHub Logo" class="hidden dark:block h-6 w-6 m-2"/>
               "GitHub Repo"
            </a>
        </div>
    }
}
