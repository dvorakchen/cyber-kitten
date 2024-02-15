use super::build_cat_view;
use gloo::timers::callback::Interval;
use leptos::*;

pub const CAT_SLEEP: [&'static str; 4] = [
    r"
   
    
  |\   _,,,---,,_
 / ,`.-'`'    -.  ;-;;,_
 |,4-  ) )-,_..;\ (   `'-'
'---' ' (_/--'  `-'\_)
",
    r"
    
   z
  |\   _,,,---,,_
 / ,`.-'`'    -.  ;-;;,_
 |,4-  ) )-,_..;\ (   `'-'
'---' ' (_/--'  `-'\_)
",
    r"
  Z 
   z
  |\   _,,,---,,_
 / ,`.-'`'    -.  ;-;;,_
 |,4-  ) )-,_..;\ (   `'-'
'---' ' (_/--'  `-'\_)
",
    r"
  Z 
   z
  |\   _,,,---,,_
 / ,`.-'`'    -.  ;-;;,_
 |,4-  ) )-,_..;\ (   `'-'
'---' ' (_/--'  `-'\_)
",
];

#[component]
pub fn CatSleep() -> impl IntoView {
    let (index, set_index) = create_signal(0usize);

    Interval::new(500, move || {
        set_index.try_update(|v| {
            if *v == CAT_SLEEP.len() - 1 {
                *v = 0;
            } else {
                *v += 1;
            }
        });
    })
    .forget();

    let place_cat = move || build_cat_view(CAT_SLEEP[index.get()]);

    view! {
        <div class="cursor-grab select-none scale-100 sm:scale-150 lg:scale-[200%] relative">
            <div class="p-2">{place_cat}</div>
            <div class="absolute top-4 left-20 w-max text-[0.5rem]
            transition-all translate-y-0 opacity-0
            animate-[cat-say_4s_4s_ease-in-out]">"Touch me to awake me~"</div>
        </div>
    }
}
