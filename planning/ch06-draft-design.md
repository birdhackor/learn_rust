# 第 6 章分段寫作安排

本文件僅供作者與協調者使用，沒有學生正文或 Rust 程式。中英文與正式 API 閱讀紀錄見 [source-notes/ch06.md](source-notes/ch06.md)。第 3 章已由 root 凍結為 ch03-prerequisite-v3；目前等待第 4 章實際稿，沒有以未完成大綱作先備。

## 可先交付 6.2，閱讀順序仍從 6.1 開始

6.2 的內容只依賴實際 4.1，不使用 closure、Fn、Iterator 或尚未完成的 5.4。第 3.3 節的 Holder 已建立「保存一個 T」的資料形狀，可以用同類資料模型說明「實作自訂 trait 時，指定這個來源交出哪種型別」。來源自己選定 Output，呼叫者再按這個選擇取得結果，仍遵守 Copy／move。

暫定的教學步驟如下，取得 4.1 後須依其實際 trait 名稱與 bounds 語法調整：

| 步驟 | 必要的理解 | 本節負責先解釋 | 可觀察的驗證 |
| --- | --- | --- | --- |
| 先建立需要 | 不同來源保留共同的「取得結果」操作，但結果型別可以不同 | 每個來源對應一種輸出；不是每次呼叫任選輸出型別 | numeric 來源取得 i32，布林來源取得 bool |
| trait 留下型別位置 | 型別選擇也是 trait 契約的一部分 | associated type 的宣告與 impl 中指定型別；不是執行期欄位 | 一個 impl 的 Output 與 method 回傳型別一致 |
| 連回 Self | Self 是實作此 trait 的具體型別 | Self::Output 表示該實作選的輸出型別 | 對照 impl 中已指定的 i32／bool |
| 讓泛型呼叫者保留關係 | 不必把所有來源強制變成同一個 i32 | 泛型參數的 Output 路徑與函式回傳契約 | 呼叫位置取得各自的具體輸出，未知 Output 不任意算術或顯示 |
| 有需要才限制輸出 | 部分呼叫者只處理已知的數字工作 | associated type equality bound 是限制該實作的選擇，不是一般型別參數的位置值 | numeric 來源通過；布林來源因輸出契約不符被拒絕 |
| 收束與練習 | 型別與值輸出的關係可以追蹤 | 依接收者需要設計單一輸出契約，保留 ownership 行為 | 輸出型別、值、ownership 分別核對，不用未教工具捷徑 |

不借用後文 TryFrom／Fn／Iterator 的完整簽名當作第一次介紹。不展示 GAT、Sized／?Sized、inherent associated type、nightly 手動 Fn 實作或 dyn／Box。若 4.1 尚未教泛型 impl 或必要的 bound 語法，本節要在首次使用前補足，不能假定已教。

## 本章開閘條件

| 小節 | 必須實際接收的前文 | 寫作限制與用途 |
| --- | --- | --- |
| 6.1 closure capture | 4.5 | 先用 read／modify／move 的小工作；不提前 thread、排序或 Iterator |
| 6.2 associated type | 4.1 | 依上表獨立建立契約，可先寫但不先發布到讀者路線 |
| 6.3 Fn／function pointer | 6.1、6.2 | 先教 receiver 能力與 associated Output，才展示 bounds；不把 move capture 當成 FnOnce 的充分判準 |
| 6.4 impl Trait | 6.3、4.4 | 用已教 Fn 的 helper 建立回傳 opaque 型別，再教本例必要的 edition 2024 lifetime capture |
| 6.5 Iterator／IntoIterator | 6.4 | 手動 next → for；清楚區分 Item 的值／reference；顯式 as_slice／as_mut_slice |
| 6.6 adaptor／collect | 6.5 與實際 5.6 | 接收到第 5 章真正教過的解析／錯誤轉換接口後，才設計 Result collect；filter 接收 &Item，collect 目標要明確 |
| 6.7 文字／entry／排序 | 6.6、4.3、3.5 | chars／lines、HashMap IntoIterator／Entry callback、Ordering／Ord／PartialOrd 都在首次使用前先教；穩定輸出有具體排序規則 |

後續每段開始前，只重查所選正式 API 與版本差異；不重做無關教材搜索。以實際凍結稿與 source snippet 更新作者依賴表，且明確分開作者自查與獨立首讀。首讀者仍只取得依閱讀順序的正文及原位必要圖。
