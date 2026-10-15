# 17.4／17.5 root回指窄修複查

2026-10-15T09:39:42.912Z：兩組提示修訂均符合actual前文。17.4:231／360的rust-version／publish來源回收12.1；17.5:3／7／29的完整分組累加回指6.6，:117的insert覆寫仍正確留6.5。

這是root在原初判seal後指出的窄複查，不算本人獨立發現。兩份before內容hash與自己的前次sealed版本一致，exact diffs及新hash見JSON；owned Rust／manifest／fixtures／commands未改。全域Cargo metadata另有並行變動，已重核本波共同設定與依賴選擇一致。沒有Cargo/Rust執行，原初判不動。
