// 刻意用已教的 match 對照有值與缺值，保留兩個分支。
#[allow(clippy::manual_map)]
pub fn selected_byte_len(text: &str, start: usize, end: usize) -> Option<usize> {
    match text.get(start..end) {
        Some(part) => Some(part.len()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::text::selected_byte_len;

    #[test]
    fn chinese_scalar_uses_three_bytes() {
        let present = match selected_byte_len("A中🦀", 1, 4) {
            Some(length) => {
                assert!(length == 3);
                true
            }
            None => false,
        };
        assert!(present);
    }

    #[test]
    fn empty_valid_range_is_present() {
        let present = match selected_byte_len("A中🦀", 8, 8) {
            Some(length) => {
                assert!(length == 0);
                true
            }
            None => false,
        };
        assert!(present);
    }

    #[test]
    fn partial_scalar_is_absent() {
        let segment_count = match selected_byte_len("A中🦀", 1, 2) {
            Some(_) => 1,
            None => 0,
        };
        assert_eq!(segment_count, 0);
        let empty_inside_scalar = match selected_byte_len("A中🦀", 2, 2) {
            Some(_) => 1,
            None => 0,
        };
        assert_eq!(empty_inside_scalar, 0);
    }

    #[test]
    fn out_of_bounds_is_absent() {
        let segment_count = match selected_byte_len("A中🦀", 4, 9) {
            Some(_) => 1,
            None => 0,
        };
        assert_eq!(segment_count, 0);
    }
}
