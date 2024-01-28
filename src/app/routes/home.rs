use leptos::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="~/home/wollaston.dev"/>
        <div class="overflow-hidden flex-1 w-full bg-stone-50 dark:bg-gray-900">
            <Hero />
            <FeatureList />
        </div>
    }
}

#[component]
pub fn Hero() -> impl IntoView {
    view! {
    <div>
        <div class="grid max-w-screen-xl px-4 py-8 pb-2 mx-auto lg:gap-8 xl:gap-0 lg:grid-cols-12">
            <div class="place-self-center lg:col-span-7">
                <h1 class="font-mono min-w-fit mb-4 text-2xl font-extrabold tracking-tight leading-none md:text-3xl xl:text-4xl dark:text-stone-100">"~$ "<span class="animate-typing inline-block overflow-hidden whitespace-nowrap align-middle font-mono after:border-r-blue-700 dark:after:border-r-[#fd8a04] after:border-r-8 after:bg-blue-700 dark:after:bg-[#fd8a04] after:animate-blink">"aim_for_the_stars "</span></h1>
                <p class="max-w-2xl mb-6 font-light text-gray-500 lg:mb-8 md:text-lg lg:text-xl dark:text-gray-400">"Looking to expand or improve your business and processes? I can help with projects of any size."</p>
                <div class="flex flex-row">
                    <div class="p-2">
                        <a href="/projects" class="inline-flex items-center justify-center mb-3 px-5 py-3 text-base font-medium text-center text-gray-900 border border-gray-300 rounded-lg bg-[#fee5b0] hover:text-[#fee5b0] hover:bg-blue-700 focus:ring-4 focus:ring-gray-100 dark:text-stone-100 dark:border-gray-700 dark:bg-indigo-900 dark:hover:bg-[#fd8a04] dark:focus:ring-gray-800">
                        "Projects"
                        </a>
                    </div>
                    <div class="p-2">
                        <a href="/contact" class="inline-flex items-center justify-center mb-3 px-5 py-3 text-base font-medium text-center text-gray-900 border border-gray-300 rounded-lg bg-[#fee5b0] hover:text-[#fee5b0] hover:bg-blue-700 focus:ring-4 focus:ring-gray-100 dark:text-stone-100 dark:border-gray-700 dark:bg-indigo-900 dark:hover:bg-[#fd8a04] dark:focus:ring-gray-800">
                        "Contact"
                        </a>
                    </div>
                </div>
            </div>
            <div class="h-full drop-shadow-xl rounded-lg lg:mt-0 lg:col-span-5 lg:flex">
                <img class="object-scale-down drop-shadow-xl rounded-lg w-full min-h-0" src="https://imagedelivery.net/-kEZoni8dAWk_nqST6IIYw/0d1f7ec7-3121-427f-45b2-e5cbf3045600/public" alt="An Astronaut Floating in Space"/>
            </div>
        </div>
    </div>
    }
}

