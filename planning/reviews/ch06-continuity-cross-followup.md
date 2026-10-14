# 第 6 章銜接交叉覆核與 v10 局部複查

## 原判保留與本輪範圍

角色仍為 `chapter04_traits`，沒有參與第 6 章寫作，但之前為第 4、5 章讀過本章片段。本輪是在 ORIGINAL 初判封存、協調者提供同行材料之後進行的交叉與修後複查，**不是新首讀、盲讀或新的全章驗收**。

原判 `planning/reviews/ch06-continuity-v6.md` 的 SHA-256 仍是 `97ad53ce4eda1b447ac3226556b7abda8b887e178c89bc51bc73a0848cf8cf9f`；原 `.sha256`、`.initial.json`、work 初判副本及原 source 指紋未改。本輪另存此檔，不把同行提示、後續處置或修稿回寫到初判。

實際修後來源為 `/workspace/work/tutorial-implementation/frozen/ch06-review-fixed-v10`，manifest SHA-256 為 `d43db1810524ae6858774b65594eae53c367305bbb50d30ca6739dd928e567f6`。逐行讀完修後 `003-002`、`004-002`、`006-002`、`006-004` 四個單位；再局部回讀下列既有方法與圖位。前文沿用原判的指定 actual 1–5，沒有用 live 或未來課綱當先備。

另核對全部 50 個 chunk 的 manifest 指紋，並比對 v6/v10 的完整 archive：**80 份來源全部位元組一致**，包含 39 個 example／答案、7 份 library、21 個 test 所在 source、15 個負例及 15 個 expected、Cargo 和 3 張 SVG。三份 frozen asset 副本也與相應 archived SVG 一致。正文扣除凍結根目錄造成的 image URL 差異後，僅上述四單位有變動。證據存於 `/workspace/work/tutorial-implementation/reviewer-ch06/continuity-v10/v10-local-checks.json`；這是差異與版本核對，不是重新執行 39 個 example 或重讀全部修後正文。

## 本輪才可見的交叉材料

已完整讀以下材料；這些是交叉提示來源，不能算成自己的原始發現：

| 材料 | 本輪讀到的 SHA-256 | 角色與證據界線 |
| --- | --- | --- |
| `ch06-first-reading-v1.md` | `5f2d9b4bef1a9a7c2dc59735f5ade18f2b76989e52f99c6fc39d318180b87bda` | AI 首讀模擬按 50 units 留下當時已知／未知；未讀外連答案。三圖曾自行 render 獨立 PNG，但不是教材實頁 |
| `ch06-technical-v6.md` | `ecfc097d2a6f0e006cd1740c3da4ee3addd53ec6a93838a79a8bfe42edaecb55` | 非作者技術初判讀完整 archive；部分正式契約及相稱 probe。其實跑是該角色證據，不算成本角色已執行 |
| `ch06-disposition.md` | 首次讀為 `390ed68bf3d73dd0f5a2d56d05d646075afb6071eca727ee22ec59a14cb5a917`；歸屬列更正後為 `c6c7dd3d5d72d1b9b78ed31218f4162e1c32f6350a02cc10805a90466501c3a4` | 協調者的採納、最小負擔與修訂範圍，不能代替修後正文閱讀 |

處置表第一次讀到時，collect 列仍留「first 與 CONT 未原始發現」的早期措辭，與該檔末段和自己的原 N2 不一致。已告知協調者；對方只更正維護列為首讀未抓到、CONT04 已獨立重現。本輪回讀更正列核實，不把維護歸屬更正列成學生正文的新缺口，也不改三角色原判。

## 原判與同行的判斷差異

