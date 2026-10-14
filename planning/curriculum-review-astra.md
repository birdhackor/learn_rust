# learn_rust 課程規劃獨立審閱

日期：2026-10-14。審閱角色：Astra 獨立課程規劃審閱者。

## 1. 範圍、輸入與判讀方式

本報告完整審閱 `planning/curriculum-plan.md` 第 1–356 行，包含使用者標準、15 大章／51 單元、lifetime 安排、範例規則、兩個最終專案、作者排程與主要前置圖。下文的單元編號與行號均指這份凍結稿，並非重新編號後的課程。

送審檔案 SHA-256 已實際核對：

`f67b41534721d2ffcd402e31e6d95be93e2e32a7bfcce793db5c724a7ba97381`

已讀的專案規範及其指紋：

| 文件 | SHA-256 |
| --- | --- |
| `AGENTS.md` | `8fd152b93b11fcdea379118b2306ccf2617f5788e6bcf42f6d116e92842efb8c` |
| `.agents/skills/clear-tutorial/SKILL.md` | `e3890d1adbf84fc530016d45a5019a4dcc614c65d75efe3c42e9ab25e0b1618f` |
| `references/rust-terminology.md`，相對於上述 skill | `711c6027313c8135ec387337dedeef7b64a96b7f85bf837e2d11b91e994774cd` |
| `references/writing-prompt.md` | `c83a3ee311ecaf3f06068321fc1c4d7eb0232e3fece2c4ed7f9ba4965a566826` |
| `references/review-protocol.md` | `aa482b1ed4fdc28c74d7a0ba520034d7cc3d9db22d1ad344daed42a934299395` |

另讀根目錄的 `rust-toolchain.toml` 與 `Cargo.toml`，只確認規劃所在的工程背景：Rust 1.99.0、edition 2024、workspace resolver 3。未把既有教材當成新規劃已教完的前文。

這是取得完整大綱與使用者標準後的規劃審閱，**不是逐段首讀、教材驗收或實際目標讀者測試**。我沒有讀取其他審閱者的報告或評語，以下判斷獨立形成。沒有撰寫教材、試作最終專案、安裝依賴、執行範例或驗證頁面；亦未修改凍結稿。協調者所做的既有專案檢查不作為本課程規劃通過的證據。

分級如下：

- **必要修正**：為履行已確認標準，規劃必須補上責任位置、拆分關卡或釐清契約；不等於已發現不存在的範例犯錯。
- **改善建議**：有具體理解收益，但也有可行替代安排，不應強制把我的偏好當成唯一順序。
- **合理保留／待驗證**：目前有成立理由，或必須等實際教材、依賴版本與程式出現後才能判斷。

## 2. 總評

**主題選擇與基本方向可以保留；目前還不宜直接依 51 個標題全面平行開寫。**規劃已認真處理 Cargo 提早、Box／dyn 禁止偷跑、lifetime 分層、第三方 API 要先教，以及最終實戰另外提供完整參考版。兩個專案互補，進階內容也沒有被塞進選讀。這些是本稿的實質優點。

主要風險在三處。第一，前半段有一段很長的概念建置期，卻把 `Vec`、可用的小成果與一些必要的開發回饋排得偏後；基本 Python 讀者容易先記住規則，尚未感受到那些規則如何幫自己完成工作。第二，單元 15、22、25、33、40、44、47 的內容不是等量的「小段落」，其中單元 33 還成為 thread 與 async 的整體門檻。第三，主要前置圖尚不足以約束實際 API、診斷、測試與最終專案的行為。

修正方向應是：**保留深入 Rust 與全部進階必修，用更小的理解關卡、更具體的 API 契約與較早的可見成果承接它們。**不需要為了初學者刪除 unsafe、FFI、Pin、GAT 或 variance，也不需要強迫兩個最終專案硬用完所有進階技術。

最優先處理六件事：

1. 把「使用 API 所需知識」與「實作該 API 所需知識」定成可執行的審查規則，補實際 API／型別／測試依賴表。
2. 拆開單元 33，撤除 GAT／variance 對基本 thread／async 的整體阻擋；HRTB 仍在真正使用前教。
3. 將幾個大單元拆成有成果的小節，讓 collections 與小型成果較早出現。
4. 補齊第三方 crate、I/O、取消、測試工具的使用契約，尤其不能把 helper 或完整官方範例當成免查區。
5. 依已完成、可引用的前文授權作者；修正目前排程中條件不夠明確的分支。
6. 先凍結兩個最終專案的行為與驗收邊界，再評估約 1,000 行與引導任務的份量。

## 3. 必要修正與具體問題

### N1. 前置規則正確，但還缺可操作的「教到哪裡才算可使用」

**來源與已具備部分：**第 1 節第 4–7 點（L17–20）明訂所有範例、型別、trait bound、helper、測試與診斷都要先教；同時 L18 允許不提前揭露所有內部實作。第 7 節要求作者取得允許使用的 API 清單（L275–279）。這個方向應保留。

**缺口與影響：**如果只要求 API 名稱已列入單元，仍會漏掉 `HashMap::get` 的借用關係、`Display` 的回傳型別、`JoinHandle` 的錯誤等。反過來，若把「API 已教」解成所有底層 trait、泛型定義與內部實作都已教完，`println!`、`for`、derive 與 async 幾乎都無法循序引入。兩種解讀會讓平行作者各自採不同標準。

**最小修正：**在作者規範中明確採以下邊界，並套用到每個實際例子：

| 使用層次 | 此時必須教的內容 | 可留到後面的內容 |
| --- | --- | --- |
| 對已知具體型別呼叫 API | 接收什麼、借用或取得哪些值、回傳什麼、必要限制、失敗／副作用，以及呼叫後哪些值仍可使用 | 未展示且不影響當前操作的泛型實作、內部資料結構與最佳化 |
| 使用 trait 提供的能力或 derive | 該能力的意義、何種型別滿足它、對本次操作有何要求；derive 生成的是什麼能力 | 每個產生的方法本體、procedural macro 的實作 |
| 展示泛型簽名、撰寫泛型 wrapper 或自行實作 trait | 簽名中的每個 bound、associated type、lifetime、回傳型別，以及實作必須遵守的契約 | 未使用的方法與無關 blanket impl |
| 引用編譯器訊息或官方文件片段 | 所選片段中讀者完成當下判斷需要的術語、型別與建議；不能貼一整頁後說陌生部分不用管 | 未引用的文件其餘部分，不因連結存在就視為讀過 |

例如，早期可以先教「數字的 `assert_eq!` 會比較左右結果，不同時測試失敗」，不必先實作 `Debug` 與 `PartialEq`；若已讓學生寫泛型 assertion helper 或分析提到這些 bounds 的診斷，就必須補上相應教學。使用 `println!` 與 derive 也適用同一原則。這不是黑箱 helper 的例外：**可省的是當下不需要的實作，不能省的是操作契約。**

