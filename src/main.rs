use std::borrow::Borrow;
use std::collections::HashMap;

use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use std::rc::Rc;

mod day1;
mod day2;
mod day3;
mod day4;
pub mod read_lines;

type Days = HashMap<&'static str, fn() -> String>;

fn main() {
    let mut days: Days = HashMap::new();

    days.insert("day1", day1::part2);
    days.insert("day2 - part 1", day2::part2);
    days.insert("day2 - part 2", day2::part2);
    days.insert("day3 - part 1", day3::part1);
    days.insert("day3 - part 2", day3::part2);
    days.insert("day4 - part 1", day4::part1);
    days.insert("day4 - part 2", day4::part2);

    let mut siv = cursive::default();

    siv.set_user_data(days.clone());

    siv.add_global_callback('q', |s| s.quit());

    let select = SelectView::<String>::new()
        //.on_submit(on_submit)
        .with_name("select")
        .fixed_size((10, 5));

    let buttons = LinearLayout::horizontal()
        .child(Button::new("Run", run))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(select)
                .child(DummyView)
                .child(buttons),
        )
        .title("Select day"),
    );

    for (key, _) in days.into_iter() {
        siv.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(key);
        });
    }

    siv.run();
}

fn run(s: &mut Cursive) {
    let day = s
        .find_name::<SelectView<String>>("select")
        .unwrap()
        .selection()
        .unwrap()
        .to_string();

    let func: &Days = s.user_data().unwrap();
    let func = func.get(day.as_str()).unwrap();

    let res = func();

    s.add_layer(
        Dialog::around(TextView::new(res))
            .title(day.to_string())
            .button("Close", |s| {
                s.pop_layer();
            }),
    );
}