| 原問題／來源 | 交叉判斷、收益與必要性 | 原始發現狀態保留 |
| --- | --- | --- |
| Fn 系列名稱 scope；首讀必要、本角色 N1 | 同意必要：已有 trait 能力說明，卻沒有這三個名稱的可見來源；不是認為 Fn 機制全未教。補 std::ops/prelude 與既有 use 規則即可，不列全 prelude | 首讀與本角色各自從原稿重現；技術初判沒有另列此項。同行同意不回填到技術的原判 |
| Vec Debug 晚於 collect fixture；技術 N1、本角色 N2 | 同意必要：錯誤 fixture 仍須隔離預定失敗機制，與之無關的整份 Vec 顯示不能靠後文補教；前移原段即可 | 技術與本角色各自重現；首讀在同單位後段讀到 Vec 契約後，原紀錄未抓到首次使用位置，仍保持未抓到 |
| evaluate 一次卻選 Fn；首讀可選 | 提示後回讀可重現：一次呼叫本身只需 FnOnce；此對照的 Fn 合法，但原文未說為何選更強的介面。可選補充能防止讀者把強 bound 誤認為一次工作的最小要求 | **本角色原判沒有報此項**；這次因首讀提示重現，不改原表的通過紀錄或發現者 |
| adaptor「不是複製原 iterator」；技術可選 I1 | 提示後回讀可重現概括措辭的界線：self 按值仍依 Copy／move；不主動 clone 不是所有來源都不可能 Copy。本節具體 slice 路線、lazy 範例及其結果仍成立，所以保留可選級別 | **本角色原判沒有報此項**，首讀亦未報。技術的 Copy iterator probe 是支持一般性限定的材料，不是學生必須新增的練習或本角色新實驗 |
| Vec FromIterator 先說一般 T；本角色可選 I1 | 保留可選：已教泛型 Vec、Item/FromIterator，加上 6.7 當下來源型別，足以理解具體轉換；一般形式減少「只支援 i32？」疑問，不需要為 char/reference/tuple 各重教一次 collect | 本角色原 I1，協調者採納；沒有把措辭改善事後改判必要 |

對首讀「未知」也按當時位置區分：`001-002` 尚未分 capture 方式，是後面 shared/mutable/move 的合理分段；`005-002` 尚未說 Iterator 的名稱來源，在第一份具體 impl 前的 `005-003` 明示後解除。這兩者不等同於直到章末仍未補足的 Fn 名稱來源；不用把每個預告未來規則的名詞都移成當場完整教學。

## v10 四單位首次使用複查

| 修後位置與指紋 | 實際順序、限制與判定 |
| --- | --- |
| `003-002.md:9`，`41c8d19787adfb2a3ae4024a846617a7b03924de259f042f6fdb36b033378bad` | std::ops 與 prelude 說明在第 16 行第一份 FnOnce bound 前；並接回自訂 trait 的 use。FnMut/Fn 此處只補名稱來源，能力仍在其原來的子節依需要教，沒有提前用未教的呼叫機制。**原 N1 局部通過** |
| `004-002.md:11`，`b146cb9214d7829455247dc3cc5920e322f0389e981d88b832a7a2f47857ae16` | 在第 14 行 evaluate source 前說這組對照共用 Fn，聚焦參數／回傳由誰選型別，並明說單獨一次需求 FnOnce 足夠。借用 6.3 已教的能力包含關係，不添新語法或更改 source。首讀可選項採納後局部通過 |
| `006-002.md:5`，`a4f30717a33e9cc2177d96f3cca703521b7e1b2d8a073eeeed57c815cf6a8613` | 在第一份 map 範例前改成按值接收、不主動 clone，並依 Copy／move。這不是說每種 iterator 有 Copy；原值是否仍可用仍看型別。逐筆、惰性與不先建完整清單的機制接下來分開說明，沒有用 ownership 措辭替代 lazy 的證據。技術可選項採納後局部通過 |
| `006-004.md:7/32`，`be52997350e070801727535bc54be1f36afd7644d13228d715f20745a98fd526` | 第 7 行先給 Vec<T> 接 T 的一般契約，再以 i32 回到 helper；第 32 行 Vec Debug/PartialEq 已在第 40 行 fixture 顯示之前，後面正常顯示、比較及自訂目的示例也保留依據。**原 N2 局部通過，原 I1 採納有效** |

`006-004` 的能力段位於型別流向表與原圖之後、負例之前，沒有讓讀者先用整份 Vec 顯示再回頭找契約。前移內容沒有打斷 filter/map/collect 的資料推進；自訂 FromIterator 仍在讀者知道從 i32 建立新目的的需要後引入。完整 fixture 仍與原 source 相同，這個修法修的是説明時序，不改 E0277 所指的 Item／目的不匹配。

