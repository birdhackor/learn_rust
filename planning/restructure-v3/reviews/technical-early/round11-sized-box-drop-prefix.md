# 第 11.1–11.3 節獨立技術初判

封存時間：2026-10-15T08:32:12.342Z。必要問題 0；可選問題 0。

這一波逐頁讀完 Sized／DST／const generics、Box recursive chain、Drop，並核對全部 40 個 snippet 來源、10 個 run target、2 個 selected example test（1 + 2 個 test）、8 組 compile-fail source／expected。完整來源 SHA-256 見同名 JSON（43 檔）。未先讀作者驗證、peer、首讀、歷史問題或 ledger；未讀 11.4 以後作先備。

- 11.1：Sized 預設 bound、size_of 與 size_of_val 的量測對象、?Sized 的放寬、str/reference 差別均在首用前交代。16-byte reference 輸出明限作者的 64-bit 平台，沒有寫成跨平台保證。const N、[T; N]、忽略參數 _ 與完整答案／E0308 對得上。
- 11.2：Box 的 owning／move、非零 heap 範例、固定大小 handle、*／&*／&mut* 在首用前完整；Box 特有的 owned move-out 沒被誤推至借用 reference。recursive inline 的 E0072、Chain 逐次消耗到 200／0、答案 240 與 helper tests 一致。
- 11.3：Drop 方法與 mem::drop、local 逆宣告順序與 fields 宣告順序、custom Drop 先執行，以及 NLL 借用期限與 owner cleanup 時點的差別均準確。四個負例錯誤碼及原因與完整 source 一致。

intro-type-stages 沒有 src/lib.rs、build.rs 或 dependencies。各命令只載入指定 example 及其明示 mod；完整 helper（含 cfg(test)）皆在正文提供。未以未讀 sibling example 補 closure。所有完整答案均已對 source／輸出靜態核對。

本波沒有新增 Rust/Cargo 執行；不把靜態結果稱作重新跑過。boxed-chain.svg 只讀 XML/source，文字與 owning arrows 對 source 一致，未作 browser／視覺驗證。這是技術整頁重讀，非盲首讀。
