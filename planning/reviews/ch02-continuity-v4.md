# 第 2 章銜接審閱：v4 獨立初判

- 角色：銜接審閱者；沒有參與第 2 章撰寫。本 agent 曾寫 7.1–7.2／9.1，但它們不作第 2 章讀者的先備。
- 目標讀者：懂基本 Python，依主文順序已讀完第 1 章。
- 本輪依 clear-tutorial 的 review protocol、calibration 與專案術語指引；**未讀首次閱讀紀錄、技術審閱、其他銜接審閱或 root 的問題列表**。
- 受審包：`/workspace/work/tutorial-implementation/frozen/ch02-complete-source-v4`，6 頁、31 個正文閱讀單位；必要前文為 `ch01-complete-source-v8`。
- 本紀錄是來源初判，不冒充逐段盲讀。完整範圍先建立依賴，再核對 source／圖與頁面；沒有修改教材、沒有擴到第 3 章、沒有重跑全套檢查。

## 初判結果與界線

**此範圍未發現必要銜接缺口；列一項可選的練習語法提示。**主要概念與範例的首次需要位置有實際前文或較早的正文支撐，沒有用未完成的後章能力補洞。

這項結果只支持 v4 的銜接與已列出的圖文檢查；不代替已封存的首次閱讀、不宣稱完整技術驗證，也不表示所有讀者都已學會。未執行新的 Rust 測試；執行正確性由其他角色與既有紀錄另核。

## 從正文建立的真正依賴

| 當下需要判斷什麼 | 較早的實際材料 | 第 2 章先教的位置 | 首次實際使用與判斷 |
| --- | --- | --- | --- |
| 一筆資料的角色不被位置順序藏住 | 1.7 `007-002`：「第一項代表單價、第二項代表數量」與 tuple 解構 | 2.1 `001-002` 定義／實例／欄位名與點號存取 | `001-003` 的 Order、欄位乘法與 count 修改；欄位順序交換在 `001-004` 仍由名字辨認。 |
| 修改欄位後，舊計算是否自動更新 | 1.4 `004-006`：「變數保存的是當時得到的值」 | 2.1 `001-003` 明確連回該規則，mut 在變數上 | before 360、after 480；答案 source 使用 i32 assertion，沿用 1.10。 |
| 把完整 Order 交出去後，原名稱能否使用 | 1.5 Fn／參數／回傳與 scope；2.1 自訂數值 struct | 2.2 `002-002` 有限記憶體模型、`002-003` owner／move、`002-004` Copy 對照 | `002-005` 依 first → second → subtotal 追蹤交接；fixture 的 E0382 所需 Copy 區別已教。 |
| 單獨讀欄位和取得整個 struct 的差別 | 2.1 dot-field、1.4 i32 | 2.2 `002-004` 明說數值 Copy、自訂 Order 不自動 Copy | moves 中讀 second.count 後仍能交出 second；沒有先加入 derive 或 trait 實作。 |
| 讀資料算小計，但呼叫者保留訂單 | 2.2 完整 Order move 與 i32 Copy | 2.3 `003-002` reference／borrowing／& 型別與 expression／*；`003-003` 自動跟隨 reference 的欄位存取 | `subtotal(&Order)` 只借用，呼叫後讀原 order；這個範例沒有在傳入函式後再使用 reference 變數，因此不依賴尚未教的 shared-reference Copy。 |
| reference 自身重新指派，與修改被借資料的差別 | 1.4 mutable 變數、2.3 shared reference 與 dereference | 2.3 `003-004` 分別交代 &mut、let mut reference、所指資料 | reference_binding 讀 3 再改借另一個 8；shared_write fixture 不能靠變數加 mut 改變借用能力。 |
| 借用衝突依「還會用到哪裡」而非名稱 scope | 1.5 scope、2.3 兩種存取契約 | 2.4 `004-002` lifetime、無分支例的最後使用、衝突區間 | first／second 最後使用後才 writable，writable 最後使用後才 owner 修改；延後再讀及兩個 &mut fixture 有已教的拒絕理由。 |
| 同一 mutable reference 串接工作，又保留原存取 | 2.3 &mut／* 的操作、2.2 move | 2.4 `004-003` 明確與自動 reborrow 的具體條件 | child 使用結束再 parent；兩次具體 &mut i32 函式呼叫得到 6／7。後文說明裸 let 指派的 move，不拿它當自動 reborrow 的通用規則。 |
| 內層值結束後，外層不能繼續借用 | 1.5 block 尾端值與 scope，2.2 owner 清理，2.3 reference 未接手資料 | 2.4 `004-004` 在 fixture 前定義資料須活到使用結束 | &inside 無效；改回傳 i32 即 Copy 到外層。練習區分存取順序與資料存在位置。 |
| 自訂資料的讀取／修改／接手介面如何呼叫 | 1.7 concrete len method、2.3–2.4 借用契約、1.7 tuple | 2.5 `005-002` impl／Self／self／receiver；`005-003` 三種工作；`005-004` associated function 與欄位簡寫 | methods source 的四種操作都有先前解釋；用 &self 交完整 non-Copy Order 的 fixture 與原本契約矛盾。 |
| 加總函式接收不同長度，而非換演算法 | 1.7 array／range／usize／len、1.8 for／累積；2.3 shared borrowing | 2.6 `006-002` slice、位置／長度與相對索引；`006-003` 省略端點／空段／shared ref Copy／for 取得 &i32 | sum_values 不依賴完整 Iterator trait；每個 concrete 操作的輸入、結果與 ownership 行為在 code 前教清楚。 |
| slice 邊界與整個 array 的邊界不同 | 1.7 runtime panic、2.6 相對索引與 len | 2.6 `006-004` 建立範圍與元素索引的兩個邊界 | 原 array 5 項、slice 3 項，選 slice[3] 被拒絕／panic 的意義不需 Option／Result。 |
| 只修改選定的一段，其他數值保持 | 2.3–2.4 &mut／*／借用用完，2.6 slice 與 for | 2.6 `006-005` mutable slice、迴圈每次取得 &mut i32 | increase 的答案只用已教能力，五個 assertion 分別驗 80／130／210／60／40，空段沒有迴圈工作。 |

