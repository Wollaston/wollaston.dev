use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="~/projects/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
        <ProjectsSection />
        </div>
    }
}

#[component]
pub fn ProjectsSection() -> impl IntoView {
    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-white">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-black after:border-r-8 after:bg-black after:animate-blink">"my_projects"</span></h1>
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg object-scale-down">
                        <img class="drop-shadow-xl rounded-lg h-max w-max" src="assets/projects_astro.png" alt="An Astronaut Organizing his Projects in a Space Station."/>
                    </div>
                </div>
            </section>
        </div>
    }
}
