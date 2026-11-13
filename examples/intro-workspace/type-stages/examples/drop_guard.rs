use std::mem::DropGuard;
use std::num::ParseIntError;

// --8<-- [start:append]
fn append_counts(counts: &mut Vec<i32>, texts: &[&str]) -> Result<(), ParseIntError> {
    let original_len = counts.len();
    let mut guarded = DropGuard::new(counts, move |counts| {
        counts.truncate(original_len);
    });

    for text in texts {
        guarded.push(text.parse::<i32>()?);
    }

    DropGuard::dismiss(guarded);
    Ok(())
}
// --8<-- [end:append]

fn main() {
    let mut counts = vec![80];
    append_counts(&mut counts, &["120", "40"]).expect("兩筆文字都是合法i32");
    assert_eq!(counts, vec![80, 120, 40]);
    println!("成功後={counts:?}");

    let failed = append_counts(&mut counts, &["60", "不是數字", "100"]);
    assert!(failed.is_err());
    assert_eq!(counts, vec![80, 120, 40]);
    println!("中途失敗後={counts:?}");
}

#[cfg(test)]
mod tests {
    use super::append_counts;

    #[test]
    fn success_keeps_all_appended_values() {
        let mut counts = vec![80];
        append_counts(&mut counts, &["0", "20"]).expect("都是合法i32");
        assert_eq!(counts, vec![80, 0, 20]);
    }

    #[test]
    fn later_failure_restores_existing_values() {
        let mut counts = vec![80, 120];
        assert!(append_counts(&mut counts, &["60", "bad", "100"]).is_err());
        assert_eq!(counts, vec![80, 120]);
    }

    #[test]
    fn empty_batch_succeeds_without_changing_existing_values() {
        let mut counts = vec![80];
        append_counts(&mut counts, &[]).expect("空批次沒有解析失敗");
        assert_eq!(counts, vec![80]);
    }
}
