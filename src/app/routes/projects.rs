use leptos::*;
use serde::{Deserialize, Serialize};

pub mod aratype;
pub mod projects_base;

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
    let projects = create_resource(|| (), |_| async move { get_projects().await });

    view! {
        <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
            <ul>
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
                                            view! {<li class="p-2 m-2 font-semibold text-3xl text-stone-800 hover:text-blue-700 hover:underline dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href={project.path}>{project.title.to_uppercase()}</a></li>}
                                        }).collect_view()
                                    }
                                }
                            })
                        }
                    }
                </Transition>
            </ul>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub github_link: String,
    pub description: String,
}

#[server]
async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use crate::db::pool;
    use futures::TryStreamExt;

    let pool = pool()?;

    let mut projects: Vec<Project> = Vec::new();
    let mut rows = sqlx::query_as::<_, Project>("SELECT * FROM projects").fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        projects.push(row);
    }

    Ok(projects)
}
