use std::rc::Rc;

use super::build_cat_view;
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

#[component]
pub fn CatSit() -> impl IntoView {
    let meow = create_node_ref();
    let purr = create_node_ref();
    let for_purr_start = Rc::new(purr);
    let for_purr_end = Rc::clone(&for_purr_start);

    let (index, set_index) = create_signal(0isize);
    let mut flag: isize = -1;

    let shake_tail_handler = Interval::new(500, move || {
        match index.get_untracked() {
            v if v == 0 || v == (CAT_SIT.len() - 1) as isize => flag = -flag,
            _ => {}
        };
        set_index.update(|v| *v += flag);
    });
    on_cleanup(|| {
        drop(shake_tail_handler);
    });

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

    let place_cat = move || build_cat_view(CAT_SIT[index.get() as usize]);

    view! {
        <div
            class="cursor-grab select-none scale-100 sm:scale-150 lg:scale-[200%]"
            on:click=handle_click_cat
            on:pointerdown=handle_touch_cat
            on:pointerup=handle_untouch_cat
        >
            <div class="p-2">{place_cat}</div>
            <div class="absolute top-4 left-20 w-max text-[0.5rem]
            transition-all translate-y-0 opacity-0
            animate-[cat-say_4s_4s_ease-in-out]">"Touch me~"</div>
            <audio src="/assets/cat_meow.ogg" hidden node_ref=meow></audio>
            <audio src="/assets/cat_purr.ogg" hidden node_ref=purr></audio>
        </div>
    }
}
