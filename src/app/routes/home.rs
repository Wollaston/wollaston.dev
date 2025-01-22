use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-1 justify-center items-center flex-col bg-stone-50 dark:bg-gray-900">
            <h1 class="text-4xl font-semibold text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">Wollaston</h1>
            <h2 class="text-3xl font-semibold text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">Matthew F</h2>
            <h3 class="text-lg font-semibold text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">Computational Linguistics and Web Development</h3>
            <h3 class="text-lg font-semibold text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href="https://www.github.com/wollaston" target="_blank">GitHub.com/Wollaston</a></h3>
            <h3 class="text-lg font-semibold text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href="mailto:matt@wollaston.dev">matt@wollaston.dev</a></h3>
        </div>
    }
}
