use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod routes;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wollaston-dev.css"/>

        // sets the document title
        <Title text="wollaston.dev"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <components::Layout/>
                <Routes>
                    <Route path="" view=routes::home::HomePage/>
                    <Route path="/about" view=||view! {"About"}/>
                    <Route path="/projects" view=||view! {"Projects"}/>
                    <Route path="/blog"  view=||view! {"Blog"}/>
                    <Route path="/contact" view=||view! {"Contact"}/>
                </Routes>
            </main>
        </Router>
    }
}