**相依連鎖：**每個單元須產出「輸入能力 → 本單元先教的能力 → 範例使用 → 後續可依賴能力」；第 8 節大圖保留作導航，另有實際 API 層級的表。作者開工前至少先凍結代表性例子的介面，完稿後再用真正程式與診斷覆核，不能在規劃階段宣稱閉合。

### N2. 有些「單元」承擔多個不同學習問題，51 不能當成工作量或理解量

**來源與已具備部分：**L7 說「單元是小段落」，但單元 15 同時放 trait、bound、associated type、三個標準 trait、derive、除錯與靜態 dispatch；22 放多種轉換與 `?`；25 放所有同步 I/O、path 與 CLI；33 放 HRTB、GAT、variance；44 同時補 C、FFI、ABI、布局與 ownership；47 同時教兩類 macro 與 derive 實作。

**缺口與影響：**這些不是同一規則的多個 API 名稱。以單元 33 為例，「函式對每次借用都有效」、「associated type 隨借用改變」與「某個型別參數能否替換成較短 lifetime」是三個不同問題。將它們當一小段，讀者很難知道自己卡在哪個關係；作者也容易用大量語法與外部連結補救。

**最小修正：**不必先增加大章數，但需承認單元可拆為多個可驗收小節，先分配各自的問題與成果：

| 原單元 | 建議拆出的理解關卡 |
| --- | --- |
| 15 | 先以同一操作需求介紹 trait／bound；再教 derive 與已有型別能力；associated type 接到 Iterator 前另教 |
| 22 | 先說清哪種轉換可能失敗；再教錯誤傳遞與 `?`；借用式轉換、解析及不同 conversion trait 的選擇另成小節 |
| 25 | 文字檔與具體 I/O 錯誤 → bytes／分塊讀寫 → path／OS 字串 → CLI／標準輸入輸出與結束狀態 |
| 33 | HRTB 的呼叫關係 → GAT 的借用產物 → variance 的合法替換與限制；不合併驗收 |
| 40 | task 生命週期 → 有上限的工作排程 → timeout → 取消與收尾；每步保留同一個小工作模型 |
| 44 | C 與呼叫／建置的最低背景 → ABI／資料布局 → 資料與 ownership 邊界 → 安全 wrapper 契約 |
| 47 | `macro_rules!` 的匹配與展開 → hygiene → proc-macro crate → token 與必要第三方 API → derive 的最小完整實作 |

**相依連鎖：**作者排程與每次審閱的單位要跟著拆；不能仍以「單元 33 已分派」當成所有後續都能引用。這是達成好學優先所需的規劃修正，不代表目前已知道每節應有多少頁或幾小時。

### N3. lifetime 三層方向成立，但中層內容過集中，進階層成為不必要的大門檻

**來源與已具備部分：**單元 9、16、33 及第 3 節（L183–188）清楚區分有效範圍、annotation、進階 lifetime 與 Pin。這比把 lifetime 延後一次教完合理。問題在單元 16 一次包含多種借用關係與 `'static`，而第 8 節直接指定 `36 ← 32、33`（L323）。

**缺口與影響：**基本 `Send`／`Sync`、`thread::spawn`、`Arc`／`Mutex` 不普遍要求先理解 GAT 與 variance。反而 closure、跨 thread ownership 與 scoped borrowing 能給進階 lifetime 更具體的需要。另一方面，不能把 33 整體移後便聲稱 thread 的所有 lifetime 依賴消失：官方 `thread::scope` 簽名確實包含 `for<'scope> FnOnce(...)`，亦有 `'env`／`'scope` 的關係。[S3]

**最小修正：**

1. 單元 9 明確處理 reference 所借的值、借用最後一次使用、重新借用與不能同時做的操作；保留 ownership 圖與程式位置的對應。此處不需 annotation。
2. 單元 16 先集中在「回傳的 reference 來自哪裡」與「持有 reference 的 struct 受誰限制」。納入方法的 elision、必要的多個 lifetime 與 outlives 關係，但不要只以一個 `longest` 特例包辦所有能力。應能說明何時改成回傳 owned value 比加 annotation 更合理。
3. `&'static str` 可接字串文字說明；`T: 'static`、`T: 'a` 與 `'a: 'b` 的必要應用分段承接。凡單元 30 的錯誤來源或單元 32 的 `Any` 已需要，就在它們之前補足；不能全部拖到 thread 才教。
4. HRTB 放在 closure／callback 已有經驗之後，並在展示 `thread::scope` 完整簽名或撰寫相應泛型介面之前教。GAT 與 variance 仍為必修，分別以借出資料的介面、mutable reference／interior mutability 的替換限制建立動機。

**相依連鎖：**將 33 拆成 33a／33b 等教學節點；36 不再依賴整包 33，37 的 scoped 部分依賴必要 HRTB。43、45 等若實際使用進階 lifetime 推理，再指向相應節點。`'static` 不代表值永遠不會被釋放；`Error::source` 中 trait object 的 `'static` bound 也不代表回傳的借用永遠有效。[S2、S3、S4]

### N4. 幾個可預見的早期 API 缺口，應現在指定教學責任位置

**來源與已具備部分：**第 4 節已提醒 `.chars()`、Deref coercion、`Error::source` 與 thread 錯誤的順序。以下延續同一標準，不把大綱未逐字列出所有 API 一律判成缺陷。

**缺口與影響：**下表是原定任務自然會遇到的具體選擇；若未先決定，作者很容易照抄熟悉寫法而超出前置。未實作前，條件式項目應列待決，不應宣稱已違規。