前文核對採實際正文與程式，不採作者 ledger 或課程題目作完成證據。`String`、`Vec`、`Box`、`dyn`、derive、完整 trait／Iterator 定義與 lifetime annotation 沒有出現在本章所需 source。

## 每小節原需、設計改變與結果的關係

| 小節與原句 | 機制／性質的已教範圍 | 需要／用途與最短銜接 | 例子能支持什麼、尚不能推出什麼 |
| --- | --- | --- | --- |
| 2.1 `001-001`：「看到 order.0 時，仍要記得第零項代表什麼」 | struct 按欄位名與型別組資料；mut 後修改同一個 count | 獨立需要是從操作位置辨認資料角色；unit_price／count 直接取代需記忆的位置，計價規則保留。 | before／after與欄位順序練習支持角色對應與更新；未宣稱 tuple 做不到運算。 |
| 2.2 `002-001`：「交出去的是完整訂單，還是另外一份可獨立使用的數字？」 | owner／move 的完整交接，Copy 的獨立 i32 | 需要是判呼叫者之後能用什麼；逐步追蹤原名稱失效與回傳小計，對照 Copy 後修改不改原數字。 | 支持眼前 non-Copy Order 與 primitive Copy；不證明預設 non-Copy 的所有語言設計理由，也沒承諾這個深度。 |
| 2.3 `003-001`：「函式只讀資料算答案，原訂單仍由呼叫者管理」 | reference 借用，不接手原 Order；shared／mutable 所指資料與 binding 不同 | 原需由保留訂單引出借用；相同計價函式改參數，呼叫後仍可讀原 count，修改版則直接改原資料。 | 支持 borrowing 操作與角色分離；shared 禁止修改明確限於本章普通數值，不提前要求 interior mutability。 |
| 2.4 `004-001`：「先讀後改……接下來是否仍要使用」 | 最後使用控制簡單借用區間、reborrow 暫讓存取、資料不能過早結束 | 需要是讓多項工作依序使用同一資料；合法與衝突例保留資料，改變使用位置即可看見限制來源。 | 3→4→5與reborrow到7支持具體序列；已限定無分支，不把圖當任意控制流程的完整推導。 |
| 2.5 `005-001`：「找訂單如何算小計、如何增加數量……把這些行為放在型別旁邊」 | impl 組織行為，三種 receiver 延續既有契約；new 不需既有實例 | 工作不變、搜尋／呼叫位置改到型別；before 160、after 240、into_parts 保留數值角色。 | 支持行為組織與使用介面，不宣稱 method 改掉 ownership 或使乘法才可算對。 |
| 2.6 `006-001`：「五筆價格……中間三筆、全部或兩筆……函式能接收哪段資料」 | slice 的位置／長度、相對索引、ref Copy、for-ref 契約 | 原需是可接不同範圍；三筆380、全部500、空段0、短array200皆用同函式，迴圈算法保留。 | 支持輸入表示彈性與借用身分；不宣稱 slice 自動加速、配置新array或已完整介紹 Iterator。 |

