use std::rc::Rc;

use gloo::timers::callback::Interval;
use leptos::*;

pub const CAT_SIT: [&'static str; 5] = [
    r"
 /\____/\
 )     (
=\    /=
  )   (
 /     \
 )     (
/       \
\       /
 \__  __/
    ))
   //
  //
☾_/
",
    r"
 /\____/\
 )     (
=\    /=
  )   (
 /     \
 )     (
/       \
\       /
 \__  __/
    ))
   //
  ((
   \)
",
    r"
 /\____/\
 )     (
=\    /=
  )   (
 /     \
 )     (
/       \
\       /
 \__  __/
    ))
    ((
    ))
    (/
",
    r"
 /\____/\
 )     (
=\    /=
  )   (
 /     \
 )     (
/       \
\       /
 \__  __/
    ((
     \\
      ))
      (/
",
    r"
 /\____/\
 )     (
=\    /=
  )   (
 /     \
 )     (
/       \
\       /
 \__  __/
    \\
     \\/)
      ˉ
       ~
",
];

pub fn get_cat_sit(cat: &str) -> impl IntoView {
    cat.lines()
        .into_iter()
        .map(|line| {
            view! { <p>{line.replace(' ', "  ")}</p> }
        })
        .collect_view()
}

#[component]
pub fn CatSit() -> impl IntoView {
    let meow = create_node_ref();
    let purr = create_node_ref();
    let for_purr_start = Rc::new(purr);
    let for_purr_end = Rc::clone(&for_purr_start);

    let (index, set_index) = create_signal(0isize);
    let mut flag: isize = -1;

    Interval::new(500, move || {
        match index.get_untracked() {
            v if v == 0 || v == (CAT_SIT.len() - 1) as isize => flag = -flag,
            _ => {}
        };
        set_index.update(|v| *v += flag);
    })
    .forget();

    let handle_click_cat = move |_| {
        let meow: HtmlElement<html::Audio> = meow.get().unwrap();
        _ = meow.play().unwrap();
    };

    let handle_touch_cat = move |_| {
        let purr: HtmlElement<html::Audio> = for_purr_start.get().unwrap();
        _ = purr.play().unwrap();
        if !purr.loop_() {
            purr.set_loop(true);
        }
    };

    let handle_untouch_cat = move |_| {
        let purr: HtmlElement<html::Audio> = for_purr_end.get().unwrap();
        _ = purr.pause().unwrap();
    };

    view! {
        <div
            class="cursor-grab select-none scale-100 sm:scale-150 lg:scale-[200%]"
            on:click=handle_click_cat
            on:pointerdown=handle_touch_cat
            on:pointerup=handle_untouch_cat
        >
            <div class="p-2">
                {move || { get_cat_sit(CAT_SIT[index.get() as usize]) }}
            </div>
            <audio src="/assets/cat_meow.ogg" hidden node_ref=meow></audio>
            <audio src="/assets/cat_purr.ogg" hidden node_ref=purr></audio>
        </div>
    }
}