| 首次可能需要處 | 已具備部分 | 尚須指定的內容與最小處理 |
| --- | --- | --- |
| 4–6：回傳、迴圈與測試 | 基本型別、函式、array、測試都已排入 | `()`、`usize`／索引、解構與基本測試 attribute 的使用意義要有位置。`#[test]`、`#[cfg(test)]` 若出現，不能等 34 的 `cfg` 才補；完整條件編譯仍可待教 |
| 9–10：修改借用資料 | borrowing、`&mut self` 已列 | `*` 在 reference 上的基本操作、重新借用，以及不能直接移出借用欄位的情況。若採 `mem::replace`／`Option::take`，先教 API；不以「借用已教」概括放行 |
| 12：建立、借用與切字串 | `String`／`str`／UTF-8、避免 Iterator 偷跑已列 | 固定初期可用 API，例如先教 `String::new`、`push_str`、`as_str` 的具體契約。`String::from`、`to_string`、`to_owned` 各有不同來源，不能視為只換名字；若用 `get` 或 UTF-8 轉換回傳 `Option`／`Result`，需先完成 14 或改選例子 |
| 15／20：比較與 hash | `Copy`／`Clone`／`Debug`、`Eq`／`Hash` 已列 | 補上 `PartialEq` 與 `Eq` 的關係，以及相等鍵必須具有相同 hash 的契約；若以 `derive(Eq)` 配合自訂型別，相關 derive 的必要條件亦須已教 |
| 20：借用查詢與 entry 更新 | `HashMap` 查詢更新、borrowing 已列 | `HashMap<String, V>` 用 `&str` 查詢涉及 `Borrow`，不能以 22 的 `AsRef` 代替。若展示完整泛型簽名還會遇到 `?Sized`，可先教具體查詢契約，完整簽名等 28；`and_modify`／`or_insert_with` 等待 23，或先採不用 closure 的更新方式。[S1] |
| 21：實作 `Display` | trait、lifetime、具體錯誤已排在前面 | 在實作前教 `Formatter<'_>`、格式化的成功／失敗、`write!`、`fmt::Result`。type alias 原在 32；應把讀懂此別名的最低知識移到這裡，或明寫已懂的 `Result<(), fmt::Error>`。本節未教 `?`，不要在 formatter 裡順手先用。[S2] |
| 24：回傳 iterator 或 closure 的 helper | associated type、closure、collections 已具備 | 若回傳 `impl Iterator`／`impl Fn`，必須將 32 的 return-position `impl Trait` 最低教學移到此處；也可用不需這類 helper 的例子。不要用極長 adapter 型別或 Box／dyn 迴避 |
| 25／35／51A：整理檔案與排序 | I/O、path、hash 已列 | 排序所需的 `Ord`／`PartialOrd`、比較 closure、穩定的輸出順序要指定位置；`Default`、`Borrow`、`ToOwned`、`Cow` 只在實際所選 API 需要時教，不必為補清單另塞一整章 |

**相依連鎖：**最小的調整通常是增加一個有用的小節或換掉不合時序的慣用寫法，而非把整個 trait／smart pointer 章提前。泛型 helper、參考答案與錯誤訊息必須受同樣限制。edition 2024 的 return-position `impl Trait` lifetime capture 與舊版不同；進階 borrowed iterator 範例應以固定版本文件核對，不直接搬用舊教材。[S5]

### N5. I/O 與第三方套件不能只列 crate 名稱；需把真實介面拆成可教的任務

**來源與已具備部分：**25、26、35、41 均承諾在整合前先教 API；L201 也把 C／HTTP／hash／JSON 列為先備。這是充分的原則，尚缺落到具體任務的內容分配。

**缺口與影響：**Python 使用者知道「讀檔」不等於知道 bytes 與 Unicode path、短讀取、泛型反序列化、HTTP 回應與傳輸失敗的差別。尤其單元 26 的三個 crate 可能讓 derive、設定、解析、錯誤與資料格式同時出現。

**最小修正：**在最終專案之前，至少落實這些有輸入與結果的小練習：

| 位置 | 最低可驗收成果 | 需事先選定／教清楚的介面 |
| --- | --- | --- |
| 25 | 能讀文字檔、分塊讀 bytes，正確處理一次讀取的有效長度與錯誤 | `Read`／`Write`、需要時的 `BufRead`／buffer、`io::Result`／`io::Error`、EOF、短讀取與短寫入；trait method 需要的 `use`。一次 `read` 少於 buffer 大小不必然表示 EOF。[S6] |
| 25 | 保留檔案路徑身分，顯示路徑並處理 CLI 輸入 | `Path`／`PathBuf`、需要時的 `OsStr`／`OsString`、`args`／`args_os` 的選擇、工作目錄、標準輸出與錯誤輸出。`Path::display()` 的輸出物件與可能的失真轉換要教，不當作 `String`。[S7] |
| 26a | 給相同參數資料可重現解析成功或失敗 | clap 的 `Parser`、實際使用的 attributes／features、`try_parse_from` 與錯誤；process 結束行為與 library 測試分開處理。其 iterator、`Into<OsString>`、`Clone` 前置要核對。[S8] |
| 26b | 對已懂資料模型讀寫 JSON，並說明錯誤屬於哪一層 | JSON 的結構與型別對應、`Serialize`／`Deserialize` 使用契約、derive、實際選用的 serde_json 函式與錯誤。先採 owned 欄位，不自動引入借用式反序列化 |
| 35 | 能區分大小篩選、hash 候選與實際內容確認 | walkdir 的 iterator item／錯誤／symbolic link 行為，blake3 的狀態、update／finalize、digest 型別與比較方式。hash 相同不能自行升格成內容已完全驗證 |
| 40–41 | 能判斷 HTTP 結果、timeout 與 task 錯誤 | URL、HTTP method／狀態碼／redirect、client 與 request／response、`Duration`／`Instant`、runtime features、`JoinError`、受控服務啟停的 API |

Serde 特別需要做一個選擇：`from_str` 的 bound 是 `Deserialize<'a>`；`from_reader` 使用 `DeserializeOwned`，後者的定義含 HRTB。[S9] 使用 owned 結果時，可以先把「結果不借用輸入」教成具體能力契約；若完整展示定義或讓讀者寫泛型 wrapper，就必須先教 `for<'de>`。不能一面說 HRTB 到 33 才教，一面把它當成 26 的無須解釋語法。

**相依連鎖：**每個最終專案列出實際採用的版本、feature、函式、回傳型別與測試 API；不要求把 crate 全部教完。第三方 library 的底層內部使用 `Box` 或 unsafe，不會自動使呼叫者違規；但若這些型別出現在讀者需理解的介面、helper、診斷或實作中，就不能藏起來。這個界線也適用未來選擇的測試服務 crate。

### N6. thread／async 使用與內部實作可以分開，但取消與 guard 的行為必須在使用階段教

**來源與已具備部分：**36–41 已涵蓋 `Send`／`Sync`、thread、channel、guard、runtime、timeout 與取消；L199 明訂手寫 `poll`／Pin 簽名延至 45–46。這個分層合理。

**缺口與影響：**目前單元 40 的「取消、select! 與收尾」仍不足以指定學生應能預測什麼。常見範例會偷偷用 `tokio::pin!`、`futures` 的 stream adapter 或 `tokio-util::CancellationToken`；它們都不是目前列出的自動可用先備。另一個實際差別是「同時發送的請求數」與「已建立的 task 數」：先建立所有 task，再用 semaphore 擋住請求，並未限制 task 總數。

**最小修正：**

- 37 明確區分 thread 的一般錯誤與 panic payload。`std::thread::Result<T>` 是含 `Box<dyn Any + Send + 'static>` 的別名，payload 不保證是 `Error`；原稿把 Box／dyn／Any 先教是對的。[S4]
- 38–40 用具體範圍呈現 guard 的持有與釋放；教會讀者判斷等待期間是否仍持有鎖，以及這與 blocking、死鎖、`Send` 有何關係，不籠統說所有 lock guard 都一樣。
- 40 明定工作生命週期：建立、開始、等待、完成、取消請求、確認收尾。丟棄 Tokio `JoinHandle` 會讓 task 脫離 handle 繼續執行；`JoinSet::abort_all` 後還需等待取消完成，不能以呼叫 abort 當成全部結束。[S10]
- 選擇一條主要排程路線，例如 `JoinSet` 中只維持上限內的 task，完成一項才補下一項。若改採 semaphore／stream，先補選用 API 與它真正限制的資源。
- `select!` 教取消安全與未完成分支的處置。39–41 的例子若需預先 pin 的 future，應換成符合當時前置的組合，或明確拆出最低 pin 使用教學；不能貼官方含 `pin!` 的範例就跳過。[S11]

