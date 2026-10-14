# 第 8 章 v9 指定六段局部覆核

這是 AI 模擬讀者的修後局部重讀，不是新的盲讀或第 8 章全章重作。當前背景為基本 Python、本人實際已讀第 1–7 章正文及局部补讀、第 8 章原始 v8 全66單位、第7章 v13 指定五段局部、第9章原始 v5 全27單位；早期各章實讀版本及範圍沿用封存 entry，不宣稱完整讀過所有最新版本。第8／9章 original 判斷、分級與 SHA 一律不回填。

唯一新正文來源為 `frozen/ch08-review-fixed-v9` 的六段，固定路線 `001-002 → 001-005 → 004-002 → 005-002 → 005-004 → 009-005`。只用相同 visible_filter 在輸出前過濾 closed 提示／答案，新 `ch08v9` gate／ledger 獨立，逐段先存四題、Q2機制／用途、短引文、來源指紋與必要未知才開下一段。收到修後位置與看圖範圍，故屬有提示局部覆核；不將後來背景或受提示的理解當原初判重現。

不讀完整 source／archive／peer／root disposition／作者紀錄／後章或第9章 peer。原位必要圖只在當段 hook 後開。005-004 另獲指定同 v9 桌面與手機截圖，僅目前圖與相鄰文字；提供者說這是固定 BODY 重組單頁且已核 served SVG bytes，這項來源說明不是我本人技術探測的結果。我只親看指定圖檔，保存所見與裁切限制，不開旁邊 render／report／source。沒有執行 Rust 或正式頁面探測。

### 001-002 — Sized 名稱來源與型別／所借資料大小

原 v8 此單位 SHA：`c1af48a3ccd69efb367ac79f585a5b79ba7f48de4c04f2e33b4f85abc3878f08`；原初判不變，本輪只核 v9 當段。
Q1：固定大小是型別性質，i32／[i32;3] 與內容長度可變的 str／[i32] 分開；reference 自身固定表示與所借資料大小不同。
Q2a（機制）：Sized 由編譯器依型別判斷，無需提供 method 或自己 impl 指定 bytes；來源 std::marker::Sized 且 prelude 可見。引文：「不是自己寫一份 impl 告訴它想要幾個 bytes」。size_of::<T> 查 Sized 型別，size_of_val(&T) 查所借資料，包含 DST。
Q2b（用途／需要）：按值局部值／參數需要可確定大小的表示；用固定 reference 表示借到的可變長度內容，保留前章 slice／str 操作。明示名稱來源使標準 trait 的直接使用不必猜 import。
Q3：i32=4、三個 i32 array=12，內容變不改同型別大小；欲查型別用 turbofish，欲查當份資料則交 reference，不將後者答案誤當 reference 本身尺寸。
Q4：兩個數字例、DST／reference 對照与 API 表直接支持上述區別；本單位未展開 sizes 實作，我不以其他新版片段補結果。
局部末：新句對名稱可見性與 compiler 判斷的責任有明確依據；沒有新增必要未知／負擔，不需要第9章後來背景。未新增 necessary／optional，原紀錄不回填。

原始SHA-256：cdadedc3e196d90f0ca4920fe9ed745038aecae5a4b2e6b9422e2be023dea029；visible SHA-256：cdadedc3e196d90f0ca4920fe9ed745038aecae5a4b2e6b9422e2be023dea029。

### 001-005 — Box 名稱來源、owned 資料與取出契約

原 v8 此單位 SHA：`99123c9948044d4314244f1d66445e88ee04336e80553f4d70aba00b2eaa1d23`。
Q1：Box 是另一處 T 的 owner；T:Sized 時 Box 本身以固定單一 pointer 表示，但 Box 自身可在 struct 或 heap 內，不必在 stack。引文：「把『保存連結』與『保存連結指向的資料』分開」。
Q2a（機制）：std::boxed::Box 由 prelude 提供；new 取得 value，非 Copy 值不能再用舊名稱。borrowed * 操作連到既有借用規則；move 整個 Box 不會 Copy，而 owned Box 可特別 move 出非 Copy T，原 Box 失效，不推及其他 pointer。表格已直接限定。
Q2b（用途／需要）：當需要用固定連結表示且擁有另一處資料，Box 負責 owned T 的清理與交接；不用讀者猜名稱 import、未知自訂 Deref 或 associated type。String／Vec 自行管理 heap 的句子也避免把 Box 當所有資料的必要外殼。
Q3：shared view 讀120後结束，再 mutable 加20，最後整個 Box 交 second 得140；Amount 非自動 Copy，從 owned Box 取出後 taken 得80。最後使用時點支持轉交；E0382 節錄則使用了已轉交的 first。
Q4：操作表、可見 main、三個固定輸出及故意錯誤例，均支持「資料操作／owner 轉交／取出」區別；未執行命令或驗證診斷。
局部末：新增名稱來源足夠，取出契約的限定仍清楚，不需要第9章知識；未新添 necessary／optional，不改原初判。

