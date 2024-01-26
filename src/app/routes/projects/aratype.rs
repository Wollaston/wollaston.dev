use aratype::buckwalter::convert_en_ar;
use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Aratype() -> impl IntoView {
    view! {
        <Title text="~/projects/aratype/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50 dark:bg-gray-900">
            <Converter/>
        </div>
    }
}

#[component]
pub fn Converter() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let converted = move || convert_en_ar(name());

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
            <div class="flex flex-col">
                <h1 class="m-2 font-medium text-stone-800 dark:text-stone-100">"A simple WASM app using Rust and Leptos that converts English letters to Arabic equivalents according to the Buckwalter transliteration table. Very much a work in progress. Note it currently does not work as well on mobile as it does on a computer."</h1>
                <a href="https://en.wikipedia.org/wiki/Buckwalter_transliteration" target="_blank" rel="noopener noreferrer" class="m-2 font-normal italic text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]">
                    "Buckwalter Transliteration Wiki"
                </a>
                <a href="https://github.com/Wollaston/aratype" target="_blank" rel="noopener noreferrer" class="inline-flex items-center text-blue-700 dark:text-stone-100 hover:underline">
                    <img src="/assets/github-mark.svg" alt="The GitHub Logo" class="block dark:hidden h-6 w-6 m-2"/>
                    <img src="/assets/github-mark-white.svg" alt="The GitHub Logo" class="hidden dark:block h-6 w-6 m-2"/>
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
