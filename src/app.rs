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
                <Routes>
                    <Route path="/" view=components::Layout>
                        <Route path="" view=routes::home::HomePage/>
                        <Route path="/about" view=routes::about::About/>
                        <ProjectsRoutes />
                        <BlogRoutes />
                        <Route path="/contact" view=routes::contact::Contact/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component(transparent)]
fn BlogRoutes() -> impl IntoView {
    view! {
        <Route path="/blog"  view=routes::blog::Blog>
            <Route path="" view=routes::blog::blog_base::BlogSection/>
            <Route path=":slug" view=routes::blog::slug::Slug/>
       </Route>
    }
}

#[component(transparent)]
fn ProjectsRoutes() -> impl IntoView {
    view! {
        <Route path="/projects" view=routes::projects::Projects>
            <Route path="" view=routes::projects::projects_base::ProjectsSection/>
            <Route path="/aratype" view=routes::projects::aratype::Aratype/>
        </Route>
    }
}