#[component]
pub fn FeatureList() -> impl IntoView {
    view! {
        <section>
            <div class="py-2 px-4 mx-auto max-w-screen-xl sm:py-4 lg:px-6">
                <div class="max-w-screen-md mb-8 lg:mb-16">
                    <h2 class="mb-4 text-4xl tracking-tight font-extrabold text-gray-900 dark:text-stone-100">"What can I help with?"</h2>
                </div>
                <div class="space-y-8 md:grid md:grid-cols-2 lg:grid-cols-3 md:gap-12 md:space-y-0">
                    <div>
                        <div class="flex justify-center items-center mb-4 w-10 h-10 rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900">
                            <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
                            <path d="M11 25h80v50H10Zm0 0 20-15 20 15h40" fill="#fd8a04"/>
                            <path d="M40 30h30v40H40Z" fill="#fff"/>
                            <path d="M40 30h30v40H40Z" fill="none" stroke="#000" stroke-width="2"/>
                            <path d="M50 35h10m-10 5h10m-10 5h10" stroke="#000"/>
                            <path fill="#00f" d="M55 50h10v20H55z"/>
                            <path fill="red" d="M50 55h10v15H50z"/>
                            <path fill="green" d="M45 60h10v10H45z"/>
                            </svg>
                        </div>
                        <h3 class="mb-2 text-xl font-bold dark:text-stone-100">"Data & Document Management"</h3>
                        <p class="text-gray-500 dark:text-gray-400">"Organize and structure your existing documents and data, or get help migrating historical records to a new system."</p>
                    </div>
                    <div>
                        <div class="flex justify-center items-center mb-4 w-10 h-10 rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900">
                            <svg height="800" width="800" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" xml:space="preserve">
                                <path fill="#fd8a04" d="M445.66 49.341H340.206L294.008 3.143c-4.192-4.191-10.99-4.191-15.183 0l-49.341 49.342c-4.192 4.192-4.192 10.99 0 15.183l46.198 46.198v38.494h-80.516c-5.929 0-10.736 4.806-10.736 10.735v21.471h-42.942v-21.471c0-5.929-4.806-10.735-10.736-10.735H66.34c-5.929 0-10.735 4.806-10.735 10.735v64.413c0 5.929 4.806 10.735 10.735 10.735h20.755v93.798c0 5.929 4.806 10.736 10.735 10.736h95.437l-10.346 10.346c-4.192 4.192-4.193 10.99 0 15.182a10.7 10.7 0 0 0 7.591 3.144c2.747 0 5.495-1.049 7.591-3.144l15.061-15.061c3.994 8.83 10.521 16.859 19.311 23.54 11.932 9.068 26.987 14.783 43.205 16.539v32.792h-42.942c-5.929 0-10.735 4.806-10.735 10.736v64.413c0 5.929 4.806 10.735 10.735 10.735h107.355c5.929 0 10.735-4.806 10.735-10.735v-64.413c0-5.929-4.806-10.736-10.735-10.736h-42.942v-32.792c16.218-1.756 31.272-7.471 43.205-16.539 8.801-6.689 15.332-14.73 19.325-23.572l15.092 15.092c2.097 2.097 4.844 3.144 7.591 3.144s5.495-1.049 7.591-3.144c4.192-4.192 4.192-10.99 0-15.182l-10.346-10.346h66.05c5.929 0 10.736-4.806 10.736-10.736V60.077c.001-5.929-4.806-10.736-10.735-10.736zM252.257 60.078l34.16-34.16 34.16 34.16-34.16 34.16-34.16-34.16zm-46.356 113.754h161.032v42.942H205.901v-42.942zM77.075 216.774v-42.942h42.942v42.942H77.075zm136.174 94.148-15.143-15.143c-4.192-4.192-10.99-4.192-15.183 0-4.192 4.192-4.192 10.99 0 15.183l10.346 10.346h-84.702v-83.063h22.187c5.929 0 10.735-4.806 10.735-10.735v-21.471h42.942v21.471c0 5.929 4.806 10.735 10.735 10.735h80.516v32.792c-16.218 1.756-31.272 7.471-43.205 16.539-8.727 6.632-15.227 14.592-19.228 23.346zm116.11 136.665v42.942h-85.884v-42.942h85.884zm-42.942-75.148c-31.078 0-56.361-18.059-56.361-40.258s25.283-40.258 56.361-40.258c31.078 0 56.361 18.059 56.361 40.258s-25.282 40.258-56.361 40.258zm93.193-51.131 10.347-10.346c4.192-4.192 4.192-10.99 0-15.183-4.192-4.191-10.99-4.191-15.183 0l-15.175 15.175c-4-8.766-10.505-16.738-19.242-23.378-11.932-9.068-26.987-14.783-43.205-16.539v-32.792h80.516c5.929 0 10.735-4.806 10.735-10.735v-64.413c0-5.929-4.806-10.735-10.735-10.735h-80.516v-38.494l43.054-43.054h94.718v250.494H379.61z"/>
                            </svg>
                        </div>
                        <h3 class="mb-2 text-xl font-bold dark:text-stone-100">"Workflow"</h3>
                        <p class="text-gray-500 dark:text-gray-400">"Review your existing workflows to identify bottlenecks, and get help building processes to improve your day-to-day work."</p>
                    </div>
                    <div>
                        <div class="flex justify-center items-center mb-4 w-10 h-10 rounded-full bg-primary-100 lg:h-12 lg:w-12 dark:bg-primary-900">
                            <svg xmlns="http://www.w3.org/2000/svg" width="800" height="800" viewBox="0 0 419.931 419.931" xml:space="preserve">
                                <path fill="#fd8a04" d="M282.895 352.367a18.64 18.64 0 0 1-5.579-5.25 18.972 18.972 0 0 1-1.771-3.125H28.282V100.276h335.624v159.138c7.165.647 13.177 5.353 15.701 11.797a18.66 18.66 0 0 1 7.344-2.213 18.654 18.654 0 0 1 5.236.293V39.561c0-12.996-10.571-23.569-23.566-23.569H23.568C10.573 15.992 0 26.565 0 39.561v309.146c0 12.996 10.573 23.568 23.568 23.568h257.179a18.522 18.522 0 0 1-1.302-13.066 18.643 18.643 0 0 1 3.45-6.842zm55.13-296.798a8.703 8.703 0 0 1 8.702-8.703h8.702a8.702 8.702 0 0 1 8.702 8.703v9.863a8.702 8.702 0 0 1-8.702 8.702h-8.702a8.701 8.701 0 0 1-8.702-8.702v-9.863zm-40.465 0a8.703 8.703 0 0 1 8.702-8.703h8.703a8.702 8.702 0 0 1 8.702 8.703v9.863a8.702 8.702 0 0 1-8.702 8.702h-8.703a8.702 8.702 0 0 1-8.702-8.702v-9.863zm-40.466 0a8.703 8.703 0 0 1 8.702-8.703h8.702a8.702 8.702 0 0 1 8.703 8.703v9.863a8.702 8.702 0 0 1-8.703 8.702h-8.702a8.703 8.703 0 0 1-8.702-8.702v-9.863z"/>
                                <path fill="#fd8a04" d="m419.875 335.77-2.615-14.83a3.673 3.673 0 0 0-4.255-2.979l-13.188 2.324a49.468 49.468 0 0 0-6.005-10.38l8.614-10.268a3.672 3.672 0 0 0 .847-2.68 3.682 3.682 0 0 0-1.3-2.494l-11.534-9.68a3.678 3.678 0 0 0-5.176.453l-8.606 10.26a49.3 49.3 0 0 0-11.271-4.104V278a3.675 3.675 0 0 0-3.673-3.674h-15.06A3.675 3.675 0 0 0 342.98 278v13.392a49.253 49.253 0 0 0-11.271 4.104l-8.608-10.259a3.674 3.674 0 0 0-5.175-.453l-11.535 9.679a3.681 3.681 0 0 0-1.299 2.494c-.084.971.22 1.937.846 2.683l8.615 10.266a49.643 49.643 0 0 0-6.005 10.38l-13.188-2.325a3.677 3.677 0 0 0-4.255 2.979l-2.614 14.83a3.671 3.671 0 0 0 2.977 4.255l13.198 2.326a49.466 49.466 0 0 0 2.073 11.812l-11.6 6.695a3.67 3.67 0 0 0-1.345 5.016l7.529 13.041a3.665 3.665 0 0 0 3.18 1.836c.639 0 1.272-.166 1.836-.492l11.609-6.703a49.482 49.482 0 0 0 9.18 7.709l-4.584 12.593c-.332.916-.289 1.926.123 2.809s1.157 1.566 2.072 1.898l14.148 5.149a3.669 3.669 0 0 0 4.708-2.194l4.583-12.593a49.83 49.83 0 0 0 11.988 0l4.584 12.593a3.668 3.668 0 0 0 4.707 2.194l14.15-5.149a3.674 3.674 0 0 0 2.193-4.707l-4.584-12.591a49.756 49.756 0 0 0 9.18-7.709l11.609 6.703a3.683 3.683 0 0 0 2.787.367 3.672 3.672 0 0 0 2.229-1.711l7.529-13.043a3.677 3.677 0 0 0 .367-2.787 3.681 3.681 0 0 0-1.712-2.229l-11.598-6.693a49.482 49.482 0 0 0 2.071-11.812l13.198-2.327a3.664 3.664 0 0 0 2.37-1.511 3.6 3.6 0 0 0 .629-2.745zm-65.691 23.566c-11.155 0-20.2-9.045-20.2-20.201s9.046-20.2 20.2-20.2c11.156 0 20.201 9.044 20.201 20.2s-9.045 20.201-20.201 20.201zM164.695 235.373a12.3 12.3 0 0 0-7.096-11.119l-39.455-18.332 39.456-18.334a12.304 12.304 0 0 0 7.095-11.118v-.319c0-4.21-2.119-8.075-5.665-10.334a12.248 12.248 0 0 0-6.606-1.916c-1.778 0-3.563.391-5.16 1.133l-63.078 29.333a12.303 12.303 0 0 0-7.092 11.117v.877c0 4.743 2.782 9.104 7.093 11.118l63.084 29.336a12.265 12.265 0 0 0 11.759-.786 12.213 12.213 0 0 0 5.666-10.335l-.001-.321zm62.237-101.361a12.301 12.301 0 0 0-9.901-5.03h-.314a12.224 12.224 0 0 0-11.679 8.516l-41.56 128.772a12.317 12.317 0 0 0 1.781 10.962 12.306 12.306 0 0 0 9.901 5.029h.315a12.217 12.217 0 0 0 11.672-8.516l41.555-128.762a12.29 12.29 0 0 0-1.77-10.971zm81.069 60.354-63.079-29.333a12.303 12.303 0 0 0-5.152-1.131c-2.358 0-4.644.661-6.605 1.912a12.2 12.2 0 0 0-5.671 10.337v.319c0 4.746 2.783 9.111 7.097 11.123l39.454 18.33-39.455 18.331a12.3 12.3 0 0 0-7.096 11.119v.321c0 4.205 2.119 8.066 5.669 10.336a12.25 12.25 0 0 0 6.595 1.923c1.792 0 3.527-.383 5.169-1.141l63.082-29.336c4.307-2.009 7.088-6.371 7.088-11.114v-.877a12.313 12.313 0 0 0-7.096-11.119z"/>
                            </svg>
                        </div>
                        <h3 class="mb-2 text-xl font-bold dark:text-stone-100">"Web Development"</h3>
                        <p class="text-gray-500 dark:text-gray-400">"Need a simple website? See how modern solutions based on web fundamentals can help you simplify your website management and deployment."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
