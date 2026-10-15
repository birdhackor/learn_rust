# 第15.5–15.6節獨立技術初判

必要問題：0。可選修正：0。

此為完整頁面的技術複讀與實際 source closure 審查，不是盲讀讀者實驗。初判前未讀作者驗證、其他角色 review 或歷史問題。讀者前綴為已實讀第1–13章與15.1–15.4；未用第14／16章替本頁補洞。

- 15.5：逐讀獨立 C11 範例的七個檔案。pointer／array／len／NULL 零長度政策、NUL 與一般 byte zero、sizeof／strlen、malloc 初始化與配對 free、signed overflow 的前提、printf 型別及各項輸出均與 source 相符。missing_length.c 確實少傳第二個長度參數。checks／solution 的有限輸入與 240→260 答案吻合。
- 15.6：scalar／buffer／label 三個階段各七檔，命令只載入所選 package 的 normal library、build script 與該階段 C。完整 source 都在各首次命令前；較後 CString／C-owned owner 不會滲入前段 closure。
- 完整 ch12-ffi：C 工廠交出五個已初始化 char 的 malloc 配置或 NULL；Greeting 的 private pointer、無 Copy／Clone、無公開寫入或 free 介面、&self getter 與專用 Drop 共同支持 CStr view 契約。配置來源與釋放函式配對，Rust 文字副本在 drop 後仍有效。六項測試及 usize constants 在 --lib 前解釋；所有五個 examples／答案都在 all-targets Clippy 前完整展示。
- repr(C)／u32／c_char／c_int、長度轉換、空 slice、counter 邊界與 CString 內零拒絕均正確；四份 compile-fail 的原因與 expected 字串相符。
- 查閱 pinned cc 1.8.0 API 原始碼；建置 dependency／host-target／archive／link metadata 角色及鏈式方法符合實際 API。三張 SVG 僅檢查 XML／文字／箭頭原文，未做視覺驗收。

範圍：47 個 owned files、75 個 snippet references、18 行 literal commands；58 個 source hashes（含一份 cached cc API）。完整清單、逐目 closure 證據與 hash 見同名 JSON。

本波沒有執行 Cargo、rustc、C compiler、Clippy 或測試；結果與失敗原因均為 source 推導，並未聲稱重現作者執行。必要疑點未出現，故沒有擴大執行。

頁面 SHA-256：

- 15.5：d7a229d638429477507f1ca2beb0d742c008be9380f724d527f8ec583d15f04a
- 15.6：ed5a9511f205772cacba2f95e4c4b5b25744e59c039fdd4c656e78d090e713a7
