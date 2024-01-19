use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full">
            <div class="flex flex-col justify-center text-center w-full">
                <Hero />
            </div>
        </div>
    }
}

#[component]
pub fn Hero() -> impl IntoView {
    view! {
    <div class="bg-stone-50 h-screen dark:bg-gray-900">
        <div class="grid max-w-screen-xl px-4 py-8 mx-auto lg:gap-8 xl:gap-0 lg:py-16 lg:grid-cols-12">
            <div class="mr-auto place-self-center lg:col-span-7">
                <h1 class="max-w-2xl mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl dark:text-white">Aiming for the stars?<br/>Get the help you need</h1>
                <p class="max-w-2xl mb-6 font-light text-gray-500 lg:mb-8 md:text-lg lg:text-xl dark:text-gray-400">From document management and organization, to web design and scripting - get the support you need today</p>
                <a href="/projects" class="inline-flex items-center justify-center px-5 py-3 text-base font-medium text-center text-gray-900 border border-gray-300 rounded-lg hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 dark:text-white dark:border-gray-700 dark:hover:bg-gray-700 dark:focus:ring-gray-800">
                   See Past Projects
                </a>
                <a href="/contact" class="inline-flex items-center justify-center px-5 py-3 text-base font-medium text-center text-gray-900 border border-gray-300 rounded-lg hover:bg-gray-100 focus:ring-4 focus:ring-gray-100 dark:text-white dark:border-gray-700 dark:hover:bg-gray-700 dark:focus:ring-gray-800">
                   Contact Me
                </a>
            </div>
            <div class="hidden drop-shadow-xl rounded-lg lg:mt-0 lg:col-span-5 lg:flex">
                <img class="drop-shadow-xl rounded-lg h-max w-max" src="assets/FloatingAstronaut.png" alt="An Astronaut Floating in Space"/>
            </div>
        </div>
    </div>
    }
}