## 限定、轉換與累積負擔

- **2.2 stack／heap：**正文先教呼叫區域、配置器與 pointer 的基本角色，再限定本例沒有 heap 資源、實際放置可能最佳化。當下用途是防止從 stack 直接推 move；沒有用後章文字型別作隱藏範例。
- **2.2 Copy：**只教當下能力，完整 trait 宣告待後章。診斷引用的 Copy 已先解釋；未引進診斷的其他補救建議作操作先備。
- **2.3 automatic field access：**先明寫 `(*order).count` 的操作關係，再說明簡寫；不靠後章 Deref 為範例補洞。
- **2.3 mutable binding／mutable borrow：**兩種 mut 的對象有變數改指向與所指數字的具體對照，是必要區別，不是無用途例外。
- **2.4 scope／有效使用／資料存在：**先合法順序、再衝突、再 reborrow、最後 dangling；每一步沿用 i32，沒有同時換資料型別或塞入 annotation。大括號例需要正常複習 1.5，不需要讀未完成章節。
- **2.5 Self／self／receiver：**名字相近但大小寫、角色、借用方式與呼叫位置在 source 前完整分開；大程式用四種已介紹工作整合，不一次引入未知能力。
- **2.6 array→slice：**保留同一份五項數值與同一段連續資料。圖、索引 1..4、相對索引 0..2、len=3、sum=380 的轉換鏈都有正文支撐。
- **閱讀負擔：**本章每頁增加一個主要關係；copy／borrow／mutation 在相同 Order 與 i32 上延續。2.4 的多個失敗例各回答不同限制，2.5 的整合較長但先解釋四項操作。沒有發現必須跳到外部教材補主文的地方。練習答案另開，前置仍在正文；銜接審閱也讀了凍結答案，未冒充讀者未看答案的體驗。

## 正常程式、練習／答案與 fixture 的範圍

- 已讀凍結包全部 **22 份 Rust source**：9 份主要／正常範例、6 份練習答案、6 份 compile-fail、1 份 runtime-fail；另讀成員 Cargo.toml。逐檔指紋與 manifest 相符。
- 答案使用的 mut 參數、數值 assertions、owned i32 block 回傳、receiver、mutable slice 迴圈，皆有較早正文解釋。沒有 struct Debug 比較、未知 helper、Box／dyn 或程序測試 API。
- 失敗 fixture 只含當下已教能力的錯誤組合；拒絕理由依 ownership、borrow、有效資料或 slice 範圍，沒有將未知 API 編譯問題混作教學目標。
- 主文數字與 source 可按已教加乘／索引／迴圈直接推得；本輪沒有重新執行 Rust，不能把這項靜態核對稱為新的執行證據。

