use leptos::*;
use leptos_meta::Title;
use leptos_router::Outlet;

pub mod blog_base;
pub mod slug;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Title text="~/blog/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
        <Outlet />
        </div>
    }
}
