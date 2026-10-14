use ch08_smart_pointers::payloads::event;

struct ExtraInfo {
    count: i32,
}

fn main() {
    let recorded = event(3, ExtraInfo { count: 2 });
    match recorded.payload.downcast::<ExtraInfo>() {
        Ok(mut info) => {
            info.count += 1;
            let taken: ExtraInfo = *info;
            assert_eq!(taken.count, 3);
            println!("取回自己的資料型別，count={}", taken.count);
        }
        Err(original) => println!("不是ExtraInfo，原資料是i32={}", original.is::<i32>()),
    }
}
