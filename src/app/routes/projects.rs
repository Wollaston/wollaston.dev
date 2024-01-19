use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="~/projects/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
            <p>"Projects"</p>
        </div>
    }
}
