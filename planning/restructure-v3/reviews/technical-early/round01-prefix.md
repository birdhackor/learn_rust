# 1.1–1.6 獨立技術初判

本輪未發現必要技術缺陷、需要修正的範例載入缺口或另外提出的可選項。結論僅涵蓋 `01-01-lesson.md` 至 `01-06-lesson.md` 與其實際選取的 sources、manifests、指令；不代表後續章節、全站整合或目標讀者實驗已通過。

## 獨立性與方法

- 角色是未參與撰寫的技術審閱者。依序完整重讀六頁，檢查已讀前文與實際引用原始碼，不是逐段盲讀。
- 封存前未看歷史問題／審閱、作者新 ledger、root dispositions 或首次閱讀紀錄；未讀 1.7、1.8 或後續新章節正文／範例內容。檔名列表中可見後續 target 的名稱，未把它們當先備。
- 已讀專案 `AGENTS.md`、clear-tutorial skill、術語指引、審閱流程及通用校準例。讀者背景維持基本 Python，沒有提高為會 Rust、C 或 C++。
- 來源版本由同名 JSON 的 SHA-256 封存。未修改教材、Rust sources、manifests、Git 狀態或格式；僅執行下述成功 targets 並寫入這兩份報告。

## 逐頁證據

| 頁面／焦點 | 原文與 source 證據 | 技術與首次必要使用判定 |
| --- | --- | --- |
| 1.1 工具與安裝 | 正文 13–19 行分開 `rustup`、`rustc`、`cargo`；27–54 行說明安裝 shell、linker 與各 OS 工具；68 行的 1.99.0／edition 2024 對應 `rust-toolchain.toml:2`。 | 角色、目前用途與下一步工作皆已交代。編譯通過不保證邏輯正確在 5 行有明確界線，沒有把後面的測試假定為已會。安裝流程僅靜態核對，未在新 OS 實做。 |
| 1.2 最小程式 | 16–18 行在完整程式前說明 `fn main()`、大括號、`println!`、macro、字串 literal 與分號。`hello/src/main.rs:1` 只有這些內容。56–64 行逐一解釋完整 manifest 的 name/version/edition，空 dependencies 可省略。 | 最小完整程式沒有隱藏 helper、library 或 dependency。`cargo check`／`build`／`run` 的差別與 target/debug 的預設角色正確；`cargo new` 先產生可用 manifest，因此前段執行不需要未介紹的手寫設定。 |
| 1.3 選 package／crate／workspace／target | 正文 15 行區分 package 與 crate；37–45 行建立 workspace 共管與 resolver 的角色；67–79 行以既有 two_lines 引入 example target；93 行明說沒有 lib；97–105 行分清 lock 與 Rust source。 | 三個 package 的完整 manifests 只有已教 name/version/edition；workspace 只有 members/resolver。所有實際選取的成功 targets 是各自一個 `.rs` 的 binary crate，沒有 `src/lib.rs`、build.rs 或 `mod`／`include!` 載入別的自有程式碼。其他同 package 未選 examples 不因此成為先備。`-p`、`--example`、`--manifest-path`、`--locked` 的操作角色都在相應命令前說明。 |
| 1.4 型別與運算 | 7–17 行交代 let/binding/型別/推導/格式擷取；21–31 行交代 `/`、`%`、浮點表示、同型別與優先順序；35–47 行交代 u8/u32/bool/char、fallback 型別及溢位界線。`types.rs:3–23`、`types_answer.rs:2–9` 與輸出一致。 | i32/u8/u32 範圍、整數除法朝零截斷、7/2=3、-7/2=-3、浮點近似、i32/f64 不直接混加與編譯時型別敘述正確。bit 的記憶體關係留到 1.7，沒有在本頁把未教的儲存布局用來推理。 |
| 1.5 mutable／shadowing | 正文 7、17、25、33–43 行分別說明 immutable、mut 更新、型別不變、建立新 binding。`mutability.rs:3–14` 與 `mutability_answer.rs:2–9` 只使用既有 i32/f64 與已說明的新寫法。 | 對這些單純 i32 變數，`+=`／`-=` 的縮寫敘述、重新指派不能換型別、shadow initializer 讀舊 binding 與新 binding 可以換型別均正確。兩個 compile-fail snippets 是獨立完整 `main`，所指失敗原因正確；未執行診斷。 |
| 1.6 函式／expression／unit／block／scope | 7–17 行先給參數與回傳、expression/statement/block 的意義；21–37 行示範按位置傳入與 unit；58–66 行說明誤加分號；76–86 行說明 block 值與名稱 scope；118 行承諾答案不依賴其他函式檔案。 | functions、blocks、functions_answer 都能只靠同頁／前文與該單一檔案理解。參數型別、函式尾端 expression、unit、內層讀外層、新 shadow 不改外層與 item_total 的名稱範圍都與 source 一致。`functions_answer.rs` 完整定義 subtotal/with_shipping/main，沒有漏掉被呼叫的函式。 |

