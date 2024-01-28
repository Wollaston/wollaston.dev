use leptos::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="~/about/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50 dark:bg-gray-900">
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
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-stone-100">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 dark:after:border-r-[#fd8a04] after:border-r-8 after:bg-blue-700 dark:after:bg-[#fd8a04] after:animate-blink">"about_me"</span></h1>
                        <p class="mb-2 lg:text-xl text-neutral-500 dark:text-neutral-300">
"Hi, I'm Matt. I have a background in legal and translation which has led me to
a deep interest in Natural Language Processing (NLP) and related technologies. I 
plan to study Computational Linguistics at the graduate level."
                        </p>
                        <p class="mb-2 lg:text-xl text-neutral-500 dark:text-neutral-300">
"I believe businesses of all sizes can improve their processes by using existing tools and/or 
incorporating proven open-source software to alleviate and even solve common issues. This 
is particularly important for small businesses who do not have leverage when negotiating contracts. 
My development ideals seek to avoid vendor lock-in and long-term contracts wherever possible, and to match 
simple problems with simple solutions."
                        </p>
                         <p class="mb-2 lg:text-xl text-neutral-500 dark:text-neutral-300">
"I love problem solving and improving the quality of life for businesses of all sizes. I also
love learning about and using different technologies. Reach out if I can be helpful or to learn more."
                        </p>
                        <a href="/contact" class="inline-flex items-center bg-[#fee5b0] hover:text-[#fee5b0] hover:bg-blue-700 justify-center px-5 py-3 text-base font-medium text-center text-gray-900 border border-gray-300 rounded-lg focus:ring-4 focus:ring-gray-100 dark:text-stone-100 dark:border-gray-700 dark:bg-indigo-900 dark:hover:bg-[#fd8a04] dark:focus:ring-gray-800">
                   "Contact"
                        </a>
                   </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg h-full">
                        <img class="object-scale-down drop-shadow-xl rounded-lg w-full min-h-0" src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/ee0b1d17-3bec-4b01-2c74-467f3b36a600/public" alt="An Astronaut Introducing Himself"/>
                    </div>
                </div>
            </section>
        </div>
    }
}
