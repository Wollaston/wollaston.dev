use aratype::buckwalter::convert_en_ar;
use leptos::*;
use leptos_meta::Title;

#[component]
pub fn Aratype() -> impl IntoView {
    view! {
        <Title text="~/projects/aratype/wollaston.dev"/>
        <div class="flex-1 w-full bg-stone-50">
            <Converter/>
        </div>
    }
}

#[component]
pub fn Converter() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let converted = move || convert_en_ar(name());

    view! {
        <div>
            <input type="text"
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }
                prop:value=name
            />
            <p>"Buckwalter Conversion: " {converted}</p>
        </div>
    }
}
