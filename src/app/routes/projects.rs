use leptos::*;
use leptos_meta::Title;
use leptos_router::Outlet;

pub mod aratype;
pub mod projects_base;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="~/projects/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50 dark:bg-gray-900">
        <Outlet />
        </div>
    }
}