## 主要概念四題的技術核對

以下是技術重讀的證據，不冒充首次讀者當時的答案。

| 焦點 | 身分／銜接 | 機制／性質 | 需要／用途 | 操作、例子與結論邊界 |
| --- | --- | --- | --- | --- |
| Cargo 的 package／workspace／target | 1.2 的單一 package 延伸為 1.3 多個共管成員與獨立成果。 | manifests 選成員；每個 main 是獨立 binary crate；workspace 不把程式合成一個。 | 1.3:3 的「今天只跑這一個」與保留早期階段，直接由 -p／--example 回應。 | 兩個 package 有同名 two_lines，-p 消除歧義；沒有把 workspace 解析 metadata 誤說成編譯所有未選 target。 |
| 數值型別與推導 | 1.4:7–17 把材料名稱、值與可做運算連起來。 | 明寫 i32 後乘法結果推導 i32；整數除法與浮點除法各自決定結果型別。 | 同頁訂單要得到 360；除法先問要整數還是小數，提供獨立選擇需要。 | 原始材料、算式與顯示值一一對應；未承諾任意金額或任意浮點運算都精確。 |
| mut 與 shadowing | 1.5:3 先給更新同一狀態／另存下一階段的兩種安排。 | mut 重新指派原 binding；let 建新 binding，不修改舊 binding。 | 1.5:43 把兩種寫法分別接回累計與下一階段保持不變，沒有只說其中一種比較好。 | 360→380、3→4→f64 4.0 及答案 180/170/340 支持本次操作，未由顯示的 4 反推整數型別。 |
| 函式與尾端值 | 1.6:3 給重複訂單規則；7–29 行說明參數、結果、block 與 unit。 | 呼叫按位置提供材料；尾端 expression 交回值；分號後無尾端值則 unit。 | 同頁重複乘法若改規則需找多處，命名函式把規則保存在一處；答案另把運費規則集中。 | 兩筆非相同訂單實際呼叫同一定義；show_total 只輸出；預期失敗的 source 承諾 i32 卻交回 unit。 |
| block／scope | 1.6:70–86 由內部小計交回含運總額，再觀察名稱可用範圍。 | 尾端值可帶到 total；item_total 名稱不能越過內部 block；內層 shadow 結束後恢復外層名稱解析。 | 用內部命名小計組織工作，同時保留外層接收的結果；沒有把 scope 等同儲存位置。 | 380/4/3 與 source 一致；104 行明說不能僅從名稱可用性推定記憶體，1.7 的實體模型仍待教而未被偷用。 |

## 實際執行

經 root 確認當時 root／作者未使用 Cargo，並暫停新增章 2 workspace 成員後，只做一次序列化成功 target 執行。每個 shell 先 `source scripts/env.sh`，使用 `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_BUILD_JOBS=2`。未執行 `--all-targets`、未選後續範例或任何 fixture。

共 12 個 `cargo run` 命令全部 exit 0，stdout 與正文一致：hello 的 member-manifest 命令、greeting 預設入口、hello/two_lines（原命令及 --locked）、greeting/two_lines，以及 steps 的 types、types_answer、mutability、mutability_answer、functions、blocks、functions_answer。完整 argv 與實際 stdout 存在同名 JSON。建置沿用既有 cache；此結果佐證指令與輸出，選取 target 的完整 source 邊界另由 manifests／source 靜態核對。

## 限制與未驗證

- 沒有在新 OS 安裝 Rust，沒有另建 repo 外 cargo new 練習專案，沒有獨立執行 cargo check/build、fmt、Clippy、tests 或預期失敗診斷。四份失敗 source 的錯誤原因是靜態判斷，不能當成已記錄的編譯器實測。
- 沒有實際瀏覽器／桌面與手機版面驗證，也沒有渲染 snippet 展開。六頁沒有必要圖檔；現有短程式與名稱／數值對應可由文字直接追蹤，未發現必須增加圖才能完成當前技術說明的缺口。
- `zensical.toml` 僅用來辨認 snippets 設定與目前整合狀態；新 lesson 檔名的導覽切換屬作者後續整合，不在本輪宣布驗收。
- 程式可以執行不代表已證明初學讀者順暢；本輪沒有查看其他角色紀錄，亦不代替獨立首讀與銜接裁定。
