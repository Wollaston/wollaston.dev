use leptos::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="~/about/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
        <AboutSection />
        </div>
    }
}

#[component]
pub fn AboutSection() -> impl IntoView {
    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-white">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 after:border-r-8 after:bg-blue-700 after:animate-blink">"about_me"</span></h1>
                        <p class="mb-2 lg:text-xl text-neutral-500 dark:text-neutral-300">
"Hi, I'm Matt. I have a background in legal and translation which has led me to
a deep interest in Natural Language Processing (NLP) and related technologies. I 
plan to study Computational Linguistics at the graduate level."
                        </p>
                        <p class="mb-2 lg:text-xl text-neutral-500 dark:text-neutral-300">
"My development ideals borrow from the Unix philosophy and the open-source-software movement.
I believe businesses of all sizes can improve their processes by using existing tools and/or 
incorporating proven open-source software to alleviate and even solve common issues. I believe this 
is particularly important for small businesses who do not have leverage when negotiating contracts. 
This philosophy seeks to avoid vendor lock-in and long-term contracts wherever possible, and to match 
simple problems with simple solutions."
                        </p>
                         <p class="mb-2 lg:text-xl text-neutral-500 dark:text-neutral-300">
"I love problem solving and improving the quality of life for businesses of all sizes. I also
love learning about and using different technologies. Reach out if I can be helpful with projects
of any size."
                        </p>
                        <a href="/contact" class="inline-flex items-center bg-[#fee5b0] hover:text-[#fee5b0] hover:bg-blue-700 justify-center px-5 py-3 text-base font-medium text-center text-gray-900 border border-gray-300 rounded-lg focus:ring-4 focus:ring-gray-100 dark:text-white dark:border-gray-700 dark:hover:bg-gray-700 dark:focus:ring-gray-800">
                   "Contact"
                        </a>
                   </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg object-scale-down">
                        <img class="drop-shadow-xl rounded-lg h-max w-max" src="/assets/about_astro.png" alt="An Astronaut Introducing Himself"/>
                    </div>
                </div>
            </section>
        </div>
    }
}
