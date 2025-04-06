use leptos::{either::Either, prelude::*};
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
    let blogs = Resource::new(|| (), |_| async move { get_blogs().await });
    view! {
        <ul class="m-2 p-2">
            <Transition fallback=move || view! {<p>"Loading..."</p>}>
                {move || {
                    blogs.get()
                    .map(move |blogs| match blogs {
                            Err(e) => {
                                Either::Left(view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view())
                            }
                            Ok(blogs) => Either::Right({
                                if blogs.is_empty() {
                                    Either::Left(view! { <p class="font-semibold text-lg text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">"No blogs were found."</p> }.into_view())
                                } else {
                                    Either::Right(blogs.into_iter().map(move |blog| {
                                        view! {<li class="font-semibold text-3xl text-stone-800 hover:text-blue-700 hover:underline dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href={format!("/blog/{}", blog.slug)}>{blog.title.to_uppercase()}</a></li>}
                                    }).collect_view())
                                }
                            })
                        })
                    }
                }
            </Transition>
        </ul>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    use crate::config::AppState;

    let state = expect_context::<AppState>();

    let blogs = state.db.select("blogs").await?;

    Ok(blogs)
}
