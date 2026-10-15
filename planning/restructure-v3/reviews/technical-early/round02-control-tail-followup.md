# TC02T-01 修後獨立來源複查

原始初判保留。本次依實際正文、現行target清單、manifest與sources核對，判定TC02T-01已修正，沒有該項剩餘必要缺口。未讀作者ledger或依賴作者的「已修」宣稱作證。

- `02-07-lesson.md:69–73` 的第一次Clippy改為 `--lib`，實際自有source閉包是control的lib.rs與pricing.rs，不載入lint_return或integration_answer範例。
- `02-07-lesson.md:81–100` 先解釋needless_return、精確allow的範圍與保留理由，再完整嵌入lint_return.rs；之後才run及 `--example lint_return` 的精確Clippy命令。這個選擇仍會連結同package library，該library與pricing已有完整來源，沒有漏掉的自有來源。
- `02-08-lesson.md:95–129` 先給完整integration_answer，再擴大到整包fmt／fmt --check與Clippy --all-targets。實際control package只有library、主要binary、assertions_answer、lint_return、integration_answer；三份example來源及allow用途至此都已由當頁／必要前文提供，沒有再次載入未展示的後續target。

對照原始獨立snapshot，control所有Rust sources與其manifest均未變；只有教學命令的選取和順序調整，保留原本有理由的精確allow示範，也沒有增加未教API。原問題的兩個最小關係——allow先說明、廣泛檢查的完整來源先提供——都已滿足。

本次僅靜態複查，未執行Cargo/rustc/Clippy/fmt/tests或browser。hash與逐命令閉包存於同名JSON；不將本次修後複查改稱原始初判已通過。
