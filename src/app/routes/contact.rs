use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="~/contact/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
            <p>"Contact"</p>
        </div>
    }
}