**相依連鎖：**本機測試服務若在 41 用 atomic 計數來驗證並行上限，便會偷用 42。可用已教的 `Mutex`，或將 atomic 的必要教學提早；不要在測試程式裡默默破例。最終專案 B 的取消、timeout、計數與驗收必須沿用同一契約。

### N7. 所有進階必修需要自己的完成證據，不能只靠兩個最終專案

**來源與已具備部分：**L262 已正確說明 unsafe、FFI、Pin 由前面專題練習驗收，最終專案不必硬用。49 另列進階測試，50 列公平效能量測。

**缺口與影響：**尚未定義各專題應讓基本 Python 起點的讀者做到哪一步。若只有列名與一個能跑的展示，容易把「看過實作」算成深入掌握；也可能因想做通用框架而讓每個題目膨脹成另一個千行專案。

**最小修正：**每個難題固定一個小而完整的產物與一個需要理由的判斷，不要求再蓋一套應用：

| 主題 | 建議的最低完成證據 |
| --- | --- |
| HRTB／GAT／variance | 分別能解釋 callback 對哪些借用成立、借用結果與輸入的關係，以及哪種 lifetime 替換不成立；GAT 可用一個借出內部資料的介面，但不要把整個高效容器當作入門題 |
| atomic／memory ordering | 區分單一計數的原子性與跨操作的同步；對一個有限範例說明 ordering 保證與未保證什麼。不要以執行多次沒出錯證明正確 |
| unsafe | 對一個小型安全 wrapper 逐項交代 pointer 來源、有效性、範圍、初始化、aliasing、lifetime 與呼叫者責任；測試通過不代替 safety 論證 |
| FFI | 能建置、呼叫一個小 C 介面並說清布局、字串／buffer 與 ownership 誰負責。必要 C 與建置工具在前，不假定已有 C 背景 |
| Pin／Future | 能區分 move、地址穩定性、`Unpin` 與 lifetime；實作一個有限的 Future，並追蹤 Pending、喚醒、重新 poll 的關係。所選範例若使用 `PhantomPinned` 或 projection，必須先教 |
| executor | 清楚限定支援範圍。可先採官方 `Wake` 建立 `Waker` 的安全途徑；它是 `RawWaker` 的安全替代，不必為了「內部機制」一開始就手寫 vtable。[S13] |
| macro | 小型 macro 能說清楚輸入、輸出、錯誤位置與 hygiene；proc-macro 的 token／syn／quote 等實際依賴先教，不用隱藏 parser helper |
| API／驗證／效能 | 對同一小介面提出相容性改動，選擇能驗證該風險的工具；效能比較先固定正確性、工作量與量測範圍 |

**相依連鎖：**49 應是驗證方法的綜整，而非第一次對 unsafe 或 atomic 做必要驗證的地方。若採 Miri，需先規劃與主課程 Rust 1.99.0 分開固定的工具鏈；官方使用方式需 nightly，且對外部功能有支援限制。若採 Loom，需教模型化程式與替代同步型別，不是直接替任意標準 library 程式加一個測試指令。[S14] 這些是未來教材的必要設定與限制，本次沒有安裝或執行工具。

### N8. 最終專案的「實際可用」與約 1,000 行需要共同的範圍契約

**來源與已具備部分：**L213 坦白列出每個 800–1,200 行、包含測試、文件另計，而且尚未實測；A 限搜尋與報告，B 不做爬蟲／登入／資料庫。這些界線合理。

**缺口與影響：**「重複占用的空間」、「錯誤狀態」、「中止與收尾」仍各有幾種實作含義。若把跨平台實際磁碟回收量、任意檔案變動的一致快照、所有 HTTP 探測策略都默認為完成版需求，會超出目前可評估的份量。若不定義，學生可能完成不同程式卻不知道是否過關。

**最小修正：**在撰寫引導前，先採第 7 節提出的最小可用契約，列清必做、明示限制、延伸題、錯誤政策與每階段驗收。行數只用來約束範圍，不能以補樣板或刪測試達標。

**相依連鎖：**專案契約決定 25／26／35／40／41 要先教哪些 API，也決定 49 的測試能力；因此應在這些單元作者開工前先對齊最終介面與行為，但不能反過來讓作者引用尚未寫完的前文。

## 4. 改善建議：學習動機、Cargo 分工與階段成果

### I1. collections 與小成果可提早；不必把每一項工具與型別細節都走完才開始做事

**來源與已具備部分：**1–6 已能建立、拆分與測試簡單程式；7 用數值 struct 教 ownership；19 才出現 `Vec`，27 才有第一個完整 CLI。

**負擔與影響：**這不是「第 27 個單元一定太晚」的實證結論，因為尚無篇幅或讀者資料。但 7–18 包含 ownership、兩種文字表示、enum、泛型、trait、完整 lifetime、外部依賴與測試工具，若每步都只用孤立示意，讀者會長時間累積抽象區別，缺少整合回饋。

**建議與連鎖：**把 `Vec` 移到基本 enum／泛型／trait 與借用之後，無須等待全部 `'static` 討論與開發工具。早期安排幾個使用已學內容的小成果：6 的可測試計算 library、12 的文字資料整理、20 的記憶體內分組／統計。它們不需要檔案、第三方 crate 或進階 Iterator。27 保留第一個完整 CLI，承接真實輸入、錯誤與程式結束行為。這能提早回收動機，而不靠偷用 `Box<dyn Error>` 加速。

### I2. Cargo 拆分基本合理，應按「當下要做的工作」控制深度

**來源與已具備部分：**1–2 首次建置與 workspace；6 module／library；17 dependencies；18 開發流程；34 工程設定。這符合使用者要 Cargo／crate 早教的要求，不應重新集中到後半本。

| 位置 | 建議保留的責任 | 避免的額外負擔 |
| --- | --- | --- |
| 1–2 | 讀者知道自己位於哪個資料夾、正在執行哪個 package／crate；理解實際用到的 manifest、`-p`、lockfile、`--locked` | 不在第一個可見結果之前講完整 dependency resolver、發布策略或 feature graph |
| 6 | 模組／路徑／可見性、library 與 binary 的最小分工、基本測試；加入最基本的 fmt 與讀錯位置 | 不把「拆檔」等同於「拆 crate」，也不立即要求複雜 public API 設計 |
| 17–18 | 在第一個外部 crate 與跨檔測試之前完成版本、features、dependency 種類、docs.rs、整合測試與文件測試 | 無需以完整 16 的所有 lifetime 題目當成 Cargo 命令的技術前置；範例實際需要的 Rust 能力另列 |
| 34 | 真正要用的 workspace 共用設定、profiles、cfg、target、build script；銜接 FFI 與 proc-macro 設定 | 不把尚無用途的每個 Cargo 選項都做成必記清單 |
| 48 | 公開 feature 契約、相容性與發布前檢查，回收前面實際維護 library 的經驗 | 不把 package 發布當作完成 Rust 入門的必要外部操作 |

