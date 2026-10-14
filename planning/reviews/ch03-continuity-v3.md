# 第 3 章銜接審閱：ch03-prerequisite-v3 初判

審閱者：`chapter02_ownership`。日期：2026-10-14。角色是獨立於第 3 章作者的銜接審閱，不是無歷史的首次盲讀。我曾為撰寫 4.4–4.5／第 5 章實讀過第 3 章，且我是第 2 章作者；這些經驗可能降低我自行發現初學者負擔的敏感度。本輪沒有閱讀第 3 章作者 ledger／source-notes、初讀紀錄、其他技術 review 或 root 的問題清單。

本檔是交叉覆核前的來源初判。只有這份初判可以支持獨立發現；後續交叉覆核另存，不回寫本檔。來源、審閱指引、畫面證據的指紋與本檔 SHA-256 保存於相鄰 JSON sidecar。

## 範圍與證據界線

- 唯一被審正文：`/workspace/work/tutorial-implementation/frozen/ch03-prerequisite-v3/`，依 reading-order 讀完六頁、32 units。37 個 archived source 含六份答案、完整 library／normal examples、七個 intentional failures 與 expected、兩張 SVG、Cargo manifest；所有 unit/source SHA-256 均與 manifest 相符。
- 必要前文：actual `ch01-complete-source-v8`／`ch02-complete-source-v4`。本輪實際回讀使用到的數值推導、tuple／array／range、if expression、return／unit、module／use／tests、struct／field shorthand、Copy／move、reference／receiver、slice／最後使用規則。沒有把後章能力當成先備。
- 先走完整正文路線，再讀 source archive 的完整答案／tests／fixtures。正文的延伸連結不作必要先備；本輪沒有開外部教材來補正文缺口。
- 本輪沒有改正文或程式，也沒有盲跑整套 tests。這份紀錄不宣稱重新驗證所有正常／失敗 target 的編譯與輸出；型別、借用、邊界與測試用途以來源銜接核對。
- 實際 Chromium 畫面只檢查 3.4／3.5 兩張圖及相鄰文字，1280×844／390×844。圖片 hash 與凍結原稿一致，圖前後文字也對得上凍結 units。當前 repo 3.4／3.6 的整頁 raw hash 已不同於原凍結稿；沒有讀它們的差異、沒有把這次畫面說成新版整頁原稿首讀。3.5 的 raw hash 仍相同。

## 重建的實際依賴與首用位置

下表由正文重建。unit 編號均指本輪凍結包；`1v8`／`2v4` 表示上述前文凍結版本。正文完整規則與範例一起核對，名稱曾出現本身不算已教。

