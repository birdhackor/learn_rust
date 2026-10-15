fn main() {
    let mut checked = 0;
    'search: for row in 1..=3 {
        for column in 1..=4 {
            checked += 1;
            if row == 2 && column == 2 {
                break 'search;
            }
        }
    }
    println!("停止搜尋前檢查{checked}格");

    let mut processed = 0;
    'rows: for row in 1..=3 {
        for column in 1..=4 {
            if column == 3 {
                continue 'rows;
            }
            processed += 1;
            println!("處理第{row}列第{column}格");
        }
    }
    println!("跳列範例共處理{processed}格");
}