教材 workspace 共用 lockfile 與 target 目錄，部分設定只在根 manifest 生效，確實值得早講到當下夠用的程度。[S15] 若早期範例 manifest 已使用 `.workspace = true` 繼承欄位，單元 2 就應解釋讀者眼前的欄位；完整共用策略仍可在 34 教。`--locked` 控制的是 dependency resolution 的變更，不能單憑它承諾所有平台與外部服務結果都一樣。

### I3. 全書入口與每次大轉折要回到讀者能辨認的工作

**來源與已具備部分：**第 1 節清楚設定讀者與目標，但大綱主要以主題名稱列路線。

**建議與影響：**第一個程式前後給一段短導覽：已懂 Python 的讀者接下來要學會哪些不同的控制方式，例如資料由誰持有、哪些錯誤需要處理、何時允許共享；必要的編譯與執行背景由教材負責教。不要把 Rust 簡化成「Python 做不到」或一律更快。到 closure／Iterator、smart pointer、thread、async、unsafe 時，各用既有小任務引出新需求：重複處理資料、共享資料、等待 I/O、跨語言邊界等。

這不要求每節都有歷史回顧或人造應用。純型別性質可以明確說在學介面限制，並在最近的應用關卡回收；只要不拿新術語本身當成學習動機。

### I4. API 設計的基本回饋宜提早，進階相容性綜整仍留在 48

**來源與已具備部分：**6 已教 library／binary 分工；27 是第一個整合 CLI；48 才集中談 API。

**建議與連鎖：**6 先問「哪個函式可單獨測試」，27 問「解析參數、讀檔與核心運算誰負責」，30 問「錯誤在哪一層轉換」，32 問「newtype 保留什麼限制」。48 再用這些已寫過的選擇談公開契約、版本與 features。這會降低最後突然設計一整套 library 的負擔，也避免讓 48 看起來是 API 設計的第一次教學。

## 5. 一份具體可討論的閱讀順序

以下以原單元編號表示移動與拆分，**沒有修改凍結稿**。這是我建議的主方案，不聲稱為唯一正確順序；所有原定主題與兩個最終專案都保留。a／b 表示將一個大單元拆成理解關卡，並不把後半變成選讀。

| 階段 | 建議順序 | 改動理由與必要條件 |
| --- | --- | --- |
| 起步與可測試程式 | 1–6 | 保留 Cargo／crate 早教；6 只做最低可用 module／library／測試與必要 attributes |
| 資料、ownership 與借用 | 7–11 | 保留數值 struct → move → reference → method → concrete slice。heap 的圖先建立角色，12 再用真正 owned 文字回收，不假裝數值 struct 的 move 必然搬動 heap 資料 |
| 缺值、文字與基本型別介面 | 13 → 14 → 12 → 15a → 16a | enum／Option／Result 在文字的安全存取 API 前備妥；15a 聚焦 trait／bound／derive；16a 教回傳借用、持有借用與必要 elision |
| collections 與失敗 | 19 → 20 → 21 → 22 | Vec 提早，搭配短成果；補具體 API 契約。22 內部分節，不把所有 conversion trait 與 `?` 一次交給學生 |
| 抽象重用與資料處理 | 15b → 23 → 32a → 24 | associated type 在 Fn／Iterator 需要前教；32a 只取本階段需要的 `impl Trait`。若 24 完全不用回傳 opaque iterator／closure，32a 可留原位置 |
| 外部套件與第一個 CLI | 17 → 18 → 25 → 26 → 27 | 17／18 在第一次實際使用外部依賴與進階測試前完成即可；25、26 按 N2／N5 拆小節 |
| 指標、資源與進階介面 | 28 → 29 → 16b → 30 → 31 → 32 其餘 | 16b 回收 trait object／Any 所需的 `'static` 與 bounds；Box／dyn 仍完整先教再用；type alias 的最低使用已在 21 教過 |
| 工程設定與檔案練習 | 34 → 35 | 原方向可留；35 提供最終 A 的可用元件與行為經驗，仍不在此完成最終整合答案 |
| thread、同步與 atomic | 33a → 36 → 37 → 38 → 42 | 33a 教 HRTB 與 callback／scoped borrowing 所需關係；GAT／variance 不阻擋整章。42 接著同步經驗教，亦可供後續受控服務計數使用 |
| async 與 HTTP | 39 → 40 → 41 | 保留先使用再實作內部的安排；40 先固定排程與取消路線，41 補領域與測試服務知識 |
| 進階 lifetime 與安全邊界 | 33b → 43 → 44 | 33b 完成 GAT、variance 與未回收的進階介面題；43 再進入 raw pointer／soundness；44 自帶足夠的 C 與 ABI 小節 |
| async 內部機制 | 45 → 46 | 在已使用 async、懂 unsafe 與資源收尾後回看 Pin、Future 與 executor；不需把 FFI 當成 Pin 的技術前置 |
| 實作抽象、設計與驗證 | 47 → 48 → 49 → 50 | 每一專題已做必要局部驗證；此處綜整 macro、公開契約、工具選擇與效能推理 |
| 最終實戰 | 51A、51B | 最後一章保留兩個互補專案；讀者按引導完成，另有完整參考入口 |

幾個取捨必須說清楚：

- 把 13／14 移到 12 前是為了文字 API 選擇更自然；若原 12 明確只採不需 `Option`／`Result` 的 API，原順序也成立。這是有具體收益的排序建議，並非已證實的硬錯。
- 把 42 移到 async 前有助於把同步與 memory ordering 接在一起，也消除某些測試計數依賴；如果 41 的測試只用已教 `Mutex`，原 42 位置可以保留。
- 真正必要的修正是拆開 33 的能力與依賴，並使 `impl Trait`、type alias、HRTB 等在實際出現前已教；不是要求照抄這張總順序表。
- 「所有進階必修」與「第一遍先完成容易部分、其餘以後有空再看」不同。這條路線仍要求走完全部進階關卡；分段只是控制當下負擔。

## 6. 依賴圖與平行寫作：哪些可以成立

### 6.1 將三種關係分開保存

目前第 8 節將部分實際必要前置、教學安排與整本必修條件放進同一張表。例如 `17 ← 16` 並不表示每個 Cargo 操作都需要完整 lifetime；`36 ← 33` 也不表示基本 `Send`／`Sync` 依賴 GAT。應至少區分：