原始SHA-256：8d02e0ae1f7e720bd2c1d176930b31ca001510cc6fff806b6638effda983f496；visible SHA-256：8d02e0ae1f7e720bd2c1d176930b31ca001510cc6fff806b6638effda983f496。

### 004-002 — 組合 bound、implied bound 與借用模型

原 v8 此單位 SHA：`7cdba550947bd24192d68ce58a1f20915cf75dd27f3adf8be67859c33ab9b0f3`。
Q1：T:'a 限制 T 內借用來源，與兩 lifetime 的 'b:'a 及值本身存活時長分開；外層 Box 擁有儲存仍不消除內層借用。
Q2a（機制）：`?Sized + 'a` 表兩項一起成立；?Sized 允許 DST，'a 限制內借用。`&'a T` 已帶出特定 implied bound，省略該 + 'a 仍合法，卻不能推成其他 trait 都自動推導。引文：「也不會替原資料延長存活」。
Q2b（用途／需要）：Borrowed 從文字視圖擴為任意 T 的借用，要同時接納 DST 並維持 T／其借用來源有效；明示 + 的組合意義，讓兩項要求不被當二擇一。判斷含 borrow 的型別時看實際範圍，不誤以為一律不能用。
Q3：同一 owned String 得 Borrowed<String>、Borrowed<str>、Borrowed<&str> 三種 T；最後 value 是另借該 shared reference，* 只取 Copy reference，沒有拿到 String owner。三次讀取時 text 與 reference 都仍有效。
Q4：四列 T 表、struct 簽名及三個 main 輸出對應內層限制／外層 owner；實作不需另寫未知 bound，命令未實跑。
局部末：組合語法與隱含要求的範圍都有原文限定；當下理解不用第9章知識，沒有新增必要前提或 necessary／optional，原初判不回填。

原始SHA-256：8bfe361c46bc98e164545a0f56fa658726952fc9be29e385f39a14ddc778f774；visible SHA-256：8bfe361c46bc98e164545a0f56fa658726952fc9be29e385f39a14ddc778f774。

### 005-002 — 共用能力與異型別 Vec fixture 的證據範圍

原 v8 此單位 SHA：`3873e92abc365d56e167aea1a74182dc665794fa2187e5b56d1f244b5160e2bd`；原 optional O1 的初判與分級保持，不把本輪修後理解回填。
Q1：同款 Order／Bundle 各實作 Charge，卻仍是不同元素型別；enum KnownCharge 能以一種型別包不同已知資料，需修改 variant／match 才加新種類。
Q2a（機制）：Vec 每份清單只選一種 T，trait 共同 amount 不讓兩個 struct 自動同型。引文：「前面的 Charge 實作提供共同 amount 能力，仍不會把 Order 與 Bundle 變成同一種 T」。借 enum match 得 borrowed 欄位再呼叫既有 amount。
Q2b（用途／需要）：固定兩種费用可由 enum 加總；新增需要是 library 使用者自定費用型別而不改中心 enum。這項需要與既有計價分開，合理導向後續介面，不是為新語法換例子。
Q3：Order 算 unit_price×count，Bundle 交固定 price；直接 Vec 混放的完整 fixture 只定 structs／main，預期 E0308。前段 trait 實作與這份 fixture 的檢驗範圍現在各有明確句子。
Q4：三段可見程式分別提供共同能力、異型別 Vec 失敗、enum 已知種類可行；fixture 不含 Charge 實作，正文也已歸因為前文提供而非 fixture 自身證明，原 O1 的證據範圍疑慮在此局部釐清。
局部末：本段所需關係已在本文及本人實讀前文提供；只核當段，不宣稱重看新版 005-003 或全5節。未新增 necessary／optional，未實跑 E0308。

原始SHA-256：083aaff75867e28dd908489f696a2fa0f1dad388d9e5578dc1342f2f9e94e4d9；visible SHA-256：083aaff75867e28dd908489f696a2fa0f1dad388d9e5578dc1342f2f9e94e4d9。

### 005-004 — owned dyn 介面與更新圖的桌面／手機原位核對

原 v8 此單位 SHA：`aff6d000a8df4d72630621447e7cdb1c51d0067baf34fbc2228b2f99b5a46b00`；原 optional O2 原判保持，本次為更新圖的局部覆核。
Q1：同型 Box<dyn Charge> handles 各擁有原具體資料，資料與對應 method 表不同；total 借 slice 呼叫共同能力，不接手 Vec，也不把 Order／Bundle 改成同欄位 struct。
Q2a（機制）：Box 的 method 查找連到 dyn 介面，dynamic dispatch 依 handle 的 method 資訊／vtable 找具體實作及原資料；generic 的具體版本可 static dispatch，monomorphization 是代入實際 T。簽名／欄位的預設 object 'static 限內部型別短借用，不延長 Box 存活。引文：「箭頭只表示找到資料或 method，不表示 clone 或 move」。
Q2b（用途／需要）：在 Vec 一種元素型別內持有不同 Charge 實作，讓 caller 新增 Shipping 而不改 total／中心 enum；圖需讓讀者分辨找到資料與 method，而不誤讀為移動／複製或 ABI 承諾。
Q3：同批 enum／dyn 都200，caller 新 Shipping fee20，extended 接手原 Vec 再 push 得220；i32 未實作 Charge，fixture 預期 E0277，介面資格仍編譯時檢查。未實跑或重新核新版其他 dyn 單位。
Q4（本人親看兩張原位截圖）：desktop 1280×900、mobile 390×844 都完整看見雙層圖：values[0] 的同型 handle 下分資料箭頭→Order 80×1、method 箭頭→Order表／amount入口，下方結果80；values[1] 相同關係對到 Bundle price120、Bundle表／amount入口及結果120。兩幅的 method表文字與右側向下箭線分離，線沒有穿過該標籤；圖底「箭頭：找到資料／method」可見，原 O2 的文字穿線疑慮在此可見範圍已釐清。

實見限制：desktop 可見圖前說明與圖後 Shipping 段，圖本體完整；mobile 可見圖前說明與完整圖，圖後正文在截圖外，未聲稱看過。desktop 左側亦呈現本節其他小節導覽標題，這些標題已在原 v8 背景見過；只見導覽標題，沒有開其新版正文或連結。頁首／單頁圖位可見，但這是提供者指定的 fixed BODY 重組單頁截圖，不是我親跑正式完整頁或檢查整章桌面／手機。served SVG=v9 bytes 的來源確認由提供者說明，我沒有開其 render/report/source 或把該說明當自己的 probe。

v9 原位 SVG SHA：`87b6e9c27fa1626382a12875d04d56bab0c3436b4e5d15fbd75644d42eb5ccca`。
desktop PNG SHA：`4e918f1c8652b33a46c22ecb93eab64d430d697ca922adf7e75ec5a39e64e76b`；mobile PNG SHA：`134cc9582e1a3fea3109f60c27108b25cfc2bad987f1d22988b335933083c540`。
局部末：正文與圖皆保留相同資料→共同介面→caller 新型別→同 total 結果的關係；圖示位置改進沒有增新概念負擔。未新增 necessary／optional，沒有將受提示的圖核對算原初判重現。

原始SHA-256：1bce42c4673da782237e720bcbdb6f2452d86a96978afe051d3559d06a37527a；visible SHA-256：1bce42c4673da782237e720bcbdb6f2452d86a96978afe051d3559d06a37527a。

### 009-005 — 本地定義、有限 newtype 介面與泛型 orphan 順序

原 v8 此單位 SHA：`c4a86c84982f43c33f3fc57a2ff1b423df553804893ba9a805fbab8a56dcc0ec`。
檢查點 A：local／newtype。
Q1：local 指正在寫 impl 的 crate 有型別或 trait 的定義，alias／import 不搬定義；MeterBatch 新型別可為 owned Vec 提供自己的 Display，而不使標準 Vec 成本地。
Q2a（機制）：無泛型參數的 Display 實作需一方 local；alias CountList 仍外部 Vec，E0117。公有型別、私有內欄位靠 from_values 接手 Vec，count／total shared 借讀；自定 BatchSize local 可 impl 外部 Vec，example 只 import 使用該既有實作。
Q2b（用途／需要）：要顯示路程集合報告，同時避免多 library 任意衝突改外部契約；wrapper 只公開計算／報告方法，保持 caller 可依賴的介面有限。引文：「限制介面是這次選擇的目的」。
Q3：兩段80／120，Display 交2段共200，之後仍可借讀總量；沒實作 Deref 或 push，mutable 名稱也不取得 push，fixture 預期 E0599。Vec 被接手，沒有另 Box 或 clone。
Q4：alias 失敗、newtype／本地 trait 成功與 push 失敗三類程式，對應來源責任、owner 與介面範圍；未実跑。

檢查點 B：泛型檢查次序。
Q1：泛型外部 trait 除 local 還看 uncovered impl 參數的位置；into_inner 在自家 Wrapped 的 inherent 區塊，與替任意 T 新增外部 From 實作分開。
Q2a（機制）：先 self 型別、後 trait arguments；第一個 local 之前不能有 uncovered T。`From<T> for Wrapped<T>` 先見 local Self 合法，反向先見 T 才 Wrapped 故 E0210。引文：「不是說所有『外部 self 型別、本地 trait argument』都禁止」。
Q2b（用途／需要）：caller 要將任意 owned T 包起並取回，先提供合法 From／對應 Into，再用 into_inner 取回；不能把「有 Wrapped」當能為所有 T 定義反向 trait 的通行證。
Q3：80.into 取得 Wrapped<i32>，String 交 Wrapped::from 再 into_inner 移回 owner；反向 trait 不合法，inherent 取回仍可行。fundamental 的 Box<LocalType> 可算 local、Box<T> 不 cover T 是額外限定，不推成所有外殼都 cover。
Q4：兩種 impl 並列與順序解說足以追到差別；本段未用 fundamental 特例設新 impl，讀者不必自行猜其他包裝型別完整清單。
局部末：修後明寫的順序／具體非泛型可合法範圍，與原先教過的 from／newtype／有限介面保持同一關係；無新必要前提或 necessary／optional，未讀其他新版單位、完整 source／答案。

原始SHA-256：c4224fe19e605af81fbe7a61109a10a669babd1e7a3704e4fbf8fb3539de22d6；visible SHA-256：c4224fe19e605af81fbe7a61109a10a669babd1e7a3704e4fbf8fb3539de22d6。

## 指定局部覆核封存

六段依路線逐一存四題／Q2兩面、短引文與 raw／visible hash 後才開下一段；closed filter 始終先於輸出，本六段計數皆0。實際開图範圍為 005-004 原位更新 dyn SVG 對應的兩张指定截圖，其他图／頁／新版段落没有擴讀。

本輪未新增 necessary／optional。六段的標準名稱來源、bound 組合／推導範圍、異型别 fixture 歸因、同型 dyn handle 的兩種資訊與泛型 orphan 檢查順序都有可見依據。原 O1 的 fixture 證據歸因已在 005-002 釐清；原 O2 的 method表穿線在兩幅更新圖可見範圍已釐清。這是修後局部理解與看圖結果，不回填原分級，也不把其他角色已知問題算成自己的原初判。

桌面1280×900及手機390×844皆親看指定截圖：完整圖的四條資料／method 找尋箭頭、兩個資料框／method入口、80／120結果可辨，method表標籤沒有箭線穿字。手機只見圖前文字及全圖，圖後正文在截圖外；桌面另見側欄導覽標題，未開其他段正文。通過範圍限這個 fixed BODY 重組單頁的兩幅可見视圖，不宣稱正式網站整頁／其他尺寸、主題、互動或全章版面都已驗。沒有讀其 render／技術 report／source；提供者所述 served SVG byte 一致性非本人 probe。

original8 的 record／sidecar SHA 未變：`5061c8bfd8266d4328da4e1f9bc50541aa1b2830c9ef9a7c13e718573d1a4eb2`／`62408aa74cbbf1f34ff37696cb52845d8af01880cc98aa10c0a8841393450c1a`；既有 sidecar 88項全部相同。
original9 的 record／sidecar SHA 未變：`7feff159c1a6f23bc9e61c53bf0893fd261c7a624b11a39cbd7d32dc4e91740d`／`6b2dc0c526547bbeb893b97c94f9ac1e38d524cdab024136d9630d9fee8f7e32`；既有 sidecar 39項全部相同。原判、grades、恢复註記保持原樣。

只核六段；未讀 sourcearchives／peer／root disposition／作者紀錄／第9章 peer／後章，未實跑 Rust或命令。封存後停下，不擴其他原稿。
