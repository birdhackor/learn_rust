# Rust 1.100 更新：來源、先備與採用範圍

2026-11-13，作者實讀與自查。既有歷史來源與1.99量測紀錄保留原版本；本紀錄不代表獨立首讀或全書重新教學驗收。

| 來源與實讀範圍 | 採用位置及界線 |
| --- | --- |
| [Rust 1.100 發布公告](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/)，never、DropGuard、Cargo lint及冷卻期段落 | 確認穩定版本與限制。未將allocator或conjure_zst加入本輪正文。 |
| [never type 1.100](https://doc.rust-lang.org/1.100.0/std/primitive.never.html)，never-to-any coercion及infallible errors | 8.1分開unit與不交回正常值；8.4用標準i32→i64的TryFrom觀察Result錯誤分支不可能成立。`-> !`既有，其他型別位置本版穩定。 |
| [DropGuard 1.100](https://doc.rust-lang.org/1.100.0/std/mem/struct.DropGuard.html)，new、dismiss、Deref／DerefMut及Drop | 新11.12只使用第9章closure、第8章?、11.3–11.4的清理與借用介面。wrapper保存mutable reference；不聲稱任意修改皆能只靠truncate還原。 |
| [Rust Book：Drop](https://doc.rust-lang.org/book/ch15-03-drop.html)及[正體中文對應章](https://rust-lang.tw/book-tw/ch15-03-drop.html)，自訂清理、scope及顯式drop | 借鑑以可觀察收尾工作連到資源責任的教法；不以println順序代替批次成功／回復契約。 |
| [Cargo lints 1.100](https://doc.rust-lang.org/1.100.0/cargo/reference/lints.html)，設定、unused_dependencies及檢查範圍 | 10.1保留分階段宣告的教學動機並解釋警告；12.1用真實小workspace示範規則繼承。未使用dev-dependencies不在此檢查範圍。 |
| [Cargo config 1.100](https://doc.rust-lang.org/1.100.0/cargo/reference/config.html#registryglobal-min-publish-age)及公告冷卻期段落 | 12.1區分Cargo.toml與.cargo/config.toml，說明lockfile、缺少pubtime與cargo install界線。十天／一天是示意，不假稱即時registry實測。 |

本輪Rust範例、規則繼承、網站與fixture實跑結果由維護者檢查保存於work/及outputs/checks/；發布前另做受影響段落的作者自查與桌面／手機實頁檢查。這些證據不代替目標讀者測試。
