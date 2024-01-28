use futures::TryStreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn ProjectsSection() -> impl IntoView {
    let projects = create_resource(|| (), |_| async move { get_projects().await });

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-stone-100">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 dark:after:border-r-[#fd8a04] after:border-r-8 after:bg-blue-700 dark:after:bg-[#fd8a04] after:animate-blink">"my_projects"</span></h1>
                        <Transition fallback=move || view! {<p>"Loading..."</p>}>
                            {move || {
                                projects.get()
                                .map(move |projects| match projects {
                                        Err(e) => {
                                            view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                                        }
                                        Ok(projects) => {
                                            if projects.is_empty() {
                                                view! { <p>"No projects were found."</p> }.into_view()
                                            } else {
                                                projects.into_iter().map(move |project| {
                                                    view! {<ProjectCard project/>}
                                                }).collect_view()
                                            }
                                        }
                                    })
                                }
                            }
                        </Transition>
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg h-full">
                        <img class="object-scale-down drop-shadow-xl rounded-lg w-full min-h-0" src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/e6db900a-4de6-4026-a664-a8d9a8603d00/public" alt="An Astronaut Organizing his Projects in a Space Station."/>
                    </div>
                </div>
            </section>
        </div>
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub title: String,
    pub path: String,
    pub github_link: String,
    pub description: String,
}

#[server]
async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use crate::content::ssr::db;

    let mut conn = db().await?;

    let mut projects: Vec<Project> = Vec::new();
    let mut rows = sqlx::query_as::<_, Project>("SELECT * FROM projects").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        projects.push(row);
    }

    Ok(projects)
}
