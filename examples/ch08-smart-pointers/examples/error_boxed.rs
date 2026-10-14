use ch08_smart_pointers::errors::count_for_report;
use ch08_smart_pointers::errors::report;

fn main() {
    for text in ["3", "abc", "101"] {
        match count_for_report(text) {
            Ok(count) => println!("成功件數={count}"),
            Err(error) => println!("{}", report(&*error).as_str()),
        }
    }
}
