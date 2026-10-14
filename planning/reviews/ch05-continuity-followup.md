# 第 5 章 continuity：交叉覆核與 v7 局部複查

## 本輪身分與先後界線

- 審閱者：`chapter11_async`，第 5 章獨立 continuity 角色，未參與寫作。
- 本輪在三角色初判封存後，依 root 指派閱讀首讀、技術與處置紀錄，再核對 `ch05-review-fixed-v7`。這是**受同行發現提示後的交叉覆核／修後複查**，不算新增獨立重現，也不是重新執行盲讀。
- 原 `ch05-continuity-v3.md` 與 `ch05-continuity-v3.initial.sha256` 未修改；重新計算仍為 `8df7753bd9349185429409952293ddb14f6907a58b73994f723f2a819dc16133`。
- 已讀同行紀錄：`ch05-first-reading-v1.md`、`ch05-technical-v3.md`；已讀 root `ch05-disposition.md`。首讀原 O2 的可選分級、自己的原 N1 必要分級，均留在各自原檔，沒有以 root 最終裁定回寫它們。
- 後續教材範圍仍只有第 5 章及初判已實讀的第 1–4 章先備；沒有讀第 6 章或第 11 章 live 稿。

## 實際複查範圍與指紋

凍結根：`/workspace/work/tutorial-implementation/frozen/ch05-review-fixed-v7`。

- manifest SHA-256：`074e5e1f97633bea531294f401ac3a87f8ca1364ae75c9cbbb80717d8f8df1ba`。
- reading-order SHA-256：`de9c5f94af5b474485815eef2c4e09240003903115416a835661855fecbbc47c`，35 units 順序不變。
- 以 v3 manifest 對照 v7，只有 7 個 units 與 1 個 source 內容變更；本輪實讀這 7 個 units 的完整內容（含保留在原位置的 code／folded answer），以及變更 source。其餘 28 個 units、39 個 sources bytes 與初判已讀的 v3 一致，不宣稱本輪重新逐篇閱讀它們。
- v7 全部 35 個 unit 與 40 個 source 的 bytes 都核對 manifest hash 一致。hash 相等支持內容沿用，不替代執行或視覺證據。

| 本輪實讀 unit | SHA-256 | 原位置／主要改動 |
| --- | --- | --- |
| `001-004.md` | `20f77e14f23ae2b94185f83fb553b74f4b9184c1478f881e5882092e6f2c4f01` | 5.1 unwrap 失敗 fixture：借用數字先 `*` 再顯示。 |
| `003-002.md` | `19926bf23d0c4ba3be12050c8c8b58536246054529e7e319f2c38f2b43b95c9b` | 5.3 首次 Display：補 `T` 有 Display 時 shared reference `&T` 也能顯示所指值。 |
| `004-002.md` | `5d5b126a5c1c5ebab9bdff241a70fd7e2b528ede6b1f17f8c675af4fad63a9f2` | 5.4 第一份轉換 code 前：edition 2024 prelude 提供 From／Into／TryFrom。 |
| `004-003.md` | `e468f862509bebf5890d0f0addd90724707f1641db24beb184e833db340f350b` | 5.4 私有 field：限制主詞收窄為 library 外的呼叫者。 |
| `005-007.md` | `828947b1052e854cf8b9edf5b34b835690a5bd1e80bbfbfd3ceae2950ecf37a0` | 5.5 tests 前：tuple pattern 在 for 的位置、bare `other` 綁定剩餘完整值。 |
| `006-002.md` | `af9691289ac9eddd04217669d2d76a99c4ee0820c848c21797b8b0c41627c7fc` | 5.6 return 前文章號由 1.5 改為 1.6。 |
| `006-003.md` | `16f62326b8e17a8a9f969768ea3d4e81a75ce456d88bc64683c68244af60cf4a` | 5.6 Result `?` 契約：標準同型別 From 原樣交回、ownership 交接。 |

唯一變更 source 是 `sources/examples/ch05-errors/fixtures/runtime-fail/unwrap_error.rs`，SHA-256 `a7d6d0563ab48a745369c25554cbee88064134a320f19e627597767c45d85c72`。原必要 SVG `assets/question-paths.svg` 與 archived source 仍同 SHA-256 `a94a79cc4eb5f56cb515e567613830b407e818c568079e71d550ad2a85773934`。

本輪互看時讀取的紀錄指紋：

- 首讀：`250798af4deec84907d60101e9cdf3d13988d45a8bacfe05c2f2d226934ba4ed`。
- 技術：`99f52e0bed1b3ef2144eeaa2a749c3853242bc33bf6e3a5bdfd299ae5c866005`。
- 處置：`a3c3dc7d97b73df1b59115e095f08d410bfd649834f4be366d3bbc574e856bc3`。

## 自己的兩項原判：修後判斷

| 原判 | 實際修正與現在可由正文回答的事 | 本輪判斷 |
| --- | --- | --- |
| 必要 N1：`other =>` 全值 binding 的最小契約 | `005-007` 在 tests 前說「接住前面尚未符合的整個值，綁定為 other」，再與 `_` 不保存值對照。依先前 FromStr／Result 型別，這裡整個值就是 `Result<Count, ParseCountError>`；先前 arm 已接受 `Err(Integer(_))`，其餘 Ok／Range error 才交到 other。 | 本輪內容銜接已補足，沒有要求額外 pattern 專章或新 API。保持原 N1 分級與發現時點；修後通過不是原 v3 已教。 |
| 可選 I1：return 回讀章號 | `006-002` 已改 1.6，與初判實讀的 `ch01-complete-source-v8/006-003.md`「提早交出結果」相合。 | 定位收益已實現；流程與 code 未改，不需重跑 Rust。保持原可選分級。 |

