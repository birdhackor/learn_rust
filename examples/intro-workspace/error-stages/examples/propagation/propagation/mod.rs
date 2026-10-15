use crate::conversions::Count;
use crate::parsing::ParseCountError;
use std::str::FromStr;

// --8<-- [start:manual-propagation]
pub fn parse_count_manual(text: &str) -> Result<Count, ParseCountError> {
    let value = match i32::from_str(text) {
        Ok(value) => value,
        Err(error) => return Err(ParseCountError::from(error)),
    };

    let count = match Count::try_from(value) {
        Ok(count) => count,
        Err(error) => return Err(ParseCountError::from(error)),
    };

    Ok(count)
}
// --8<-- [end:manual-propagation]

// --8<-- [start:question-propagation]
pub fn parse_count(text: &str) -> Result<Count, ParseCountError> {
    let value = i32::from_str(text)?;
    let count = Count::try_from(value)?;
    Ok(count)
}
// --8<-- [end:question-propagation]

// --8<-- [start:option-question]
pub fn second_number(values: &[i32]) -> Option<i32> {
    let value = values.get(1)?;
    Some(*value)
}
// --8<-- [end:option-question]

// --8<-- [start:propagation-tests]
#[cfg(test)]
mod tests {
    use crate::propagation::parse_count;
    use crate::propagation::parse_count_manual;
    use crate::propagation::second_number;

    #[test]
    fn manual_and_question_mark_preserve_all_three_paths() {
        for text in ["0", "3", "100", "abc", "-1", "101"] {
            assert_eq!(parse_count(text), parse_count_manual(text));
        }
    }

    #[test]
    fn absent_second_element_returns_none() {
        let empty: [i32; 0] = [];
        let one = [10];
        let two = [10, 20];
        assert_eq!(second_number(&empty[..]), None);
        assert_eq!(second_number(&one[..]), None);
        assert_eq!(second_number(&two[..]), Some(20));
    }
}
// --8<-- [end:propagation-tests]
