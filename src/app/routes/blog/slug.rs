use chrono::NaiveDateTime;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use serde::{Deserialize, Serialize};

#[derive(Params, Clone, PartialEq, Eq, Debug)]
struct BlogParams {
    slug: Option<String>,
}

#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params::<BlogParams>();

    let blog = Resource::new(
        move || params.get(),
        move |params| async move { get_blog(params.unwrap().slug.unwrap()).await },
    );

    view! {
        <div class="flex-1 justify-center items-center h-screen overflow-y-auto bg-stone-50 dark:bg-gray-900">
            <main class="pt-8 pb-16 lg:pt-16 lg:pb-24 dark:bg-gray-900 antialiased">
                <div class="flex justify-between px-4 mx-auto max-w-(--breakpoint-xl) ">
                    <article class="mx-auto w-full max-w-2xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert">
                        <Suspense
                        fallback=move || view! {<p>"Loading..."</p> }
                        >
                            {move || match blog.get() {
                                None => view! {<h1>"Error Loading Content"</h1>}.into_any(),
                                Some(blog) => match blog {
                                    Ok(blog) => view! {
                                    <div>
                                        <Title text=format!("~/blog/{}", blog.0.slug)/>
                                        <div>
                                            <header class="mb-4 lg:mb-6 not-format">
                                                <h1 class="mb-4 text-3xl font-extrabold leading-tight text-blue-700 lg:mb-6 lg:text-4xl dark:text-stone-100">{blog.0.title}</h1>
                                                <h3 class="mb-2 text-xl font-bold leading-tight text-blue-500 lg:m3-6 lg:text-2xl dark:text-stone-100">
                                                    {move || NaiveDateTime::parse_from_str(blog.0.created_date.as_str(), "%F %H:%M:%S%.3f").unwrap().format("%e %B %Y").to_string()}
                                                </h3>
                                            </header>
                                            <div inner_html=blog.1 />
                                            // <h1 class="md-h1">"test"</h1>
                                        </div>
                                    </div>
                                }.into_any(),
                                    Err(_) => view! {<h1>"Error Loading Content"</h1>}.into_any(),
                                }
                            }}
                        </Suspense>
                    </article>
                </div>
            </main>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blog {
    id: i32,
    title: String,
    slug: String,
    description: String,
    created_date: String,
    last_modified_date: String,
    content: String,
}

#[server]
pub async fn get_blog(slug: String) -> Result<(Blog, String), ServerFnError> {
    use crate::config::AppState;
    use pulldown_cmark::{html, Options, Parser};

    let state = expect_context::<AppState>();

    let blog: Blog = state
        .db
        .select(("blogs", slug))
        .await?
        .ok_or_else(|| ServerFnError::new("Could not fetch blog"))?;

    let content = blog.content.as_str();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok((blog, html_output))
}
