# 第 3 章銜接交叉與 v6 局部覆核

審閱者：`chapter02_ownership`。日期：2026-10-14。這是原初判封存後、取得同行紀錄與 root 處置提示的交叉覆核，不是新的盲讀。沒有參與第 3 章撰寫；先前為後章作者讀過第 3 章的歷史仍存在。

## 原始紀錄與本輪可見範圍

原初判 `ch03-continuity-v3.md` 與兩個 hash sidecar 本輪都未修改。正文 SHA-256 仍為 `194ceda63552edf5d768bbb9f2e02e906114d5cc88f142a4d324f5423611b653`，相鄰 `.sha256` 與 JSON 宣告同一指紋。

本輪按 root 指示實讀以下五份紀錄：`ch03-first-reading-v3.md`、`ch03-technical-v3.md`、`ch03-technical-followup.md`、`ch03-disposition.md`、`ch03-first-reading-followup.md`。合併工具輸出曾截斷，之後分開補讀 first-reading 的 201–339 行及 technical 的 87–108 行；沒有將截斷部分算作已讀。

修後來源限定 `/workspace/work/tutorial-implementation/frozen/ch03-review-fixed-v6/`，manifest SHA-256 為 `45146f5eafd8d8b3af1cf74b5cb1f3cb7e173c9f04b26108a06d060c1f02ccc7`。實讀六個有修改的完整 units、Cargo manifest，並回讀受影響的相鄰 `004-002`／`004-005`／`005-003`／`006-001`／`006-005`。只看第 3 章末尾導覽，不擴到第 4 章正文。

本輪使用 Python 對照固定 sources 的 SHA-256、去掉 snapshot asset 路徑差異後的正文 diff，並以 TOML parser 比較原／新設定物件。32 個 units 與 37 個 archived sources 都符合 v6 manifest。正文實質變更只有下表六個單位；26 個其餘單位相同。來源只有 Cargo.toml 註解改變，其他 36 個 archived 檔案——包括全部 Rust、答案、tests、fixtures／expected 與兩張 SVG——的 bytes 都與 v3 相同。

本輪沒有執行 Rust、Clippy、fmt 或整套 tests，沒有開 v6 實際桌面／手機頁面，也沒有重跑 Markdown parser。原初判兩張 SVG 的親看證據保留原時點／範圍；圖 bytes 相同不等於這次已看新頁面位置。root 的 DOM／畫面檢查與技術角色的 parser 實驗各屬於他們，不能轉記成本人的親自執行。

## 發現來源、分級差異與裁定

以下以 C-N1／C-O1 表示我的原兩項，以 T-N1／T-I1 表示技術角色原兩項，避免同名 N1 混淆。

| 焦點／發現來源 | 自己原判與同行紀錄的關係 | 本輪核實的理解收益、負擔與裁定 |
| --- | --- | --- |
| C-N1：Clippy 設定缺少角色；我的獨立初判 | 我的初判列必要的小型完整 manifest 依賴。技術角色原先未報，交叉後同意；first-reader 原路線沒有讀完整 manifest，不能算它原先驗過這項 | TOML 的區段與 Clippy 名稱不足以推出 manual_map／allow 會關閉哪項提醒，以及為何保留 match。設定前四行註解補角色，沒有要求先學 map／closure。依完整參考包也遵守先備的範圍，維持必要分級；v6 局部已補足 |
| C-O1：char 的 `{}` 顯示；我的獨立初判 | 我的原分級可選；first-reader 原稿從下一單位的 code／輸出知道這項能力，技術角色原先也未另列。交叉後三者都沒有把它升為 Unicode 必要概念缺口 | 先說這個具體標準型別可顯示，再進第一個 println!，避免從首次 code 補出能力，與 String／&str 介紹方式一致。一句即可，沒有提前 Display trait；可選採納的收益足以抵銷小幅篇幅，v6 已在需要前補上 |
| T-N1：String 首次指派前尚無 non-Copy 契約；技術角色獨立初判 | **我原初判漏列首次使用時點**：只確認 `004-004` 說明 non-Copy，沒有把 `004-003` 已有的 `let moved = greeting` 列為較早的必要檢查。first-reader 原稿保留「本例未再用 greeting，尚未以例子證明舊名不可用」，也未列必要缺口；不能將此觀察回算成獨立報出 T-N1 | 同意技術角色的必要性。2.2 明說值是否 move 要看型別／操作；「有 ownership」不能推出 String 無 Copy，因為 i32 也有 ownership。首次指派已改變本節關心的管理者，需先給該型別的實際規則。前移既有一句並明指 greeting→moved 就足夠，沒有增加 Clone／derive／新 source。這是本輪受提示核實，不算我原獨立重現 |
| T-I1：Vec 的 heap 概括與空容器；技術角色獨立初判 | 我原判把元素管理／操作主線判足夠，沒有提出配置精確化。first-reader 未需要判配置成本，沒有報缺；技術角色列可選，root 採納後仍保留可選分級 | 同意可選。區分 Vec::new 的空容器與已配置元素空間，可接回 3.4 空 String 的模型，避免把 new 理解為先配置元素。限定本節 i32／String 的既有配置，不需 ZST／allocator；也沒有把任意長度零的 Vec 說成未配置。短限定有具體收益，v6 已達成 |
| R1：泛型文字被當 HTML tag；root 實頁／DOM 查漏 | 我的原視覺範圍只含 3.4／3.5 兩圖及相鄰文字，沒有看 3.6 的那些表格／段落。原文字來源理解可讀出型別，不代表實際 Markdown 畫面保留它；不宣稱本輪前曾獨立發現 | 保留型別參數對精確理解 owned 元素、String 元素與輸出 Vec<i32> 有用。只加 inline code 即恢復應呈現的詞，不引入新泛型概念、修改型別或計算。v6 source 標記已核對；實際 HTML 結果依 root／技術角色的已存證據，本人沒有另作 parser 或頁面驗證 |