| 關係 | 用途 | 可否作為作者阻擋條件 |
| --- | --- | --- |
| 實際概念／API 前置 | 這個例子、契約、測試或診斷需要哪些已教能力 | 必須；只有可引用的完成稿才解除阻擋 |
| 教學順序 | 讀者先有哪種經驗，較容易理解下一個問題 | 決定出版與閱讀順序；作者若不使用未完成內容，仍可能先寫獨立分支 |
| 全課程完成條件 | 所有進階主題與最終練習均須完成 | 不表示每個後章程式都要使用全部先前主題，也不必強迫無依賴作者等待所有分支 |

建議依賴紀錄至少包含：實際能力或 API、首次教學位置、所用前文版本、具體例子／測試位置、狀態（已完成／待寫／待確認）。不要只有「已教 Iterator」；要分清 `.iter()`／`.iter_mut()`／`.into_iter()`、所用 adapter、借用與消耗、`collect` 的目標型別與錯誤結果處理。

### 6.2 對原排程逐項裁定

| 原排程（L268–271） | 裁定 | 使其成立的具體條件 |
| --- | --- | --- |
| 1–18 完成後，19→20、21→22、23、28→29 平行 | **可保留，但需條件化** | collections 分支不先用 23 的 `and_modify`／`or_insert_with`；錯誤分支不先用 closure 型 `map_err`；Deref 分支若用 `Vec` wrapper，須等 19，否則選已教 array／自訂簡單型別。各自可在使用前教本分支的新 API，但不可引用另一分支尚未完成的教學 |
| 24 等待 19＋23；25 等待 collections、錯誤、Iterator；30 等待必要型別、錯誤與 smart pointer | **方向可保留** | 若 24 回收 `HashMap`，增加 20；若有 `impl Iterator` helper，加入提前拆出的 32a；30 明確指向 Display、trait object 所需 lifetime 與實際錯誤 API。不要把「必要型別」保留成無法檢查的字樣 |
| 至 35 完成後，thread、unsafe／FFI、macro 平行 | **需拆條件** | macro 的新增 crate／設定可以在本分支先教。43 若只寫單 thread 的 raw pointer／unsafe 基礎，可獨立開始；若要用 `Send`／`Sync`、資料競爭、atomic 或跨 thread 範例，須等待 36–38／42 的實際必要部分，不能依大綱猜後文。FFI 等 43 與所用建置能力完成 |
| async 接 thread，Pin 等 unsafe 與必要 async | **保留學習理由，細化實際依賴** | runtime／單一 future 使用與 task 的跨 thread 限制不是完全相同前置；可拆作者工作，但 40 的 task／guard／收尾只能用已完成內容。45 等所選 unsafe、lifetime 與 async 動機，不必硬等無關 FFI 實作 |
| 最終專案必要前置完成後，A、B 分別由作者撰寫 | **可保留** | 各有獨立 API／測試／設定清單；A 不必技術上依賴 reqwest，B 不必依賴 walkdir。兩者仍放最後一章，且共同使用的 clap／serde、錯誤與測試能力須有同一份已完成來源 |

### 6.3 建議的交付門檻

1. **分派前：**作者取得已完成前文的版本與實際位置、允許能力、預定範例介面；未完成的章節不能列為可依賴來源。
2. **寫作時：**新增語法、API 或 helper 必須先在本單元教到當下夠用；若它又需要未完成前置，就改例子、縮小本次操作或等待，不能以將來補寫為授權。
3. **交稿時：**逐一核對正文、完整程式、提示、答案、測試、manifest 與引用診斷。編譯與測試只能證明相應技術行為，不能代替順序理解檢查。
4. **整合時：**依讀者順序核對轉折與累積負擔。前置稿改動時，追蹤真正使用該能力的後續例子，不必無差別重做所有單元。

共用 Cargo 設定由主 agent 整合的安排（L209）可以保留。尤其測試用 dependency、features、proc-macro target 與 nightly 工具設定，不可由各分支自行引入後再要求讀者接受。

## 7. 兩個最終專案的具體評估

### 7.1 共同判斷與行數範圍

**兩題都適合保留，且不應再加第三題。**A 主要驗收資料表示、借用、I/O 與錯誤政策；B 驗收 task、並行上限、取消、HTTP 與外部結果。它們各自具備可使用的完成品，也能重用前面小練習，而不是只把一個章節例子加長。

每題約 800–1,200 行手寫 Rust、包含測試，作為規劃尺度是合理的，但目前只能判為**可行假設，未經實作證實**。使用成熟 crate 後，有些功能可能很短；可靠測試與錯誤處理又會佔不少篇幅。可以先採以下份量預算，之後依真實實作調整：

| 專案 | 應用程式本體的粗估預算 | 測試、fixture 與必要測試服務的粗估預算 | 判讀 |
| --- | --- | --- | --- |
| A | 約 500–750 行 | 約 250–400 行 | 約 750–1,150 行，接近既定尺度；不需為達 800 行擴寫樣板 |
| B | 約 550–800 行 | 約 300–450 行 | 約 850–1,250 行；受控服務與取消測試可能決定上限 |

這些數字是本次提出的預算，**不是量測、不是已存在程式的行數**。應先統一計數：以完整參考版的一份手寫 Rust 為準，包含測試支援；不累加每個教學快照、不算生成碼、第三方 source、lockfile 或教材文字。行數不是熟練程度的證明。

### 7.2 專案 A：重複檔案搜尋器

原稿的「大小篩選 → hash → 實際內容確認」很適合教分層排除工作。保留搜尋／報告，不加入刪除、同步與背景監控。

**開始寫引導前，先固定這些契約：**

| 問題 | 建議的最低可用契約 | 為何不能留到最後猜 |
| --- | --- | --- |
| 掃描對象 | 明定一般檔案、symbolic link／根目錄 link 的處理，並排除其他檔案種類；保留選擇的理由 | 影響 walkdir 設定、metadata 的來源、循環與重複路徑；預設值不能代替教學 |
| 「重複占用空間」 | 最小版可報「依路徑計算的重複內容位元組」：每組 `(檔案數 − 1) × 長度`，清楚說明這不是保證可回收的實際磁碟空間 | hard link、共享儲存與稀疏檔案使兩個量不同。若要準確實體回收量，需另列平台／檔案身分能力與額外範圍 |
| 部分失敗 | 單一項目無法讀取時記錄路徑、動作與原因，繼續處理可讀項目；報告清楚標示掃描不完整，結束狀態有固定規則 | 「有找到一些重複」不等於整個資料夾已完整檢查；錯誤不應偷偷變成一般無結果 |
| 掃描時檔案變動 | 完成版提供盡力而為的觀察，不承諾檔案系統快照；能辨識的變動／讀取失敗納入報告 | 不必建造快照系統，但不能把一次 size／hash／比較當成永久事實 |
| 路徑與 JSON | 計算過程保留 `PathBuf`；明定非 Unicode 路徑在文字與 JSON 的表示或拒絕政策 | 不以失真顯示字串當成路徑身分。若決定只接受可表示的路徑，需報為明示限制與項目錯誤 |
| 排序與內容確認 | 結果有可重現的排序；hash 相同後仍正確劃分真正內容相同的群組 | 不能把整個相同 hash 桶直接算成一組；讀取 chunk 的邊界也不能被誤認為內容差異 |

