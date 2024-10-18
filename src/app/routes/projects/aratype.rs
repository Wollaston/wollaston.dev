use aratype::buckwalter::convert_en_ar;
use leptos::*;
use leptos_meta::Title;

use crate::app::routes::projects::Project;

#[component]
pub fn Aratype() -> impl IntoView {
    let project = create_resource(|| (), |_| async move { get_project().await });

    view! {
        <Title text="~/projects/aratype/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50 dark:bg-gray-900">
            <Transition fallback=move || view! {<p>"Loading..."</p>}>
                {move || {
                    project.get()
                    .map(move |project| match project {
                            Err(e) => {
                                view! { <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view()
                            }
                            Ok(project) => {
                                    view! {<Converter project/>}.into_view()
                                }
                            }
                        )
                    }
                }
            </Transition>
        </div>
    }
}

#[component]
pub fn Converter(project: Project) -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let converted = move || convert_en_ar(name());

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
            <div class="flex flex-col">
                <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-stone-100">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 dark:after:border-r-[#fd8a04] after:border-r-8 after:bg-blue-700 dark:after:bg-[#fd8a04] after:animate-blink">{project.title}</span></h1>
                <h1 class="m-2 font-medium text-stone-800 dark:text-stone-100">{project.description}</h1>
                <a href="https://en.wikipedia.org/wiki/Buckwalter_transliteration" target="_blank" rel="noopener noreferrer" class="m-2 font-normal italic text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">
                    "Buckwalter Transliteration Wiki"
                </a>
                <a href={project.github_link} target="_blank" rel="noopener noreferrer" class="inline-flex items-center text-blue-700 dark:text-stone-100 hover:underline">
                    <img src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/9848fd3b-3d8e-451f-6cb2-5973b9395700/public" alt="The GitHub Logo" class="block dark:hidden h-6 w-6 m-2"/>
                    <img src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/868ade0e-a4f8-4149-b122-6f1c4f652200/public" alt="The GitHub Logo" class="hidden dark:block h-6 w-6 m-2"/>
                    "GitHub Repo"
                </a>
            </div>
                <div class="flex-1 grid grid-cols-2 p-2 m-2 border border-blue-700 dark:border-indigo-900 rounded-md">
                    <div class="m-2">
                        <div>
                            <label for="input" class="block mb-2 text-sm font-medium text-gray-900 dark:text-stone-100"
                            >"Input: "</label>
                            <textarea id="input" rows="10" class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:border-blue-500 focus:outline-none focus:ring-0 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-stone-100 dark:focus:border-[#fd8a04]"
                                                on:input=move |ev| {
                                    set_name(event_target_value(&ev));
                                }
                                prop:value=name
                            placeholder="Type your input here..."></textarea>
                        </div>
                    </div>
                    <div class="flex flex-col flex-1 m-2">
                        <p class="block mb-2 mt-0 text-sm font-medium text-gray-900 dark:text-stone-100">"Transliteration: "</p>
                        <div class="flex flex-1 p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-stone-100 dark:focus:ring-blue-500 dark:focus:border-blue-500" dir="rtl">{converted}</div>
                    </div>
                </div>
            </section>
        </div>
    }
}

#[server]
async fn get_project() -> Result<Project, ServerFnError> {
    use crate::db::pool;

    let pool = pool()?;

    let project: Project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE title = $1")
        .bind("aratype")
        .fetch_one(&pool)
        .await?;

    Ok(project)
}
