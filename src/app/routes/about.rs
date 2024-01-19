use leptos::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="~/about/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
            <p>"About"</p>
        </div>
    }
}
