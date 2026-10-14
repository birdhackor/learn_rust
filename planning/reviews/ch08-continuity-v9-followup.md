# 第 8 章銜接交叉與局部修後覆核：v9

本輪核對五項可選處置、六個實質受影響 BODY 單位，未發現新增的必要缺口；採用的短句與圖標修正都有明確理解收益，沒有新增 Rust 機制或提高讀者入口。這是原初判封存後、已受同行與 root 處置提示的交叉及局部覆核，不是第二份獨立初判、盲讀或整章發布驗收。

## 版本、角色與原始紀錄

- 審閱者 `chapter02_ownership` 不是第 8 章作者；先前為 9.2–9.4 作者使用過實際第 8 章先備，也做過第 7 章銜接審閱。原初判已申明這項背景，本輪不改稱新的首次讀者。
- 原初判 `planning/reviews/ch08-continuity-v8.md` SHA-256 保持 `4fc7fb3691c31e4bcf6a6939a433eec120f2676a454ac4635bed0a8a94b99930`；原稿、SHA sidecars、`work/ch08-continuity-v8/sealed/` 與原逐段筆記未改。
- 本輪在 root 開放交叉後，完整讀取 FIRST8 的 `ch08-first-reading-v1.md`（689 行）、TECH8 的 `ch08-technical-v8.md`（161 行）與 `ch08-root-disposition.md`。只讀指定第 8 章同行原稿，沒有讀第 10／11 章同行紀錄或第 8 章後續 peer 答案。
- 修後唯一來源是 `/workspace/work/tutorial-implementation/frozen/ch08-review-fixed-v9`；manifest SHA-256 為 `92d0d7718b079bd60039245b41166a056b4a815ed653cb69d6bbd06a142b5978`。10 頁、66 BODY、150 sources；本輪逐一驗其 manifest 中的 chunk／source hash，但沒有把 hash 核對說成重新全文審閱全部材料。
- 原 v8 manifest SHA-256 是 `79036df9490cd5ad5874be7978062928ff185e1d25abc76adf610c2678eae9c1`。交叉輸入、修後全部檔案指紋、差異與執行證據另存 `work/ch08-continuity-v9-followup/`。

## 原始發現之間的關係

三份原初判各自確認的必要問題均為 0；本輪不回寫原分級。root 核實並採用的可選改善，按同一缺少關係合併後為五項：

| 缺少關係／位置 | 封存前的真正發現者 | 本人的交叉判斷與漏查範圍 |
| --- | --- | --- |
| `Sized`／`Box` 的標準 module 與 prelude 名稱來源，8.1 | TECH I1 | 本人原初判沒有提出。FIRST 在 001-003 曾記 module 定位未提供、操作不需另猜，沒有報同一可選改善；這項當時未知不能補算發現。TECH 的補充有助對照哪些標準名稱需 use，但角色／契約已足，原 optional 分級合理。 |
| mixed-Vec 獨立 fixture 與共同 Charge 能力的證據歸因，8.5 | FIRST O1 | 本人原初判沒有提出；TECH 也未列問題。本輪受提示後同意窄化歸因。原 fixture 只證明不同 T 不能直接混放，並未包含 Charge impl；前面的完整模型與既有 Vec 規則另支持共同能力不使 T 相同。這是證據表述改善，不是 dyn 理由未教或 source 錯誤。 |
| dyn 圖兩條右箭線穿過 method表標籤，8.5 | FIRST O2 與本人 CONT O3，各自封存前實見 | 是同一缺少關係的兩份獨立重現；FIRST 親看原位 native 圖，本人親看原位桌面／手機。TECH 只核來源、沒有實頁視覺，不能將其一般 source 通過算成視覺重現。兩個原判都可選，沒有升為主要概念阻礙。 |
| `+` 已在 7.6 教過，8.4 不宜稱本節新增 | 本人 CONT O1 | FIRST／TECH 對組合機制判充分，沒有提出首次歷史措辭問題。不是分級衝突：機制足夠與前文回指可更準確可以同時成立。 |
| binary 輸出「本crate」與實際 library 定義處不一致，8.9 | 本人 CONT O2 | FIRST 原讀已能由相鄰文字辨識 library 定義／example import；TECH 也核 local 規則合法，未報此輸出主詞。本人提出的是避免回找主體的可選改字，不是 orphan 必要依賴缺失。 |