| 位置／第一次需要的能力 | 原文或前文的實際介紹 | 當下要能解釋／預測的關係 | 初判 |
| --- | --- | --- | --- |
| 001-001：缺值與零 | 「把沒有報價寫成 0，就無法區分」 | 原需是保留不同情況，不是先要求讀者想使用 enum | 足夠 |
| 001-002：enum／variant／位置資料與命名 field | 三種形式表＋「一個 enum 值…只屬於其中一種」；struct 已在 2v4/001-002 | 一筆 Quote 同時只有某一種形態，不會同時有 Amount 與 Rejected 資料 | 足夠 |
| 001-003：pattern／arm／match 順序與完整性 | 在完整 code 前逐項解釋 pattern、=>、第一個符合分支、所有情況 | 建構值與解構形狀相似、角色不同；不執行三個 arms | 足夠 |
| 001-003：命名 pattern shorthand／_／literal pattern | 本 unit 明說 `{ code }`、_ 不綁定；1v8 的 literal／2v4 field 先備 | field 名稱與取出的值對應；其餘分支應放最後 | 足夠 |
| 001-003：match expression 與 E0004 | 「每個 arm 都交出 bool」連回 1.6；missing_variant fixture | 分支型別一致與涵蓋所有情況是不同條件，不能亂補假數字 | 足夠 |
| 001-004：對 &Quote 做 pattern 的借用解構 | code 前明說 amount/code 為 &i32，* 讀數字；2v4/003、002-004 | repeated call 不接手 Quote；Copy 數字不代表 Quote 自動 Copy | 足夠 |
| 001-005：if let／let else／名稱可用範圍 | code 前定義條件、略過、else 必須離開流程；return 先備 1.5 | if let 名稱只在 body；let else 成功名稱留後續，Missing 不算運費 | 足夠 |
| 001-006：Shipment exercise／numeric assertion | 題目明說 Waiting 的 0 是統計規則；答案只 assert 回傳數字 | 資料仍保留 Waiting，不把 Missing 等同真實零資料 | 足夠 |
| 002-001：Option 與 Result 的不同工作 | 用「有答案或無答案」／「成功或保存原因」銜接 Quote | 此刻先操作具體數字，不需尚未教的型別參數語法 | 足夠 |
| 002-002：Some／None 名稱與推導 | 「自動帶入…不需要…Quote::」；if 的 Some 提供型別線索 | Some 包數字，不是可直接乘的數字；None 無 payload | 足夠 |
| 002-003：Ok／Err／DivideError | code 前保存成功商或具體零除數原因；先檢查才除 | Err 是資料，不會自動停止；失敗沒有假成功數字 | 足夠 |
| 002-004：選 Option 或 Result | 資訊需要表＋「None 不會告訴…」 | 接收者需要原因時，不能把 Err 改 None 或省掉分支 | 足夠 |
| 002-005：零／負數 exercise | 0、−5 的判準先給；答案不使用完整 Option／Result signature | 成功零與失敗不同；完整型別仍等 3.3 | 足夠 |
| 003-001／002：泛型 T、值參數與型別參數 | 「型別參數則代表 i32、bool」；Holder<T>／hold<T> 的位置先教 | 保留相同資料形狀，替換具體型別；不能執行期任意改盒子型別 | 足夠 |
| 003-002／003：generic function／Holder shorthand | Hold 只保存 T，不假設算術／顯示；2v4/005-004 已教同名 field 簡寫 | body 對未知 T 沒偷用能力；具體欄位 i32/bool 才能用已知操作 | 足夠 |
| 003-003：推導、Holder<i32>、turbofish | 在 code 前解釋 `hold::<i32>(80)` 與兩種括號 | 角括號放型別、小括號放值；推導不是放棄型別檢查 | 足夠 |
| 003-004：同一 T／Pair<T,U>／E0308 | 同 T 的錯例＋兩參數逗號規則 | 相同 T 約束兩欄；不同角色可改兩個參數，不靠任意執行期混型別 | 足夠 |
| 003-005：Option<T>／Result<T,E>／None 註記 | 首次完整 signature 前各解釋 T、E／兩個具體型別 | 成功／錯誤型別由 signature 公布；None 無內容時需線索 | 足夠 |
| 003-006：two-parameter exercise | 題目先指定各欄角色；完整答案 Pair<T,U> 只保存值 | generic syntax 不借用未教 bound、derive、Clone、String | 足夠 |
| 004-001／002：String／str／&str／UTF-8 | 動機是接問候語、顯示後追加；bytes/scalar 後續深入 | owned 文字管理與 shared 借用不同；literal 是 &str，不是另一個 String | 足夠 |
| 004-003：new／push_str／as_str 三份契約 | 表明參數、結果、mutable/shared 與原 owner；2v4/005-002 receiver | new 無 receiver；push_str 拷入片段不接手片段；as_str 不拷內容 | 足夠 |
| 004-003：String／&str 的 {}、heap 管理、圖意 | code 前明說顯示內容；空 String 可未配置；模型不是 layout | 管理箭頭與借用箭頭指向同一文字；函式借用用完後才能追加 | 足夠 |
| 004-004：String non-Copy move／&str Copy／E0382 | 「沒有 Copy」「複製的是位置與長度」；2v4/002-004／006-003 | move 交責任不要求再拷 heap；shared ref Copy 不產生獨立文字 | 足夠 |
| 004-004：as_str 借用與 push_str 衝突／E0502 | 先保存 view、修改、最後讀；2v4/004 的最後使用範圍 | 不因曾建立 reference 就 immutable 到 scope 結束；不能依某次追加沒搬空間繞契約 | 足夠 |
| 004-005：&mut String／&str helper exercise | 參數目的先給；答案只用 push_str／new／as_str | 原 output 被改，label 未被接手；沒有新 output 取代原資料 | 足夠 |
| 005-001／002：Unicode scalar／char／byte／字形 | 先說 scalar 1–4 bytes／char literal；結合重音反例 | 顯示字形不等同 char，不把 len 當 char 數；char 的 {} 可補明，見 O1 | 主線足夠，O1 可選 |
| 005-002：str.len／具體 UTF-8 資料／range 分界 | len 明說 bytes/usize；A、中、crab 表＋圖；1v8/007、2v4/006 range | 0、1、4、8 是合法端點；結尾 8 不是最後 byte 索引 7 | 足夠 |
| 005-003：str.get(range)／Option<&str>／空段 | code 前列三條合法條件及 8..8／2..2；顯式 as_str | Some 是借來的文字；None 沒有原因分類；空長度不等同成功 | 足夠 |
| 005-004：直接 range indexing 與 panic | 索引語法與相同合法條件先說，再給 runtime fixture | &str range 取值與 get 不同失败政策；不假設單整數索引代表 char | 足夠 |
| 005-005：Option<usize> helper／text tests／library | 函式回傳數字目的、pub mod/use 回收1.9；text.rs 完整答案 | Some(0) 與 None 保持不同；tests 用 match/數字/bool，不靠未教 Option 比較 | 足夠 |
| 006-001／002：Vec<T>／new／vec! | 泛型已在3.3；macro 在1.2；本節先給建立與型別線索 | 長度不在型別上，元素同 T；已有 array/slice 未被抹去 | 足夠 |
| 006-002：push／len／pop／remove／as_slice／as_mut_slice | 八列契約在首次 code 前逐項給 ownership、receiver、結果、邊界 | pop 交 owned Option<T>；remove 交 owned T/越界panic；slice只是借用 | 足夠 |
| 006-002：slice.get(index)／Option<&T> | code 前明說 safe borrow；numeric indexing Copy；2v4 slice 邊界 | 不和 str.get(range) 混用單位；成功 get 不移除/交元素 ownership | 足夠 |
| 006-002：remove 位移／mutable slice 最後使用 | 操作後明確追 80、120、200 →120、200 →120 →130 | 相同索引的對象可能改變；writable 結束使用才回 owner讀 | 足夠 |
| 006-003：Vec<String> push/pop、Vec non-Copy、隱藏 borrow | 成功/缺值 let else 先備3.1；Option<&T> 保存借用明說 | 包 Some 不解除借用；分別追 message→messages→taken | 足夠 |
| 006-004：keep_nonnegative／while 與 owned output | 比較原需「保留順序非負數」；輸入 &[]、結果 Vec、position bounds 先說 | 讀所有輸入但只push符合值；不改input、不排序、不刪原資料 | 足夠 |
| 006-004：usize assertions／empty array／cfg tests | 1v8/004-002 整數推導及 010 數值 assertion；2v4已比較slice長度 | 數字與長度是已知值；空長度0沿用array syntax，不靠Vec整體 Eq/Debug | 足夠 |
| 006-005：ceiling exercise／完整答案 | 先聲明非負 ceiling；比對原輸入與篩選後再截值 | 負數略過不能先變0；順序與原資料保留；只用既有numeric/controlflow/API | 足夠 |
| Cargo.toml：新增 Clippy lint 設定 | archived manifest 8–9；前文只教執行 Clippy，未教這個配置 | 完整可執行參考包的工具設定不能從已教 Cargo 欄位得知角色 | N1 待補 |

