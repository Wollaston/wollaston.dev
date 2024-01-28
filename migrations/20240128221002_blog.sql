-- Add migration script here
CREATE TABLE IF NOT EXISTS blog (
    id          INTEGER PRIMARY KEY NOT NULL,
    title       VARCHAR(250)        NOT NULL,
    slug        VARCHAR(250)        NOT NULL,
    description VARCHAR(250)        NOT NULL,
    created_date TEXT               NOT NULL,
    last_modified_date TEXT,
    content     VARCHAR(250)        NOT NULL
);

INSERT INTO blog (title, slug, description, created_date, last_modified_date, content)
VALUES
  ("Me and My Website", 
    "me-and-my-website", 
    "A short blog discussing my experience with Rust and building wollaston.dev using Leptos and axum.",
    "2024-01-23 18:00:00.000",
    "2024-01-23 18:00:00.000",
    "# *Preface* { .md-h1 }

*A short blog discussing my experience with Rust and building wollaston.dev using [Leptos](https://leptos.dev/) and [axum](https://docs.rs/axum/latest/axum/).*

*[GitHub repo](https://github.com/Wollaston/wollaston.dev)*

# Why Rust? { .md-h1 }

There is a lot that can be said about building a personal website with Rust and its various crates. You could simply call it 'overkill' or be more poetic and call it 'using a sledgehammer to crack a nut' or 'killing a fly with a shotgun.' But in any case, I just thought it would be a fun way to apply some of the skills I had learned dabbling with Rust over the preceding year and a half. And so by way of some background - 

I first found Rust by accident, finding myself with a short break from my regular job (unrelated to programming, but it did include a lot of data management and wrangling) where the only programming-like tools available were VBA in the Office suite of products and PowerShell in local environments. I used these tools wherever applicable, but they were naturally more limited. I, like many, had dabbled with Python in the past on personal projects, but I never really stuck with it. And there was no way I would be able to get Python on a work machine, and for good reason.  

While it had been fun to write short Python scripts to scrape a website, or complete various tutorials (I particularly remember dabbling with [spaCy's](https://spacy.io/) tutorials), I always quickly grew tired of it and moved on.  

Besides tutorial hell, I think I had two main issues at this time. The first was that it was hard to see exactly what I was building - most of these little toy projects would display something to a terminal output, which did get boring. And I do not remember why, but I could never get PyQt to work properly (and was essentially ignorant of web development). And the second was that I did not have a solid background in computer science and programming fundamentals, and Python obscures a lot of that so that you can still be productive without needing to know the details. But I like knowing the details.

Doing some Googling, I was led to the [The Rust Programming Language](https://doc.rust-lang.org/book/), better known as 'The Book.' It had a lot of what I was looking for - it had structure, examples, and comprehensive explanations. And Rust had this mysterious 'Macro' feature that was totally unlike an Excel Macro, and it just sounded cool. And being inexperienced, the compiler held me responsible for my programming mistakes that I did not even know I was making. But that, and the same terminal output issue, did grow tiring after a while. My next search led me to web development via [The Odin Project](https://www.theodinproject.com/), which I finally did stick with and learned a lot from. 

But the JavaScript world did get tiring quickly, especially as someone who treated programming and development more as a hobby at the time. It was almost like a full-time job keeping up with JavaScript tooling and frameworks. And while React was cool at first, I did find it a little annoying and had a hard time wrapping my head around its quirks. I did then find Svelte, and tried out SvelteKit when it hit 1.0, and things started making a lot more sense to me. 

Something was still missing though. Maybe it was working in some environments that were too opinionated, or being stuck with the web. But I got busy with work and real life and no longer had time for programming. But as soon as I did have time, I quickly found my way back to programming and learning about [Web Assembly](https://webassembly.org/) and web development using Rust. And being dissatisfied with my current career path, I decided I would give it more effort this time and try to pivot into the tech world. 

And going back to Rust after some more in-depth and practical JavaScript and TypeScript experience, things did start making more sense. And learning that I could actually build a whole tech stack in Rust (you can in JavaScript too I suppose, but it's not quite the same thing), while taking advantage of much of the crate ecosystem, made a lot of sense to me. So I set out to learn more about Rust and its ecosystem so that it could be my Swiss army knife. Later learning about Rust's robustness in handling text data (as my background, experience, and interests had a lot of translation and foreign language study) essentially settled the decision for me, as someday I could even develop a tool for those spaces. 

# Why Leptos? { .md-h1 }

Since keeping up with the JavaScript world was almost a job in and of itself, I turned to content creators to help keep me updated and educated, as much as possible. It was easy to put on a video in the background than trying everything myself. My YouTube algorithm agreed, and I was quickly recommended [ThePrimeagen](https://www.youtube.com/c/theprimeagen) and started learning from his content. By coincidence, Prime interviewed Greg, the creator of Leptos, on his channel, and they had a really great [conversation](https://www.youtube.com/watch?v=UrMHPrumJEs) on Leptos and Greg's design philosophy. And while in no way am I qualified to rank frameworks and their relative benefits, Greg's opinions just made sense to me. And I really liked Greg too. And when I tried Leptos, I liked it too. And it was Rust. It made the decision really easy for me.

# Why Axum? { .md-h1 }

I wish I had a better story for axum, but it frankly just came highly recommended. I liked building with it too, as the builder syntax it uses made a lot of sense to me. And it already integrated with Leptos, which made my life a lot easier. So I went for it. 

# Putting It All Together { .md-h1 }

And so, I found myself with some extra time. I had always wanted to develop my own personal website and develop a small brand. With these experiences behind me, it only made sense to put it all together and build this website using Rust, Leptos, and axum.

There is still so much more that can be done, but I like how far it has come. I even have a space where I can show examples of Rust projects I work on, and I can blog comfortably using Markdown like so many other content-focused sites already do.

I plan to follow this blog with another focusing on some of the technical details and lessons learned during this project."
  );

