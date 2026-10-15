use std::any::Any;

// --8<-- [start:panic-message]
pub fn panic_message(payload: Box<dyn Any + Send + 'static>) -> String {
    match payload.downcast::<String>() {
        Ok(text) => *text,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(text) => {
                let mut message = String::new();
                message.push_str(*text);
                message
            }
            Err(_) => {
                let mut message = String::new();
                message.push_str("非文字panic payload");
                message
            }
        },
    }
}
// --8<-- [end:panic-message]

// --8<-- [start:sum]
pub fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for value in values {
        total += *value;
    }
    total
}
// --8<-- [end:sum]

// --8<-- [start:panic-tests]
#[cfg(test)]
mod tests {
    use crate::workers::panic_message;

    #[test]
    fn accepts_both_text_types_and_reports_other_types() {
        let mut owned = String::new();
        owned.push_str("owned文字");
        let message = panic_message(Box::new(owned));
        assert_eq!(message.as_str(), "owned文字");
        let message = panic_message(Box::new("literal文字"));
        assert_eq!(message.as_str(), "literal文字");
        let message = panic_message(Box::new(7_u32));
        assert_eq!(message.as_str(), "非文字panic payload");
    }
}
// --8<-- [end:panic-tests]