## 原需要、設計改變與例子的用途

以下分開保存四題，第二題的「機制」與「用途」不互相代答；它們是回收原文的需要，不是審閱者另加的需求。

| 節／原需要與引文 | Q1／Q3：是什麼、如何用 | Q2a：設計改變 | Q2b：改變為什麼回應原需要 | Q4：實際材料支持範圍 |
| --- | --- | --- | --- | --- |
| 3.1：「無法區分缺少資料與真的報價為零」 | enum 保有互斥 variant/payload；match/借用pattern取可用內容 | 把情況本身存成值，分支依形態，不塞零暗號 | 接收者能分辨 Missing、Amount、Rejected，只有適當形態才計算；exercise Waiting的0是明訂統計，未丟原狀態 | 三種 variant 全呈現、shared重用、if let/let else比較scope與早退；未聲稱代碼能查外部原因 |
| 3.2：「有答案或沒有」「失败並保留原因」 | Some/None、Ok/Err 的兩種結果形狀；caller match | 將缺值與錯誤作資料，把政策留接收者 | 缺報價只需缺值；零除數需原因，不能用假商繼续除 | 成功商6／零除數Err同材料對照，加上金額0／−5exercise；已限定小整數，不虛稱完整i32安全除法 |
| 3.3：「保留的是結果形狀，改變的是…型別」 | struct/function型別參數、推導/turbofish、Result兩角色 | 一份欄位形狀與保存規則供不同具體型別使用 | bool設定與numeric報价具有共同Some/None需要，少寫只型別不同的定義；類型检查與ownership保留 | Holder i32/bool與同T錯例／twoT答案；不假設任意T可乘、可印，沒有提前需要traitbound |
| 3.4：「接成問候語…顯示…追加驚嘆號」 | String 管理文字；as_str借來讀；push_str借用修改 | 讀取介面不接手owner；move界面明確轉交 | show以後還要追加，用借用保留原String責任；完成借用後再修改同一份内容 | 同一greeting先顯示、追加、再顯示，然後move；兩失败fixture區分失效名稱/借用重叠；不聲稱每次push必搬空間 |
| 3.5：「從 A中🦀 取出中間的中」 | byte長度/UTF8scalar邊界，str.get(range)→Option<&str> | byte位置與合法scalar端點一起作為區段條件；get留下失敗結果 | 1..4涵蓋完整中，1..2仅一個byte；len内不保證合法，get允caller處理失败 | 1/3/4 bytes真實材料、越界、合法空range、切斷、答案的2..2無效；不把scalar稱完整字形、不聲稱三byte是所有中文通則 |
| 3.6：「保留非負數…按原順序…三、四或零項」 | Vec擁有variable-length結果；slice借輸入；push依次保存 | 長度從輸出類型中移到運行結果；遍歷/篩選規則沿用while/if | 可保存未知結果數，仍保持原輸入與順序；Vec沒有替代篩選規則 | 主例三項、重複/零/空/全負tests及ceiling答案；操作例明確remove index換對象，所有結論限定選定API，不聲稱清單正確性覆蓋未來工作 |

