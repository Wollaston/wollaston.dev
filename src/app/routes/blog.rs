use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Title text="~/blog/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
            <p>"Blog"</p>
        </div>
    }
}