本人原初判提出三個採納關係，其中 dyn 圖與 FIRST 同題；另外兩項是本人獨有。TECH 的名稱來源與 FIRST 的 fixture 歸因，是本人本輪才取得的同行發現，清楚記為原初判漏查的可選項，不作第二份獨立發現，也不改原必要 0。表格按實際關係記錄，不以位置相近或後來同意補算覆蓋。

原初判的 where 首次、短片段 imports、Box Error／Any Debug 候選仍保留否證：實際第 6 章沒有先教 where；8.5 在真正需要前教 where，完整 newtypes module 已逐條 use fmt／Add；完整 Box 結果 tests 用 match，不要求未教的錯誤 Debug。FIRST 亦在當下記 where、標準來源與 import 推論，TECH 核完整 source 的契約；這些支持原通過範圍，沒有因此新增修法或回填原閱讀紀錄。

## 修後範圍與六個單位

本人重新比對原／修後 manifest、全部 source hash、66 BODY bytes 與 `ch08-fix-scope-proof.json`。150 sources 中只有兩份改動：`dyn-dispatch.svg` 的兩個文字 x 座標，以及 `newtype_interface.rs` 的一句 println 文字；其餘 148 份 source 逐 byte 相同。8 份原位 assets 複本也與各自 archive SVG 相同。

13 個 BODY raw hash 改變。只正規化原／修後的凍結 asset-directory 前綴，文字本身有五個單位改動；005-004 的文字只有資產 URL 改動，但所引用 dyn SVG 改了，因此實質受影響單位合計六個。其餘七個只改 URL，所引用的 SVG bytes 未變：001-006、003-004、006-003、007-003、007-006、008-004、010-005。沒有把 005-004 因文字相同誤歸成完全沒有實質改動。

本輪全文回讀修後六個單位與其相鄰需要／使用過渡，共 17 個 BODY：001-001～006、004-001～003、005-001～005、009-004～006；另回讀 actual 5v7 的 004-002 prelude 與 actual 7v13 的 006-004 組合 bound。具體 source 回讀包含 newtypes.rs（含 tests）、修後 normal example 及 dyn SVG；其餘未改來源沿用本人已封存的原全文核閱，不宣稱本輪再讀全部 150 份。

| 修後單位 | 實際改動與收益／首次需要核對 | 本輪判定 |
| --- | --- | --- |
| 001-002 | 「來自 std::marker::Sized，也由5.4已教的 prelude 提供」放在 Sized 首次定義後、T:?Sized code 前。actual 5.4 已教 prelude 是預先匯入名稱；新增 path 是來源定位，沒有要求先會 marker 的其他機制。大小／DST／兩種大小查詢仍各有自己的角色。 | 採納收益明確，無新增必要先備；無需重跑未變範例。 |
| 001-005 | 「來自 std::boxed::Box，同樣由 prelude 提供」在 Box::new 首次操作前，連回剛補的名稱來源。原 Box ownership、借用、單一 pointer／非零資料限定與特殊 move-out 說明未刪。 | 同意兩句名稱補充，不擴成 API 匯入清單；清楚區分來源與機制。 |
| 004-002 | 「沿用7.6的bound組合符號+」取代「本節新增」。actual 7.6 已說 Into<OsString> + Clone 同時要求；這次真正新關係仍是大小與型別內 borrow 的組合、T:'a 與 implied bound。 | 解決本人 O1，程式／限定不變。 |
| 005-002 | 明說獨立 fixture 驗不同型別不能直接進同一 Vec；共同 amount 的 Charge 推論回指前段實作。後段仍承認 enum 能完成固定種類，dyn 的需要是 caller 擴充而不改 library 分支。 | 解決 FIRST O1 的歸因，沒有把原做法貶成不能加總，也未新增未教操作。 |
| 005-004 | SVG 兩個 method表標籤由 x200 移到 x160；兩條 x290 箭線、對應資料／表／80與120、vtable 示意及非 ABI 限定全部不變。正文與 caller Shipping 加20得220仍支持原設計需要。 | 解決 FIRST O2／CONT O3，實頁親看如下；不是改 dispatch 契約。 |
| 009-005 | normal 程式及正文預期第三行都改為「library定義的trait能實作在Vec上」。本地 BatchSize 仍在 library，example 只 use／呼叫；private Vec wrapper、借讀報告、local trait／外部型別及後續 uncovered 規則未變。 | 解決本人 O2，獨立實跑三行與修後正文逐 byte 相同。 |