主要通過項的限定也逐項核對：數值除法小材料、memory model 非布局、move 不必拷 heap、String／Vec 修改不必每次搬空間、scalar 不等同字形、Option 不保存失敗原因、get 不交 ownership、tests 只覆蓋選定契約。這些限定回應當前可能誤讀，沒有拿後章技術補推理。第 4 章只檢查最後導覽是後續能力目的，不擴大審閱其正文。

## 初判發現

### N1：完整參考包的 Clippy 設定未有先教角色

- 版本／位置：原凍結 archived `examples/ch03-data/Cargo.toml:8`，hash `ad8660fd549f0764df96f4f19d1eed21e959f4e97f9b4fa7a0e8ff9b329a783c`。
- 原句／實作：`[lints.clippy]` 與 `manual_map = "allow"`。1v8/010-004 只解釋 cargo clippy 找常見問題及 --all-targets；1.3 package/workspace 欄位未引入 lint table 或 allow level。本章正文亦沒有這項配置角色。
- 已教部分：讀者知道 TOML 區段／字串與 Clippy 是工具，不需要重教這些；Rust 的手動 match 完整契約也已教。
- 缺少關係：這個 table 選哪個工具、這個 key 與 allow 如何改變提醒，以及教材為何採用它。不能從「工具沒警告」理解哪些提醒已被關閉。
- 影響／分級：**必要的小型設定依賴，增加完整參考包閱讀負擔**；不阻礙 enum／泛型／文字／Vec 主概念，也不是教學 Rust code 偷用 closure。依本專案完整 manifest 也須符合先備的標準列待補。
- 最小建議：在這項設定首次交付處或 manifest 相鄰注解，說它關閉 Clippy 某項簡寫建議、為保留當前已教的 match 教法，其餘檢查仍照常。只交代設定角色，不為此提前教 map／closure 或完整 lint 架構。未修改、未驗修後版本。
- 發現者／提示狀態：本審閱者讀 source archive 自行发現，交叉覆核前；不依其他review/root問題。

