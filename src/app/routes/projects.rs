use leptos::{either::Either, prelude::*};
use serde::{Deserialize, Serialize};

pub mod aratype;

#[component]
pub fn Project() -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col justify-center items-center bg-stone-50 dark:bg-gray-900">
            <Projects />
        </div>
    }
}

#[component]
fn Projects() -> impl IntoView {
    let projects = Resource::new(|| (), |_| async move { get_projects().await });

    view! {
        <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
            <ul>
                <Transition fallback=move || view! {<p>"Loading..."</p>}>
                    {move || {
                        projects.get()
                        .map(move |projects| match projects {
                                Err(e) => {
                                    Either::Left(view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view())
                                }
                                Ok(projects) => Either::Right({
                                    if projects.is_empty() {
                                        Either::Left(view! { <p>"No projects were found."</p> }.into_any())
                                    } else {
                                        Either::Right(projects.into_iter().map(move |project| {
                                            view! {<li class="p-2 m-2 font-semibold text-3xl text-stone-800 hover:text-blue-700 hover:underline dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href={project.path}>{project.title.to_uppercase()}</a></li>}
                                        }).collect_view())
                                    }
                                })
                            })
                        }
                    }
                </Transition>
            </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub path: String,
    pub github_link: Option<String>,
    pub description: String,
}

#[server]
async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use crate::config::AppState;

    let state = expect_context::<AppState>();

    let projects = state.db.select("projects").await?;

    Ok(projects)
}
