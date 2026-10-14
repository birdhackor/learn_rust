// --8<-- [start:parameter]
pub fn evaluate(task: impl Fn(i32) -> i32, value: i32) -> i32 {
    task(value)
}
// --8<-- [end:parameter]

// --8<-- [start:owned]
pub fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    move |value| value + base
}
// --8<-- [end:owned]

// --8<-- [start:branches]
pub fn make_adjuster(subtract: bool, base: i32) -> impl Fn(i32) -> i32 {
    move |value| {
        if subtract { value - base } else { value + base }
    }
}
// --8<-- [end:branches]

// --8<-- [start:borrowed]
pub fn byte_reader(text: &str) -> impl Fn() -> usize {
    move || text.len()
}
// --8<-- [end:borrowed]

// --8<-- [start:precise]
pub fn precise_reader<'a>(text: &'a str) -> impl Fn() -> usize + use<'a> {
    move || text.len()
}

pub fn fixed_reader(_: &str) -> impl Fn() -> usize + use<> {
    || 4
}
// --8<-- [end:precise]

#[cfg(test)]
mod tests {
    use crate::opaque::byte_reader;
    use crate::opaque::evaluate;
    use crate::opaque::fixed_reader;
    use crate::opaque::make_adder;
    use crate::opaque::make_adjuster;
    use crate::opaque::precise_reader;

    #[test]
    fn owned_configuration_and_both_branches_remain_callable() {
        let add = make_adder(10);
        assert_eq!(add(3), 13);
        assert_eq!(add(4), 14);
        assert_eq!(evaluate(make_adder(20), 3), 23);
        let subtract = make_adjuster(true, 10);
        let increase = make_adjuster(false, 10);
        assert_eq!(subtract(30), 20);
        assert_eq!(increase(30), 40);
    }

    #[test]
    fn implicit_and_precise_readers_use_the_same_live_source() {
        let mut text = String::new();
        text.push_str("教學");
        let implicit = byte_reader(text.as_str());
        let precise = precise_reader(text.as_str());
        assert_eq!(implicit(), 6);
        assert_eq!(precise(), 6);
    }

    #[test]
    fn precise_empty_capture_does_not_depend_on_the_input() {
        let read = {
            let mut inside = String::new();
            inside.push_str("Rust");
            fixed_reader(inside.as_str())
        };
        assert_eq!(read(), 4);
    }
}
