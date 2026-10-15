use ch14_api::LimitError;

fn explain(error: &LimitError) -> &str {
    match error {
        LimitError::Zero => "零上限",
        LimitError::TooLarge(_) => "過大上限",
    }
}

fn main() {
    println!("{}", explain(&LimitError::Zero));
}