五項都保留 optional 性質：原主要理解已有有效依據，最小修改讓名稱、證據主體、前文及線字關係更直接；收益不需以新節、更多例子或較高背景抵換。沒有新發現必要未教用法，也沒有把原可選項事後升級成原阻礙。

## 本人的局部實測與原位圖

**修後輸出。** 新 shell 先 source scripts/env.sh，指定 Rust 1.99.0／edition 2024；直接以凍結 library 編 rlib（不編 tests），再以 `--extern` 編修後 newtype_interface。兩次編譯均 0，正常執行 0；stdout 的三行文字／空白／換行與 009-005 的 text fence 完全相同。`normal-output-probe.json` 保存完整 argv、cwd、exit、stdout、stderr 與期待值。這是本人實跑，與 root 的相同結果分開；沒有跑 Cargo、全部 examples／tests 或 compile negatives。

**名稱來源。** 受 TECH I1 提示後實際開啟標準 prelude v1 頁面，只核 re-exports 的 `pub use crate::marker::Sized;` 與 `pub use crate::boxed::Box;`，確認兩個 path 與直接名稱來源。actual 5.4 先備已在 BODY；外部頁面只驗正確性，不替主文補必要角色。URL、原 HTML／SHA 與選段另存 official-prelude-selected.json。首次抽選程式因 HTML 連結兩側空白而斷言失敗，重讀已下載 bytes、調整比對後取得真段落；不是教材錯誤，也沒有為此重跑 Rust。

**修後 dyn 圖。** 由 v9 8.5 的八個凍結 BODY 按原順序重組獨立同 theme preview；只去掉重複 H1、改本地 asset URL，不取 live 改稿或同行視覺證據。Chromium 實載兩個 viewport 1280×1000／390×1000，保存 screenshot 並用 view_image 親看兩張；另外核 HTTP 實載 SVG 與 v9 archive bytes 相同。desktop 圖寬390、高690；mobile 約343×607，24px字縮後約21px。

在兩張實頁中，原先穿字的兩條右箭線都在 method表字右側，留出可見空隙；values[0] 的左箭到 Order 80×1、右箭到 Order表 amount入口，values[1] 相應到 Bundle price120／Bundle表，兩個結果80／120仍可追蹤。圖前的「找到資料或method，非clone／move／ABI」與圖後 caller 新增 Shipping 的過渡仍相鄰，沒有裁切必要 label 或依賴放大。兩個 PNG、capture.cjs、served-byte 檢查與 bbox 保存於 visual-metadata.json。

原 v8 的16張圖文畫面保留；另外七個 SVG bytes 與其正文（扣除凍結 URL）相同，原14張局部圖證據可繼續支持原已看範圍，本輪沒有重拍或重稱 v9 全部實頁親看。原 dyn 兩張則只保留為原問題證據，由本輪 v9 兩張支持修改後呈現。局部 preview strict=false／build無 issues 只證明這個 preview 可建，沒有宣稱正式站點 strict build、完整 navigation／downloads／fold 或發布行為通過。

## 限制、封存與停止

本輪沒有修改教材、程式、根 Cargo／lock／nav 或其他作者檔。沒有重跑完整 workspace checks／Clippy／fmt、所有正常／負例、跨平台或真人讀者測試，也沒有擴到第 9 章作者工作或第 10／11 章審閱。root 的候選發布 checks 與其他角色的修後閱讀仍各有獨立範圍。

本文件另存 SHA sidecars 與 immutable 後續副本；封存前再核原 initial SHA 不變、v9 全部指紋未變。原 initial 的判斷與原始發現時點完整保留。本輪交叉／局部覆核到此停止，等待 root 下一項明確任務。
