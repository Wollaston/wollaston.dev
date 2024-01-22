use leptos::*;
use leptos_meta::Title;

pub mod website;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Title text="~/blog/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
        <BlogSection />
        </div>
    }
}

#[component]
pub fn BlogSection() -> impl IntoView {
    let test_blog: Blog = Blog{
        title: String::from("Building wollaston.dev"),
        path: String::from("blog/website"),
        description: String::from("A short blog discussing my experience building the wollaston.dev website using Rust, Leptos, and Axum."),
    };

    view! {
        <div class="container my-8 mx-auto md:px-6">
            <section class="mb-4">
                <div class="grid grid-cols-1 lg:grid-cols-2">
                    <div class="mb-2 p-2 lg:col-span-1 md:px-3 lg:px-6">
                        <h1 class="display:block font-mono min-w-fit mb-4 text-3xl lg:text-4xl font-extrabold leading-none dark:text-white">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 after:border-r-8 after:bg-blue-700 after:animate-blink">"my_blog"</span></h1>
                        <BlogCard blog=test_blog />
                    </div>
                    <div class="mb-4 p-2 lg:col-span-1 drop-shadow-xl rounded-lg object-scale-down">
                        <img class="drop-shadow-xl rounded-lg h-max w-max" src="assets/blog_astro.png" alt="An Astronaut Writing a Blog in a Space Station."/>
                    </div>
                </div>
            </section>
        </div>
    }
}

struct Blog {
    title: String,
    path: String,
    description: String,
}

#[component]
fn BlogCard(blog: Blog) -> impl IntoView {
    view! {
        <div class="max-w p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
            <a href={blog.path}>
                <h5 class="mb-2 text-2xl font-semibold tracking-tight text-gray-900 hover:text-blue-700 dark:text-white">{blog.title}</h5>
            </a>
            <p class="mb-3 font-normal text-gray-500 dark:text-gray-400">{blog.description}</p>
        </div>
    }
}
