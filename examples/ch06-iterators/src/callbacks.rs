// --8<-- [start:once]
pub fn run_once<T, F: FnOnce() -> T>(task: F) -> T {
    task()
}
// --8<-- [end:once]

// --8<-- [start:mutable]
pub fn call_twice<F: FnMut() -> i32>(mut task: F) -> (i32, i32) {
    (task(), task())
}
// --8<-- [end:mutable]

// --8<-- [start:shared]
pub fn apply_shared<F: Fn(i32) -> i32>(task: &F, value: i32) -> i32 {
    task(value)
}
// --8<-- [end:shared]

#[cfg(test)]
mod tests {
    use crate::callbacks::apply_shared;
    use crate::callbacks::call_twice;
    use crate::callbacks::run_once;

    #[test]
    fn one_call_can_transfer_owned_text() {
        let mut text = String::new();
        text.push_str("Rust");
        let mut saved = run_once(move || text);
        saved.push_str(" 教學");
        assert_eq!(saved.as_str(), "Rust 教學");
        assert_eq!(run_once(|| 7), 7);
    }

    #[test]
    fn two_calls_preserve_mutable_capture_state() {
        let mut count = 0;
        let (first, second) = call_twice(|| {
            count += 1;
            count
        });
        assert_eq!(first, 1);
        assert_eq!(second, 2);
        assert_eq!(count, 2);
        let (first_constant, second_constant) = call_twice(|| 7);
        assert_eq!(first_constant, 7);
        assert_eq!(second_constant, 7);
    }

    #[test]
    fn shared_access_accepts_a_capture_and_a_function_pointer() {
        let base = 10;
        let add = |value| value + base;
        assert_eq!(apply_shared(&add, 3), 13);
        let double: fn(i32) -> i32 = |value| value * 2;
        assert_eq!(apply_shared(&double, 3), 6);
    }
}
