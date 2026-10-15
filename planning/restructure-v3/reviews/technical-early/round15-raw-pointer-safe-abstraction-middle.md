# 第15.3–15.4節獨立技術初判

封存：2026-10-15T09:35:11.404Z。必要問題0；可選問題0。

全文、32snippets、8run與1example test（6函式）、5組完整compile-fail／expected、2SVG source已靜態核對。39份SHA-256、8份snapshot bytes比較及逐target閉包見JSON；未讀作者驗證或其他角色紀錄。

raw pointer型別／dereference星號、Copy與無ownership、raw borrow operator、null／alignment／provenance角色準確。array→slice先教，pointer只由整份array衍生；add在同allocation與端點限制、無其他路徑raw期間存取均有明確依據。

unsafe fn呼叫條件與body block分工清楚；safe split在mid檢查後才運算，安全理由包括輸入slice的storage／alignment／初始化／來源資格，以及輸出不重疊與借用有效性。空Vec、0／len、one-past零長度都有正確限定，不把mid檢查當全部證明。

ReadOnlyI32真實reference與private pointer對同一i32，全部公有入口只讀；unsafe Send與Sync分別有來源、存取及清理理由。兩scoped workers均等待後判結果；source不會被wrapper保活到超出其borrow。正常targets全部獨立no-lib，不提前載完整後段快照。

完整練習與6項tests的值和責任正確，未由有限PASS聲稱全soundness。沒有本波Cargo/Rust/Miri或UB執行；SVG只讀XML/source。