上述第一、二、四點不要求全做高階檔案系統工程。最小修正是誠實且可驗收的契約。若使用者希望「實際磁碟回收量、跨平台精確辨識 hard link、掃描期間一致快照」都屬必做，便是需要另行決定的範圍取捨，可能增加行數或減少其他功能；本報告沒有默默承諾它們。

**引導里程碑應讓讀者每次完成一個可見的行為：**

| 階段 | 讀者要完成什麼 | 驗收與已教能力 |
| --- | --- | --- |
| A1 收集 | 對指定根目錄產生路徑、長度與項目錯誤 | fixture 有子目錄、空檔與被排除的項目；以 25／35 已教的 path、walkdir、錯誤處理驗證，不先給完整搜尋器 |
| A2 判定 | 依大小縮小候選，再做串流 hash 與真正內容分組 | 同內容不同檔名、同大小不同內容、空檔、跨 buffer 大小；比較邏輯測試可直接提供候選桶，不必期待碰巧找到真實 hash collision |
| A3 報告 | 排序群組、計算已定義的位元組指標、保留部分失敗 | 固定資料能得到固定群組與數字；文字與 JSON 表達相同結果；用已教 `Ord`／比較、serde 與錯誤模型 |
| A4 CLI | 接上參數、輸出選項、使用說明與結束狀態 | library 測試與少量 CLI 驗收分開；臨時資料夾或 process 測試工具的 API 必須已在 25／35 等位置教過 |

讀寫核心應對短讀取正確；若測試自訂 `Read` 實作或臨時資料夾 crate，這些也要加入前置表，不能因為位於 tests 目錄就豁免。[S6] 不建議為了顯得進階，強制 A 再加入多 thread hash；B 已提供並行整合，A 的題目本身已有足夠設計份量。

### 7.3 專案 B：async 網址檢查器

先做單一請求，再做依序批次與有上限並行的里程碑合理。最容易失控的部分是「檢查成功」的定義與取消後的狀態，不是發出 HTTP 請求本身。

**建議先固定的最低可用契約：**

| 問題 | 建議契約與必要教學 |
| --- | --- |
| 檢查什麼 | 固定使用的 HTTP method、接受的 URL scheme、redirect 政策，以及哪些狀態碼算檢查成功。收到 404 與連線失敗是不同結果，不能混成同一個 `Err` 後丟掉狀態碼 |
| 耗時量什麼 | 例如定義為開始此項請求至收到最終回應 headers，包含 redirect；若要讀完 body，則另定大小／串流策略。不要讓 UI 顯示的「耗時」與測量範圍不一致 |
| timeout | 明確區分每項請求期限與整批期限；reqwest 的 total timeout 涵蓋連線開始至 body 完成，實際程式若只取得 headers 就停止，驗收也須以那個工作範圍描述。[S12] |
| 並行上限 | 上限為正數；定義它限制的是正在執行的檢查，並避免無限制預先建立 task。已接收輸入清單可先存於記憶體，不需再加無限串流輸入需求 |
| 中止與收尾 | 中止後不再排新工作；依已定策略取消或等待執行中項目；等待 task 收尾，再產生可辨識的完成／失敗／取消／未開始結果。不要把未開始的網址算成網路失敗 |
| 輸出與錯誤 | 保留輸入索引以呈現穩定結果，重複 URL 也有明確身分；設定錯誤、輸入格式、HTTP 狀態、transport error、timeout、task panic／取消分清處理位置 |
| 外部依賴 | 固定 Tokio／reqwest features 與測試服務選擇；若加入 `tokio-util`、`futures` 或其他 crate，先補前置，不視為 Tokio 自帶知識 |

不必同時教 HEAD 探測、GET fallback、重試、認證、爬取與監控。選一種可解釋的請求政策即可。reqwest 的預設 redirect 會跟隨最多十次；是否採用應寫入契約，不能讓 library 預設替課程做未說明的決策。[S12]

| 階段 | 讀者要完成什麼 | 驗收與已教能力 |
| --- | --- | --- |
| B1 單一網址 | 從設定與一個 URL 產生有分類的結果 | 受控服務能回應正常、錯誤狀態與延遲 headers；測試 URL 解析、client、request／response、timeout 的契約 |
| B2 依序批次 | 對清單逐項執行、保留每筆身分與錯誤 | 混合有效／無效輸入不把整份結果吞掉；結果數量與原定政策對應 |
| B3 有上限的並行 | 重用 client，維持不超過上限的工作，收齊結果 | 受控服務記錄同時執行量；用同步訊號或清楚的事件順序驗證，避免只以很短的 sleep 猜排程。不要用真實公共網站的瞬時狀態當固定答案 |
| B4 中止與完成品 | 取消、收尾、摘要、JSON、CLI 與完整測試 | 在尚有排隊與執行中工作時觸發可控制的中止來源，驗證不再排新工作且沒有遺留 task；CLI 的 Ctrl-C 介面沿用已教取消契約 |

41 的受控服務需先教啟動完成通知、動態 port、回應控制與關閉。只提供一個叫 `start_test_server()` 的 helper，並不足以交代這些關係；但也不需要教完一套 Web framework 的內部。若用測試服務 crate，先教本課程實際使用的介面即可。

### 7.4 引導型正文與完整參考版如何配合

原稿 L256–261 的七項方向應保留，並補上以下可執行安排：

1. 每個里程碑先給可觀察需求與一小組輸入／預期結果，再給必要介面與起始檔案。不要把核心演算法先藏在骨架 helper 裡。
2. 任務先讓讀者決定資料如何流動、誰持有什麼，再寫一個可測試部分。提示依序提供思考方向、相關已學能力、局部結構，完整實作另有入口。
3. 驗收包含一般情況與對本階段有意義的失敗；測試本身所有 API、泛型與設定都在前置表內。必要的操作說明不能只藏在最深一層提示。
4. 每階段保留能執行的完成快照，對應引導步驟與最終版；起始版若故意未完成，清楚指出待完成範圍與如何檢查進度。不要讓前一章引用後來才加入的功能。
5. 完整參考版是一份完整可用的程式，附建置、安裝／使用、錯誤政策與設計理由；不只是所有片段的連結集合，也不採用主線未教過的另一套高手寫法。
6. 正文用小節回收設計選擇，例如為何先按大小分組、為何保存每筆結果、為何取消後仍要等待。讓讀者能解釋選擇，而非只是把空格補完。

這些工作本身就是最終章的重要篇幅。兩題是否必須由每位讀者都親手完整重做，與是否都列在必修教材中仍可分開討論；本報告的預設是兩題都完整教，不以「自行選一題」默默降低既定進階與整合要求。

## 8. 合理保留、未驗證與需要另行決定的取捨

