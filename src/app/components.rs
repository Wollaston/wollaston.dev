use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="flex flex-col lg:flex-row min-h-screen overflow-hidden">
            <Sidebar />
            <Outlet />
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <div class="flex flex-col h-min lg:h-screen overflow-hidden justify-between lg:w-32 bg-[#fee5b0] drop-shadow-xl dark:bg-indigo-900">
            <ul class="m-2 p-2">
                <li class="font-semibold text-lg text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href="/">HOME</a></li>
                <li class="font-semibold text-lg text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href="/blog">BLOG</a></li>
                <li class="font-semibold text-lg text-stone-800 hover:text-blue-700 dark:text-stone-100 dark:hover:text-[#fd8a04]"><a href="/projects">PROJECTS</a></li>
            </ul>
        </div>
    }
}
