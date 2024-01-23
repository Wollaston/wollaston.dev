use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="~/contact/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
            <ContactSection />
        </div>
    }
}

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-white">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 after:border-r-8 after:bg-blue-700 after:animate-blink">"contact_me"</span></h1>
                        <p class="mb-2 lg:text-2xl">
                            <a class="text-stone-800 hover:text-blue-700" href="mailto:info@wollaston.dev">"info@wollaston.dev"</a>
                        </p>
                        <p class="mb-2 lg:text-2xl">
                            <a class="text-stone-800 hover:text-blue-700" href="https://github.com/Wollaston" target="_blank" rel="noopener noreferrer" class="mr-4 hover:text-blue-700 md:mr-6">"GitHub"</a>
                        </p>
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg object-scale-down">
                        <img class="drop-shadow-xl rounded-lg h-max w-max" src="/assets/contact_me_astro.png" alt="An Astronaut Floating in Space"/>
                    </div>
                </div>
            </section>
        </div>
    }
}