| 項目 | 裁定與理由 |
| --- | --- |
| 數值 struct 先於 String 教 move | 保留。能先隔離 ownership 規則，避免把 move 誤當成字串專屬行為；須在 12 回收 owned heap 資料，不把 heap 圖當成已驗證的操作模型 |
| 先教 concrete slice，再教文字 | 保留。先建立連續資料的借用，再加入 UTF-8；不要求把 Iterator 與 grapheme 處理一起塞進 12 |
| 先教 macro／derive 使用，晚些實作 macro | 保留。操作契約與生成機制是不同教學深度；只要未教內容不出現在當下必要推理中，並不違反先教再用 |
| 完整 Error 與 Box／dyn 延至 30 | 保留。官方 Error 的 source 介面確有 trait object 與 lifetime；前面用具體錯誤完成 CLI 有足夠教學價值。[S2] |
| async 使用先於 poll／Pin 實作 | 保留。Future 的等待行為、task 與 runtime 可以先教；不需要為第一次 async 使用提前帶入整套 unsafe pinning。Pin 的地址穩定與 Drop 契約仍必須在深入實作時完成。[S13] |
| 全部進階必修 | 保留。拆節、調整位置與以有限題目驗收，都是減少負擔的方式；不以刪除 HRTB／GAT／variance／unsafe／FFI／Pin 解決難度 |
| 最終專案不強迫使用 unsafe／FFI／Pin | 保留。這些主題的必要性應由適合的專題驗收，不因最終程式安全且使用成熟 library 就判定課程未深入 |
| 一律把每個未列出的標準 trait 判成缺漏 | 不採用。只有實際選用 API、型別、推理與診斷需要時才成為必要前置；本報告提出的條件式項目仍要等實際選例核對 |
| 每節閱讀時間、整課總時數、是否確實好學 | 未驗證。缺少教材篇幅、實際任務與目標讀者資料，不提供虛構的工時或通過率 |
| 兩個完整參考版行數、功能與可用性 | 未驗證。沒有試作、編譯、測試、安裝或實際使用；第 7 節只有範圍與驗收建議 |
| 外部 crate 的確切 API／feature 全部閉合 | 未驗證。已核對指定技術疑點，尚未選定所有依賴版本與測試服務；版本凍結後須依實際介面重查 |

本報告提出的主要修正不需要改變使用者目標。只有下列情況需要使用者另行決定：將進階題目改成選讀；減為只完整教一個最終專案；把精確實體儲存回收量／一致快照或更多 HTTP 策略加入完成版；或把 51 個單元視為不可拆的固定篇幅上限。這些都會改變範圍或學習負擔，不能由作者悄悄選擇。本次沒有要求對尚不存在的實作作批准。

## 9. 官方來源與查證邊界

以下來源於本次審閱中實際讀取，查證日期為 2026-10-14。Rust 標準 library 頁面顯示 1.99.0，與專案固定工具鏈相符；第三方 docs.rs 的 `latest` 是浮動入口，未代替未來教材的版本鎖定。連結用於支持下列具體技術判斷，並不表示官方教學順序自動適合本課程。

| 編號 | 原始／官方發布來源 | 本次核對的內容 |
| --- | --- | --- |
| S1 | [std::collections::HashMap][S1] | `get` 的 `Borrow`／`Hash`／`Eq`／`?Sized`、借用鍵一致性、entry 回傳型別 |
| S2 | [std::fmt::Display][S2]；[std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html) | Formatter／格式化回傳型別、ToString 關係、Error 的 Debug／Display 與 source 簽名 |
| S3 | [std::thread::scope][S3] | HRTB 簽名、`'scope`／`'env`、結束前 join 的保證 |
| S4 | [std::thread::Result][S4] | panic payload 的 `Box<dyn Any + Send + 'static>` 與不保證實作 Error |
| S5 | [Rust Reference：impl trait][S5] | return-position `impl Trait` 的 generic capture 與 edition 2024 差異 |
| S6 | [std::io::Read][S6]；[std::fs::read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html) | Read 簽名與短讀取；目錄 iterator 可在開始後產生錯誤、順序不固定 |
| S7 | [std::path::Path][S7] | path 顯示物件、非 Unicode 資料與失真顯示的限制 |
| S8 | [clap::Parser][S8] | `try_parse_from` 的輸入、bound 與錯誤介面 |
| S9 | [serde::DeserializeOwned][S9]；[serde_json::from_str](https://docs.rs/serde_json/latest/serde_json/fn.from_str.html)；[serde_json::from_reader](https://docs.rs/serde_json/latest/serde_json/fn.from_reader.html) | owned／borrowed 反序列化契約、HRTB 與不同 API bounds |
| S10 | [Tokio JoinSet][S10]；[Tokio JoinHandle](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html) | task 脫離 handle、abort 與等待取消完成、`join_next` 的取消安全 |
| S11 | [Tokio select!][S11] | 多分支等待、取消與分支停用的行為 |
| S12 | [reqwest::ClientBuilder][S12] | total timeout、read／connect timeout 與預設 redirect 上限 |
| S13 | [std::pin][S13]；[std::task::Wake](https://doc.rust-lang.org/std/task/trait.Wake.html) | 地址敏感資料、Unpin／Drop 契約；以 Wake 安全建立 Waker 的途徑 |
| S14 | [rust-lang/miri][S14]；[Loom 官方發布文件](https://docs.rs/loom/latest/loom/) | Miri 的 nightly 與外部操作限制；Loom 的模型與替代同步操作 |
| S15 | [Cargo Book：Workspaces][S15] | workspace 共用 lockfile／輸出目錄與根設定責任 |
| S16 | [walkdir::WalkDir][S16] | traversal builder、iterator、順序與 link／loop 的處理；最終選用政策仍待固定 |

`serde.rs/lifetimes.html` 本次請求回傳 403，未宣稱讀過該頁；相關判斷改由 S9 的官方發布 API 文件核對。沒有查證具體目標平台的實體儲存回收方法、完整 C toolchain 組合、尚未選定的本機 HTTP 測試 crate，以及兩個未實作專案的行數與行為。這些保留為明示待決／待驗證，沒有用不確定處推定教材已錯。

[S1]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[S2]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[S3]: https://doc.rust-lang.org/std/thread/fn.scope.html
[S4]: https://doc.rust-lang.org/std/thread/type.Result.html
[S5]: https://doc.rust-lang.org/reference/types/impl-trait.html
[S6]: https://doc.rust-lang.org/std/io/trait.Read.html
[S7]: https://doc.rust-lang.org/std/path/struct.Path.html
[S8]: https://docs.rs/clap/latest/clap/trait.Parser.html
[S9]: https://docs.rs/serde/latest/serde/de/trait.DeserializeOwned.html
[S10]: https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html
[S11]: https://docs.rs/tokio/latest/tokio/macro.select.html
[S12]: https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html
[S13]: https://doc.rust-lang.org/std/pin/index.html
[S14]: https://github.com/rust-lang/miri
[S15]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[S16]: https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html

