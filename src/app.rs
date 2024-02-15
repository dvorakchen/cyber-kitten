use crate::cats::{CatSit, CatSleep};
use gloo::timers::callback::Timeout;
use js_sys::Date;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex place-items-center place-content-center
        h-screen">
            <main class="whitespace-pre-wrap -translate-y-4">
                <Cat/>
            </main>
        </div>
    }
}

#[component]
fn Cat() -> impl IntoView {
    use crate::cats::CatAction::*;

    let now = Date::new_0();
    let hours: u32 = now.get_hours();
    let is_day = match hours {
        6..=17 => true,
        _ => false,
    };

    let (cat_action, set_cat_action) = create_signal(if is_day { Sleep } else { Sit });

    let show_cat = move || match cat_action.get() {
        Sit => view! { <CatSit/> },
        Sleep => view! { <CatSleep/> },
    };

    let cat_box = create_node_ref();

    let handle_click_cat = move |_| match cat_action.get() {
        Sleep => {
            let cat_box: HtmlElement<html::Div> = cat_box.get().unwrap();
            let classes = cat_box.class_list();
            classes.replace("opacity-100", "opacity-0").unwrap();
            Timeout::new(1_000, move || {
                set_cat_action.set(Sit);
                classes.replace("opacity-0", "opacity-100").unwrap();
            })
            .forget();
        }
        _ => {}
    };

    view! {
        <div on:click=handle_click_cat>
            <div class="opacity-100 transition-all duration-1000" node_ref=cat_box>
                {show_cat}
            </div>
        </div>
    }
}
