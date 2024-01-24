use std::fmt::format;

use gloo_net::http::Request;
use leptos::*;
use leptos_meta::Title;
use leptos_router::{use_params, Params};
use serde::{Deserialize, Serialize};

#[derive(Params, Clone, PartialEq, Eq, Debug)]
struct BlogParams {
    slug: String,
}

#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params::<BlogParams>();

    let content = create_resource(
        move || params.get(),
        move |params| async move { get_post(params.unwrap().slug).await },
    );

    view! {
    <main class="pt-8 pb-16 lg:pt-16 lg:pb-24 dark:bg-gray-900 antialiased">
        <div class="flex justify-between px-4 mx-auto max-w-screen-xl ">
            <article class="mx-auto w-full max-w-2xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert">
                <Suspense
                fallback=move || view! {<p>"Loading..."</p> }
                >
                    {move || match content.get() {
                        None => view! {<h1>"Error Loading Content"</h1>}.into_any(),
                        Some(content) => match content {
                            Ok(content) => view! {
                            <div>
                                <Title text=format!("~/blog/{}", content.front_matter.slug)/>
                                <div>
                                    <header class="mb-4 lg:mb-6 not-format">
                                        <h1 class="mb-4 text-3xl font-extrabold leading-tight text-blue-700 lg:mb-6 lg:text-4xl dark:text-white">{content.front_matter.title}</h1>
                                        <h3 class="mb-2 text-xl font-bold leading-tight text-blue-500 lg:m3-6 lg:text-2xl dark:text-white">{content.front_matter.date}</h3>
                                    </header>
                                    <div inner_html=content.content />
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
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrontMatter {
    pub date: String,
    pub description: String,
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub front_matter: FrontMatter,
    pub content: String,
}

pub async fn get_post(slug: String) -> Result<Content, ServerFnError> {
    use gray_matter::engine::YAML;
    use gray_matter::Matter;
    use pulldown_cmark::{html, Options, Parser};

    let res = Request::get(format!("/assets/blog/{slug}.md").as_str())
        .send()
        .await
        .unwrap();
    let content = res.text().await?;
    let content = content.as_str();

    let matter = Matter::<YAML>::new();
    let front_matter = matter.parse_with_struct::<FrontMatter>(content).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&front_matter.content, options);

    let mut html_ouput = String::new();
    html::push_html(&mut html_ouput, parser);

    Ok(Content {
        front_matter: front_matter.data,
        content: html_ouput,
    })
}
