fn local_twice<F>(mut callback: F) -> (i32, i32)
where
    F: for<'a> FnMut(&'a i32) -> i32,
{
    let first_result = {
        let first = 80;
        callback(&first)
    };
    let second_result = {
        let second = 120;
        callback(&second)
    };
    (first_result, second_result)
}

fn main() {
    let mut visits = 0;
    let transform = |number: &i32| {
        visits += 1;
        *number + 20
    };
    let (first, second) = local_twice(transform);
    assert_eq!(first, 100);
    assert_eq!(second, 140);
    assert_eq!(visits, 2);
    println!("兩次結果={first}、{second}，處理次數={visits}");
}