首讀原 O2 對 tuple／other 的讀法明記為由已讀前文和 code 組合而來，列可選；自己的 N1 則聚焦完整 Result binding 的必要規則。root 最終將 bare binding 列必要、for tuple 維持可選，兩者在同一小段補清。本輪尊重這項差異，不把首讀原推論或兩個原分級改寫成一致，也不把互看後同意當第二次獨立發現。

## 同行提示後的內容覆核

### reference Display

首讀原 `001-004` 保留 `&i32` 直接格式化的未知，又在 `005-003` 延伸到借用錯誤。自己的原 continuity 表在 `001-004`／`005-003` 判足夠時，保留了 unwrap 政策與原因封裝的有效部分，卻沒有查清直接顯示 reference 的具體能力；此局部缺口本輪是在同行提示後覆核，不能算原獨立發現。

v7 選了兩種相稱修法：5.1 在 Display 尚未教時，以已教的 `*count`／`*value` Copy 讀出數字，再交給 println；5.3 首次 Display 處明說 `T` 有 Display，shared reference `&T` 也能顯示所指值，且不取得資料 ownership。後者在5.5 borrowed error 的 write! 之前，接起 pattern 所得到的 reference 與顯示契約。沒有提前帶入 Formatter 內部、Deref／Box／dyn，未改業務規則。**局部依賴補足。**

### prelude 與 trait scope

首讀 scope 未知與技術原必要 N1 都指向 caller 只引入 Count，卻可用 Into／TryFrom 的名稱來源。自己的初判已讀 4.1 trait 要進 scope，卻未在這個跨 crate consumer 對照 library 的 use；現在只能記為同行提示後補查。

v7 `004-002` 在第一份轉換 example 前，以「預先匯入的常用名稱」教 prelude 身分，指定本專案 edition 2024 已提供 From／Into／稍後 TryFrom；並保留明寫 use 的來源追蹤用途。讀者不必誤以為 library module 的 use 傳到 example crate，也不必推斷所有標準 trait 都自動可用。自訂 trait 繼續依4.1引入，規則一致。這只預告稍後 TryFrom 的名稱可用，並未在解釋它的操作之前呼叫。**首用名稱範圍已補足。**

### 同型別錯誤的 From

首讀在 count_pair 練習指出同一 `ParseCountError` 的 `?` 如何滿足已教 From 條件。自己的原 `006-007` 正面判斷主要以早退與結果形狀為依據，沒有核對「相同 E 仍符合 From」的最小保證；本輪承認這個覆蓋差異，保留原紀錄，不將先前能預測早退說成當時已解釋完合法性。

v7 `006-003` 先列 `?` 的 From 條件，接著說標準庫為每個 T 提供 T→同T 的 From，原樣交回收到的值。以 `ParseCountError` 對回目前的具體 error，並明說仍是 ownership 交接，而非再 clone 或把error變成別的原因。這位於 `006-007` 練習前，與5.4 From 的來源／目的及5.5包裝原因契約相合。無須展示泛型 blanket impl 才能教當下使用。**練習必要條件已補足。**

### private field 主詞與 tuple-for 可選補充

技術可選 I1 將「其他 module」收窄到本例真正的 library 外 caller。v7 正好採這個主詞，保留 private field、只讀 method 與同 module 實作者須維持規則的原脈絡；沒有聲稱所有不同 module 一律隔絕。這是精確化收益，不要求提前完整 visibility 規則。

首讀 O2 的 tuple-for 部分由1.7解構與1.8走訪可合理組合，因此root維持可選。v7 在tests前說明for名稱位置可放tuple pattern，每次拆成兩名供當次body；沒有改變材料或加入不同演算法。少量補充能降低套用到新位置的負擔，不必把原可推得理解改判必然阻礙。

## 改動對整體銜接與圖的影響

7個變更 units 維持原順序與同一件數材料。reference Display、prelude、bare binding與同E From都在各自第一次需要之前補最低契約；沒有把新能力塞到更早的 code 裡再靠後文解釋。個別補句仍服務當節顯示、名稱查找、測試檢查、錯誤傳遞的原需要。

`006-003` 的 SVG source未改，只因snapshot路徑改寫而使用v7 asset；左Ok繼續、右Err轉換後整函式回傳的語義仍與新增same-E段落相容。同E的轉換是原樣交接，不會把錯誤路合流回成功路。其餘 unchanged code／答案與初判已讀內容一致，修正沒有新增測試用 helper 或未教第三方 API。

**本輪沒有重新執行本章 Rust／fixture／tests、fmt／Clippy／文件建置，也未檢查實際桌面／手機頁面或渲染SVG。**已有root或其他角色的執行／視覺證據不改稱本角色親自完成。此處只有凍結來源、依賴、原位引用與局部轉場的修後覆核。

## 本輪結論

自己的必要N1／可選I1已在v7補足；同行提示的reference Display、prelude、same-E From及private主詞／tuple-for補充也已確認對應實際改動與首次需要位置。本輪未發現新增的內容銜接缺口。

原初判、分級與漏查範圍都保留；新增同行提示、root裁定、修後內容證據與未做的實頁／執行各自分開。這支持第5章**指定修正範圍的continuity複查**，不宣稱重做全書首讀、驗證真實新手成效或新增全套執行／視覺通過。