以上是五項發現的覆核，不把必要與可選合併計數。我原獨立初判維持 C-N1、C-O1 兩項；T-N1／T-I1／R1 都是互看或 root 提示後才核實。接受別人的有效理由不改寫原初判，也不把後見同意計為自行發現。

## 修後單位、首次使用與過渡

| v6 單位／hash | 本輪實讀與局部判斷 |
| --- | --- |
| `004-003`：`3688dacfaff0c7ef1762b85649c6f1982fc19b025146a105679ded56335e3310` | 第一份完整程式前明說 String 沒有 Copy，最後指派交給 moved、不能再從 greeting 使用該 String；先前 show 則只 as_str 借用。原需要仍是接文字、顯示後追加，同一份資料上的借用與交接可由既有規則判斷。T-N1 在首次需要處補足 |
| `004-004`：`c7dfc1bf8c2e3b1d3d5cf4f2b1af43be22e22424588408c78309b0efe6f0cd27` | 開頭回收前段已說的指派，接著解釋 move 的 heap／清理責任，原 E0382／E0502 source 沒改。能力介紹前移後，後文仍提供失敗證據；沒有斷掉 &str Copy／最後使用／追加可能搬空間的關係 |
| `005-002`：`6b55ed58cd8f212671fab918c95d4fd96ecc1306c5c1b7ec97f65ca8272c47f1` | char 定義與 literal 後先給 `{}`，再接 scalar／字形區別、len 的 byte 單位及編碼表。O1 沒有把 char 稱為任意完整字形，也沒有改 UTF-8 分界。下一 unit 的 marker 顯示已有具體能力依據 |
| `006-002`：`4d9240a8052a9362bd501952e6f73e11b88d8e167f31e176f21293c2025cfec5` | Vec 的變長／同型／owned 角色不變；heap 句改成 i32／String 的已配置空間，new 不預配。pop 的 `Option<T>` 已以 code 標記保留型別參數。八項 method 與 slice.get 的契約仍在 code 前，沒有新 API 或篩選政策 |
| `006-003`：`6dedb66d897ebca64d5acdd758309023f344ef15978d19bac03e21ed1560a535` | `Vec<String>` inline code 恢復應呈現的元素型別；message→messages→taken 仍是同一份 String 的交接。pop 與 get 的 owned／borrowed 差別、Some 包 reference 不解除借用、不能依本次未搬空間繞過契約都維持 |
| `006-004`：`c92f6babe3fab5f0acbc966e732e378b4aa736ef0996ac3ccb618bb2d7b59846` | 入口把 `&[i32]`、`Vec<i32>` 作 code 保留；原需仍是保序保留非負數、原輸入不改。while 的合法索引→條件 push→position 前進沒有變，兩份原 tests 也沒有變。不把表示修正理解為新的篩選或效能收益 |
| Cargo.toml：`633e1d3caf2ea700e3d6cb4495696f93e054532907ec43243221721a9c5f5ed7` | 四行註解都在 table/key 前：先介紹 TOML # 註解，再說 table 選 Clippy、key 是提醒名稱、allow 單項不報以保留已教 match。first-reader 的指定補讀也能依此解讀。Python TOML parser 證明新／舊設定物件一致，沒有悄改政策；C-N1 局部關閉 |

相鄰過渡的覆核：

- `004-002` 先給 String／str／&str 身分與 ownership 角色，仍不暗示所有 owner 都不能 Copy；`004-003` 再補 String 的具體能力，安排正確。`004-005` 的 mutable output／shared label 練習不因前移受到新先備影響。
- `005-003` 接收 `&str` 以 get 取合法區段，marker 的 char 顯示現在有前置；其後「合法空段不是 None」「兩種失敗原因都成 None」仍不借可選補充代答核心分界。
- `006-001` 的不定項數需求先於新容器；`006-002` 的配置限定沒有改寫需要或 API 規則。`006-003` 將 pop／get 的差別落到 String 元素，再進 `006-004` 的 shared 輸入／owned 結果，角色連續。
- `006-005` 原非負 ceiling 約定、負數先略過、保序／重複／原資料仍維持。型別標記恢復不意味所有新泛型能力已教；末尾 trait 仍只是後續比較／複製／顯示契約的導覽。

## 判斷與尚未驗證

在上述局部範圍內，五項必要／可選處置都回應了原發現的具體關係，沒有藉補句偷用後章知識、擴寫無用途的記憶體細節或改變任務。沒有新待修依賴。其餘未改內容保留原初判已核範圍，不宣稱本輪重驗整章。

獨立首讀 v3 與其 v6 局部核對各自保留實際背景、未開答案、未跑命令與未看整頁的界線；與本人 actual 1v8／2v4 先備不是完全相同的角色實驗。它支持修後句子能被該讀者理解，不能代替 runtime／whole-site 或真人學習效果。

原初判與 sidecar 本輪未改；本 follow-up 另附局部證據與指紋。本角色完成這次交叉與局部覆核後停止，等待 root 另派。
