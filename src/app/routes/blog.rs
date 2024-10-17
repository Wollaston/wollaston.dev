use leptos::*;
use serde::{Deserialize, Serialize};

pub mod slug;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col justify-center items-center bg-stone-50 dark:bg-gray-900">
            <Blogs />
        </div>
    }
}

#[component]
fn Blogs() -> impl IntoView {
    let blogs = create_resource(|| (), |_| async move { get_blogs().await });
    view! {
        <ul class="m-2 p-2">
            <Transition fallback=move || view! {<p>"Loading..."</p>}>
                {move || {
                    blogs.get()
                    .map(move |blogs| match blogs {
                            Err(e) => {
                                view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                            }
                            Ok(blogs) => {
                                if blogs.is_empty() {
                                    view! { <p>"No blogs were found."</p> }.into_view()
                                } else {
                                    blogs.into_iter().map(move |blog| {
                                        view! {<li class="font-semibold text-3xl text-stone-800 hover:text-blue-700 hover:underline dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href={format!("/blog/{}", blog.slug)}>{blog.title.to_uppercase()}</a></li>}
                                    }).collect_view()
                                }
                            }
                        })
                    }
                }
            </Transition>
        </ul>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct BlogMetadata {
    id: i32,
    title: String,
    slug: String,
    description: String,
    created_date: String,
    last_modified_date: String,
}

#[server]
async fn get_blogs() -> Result<Vec<BlogMetadata>, ServerFnError> {
    use crate::db::pool;

    use futures::TryStreamExt;

    let pool = pool()?;

    let mut blogs: Vec<BlogMetadata> = Vec::new();
    let mut rows = sqlx::query_as::<_, BlogMetadata>(
        "SELECT id, title, slug, description, created_date, last_modified_date FROM blog",
    )
    .fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        blogs.push(row);
    }

    Ok(blogs)
}
