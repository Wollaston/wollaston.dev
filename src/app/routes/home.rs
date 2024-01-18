use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col w-full">
            <div class="flex flex-col justify-center text-center w-full">
                <h1 class="text-blue-700 justify-center text-lg text-center font-bold">"Welcome to my Personal Website!"</h1>
            </div>
        </div>
    }
}
