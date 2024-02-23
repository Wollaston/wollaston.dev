use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn BlogSection() -> impl IntoView {
    let blogs = create_resource(|| (), |_| async move { get_blogs().await });

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-stone-100">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 dark:after:border-r-[#fd8a04] after:border-r-8 after:bg-blue-700 dark:after:bg-[#fd8a04] after:animate-blink">"my_blog"</span></h1>
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
                                                    view! {<BlogCard blog/>}
                                                }).collect_view()
                                            }
                                        }
                                    })
                                }
                            }
                        </Transition>
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg h-full">
                        <img class="object-scale-down drop-shadow-xl rounded-lg w-full min-h-0" src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/7f26b186-5e5a-4037-cd8b-1d055615d700/public" alt="An Astronaut Writing a Blog in a Space Station."/>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[component]
fn BlogCard(blog: BlogMetadata) -> impl IntoView {
    view! {
        <div class="max-w p-6 bg-stone-100 border border-gray-200 rounded-lg shadow dark:bg-indigo-900 dark:border-gray-700">
            <a href={format!("blog/{}",blog.slug)}>
                <h5 class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">{blog.title}</h5>
            </a>
            <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">{blog.description}</p>
        </div>
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
