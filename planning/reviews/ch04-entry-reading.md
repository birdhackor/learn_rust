# 第4章進入前：指定背景補讀

日期：2026-10-14。角色：以基本 Python 語法讀者為起點的 AI 輔助讀者模擬；不宣稱是真正 Python 新手，也未執行教材程式。

## 已讀範圍

依指定 reading-order 順序完整讀取正文單位：ch01-complete-source-v8 的 50 單位（1.1–1.10）、ch02-complete-source-v4 的 31 單位（2.1–2.6）、ch03-prerequisite-v3 的 32 單位（3.1–3.6）。正文遇到的圖片已查閱其指定原 SVG：library-split、move-copy、reference-owner、borrow-scope、slice-part、string-view、utf8-boundaries。未讀 manifest／sources、外連選讀或答案、作者歷史、大綱、source-notes、ledgers、其他 reviews 或第4章後文。

這是本輪指定的前文補讀，不宣稱已做第1–3章逐段首次閱讀驗收；下列理解直接來自實際正文，未用作者摘要代替。

## 進入第4章需要的理解

- 1.4–1.6：型別決定有效值與操作；函式簽名寫明輸入／回傳型別，block 尾端交出值。編譯通過與計算規則正確不同（1.10）。
- 1.9：module 組織名稱與可見性，pub／use／crate 路徑組合 library 與呼叫者；拆檔不會另改計算規則。
- 2.1–2.2：struct 定義欄位形狀，實例是具體值；ownership 是使用與清理責任。無 Copy 的 Order 整筆指派會 move，i32 指派產生獨立數值。不能由 stack／heap 推断 move。
- 2.3–2.5：shared reference 只借用讀取；mutable reference 修改原資料並要求獨占。借用使用區間與名稱 scope 不同。impl Order 組織其行為；self、&self、&mut self 分別接手、只讀、修改，Self 指型別。associated function 沒有既有 receiver。
- 2.6、3.4–3.6：slice／&str 借用既有範圍，String／Vec 擁有資料；借用、複製借用資訊與複製內容是不同工作。方法契約須逐個讀，get／pop 的 Option 與直接索引／remove 的 panic 不能混用。
- 3.1–3.3：enum 保存互斥情況，pattern 先辨形態再取得資料；Option／Result 保留缺值／失敗資訊。泛型保留資料形狀而替換型別，T 是型別參數；同一 T 必須同型別，未限定的 T 不能假定可算術或顯示。

## 仍不確定／尚未學到

已知標準數值具備 Copy、自訂 struct 不會自動具備；尚不知道「能力」如何正式宣告或如何限制泛型操作。尚未學自訂型別整體的相等比較、內容複製與顯示契約，也未把前文的 impl Order 理解為「實作某個共同能力」。這些位置保持未知，等待第4章实际正文；不提前以 Rust 既有知識補足。
