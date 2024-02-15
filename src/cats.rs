mod cat_sit;
mod cat_sleep;

pub use cat_sit::CatSit;
pub use cat_sleep::CatSleep;

use leptos::*;

#[derive(Clone, Copy)]
pub enum CatAction {
    Sleep,
    Sit,
}

fn build_cat_view(cat: &str) -> impl IntoView {
    cat.lines()
        .into_iter()
        .map(|line| {
            view! { <p>{line.replace(' ', "  ")}</p> }
        })
        .collect_view()
}
