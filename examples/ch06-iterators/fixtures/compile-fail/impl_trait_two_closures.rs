fn make_adjuster(subtract: bool, base: i32) -> impl Fn(i32) -> i32 {
    if subtract {
        move |value| value - base
    } else {
        move |value| value + base
    }
}

fn main() {
    let adjust = make_adjuster(true, 10);
    println!("{}", adjust(30));
}