### O1：char 的具體顯示能力可在首次 code 前補一句

- 位置：005-002 定義 char/scalar/literal；005-003:22–23 首次 `println!("一個 char={marker}")`。
- 原已教：println! 的變數顯示形式與 char literal；3.4 還明確說 String／&str 支援 {}；3.3提醒不能假設任意T都能顯示。這裡的輸出明確顯示中，沒有單位或字形判斷的歧義。
- 建議：char 介紹後補「char 也能以 {} 顯示它表示的文字」。收益是讓讀者不必從首次code觀察這項具體能力，保持新標準型別的操作說明一致。
- 分級／界線：**可選的具體使用補充**；既有 macro 的應用和後面完整輸出已足以跟上主線，沒有把它升級成Unicode核心概念缺失，不要求提前教Display trait。未修改。
- 發現者／提示狀態：本審閱者自行注意，未受其他紀錄提示。

本輪未發現其餘未教 Rust 語法或 selected API 偷渡：包括六份完整答案、四項 text tests、兩項 list tests、五個 compile failures、兩個 runtime failures。這是逐項來源依賴檢查的完成範圍，不是宣稱所有初學者一定讀得懂或已通過整體發布驗收。

## 圖文與原位畫面

- `string-view.svg`：圖上方 `greeting: String` →「管理」→中間唯一 `heap: Hello, Rust`，下方 `show 的 text: &str` →「借用」→相同中間框。指向同一內容的依據是同一個目標框與兩個命名箭頭，不只因文字相等。正文緊前指定第一次呼叫，緊後回收借用結束才追加 !，沒有把第一次圖誤讀成第二次字串內容。模型非layout的限定相鄰。兩尺寸實看字／線／箭頭可辨，手機圖寬約342.98、字約20.58px；無需放大。
- `utf8-boundaries.svg`：三張卡分別明確range、byte值、位置；正文明確crab=第三項🦀。A結尾1、中結尾4、crab結尾8，與前文不含終點的range關系一致；下兩框用同一「中」區分1..4完整與1..2切斷。合法結尾8不冒充索引7。圖未靠颜色單獨區分對象。桌面400×680／手機342.98×583.09，兩處必要文字清楚；只需一般上下捲動閱讀相邻解釋。
- 其他四頁沒有必要素材缺圖：enum 的互斥形態表與branch code足以追；generic兩種參數由syntax/文字對照；Vec三個數的移位由明確原序列和操作後逐步叙述完成，尚不需額外圖才能预测index0換對象。
- 畫面與 DOM／image hash 保存 `/workspace/work/ch03-continuity-v3-visual/`。我沒有借用root舊QA結果或其他review的判斷。手機一般長指令/部分表格可水平捲動，圖的必要信息沒有這項負擔；本次不是六頁逐段視覺驗收。

## 封存狀態

初判完成、待 root 開放交叉覆核。N1／O1 都只列出處與最小建议；沒有自行修正文、改設定、讀別人的問題或把後续章节當先备。所有後续裁定與修後複查應追加到另一份文件。
