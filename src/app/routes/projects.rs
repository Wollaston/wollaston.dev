use leptos::*;
use leptos_meta::Title;

pub mod aratype;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="~/projects/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
        <ProjectsSection />
        </div>
    }
}

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let test_project: Project = Project {
        title: String::from("aratype"),
        path: String::from("projects/aratype"),
        github_link: String::from("https://github.com/Wollaston/aratype"),
        description: String::from("A simple WASM app using Rust and Leptos that converts English letters to Arabic equivalents according to the Buckwalter transliteration table."),
    };

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-white">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-black after:border-r-8 after:bg-black after:animate-blink">"my_projects"</span></h1>
                        <Project project=test_project />
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg object-scale-down">
                        <img class="drop-shadow-xl rounded-lg h-max w-max" src="assets/projects_astro.png" alt="An Astronaut Organizing his Projects in a Space Station."/>
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
fn Project(project: Project) -> impl IntoView {
    view! {
        <div class="max-w p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
            <a href={project.path}>
                <h5 class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 dark:text-white">{project.title}</h5>
            </a>
            <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">{project.description}</p>
            <a href={project.github_link} target="_blank" rel="noopener noreferrer" class="text-x inline-flex items-center text-blue-600 hover:underline">
                <img src="assets/github-mark.svg" alt="The GitHub Logo" class="h-6 w-6 m-2"/>

               "GitHub Repo"
            </a>
        </div>
    }
}
