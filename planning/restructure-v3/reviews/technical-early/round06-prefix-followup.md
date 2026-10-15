# 第6章前綴兩項窄修複核

6.3–6.6尾段初判封存後，僅核對root通知的6.1／6.2實際source差異。原prefix與tail初判保留；未讀continuity報告、作者驗證或其他角色結果，沒有重跑Cargo、rustc、Clippy、測試或瀏覽器。

- **TC06P-O1已解決。** 本角色原有選讀建議：6.1:119「非負小數」只補一個「整」字，成為「非負小整數」，與同題i32及5行的材料範圍一致，沒有增添概念。
- **C6a01修後已解決，受root提示，不算本角色獨立發現。** 原本整頁技術初判未報這項首次Debug格式化的ownership說明。我同意需要在這個位置交代：先前primitive／as_str顯示不能單獨推出對整個non-Copy struct格式化時是否會接手。6.2:7現已明說本次格式化只借讀summary、不取得ownership，顯示後仍可讀欄位；位置在首個完整debug snippet之前。實際debug.rs於println後讀amount／count，型別只derive Debug而未Copy，與新增說明相符。句子限於「這次格式化」，沒有錯稱任意argument expression都不會move。

完整diff僅上述一字與一個新增句子，前綴已封存的trait-stages Rust／fixtures／expected／package設定hash皆未變。這是修後source時序與正確性的窄核，不回寫為我在舊版已獨立發現，也不等同瀏覽器或目標讀者實驗。前後hash與逐項證據保存於同名JSON。
