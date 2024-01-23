use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::*;
use leptos_router::{use_params, Params};
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;

#[derive(Params, Clone, PartialEq, Eq, Debug)]
struct BlogParams {
    slug: String,
}

#[component]
pub fn Slug() -> impl IntoView {
    let params = use_params::<BlogParams>();

    let raw_content = create_resource(
        move || params.get(),
        move |params| async move { get_post(params.unwrap().slug).await },
    );
    // leptos::logging::log!("{:?}", raw_content);
    // let content = raw_content.get().unwrap().unwrap();
    // let content = content.as_str();
    //
    // let matter = Matter::<YAML>::new();
    // let front_matter = matter.parse_with_struct::<FrontMatter>(content).unwrap();
    //
    // let mut options = Options::empty();
    // options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    //
    // let parser = Parser::new_ext(&front_matter.content, options);
    //
    // let mut html_ouput = String::new();
    // html::push_html(&mut html_ouput, parser);

    view! {
    <main class="pt-8 pb-16 lg:pt-16 lg:pb-24 dark:bg-gray-900 antialiased">
        <div class="flex justify-between px-4 mx-auto max-w-screen-xl ">
            <article class="mx-auto w-full max-w-2xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert">
                <Suspense
                fallback=move || view! {<p>"Loading..."</p> }
                >
                    <header class="mb-4 lg:mb-6 not-format">
                        <h1 class="mb-4 text-3xl font-extrabold leading-tight text-blue-700 lg:mb-6 lg:text-4xl dark:text-white">"TITLE"</h1>
                    </header>
                    {move || match raw_content.get() {
                        None => view! {<h1>"Error Loading Content"</h1>}.into_any(),
                        Some(raw_content) => view! {<h1>{raw_content}</h1>}.into_any(),
                    }}
                </Suspense>
            </article>
        </div>
    </main>
    }
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    date: String,
    description: String,
    slug: String,
    title: String,
}

#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<String, ServerFnError> {
    let res = reqwest::get(format!("http://127.0.0.1:3000/assets/blog/{slug}.md")).await?;
    leptos::logging::log!("{:#?}", res);
    let content = res.text().await?;
    leptos::logging::log!("{:#?}", content);
    Ok(content)
}
