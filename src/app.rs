use crate::cats::CatSit;
use leptos::*;

#[component]
pub fn App() -> impl IntoView {

    view! {
        <div class="flex place-items-center place-content-center
        h-screen">
            <main class="whitespace-pre-wrap -translate-y-4">
                <CatSit/>
            </main>
        </div>
    }
}