## 必要圖文對應與實際頁面

先建立正文對應，再查看來源 SVG，最後使用 **實際 site 頁面** 截圖核對；不是只看 SVG source 就說版面通過。六頁 live Markdown 的 SHA-256 與 v4 page 指紋相同；2.6 生成頁包含 v4 的 shared-reference Copy 契約。頁面／圖位置與本文相符。

| 圖與正文位置 | 當下必須辨認的對應 | 實看結論與通過範圍 |
| --- | --- | --- |
| move-copy，2.2 `002-004` | first 的失效、second 的 owner、count 的3仍存在、another_count 的3→4；轉交箭頭與複製箭頭語意 | 四個名稱清楚，兩種箭頭有文字；與原句限定一致，沒有畫出假定 heap／實體 bytes 搬移。 |
| reference-owner，2.3 `003-002` | reference:&Order 指向同一個 order 所持資料；&order 建立借用、*reference 找到原 Order | 一份 Order 與兩種角色可直接追蹤；借用箭頭不讀成另建一份 Order。圖後才進函式參數與欄位自動跟隨的解釋。 |
| borrow-scope，2.4 `004-002` | first／second 的短借用線到最後讀取；名稱 scope 長線到 main 結尾；count+=1 在其後 | 藍／灰圖例與名稱清楚。圖聚焦 first／second，沒有承諾畫完整 main 的每項工作；略去 writable 不影響這組已明指的比較。 |
| slice-part，2.6 `006-002` | 同一 row 的原索引1／2／3、值120／200／60、新索引0／1／2；0的80與4的40不在框內 | 三欄及藍框可定位同一批資料；圖下 `selected=&prices[1..4]` 及上界4不含把來源與新索引連起，不靠數值相等猜同一對象。 |

視覺範圍：Chromium，desktop **1280×800**、mobile **390×844**；四張 SVG 與相鄰說明各看兩種尺寸，圖實際寬度都是340px，字與關鍵標記無須放大即可讀。另看2.1入口與2.5 receiver表格兩種尺寸；手機三欄表可在頁寬內閱讀。

截圖、viewport／bounds／URL／來源比對記錄在 `/workspace/work/ch02-continuity-v4-visuals/`，共12張截圖及兩份 JSON。未逐一截取每個 code block 的所有水平捲動狀態，未檢查 dark theme、不同字級或其他手機尺寸；不宣稱全章所有畫面變體驗收。

## 可選項 O1：讓第一個 mutable 參數的語法位置更直接

- **位置／原句：**2.2 `002-006`：「寫 add_one，接收一筆完整 Order，把數量加一，再回傳這筆 Order。參數需要 mut，因為函式會修改接手後的值。」
- **已教部分：**1.4 先教 `let mut`，1.5 先教有型別參數；當下已說清為什麼參數需要 mutable，以及完整值如何交回。答案 source 的 `fn add_one(mut order: Order) -> Order` 可與這些說明銜接。
- **尚可更直接的關係：**練習正文尚未明寫 mut 放在參數名稱前的語法位置；這是新位置的具體寫法，不是新的 ownership 機制。
- **影響與分級：**主線的 move／回傳已能理解，讀者可由參數需 mut 與既有 binding 寫法組合，不列必要阻礙；但首次自己寫函式時可能先猜位置或開答案查。此為審閱者合理預見，並非已觀察的首讀回讀。
- **具體收益／最小修法：**在題目加「參數寫成 `mut order: Order`」這個短片段，仍把修改與回傳 body 留給讀者。它不增加新前置，也不需要新增完整範例或提前展示答案。
- **處置：**本輪只列可選改善，未修改。Root 可按是否值得減少習作語法搜尋的負擔裁定；保留原文也不改變本輪主線通過範圍。

## 初判封存狀態

此文件先獨立保存，之後才可互看其他角色。凍結包 manifest 指紋與本文件初判指紋另由保存工具寫到 sidecar；交叉覆核將另開檔案，不回寫早期判斷。