## 抽查原未列問題的方法通過項

除四個變動單位外，本輪在 v10 實讀以下既有單位，核對原本通過的理由，沒有以「其他角色沒報問題」代替自己的正文判斷。

| v10 原未變正文 | 主問題、機制與材料界線 |
| --- | --- |
| `002-003/002-006` associated type 與 equality bound | 先有不同資料要交出自己內容的需要，self 接手是為 String/non-Copy；型別槽不是值欄位。再因乘二需要 i32 才限 Output，bool 失敗對照說明不能改 bound 卻仍假定算術。材料支持這些關係，不證 trait 自動正確或 caller 可改 impl 選型別 |
| `005-007` StepDown 練習 | 先 next 取得 5，再把同一份交 for，3/1 合 4直接檢查目前進度。5 與零的材料可由已教 state/next/IntoIterator 推出；原答案與 source 全未變，沒有提前依賴 fold 或 Entry |
| `006-006` Result collect | 先指定整批全成功／首次失敗的政策，再說 Ok、Err、空輸入及不轉 E。callback 計次放在 parse 前，visits=2 與來源 3 筆支持第三筆沒有執行，不只是印出一個 Err。這不能證所有 iterator 都讀完或 collect 會保留成功前綴；正文已明確排除這些推論 |
| `007-007` 完整報表排序 | 原規則就是 count 降序、name 升序；左右數值例與表在 code 前說明反向 cmp。stable 保任意來源順序不能滿足 deterministic 報表，第二鍵補足其需要；owned 名稱來自較早的 to_owned 再移交，不由 collect 推出複製原文字 |

這些判斷與首讀/技術的通過項相容；支持它們的是各單位的需求、角色與例子，而不是同行的結論。原完整 sources、答案及設定的核對可沿用已封存 v6：此輪證明其位元組未變，但沒有冒稱重新執行 test、重新核全部工具輸出或再做一次全章首次閱讀。

## 三圖的必要對應與未驗界線

本輪從 v10 實讀 `001-007`、`005-003`、`006-004` 的原位正文、圖說，並解析三份 frozen SVG 的文字節點與箭頭 source；hash 與原判一致。

| 圖與原位 | 這次核對到的具體對象／動作 | 判定界線 |
| --- | --- | --- |
| closure-transfer／`001-007` | text 持有文字→建立 take 按值保存→呼叫 take body 移出→saved 接手；兩條箭頭各對一個時點 | 同一條文字用箭頭與標籤追蹤，不是三個 owner 同時存在或三份 clone。相鄰失敗 code 的第二次呼叫仍由 non-Copy 移出解釋 |
| iterator-next／`005-003` | 同一份 remaining 3→2→1→0→0，依次交 Some(3/2/1)/None；state 留來源，Item 交 caller | 不是四份獨立來源；這個有限倒數持續 None 不被圖說泛化成全部 Iterator 永久結束。first None 停 for 的提示與正文一致 |
| adaptor-pipeline／`006-004` | slice Item=&i32→filter callback &&i32／保留原 Item→map &i32 取 Copy 回 i32→collect Vec<i32>；固定小材料 [0,30,30] | 明確真才往下及 collect next 驅動。型別圖沒有畫完整中間 Vec；新增一般 Vec<T> 契約仍允許這個具體 i32 例子，前移 Debug 段不改資料流 |

三圖 source 的必要角色與原判保持一致，未新增靠圖才能理解但未說明的關係。**本輪未渲染或看 PNG，未看桌面／手機教材實頁，也未核 DOM、縮放、字體或碰撞**。首讀角色的獨立 PNG 與技術角色的 probe 僅按來源註記，不算成本角色做過的驗證。

## 本輪結論

原 N1/N2 的首次使用缺口在 v10 的必要位置已補足；三項可選採納都連回已教能力，沒有改程式或增添新範例。局部過渡、抽查方法與三圖 source 未查到新的必要銜接問題。此結論限原 v6 全來源初判加本輪明列的 v10 差異與局部複查，不宣稱已完成網站呈現驗收或新盲讀。本輪完成後停止，等待協調者安排後續任務。
