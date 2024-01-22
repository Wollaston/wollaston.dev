use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            <NavBar />
            <Outlet />
            <Footer />
        </div>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let (hamburger, set_hamburger) = create_signal(false);

    view! {
    <nav class="bg-[#fee5b0] border-stone-500 drop-shadow-xl dark:bg-gray-900">
      <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
        <a href="/" class="flex items-center space-x-3 rtl:space-x-reverse">
            <img src="/assets/logo.png" class="h-12 rounded-lg" alt="Wollaston Logo" />
            <span class="self-center text-2xl hover:text-blue-700 font-semibold whitespace-nowrap dark:text-white">"Wollaston"</span>
        </a>
        <button class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-stone-800 rounded-lg md:hidden hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false"
            on:click=move |_| {set_hamburger.update(|h| *h = !*h)}
        >
            <span class="sr-only">"Open main menu"</span>
            <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
            </svg>
        </button>
        <NavbarDefault />
      </div>
        <Show
        when=hamburger
        >
            <NavbarHamburger />
        </Show>
    </nav>
            }
}

#[component]
fn NavbarDefault() -> impl IntoView {
    view! {
        <div class="hidden w-full md:block md:w-auto">
          <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border rounded-lg md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
            <li>
               <a href="/" class="block py-2 px-3 text-stone-800 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Home</a>
            </li>
            <li>
              <a href="/about" class="block py-2 px-3 text-stone-800 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">About</a>
            </li>
            <li>
              <a href="/projects" class="block py-2 px-3 text-stone-800 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Projects</a>
            </li>
             <li>
              <a href="/blog" class="block py-2 px-3 text-stone-800 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Blog</a>
            </li>
            <li>
              <a href="/contact" class="block py-2 px-3 text-stone-800 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Contact</a>
            </li>
          </ul>
        </div>
    }
}

#[component]
fn NavbarHamburger() -> impl IntoView {
    view! {
        <div class="flex flex-col align-middle items-center w-full justify-center text-sm text-stone-800 rounded-lg md:hidden dark:text-gray-400">
            <ul class="flex flex-col align-middle items-center w-full justify-center text-sm text-stone-800  font-medium dark:bg-gray-800 dark:border-gray-700">
                <li class="w-full">
                    <a href="/" class="block border-2 border-stone-50 py-2 px-3 text-stone-800 rounded hover:text-[#fee5b0] hover:bg-blue-700  dark:text-white dark:hover:bg-gray-700 dark:hover:text-white">"Home"</a>
                </li>
                <li class="w-full">
                    <a href="/about" class="block border-2 border-stone-50 py-2 px-3 text-stone-800 rounded hover:text-[#fee5b0] hover:bg-blue-700  dark:text-white dark:hover:bg-gray-700 dark:hover:text-white">"About"</a>
                </li>
                <li class="w-full">
                    <a href="/projects" class="block border-2 border-stone-50 py-2 px-3 text-stone-800 rounded hover:text-[#fee5b0] hover:bg-blue-700  dark:text-white dark:hover:bg-gray-700 dark:hover:text-white">"Projects"</a>
                </li>
                <li class="w-full">
                    <a href="/blog" class="block border-2 border-stone-50 py-2 px-3 text-stone-800 rounded hover:text-[#fee5b0] hover:bg-blue-700  dark:text-white dark:hover:bg-gray-700 dark:hover:text-white">"Blog"</a>
                </li>
                <li class="w-full">
                    <a href="/contact" class="block border-2 border-stone-50 py-2 px-3 text-stone-800 rounded hover:text-[#fee5b0] hover:bg-blue-700  dark:text-white dark:hover:bg-gray-700 dark:hover:text-white">"Contact"</a>
                </li>
            </ul>
        </div>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="p-1 bg-[#fee5b0] md:p-2 lg:p-4 dark:bg-gray-800">
            <div class="flex flex-col justify-center items-center mx-auto max-w-screen-xl text-center"> <img src="/assets/logo.png" class="m-4 h-12 rounded-lg" alt="Wollaston Logo" /> <ul class="flex flex-wrap justify-center items-center mb-6 text-gray-900 dark:text-white">
                    <li>
                        <a href="/about" class="mr-4 hover:text-blue-700 md:mr-6 ">"About"</a>
                    </li>
                    <li>
                        <a href="/blog" class="mr-4 hover:text-blue-700 md:mr-6">"Blog"</a>
                    </li>
                    <li>
                        <a href="https://github.com/Wollaston" target="_blank" rel="noopener noreferrer" class="mr-4 hover:text-blue-700 md:mr-6">"Github"</a>
                    </li>
                    <li>
                        <a href="/contact" class="mr-4 hover:text-blue-700 md:mr-6">"Contact"</a>
                    </li>
                </ul>
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">"© 2024 All Rights Reserved"</span>
            </div>
        </footer>
    }
}
