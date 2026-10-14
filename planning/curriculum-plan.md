# learn_rust 課程規劃：Astra 審閱用凍結稿

- 版本：v1，2026-10-14。
- 狀態：討論稿；尚未獲得開始撰寫全部教材的指示。
- 本次用途：獨立課程規劃審閱；不是教材逐段首次閱讀審閱，也不代表任何教材或範例已驗收。
- 專案：`/workspace/learn_rust`。
- 結構：15 個大章、51 個循序教學單元。單元是小段落，不代表每個都會是很長的章節。
- 凍結原則：保留本稿供對照；審閱建議另存，不直接覆寫本稿。

## 1. 使用者已確認的目標與標準

本節是審閱時不可忽略的使用者要求；後面的具體排序仍是待審的規劃提案。

1. **好懂、好學是最高優先。**教學動機、理解負擔與漸進順序優先於作者方便、教材常見順序及平行寫作速度。
2. **讀者懂基本 Python。**不假設懂 C／C++、pointer、系統程式設計、HTTP、複雜型別系統或建置工具；必要的領域知識由教材負責建立。
3. **目標是深入掌握 Rust。進階篇也列為必修。**不能以選讀、外部連結或未讀附錄替主線補必要知識。
4. **每個教學 example code 都只能使用前面實際教過的東西。**概念、語法、型別、API、必要的 trait bound、回傳型別與錯誤不能提前出現。新概念在本單元也必須先教清楚，再用於程式。
5. **只有出現名稱不算教過。**介紹至少交代當下必要的意義、角色、輸入／輸出或行為，以及與前文的關係。沒有必要提前揭露所有內部實作，但不能以此藏掉讀懂範例所需的前提。
6. **檢查範圍包括所有教學材料。**程式片段、完整範例、型別簽名、helper、測試、練習、提示、參考答案、必要 Cargo 設定與引用的編譯器訊息，都要核對前置教學。
7. **嚴格避免先用 Box 或動態指標／dyn，再說後面會介紹。**不可用黑箱 helper、第三方 crate 或一次塞入很多新概念的整合範例繞過這項要求。
8. **Cargo／crate 從開頭開始。**第一次建立、執行教材範例，就說明所需的 package／crate／workspace 關係與指令；module／library crate 在函式與基本控制流程之後早期介紹，不集中拖到中後段。
9. **lifetime 明確列為必修。**先教 reference 的有效範圍，再教 annotation，最後進階；不可混同 scope、值存活時間、reference 有效性或地址穩定性。
10. **可以安排不同 subagent 平行寫大章節，但所有作者都要守同樣的前置規則。**排程依實際完成的前文與必要概念，不依尚未兌現的大綱；學習順序優先於加速。
11. **最後一章放一到兩個最終練功專案。**規模約 1,000 行上下，不需精準；可以使用第三方 library，重點是份量適中、有完成感。
12. **最終實戰採引導型寫法。**讓想練習的讀者邊看邊寫；提供逐步任務、分層提示與驗證，再另提供功能完整、實際可用的參考實作，供需要時查答案。
13. **維持已對齊的術語風格。**正體中文敘述、台灣技術用語；核心英文術語保留，但第一次需要時要用中文教懂。完整規則見專案 clear-tutorial 指引。

使用者特別強調的原話：

> 每個教學的 example code 都只能用前面有教過的東西，平行寫的 subagent 要能遵守這一點。

> 同時要以好學好理解為最優先準則。

> 假設懂基本 python 的程度。

> 以深入掌握 Rust 為目標，進階篇也列為必修。

### 術語約定

| 類別 | 本專案寫法 |
| --- | --- |
| stack／heap | 英文；不以「棧／堆」作主要名稱 |
| ownership／borrowing／move | 術語英文，動作以自然中文敘述，例如「函式借用這個值」 |
| reference／pointer | 英文為主，首次補中文意思，再解釋用途 |
| mutable／immutable | 直接英文 |
| generics | 首次「泛型（generics）」，之後「泛型」 |
| trait／lifetime／closure／slice／crate | 核心術語保留英文 |
| 變數／函式／型別／編譯器／記憶體 | 使用中文 |
| 關鍵字、API、型別名 | 保留原文、大小寫及程式碼格式 |

專案指引：`/workspace/learn_rust/.agents/skills/clear-tutorial/SKILL.md`，以及 `references/rust-terminology.md`。單詞對照不代替概念教學。

## 2. 完整排序

以下全部列入主線必修。這是一份內容與排序規劃，尚未證明每個實際範例的依賴都已滿足。

### 大章 1：工具與程式基礎

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 1 | 環境、Cargo 與第一個程式 | rustup、rustc、Cargo 的角色；cargo new/check/build/run；Cargo.toml、src/main.rs、edition、package 與 binary crate；先介紹 main、字串文字與 println! 的基本使用方式，再展示第一段程式。 |
| 2 | 教材 workspace 與執行方式 | workspace 的基本關係、教材範例 package、-p、--locked、Cargo.lock 與指令執行位置。讀者會使用的指令與設定在首次使用前說明。 |
| 3 | 基本型別、變數與運算 | 數字、布林值、型別推導、mutable／immutable、shadowing 與基本運算。 |
| 4 | expression、block、函式與條件 | expression、block、函式參數與回傳值、分號、if 與條件判斷。 |
| 5 | tuple、array、range 與迴圈 | tuple、array、range、索引、解構、loop／while／for；先建立迭代操作的基本意義。 |
| 6 | module、library crate 與基本測試 | mod／use／pub、路徑、拆檔、library／binary crate 分工；使用已學的數字運算函式，加入最基本的測試與 assert 用法。 |

### 大章 2：ownership、reference 與文字

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 7 | struct 與資料模型 | 先使用簡單數值欄位，練習 struct 的建立與存取；不提前使用尚未介紹的 derive。 |
| 8 | 記憶體模型與 ownership | stack／heap、scope、ownership、move、基本型別的 Copy 行為；以簡單 struct 呈現 move，交代讀懂必要診斷所需的 Copy 意義。 |
| 9 | reference、borrowing 與有效範圍 | shared／mutable reference、borrowing 規則、最後一次使用、reference 的有效範圍，以及 lifetime 與 scope 的差別；先不使用 lifetime annotation。 |
| 10 | impl、method 與 self | impl、method、Self、self／&self／&mut self；取得 ownership、借用與修改資料的不同介面。 |
| 11 | slice | 先用數值 array 教 concrete slice，例如 &[i32]，理解一段資料的借用。 |
| 12 | String、str 與 UTF-8 | String、str、&str、UTF-8、byte 與字元、長度與切片限制；必要 API 先解釋再使用，不提前使用尚未教過的 Iterator adaptor。 |

### 大章 3：型別、trait 與 lifetime

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 13 | enum 與 pattern matching | 自訂 enum、variant、pattern、match、if let、let else 與解構；先不靠尚未介紹的 Option／Result 教 enum。 |
| 14 | 泛型、Option 與 Result | 泛型（generics）、型別參數、Option／Result、Some／None／Ok／Err；建立缺值與成功／失敗的表示方式。 |
| 15 | trait、bound、associated type 與 derive | 自訂 trait、bounds、associated types；Clone／Copy／Debug、derive、除錯輸出與靜態 dispatch 的基本角色。 |
| 16 | lifetime annotation | lifetime annotation、多個 reference 的關係、回傳 reference、含 reference 的 struct、elision、&'static T 與 T: 'static；annotation 不會延長值的存活時間。 |

### 大章 4：Cargo 生態系與開發流程

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 17 | 外部 dependency 與 crate 生態系 | crates.io、docs.rs、cargo add、dependencies／dev-dependencies、版本需求、lockfile、cargo update、基本 features 與 dependency 名稱／crate 路徑的關係。 |
| 18 | 測試、文件與日常工具 | unit／integration／doc tests、文件註解、cargo test／fmt／clippy／doc、閱讀編譯器訊息與可重現的日常開發流程。 |

### 大章 5：collections 與錯誤處理

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 19 | Vec | Vec 的建立、增刪、存取與借用；先不用尚未介紹的 Iterator adaptor，必要的 slice 轉換明確交代。 |
| 20 | HashMap、Eq 與 Hash | HashMap 的鍵值模型、查詢、更新、Eq／Hash，以及查詢結果與更新時的 borrowing 關係。 |
| 21 | panic、具體錯誤型別與 Display | panic、可恢復失敗與程式契約、具體錯誤 enum、Display；完整 std::error::Error 延至 dyn 已教過之後。 |
| 22 | 型別轉換、FromStr 與問號運算子 | From／Into／TryFrom／AsRef／FromStr；型別與錯誤轉換；再教 ? 的傳遞、回傳與轉換行為。 |

### 大章 6：closure 與 Iterator

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 23 | closure、function pointer 與 Fn 系列 | closure、function pointer、捕捉環境、借用與 move、Fn／FnMut／FnOnce。 |
| 24 | Iterator 與 IntoIterator | Iterator／IntoIterator、associated type、惰性執行、adaptor、消耗、collect／FromIterator；回頭整合文字與 collections 的處理。 |

### 大章 7：I/O 與第一個完整 CLI

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 25 | 同步 I/O、path 與 CLI 輸入 | 標準輸入輸出、檔案、分塊讀寫、path、命令列參數、相關型別與 I/O 錯誤；API 在整合專案前先各自練習。 |
| 26 | clap、serde 與 JSON | 以小練習介紹 clap、serde／serde_json：必要型別、trait、derive、輸入輸出、錯誤與 feature 設定。安裝 dependency 不等於已教過 API。 |
| 27 | 第一個完整 CLI 小專案 | 文字搜尋或統計 CLI，整合參數、檔案、Iterator、具體錯誤與測試；比最終專案小，不使用尚未教過的 Box<dyn Error>。 |

### 大章 8：smart pointer 與進階型別

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 28 | Sized、DST 與 Box | Sized、dynamically sized types（DST）、Box、大小、間接存放、遞迴資料與 heap 配置。 |
| 29 | Deref、coercion 與 Drop | Deref、coercion、Drop、自訂型別的借用介面與資源收尾；與既有型別的使用經驗連接。 |
| 30 | dyn、trait object 與 std::error::Error | dyn、trait object、動態 dispatch、dyn 相容性；此時完整介紹 std::error::Error、source 錯誤來源鏈與 Box<dyn Error>。 |
| 31 | Rc、Weak、Cell 與 RefCell | Rc、Weak、Cell、RefCell、共享 ownership、interior mutability、借用 guard 與循環；逐一教過再組合。 |
| 32 | 進階型別與 trait 設計 | impl Trait、type alias、newtype、coherence／orphan rules、運算子 trait、Any 與 downcast。 |
| 33 | 進階 lifetime、HRTB、GAT 與 variance | 進階 lifetime：higher-ranked trait bounds（HRTB）、generic associated types（GAT）、variance，以及對介面設計與 borrowing 的影響。 |

### 大章 9：Cargo 工程設定與檔案處理套件

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 34 | Cargo 進階設定與建置 | workspace 進階設定、共用依賴、profiles、features、cfg、build.rs、target 與平台差異；在 FFI 等需要建置設定前先教。 |
| 35 | walkdir 與串流 hash 練習 | walkdir、blake3、分塊計算 hash、候選檔案比較；用小練習建立重複檔案搜尋器的必要先備。 |

### 大章 10：thread 與同步

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 36 | 並行模型、Send 與 Sync | 並行與平行的模型、Send／Sync、ownership 與資料跨 thread 的限制、T: 'static 與實際存活時間的區別。 |
| 37 | thread、scoped thread 與 channel | thread、scoped thread、spawn／join、channel、傳送資料、錯誤與收尾；先備包括 closure、lifetime，以及相關錯誤型別。 |
| 38 | Arc、Mutex 與 RwLock | Arc、Mutex、RwLock、共享狀態、guard、poisoning、死鎖與鎖的範圍。 |

### 大章 11：async 與 HTTP

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 39 | Future、async/await 與 Tokio runtime | Future、async／await、Tokio runtime、執行模型、等待與 blocking 工作；先教使用，不提前展示尚未教過的 poll／Pin 實作。 |
| 40 | async task、流量控制與取消 | async task、並行數量限制、timeout、取消、select! 與收尾；必要 runtime feature、型別、錯誤與 API 先教。 |
| 41 | HTTP、reqwest 與受控測試服務 | HTTP／URL／請求與回應／狀態碼的必要基礎；reqwest client 重用、錯誤與耗時；介紹受控本機測試服務與必要測試 API，為最終網址專案建立先備。 |

### 大章 12：atomic、unsafe 與 FFI

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 42 | atomic 與 memory ordering | atomic、原子操作、可見性、memory ordering 與同步關係。 |
| 43 | raw pointer 與 unsafe | raw pointer、unsafe、有效性、aliasing、資料競爭、安全契約與 soundness。 |
| 44 | FFI 與 ABI | 先補必要的 C 基礎，再教 FFI、ABI、資料布局、字串、呼叫與 ownership 邊界；使用的建置設定已有前置教學。 |

### 大章 13：Pin 與 async 內部機制

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 45 | self-reference、Pin 與 Unpin | self-reference、Pin、Unpin、地址穩定性與安全使用條件；連回 async 的需求。 |
| 46 | Poll、Context、Waker 與 executor | Poll、Context、Waker、手動實作 Future、簡易 executor；連回單元 39 的執行模型。 |

### 大章 14：macro、API 設計與驗證

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 47 | macro_rules 與 procedural macro | macro_rules!、hygiene、procedural macro 與 derive 的實作；需要的 proc-macro crate 設定與第三方 API 先教再用。 |
| 48 | library API 設計與相容性 | library API、公開介面、ownership／錯誤邊界、feature 契約、版本相容性與發布前檢查。 |
| 49 | 進階測試與驗證工具 | property testing、並行測試與 unsafe 檢查工具；練習依問題選擇驗證方式，工具／crate 必要 API 在使用前介紹。 |
| 50 | 效能量測與改善 | release 設定、benchmark、profiling、配置與複製成本；公平比較、量測範圍與改善依據。 |

### 大章 15：最終練功專案

| 單元 | 主題 | 預定教學範圍與限制 |
| --- | --- | --- |
| 51 | 兩個最終練功專案 | 兩個引導型最終練功專案：重複檔案搜尋器、async 網址檢查器；各自提供逐步任務、分層提示、驗收方式與完整可用的參考實作。 |

## 3. lifetime 的三層安排

- 單元 9：reference 的有效範圍、借用何時仍在使用，以及與 scope 的差別；尚不需要 annotation。
- 單元 16：annotation 表示借用關係，不能延長值存活；回傳 reference、含 reference 的 struct、elision、&'static T／T: 'static。
- 單元 33：HRTB、GAT、variance 與進階介面問題；單元 36–38 再於跨 thread 限制中應用。
- Pin／self-reference 放在單元 45；不把 lifetime 說成解決地址穩定性的工具。

## 4. 幾個明確的順序約束

1. 第一段 Rust 程式前先教 main、println! 與使用的字串文字形式。macro 的使用意義先教，macro 的實作則在單元 47 深入。
2. 單元 7 的 struct 不先用 derive；單元 8 只教讀懂 move／Copy 所需的能力意義；trait 與 derive 語法在單元 15 正式教。
3. 單元 12 不提前使用尚未介紹的 .chars() 等 Iterator 操作；相關處理在單元 24 再整合。
4. String／Vec 的早期借用範例要明確教所用的轉換 API；不拿尚未說明的自動 Deref coercion 當成讀者已懂。
5. 單元 21 的錯誤章先使用具體錯誤與 Display；單元 27 小專案延續這種設計。
6. 完整 std::error::Error 的 source 介面含 dyn 與 lifetime 關係，因此與 Box<dyn Error> 一起等到單元 30。
7. thread 的型別／錯誤中涉及的 closure、Send、'static、Box、dyn、Any，要在必要程式或診斷出現前已有足夠教學。
8. 單元 39–41 先教 async 使用，不提前展示尚未教過的手寫 poll 或 Pin 型別簽名；單元 45–46 才實作其內部模型。
9. 第三方 crate 先在相關章節教必要型別、API、輸入輸出、錯誤與 feature，再用於整合範例。
10. C／HTTP／hash／JSON 等領域知識也算先備；不能只核對 Rust 關鍵字。

## 5. 範例與專案的組織

- 可獨立執行的範例放在自己的 Cargo package，加入根 workspace；初期通常只有一個 binary crate。
- 單元 6 以已懂的函式介紹 module、library crate 與 binary crate 的分工；小範例不必都拆成多個 crate。
- 連續專案的不同教學階段保留可執行版本，避免前章引用後章才增加的程式。
- 文件引用真正的範例來源，不維護第二份漂移的程式碼。
- package 名稱、依賴版本、workspace 共用設定與整合由主 agent 統一協調，避免平行作者互相修改共用檔案。

## 6. 最後一章：兩個最終練功專案

本稿目前提案為兩個互補專案。每個預估約 800–1,200 行手寫 Rust，包含測試、文件另計；尚未實作量測，不以湊行數為目標。

### 專案 A：重複檔案搜尋器

指定一個資料夾，找出內容相同的檔案，列出群組與重複占用的空間。

核心功能：

- 遍歷資料夾，收集路徑與檔案大小。
- 先按大小篩選，分塊讀取計算 hash，再確認候選檔案的實際內容。
- 整理與排序群組，輸出文字或 JSON 報告。
- 回報讀取失敗，測試空檔案、同內容不同檔名、同大小不同內容等情況。

第三方 crate：walkdir、blake3，搭配共通的 clap、serde／serde_json。

主要整合：資料模型、ownership、borrowing、Iterator、collections、檔案 I/O、錯誤處理、library／binary 分工與測試。

里程碑：列出檔案 → 找出重複內容 → 報告與錯誤 → CLI 與完整測試。

完成版範圍為搜尋與報告；刪除、同步、背景監控留作延伸題。

### 專案 B：async 網址檢查器

讀入網址清單，在指定並行數量下發送 HTTP 請求，整理狀態碼、耗時、逾時與連線失敗。

核心功能：

- 解析輸入與設定，先完成單一網址檢查。
- 重用 HTTP client，設定每個請求的 timeout。
- 限制同時執行的任務數量，收集成功與失敗結果。
- 支援中止與收尾，輸出摘要及 JSON。
- 以受控的本機測試服務驗證正常回應、錯誤狀態及逾時。

第三方 crate：Tokio、reqwest，沿用共通的 clap、serde／serde_json；測試服務所需 crate 的 API 也必須先教。

主要整合：async／await、task、ownership 跨任務轉移、Send／'static、timeout、取消、錯誤邊界與測試。

里程碑：單一請求 → 依序批次 → 有上限的並行 → 中止、報告與測試。

爬取網頁、登入、定期監控、資料庫與 dashboard 留作延伸題。

### 引導型正文與完整參考實作

1. 先介紹使用情境、功能範圍、操作方式與預期結果。
2. 每個里程碑提出明確任務，給必要的介面或起始骨架，讓讀者自己寫核心邏輯。
3. 提示由思考方向逐步到已學概念／API，保留動手完成的空間。
4. 每個階段提供測試資料、預期行為與驗證方式。
5. 完整參考版另有清楚入口；功能完整、可執行，具備錯誤處理、測試、安裝／使用說明與設計理由。
6. 引導正文、提示、測試與完整參考版使用相同的已教知識範圍。
7. unsafe、FFI、Pin 等由前面的專題練習驗收；最終專案著重自然的整合與設計。

## 7. 平行寫作規劃

| 實際完成並確認的前置 | 預定可平行的工作 |
| --- | --- |
| 單元 1–18 | collections 19→20；錯誤 21→22；closure 23；Box／Deref／Drop 28→29 |
| 各分支必要單元已完成 | Iterator 24 等待 19＋23；I/O 25 等待 collections、錯誤、Iterator；dyn 30 等待必要型別、錯誤與 smart pointer |
| 共通前置至單元 35 | thread 36→38；unsafe／FFI 43→44；macro 47；async 接 thread，Pin 等待 unsafe 與必要 async |
| 最終專案必要前置已完成 | 兩個專案分別由不同 subagent 撰寫，各自負責獨立 package |

排程規則：

- 每位作者取得實際完成的前文、允許使用的概念／API 清單及實際介紹位置。
- 只授權使用其已確認前置，不把待寫章節的標題當成已教證據。
- 新 API 的必要使用知識也列入清單；不得用 helper 或 dependency 藏掉未教內容。
- 若範例需要未完成的前置，調整例子或等待前置完成；不可先用再補。
- 作者自查後，仍需按實際讀者順序檢查銜接；技術可編譯不代替首次閱讀證據。
- 課程依賴圖與作者排程圖分開：讀者順序不因寫作分工改變。

## 8. 已列出的主要前置關係

下表供規劃審閱檢查，不是完整 API 白名單。尚無教材正文，因此不能把這張圖當作所有範例已通過驗收。

| 單元 | 目前列出的主要前置單元 |
| --- | --- |
| 1 | 無 |
| 2 | 1 |
| 3 | 2 |
| 4 | 3 |
| 5 | 4 |
| 6 | 5 |
| 7 | 6 |
| 8 | 7 |
| 9 | 8 |
| 10 | 9 |
| 11 | 9 |
| 12 | 10、11 |
| 13 | 7、10 |
| 14 | 13 |
| 15 | 14 |
| 16 | 12、15 |
| 17 | 6、16 |
| 18 | 17 |
| 19 | 18 |
| 20 | 19、15、16 |
| 21 | 18、15 |
| 22 | 21 |
| 23 | 18、15、16 |
| 24 | 19、23、15 |
| 25 | 20、22、24 |
| 26 | 25、17、15 |
| 27 | 26 |
| 28 | 18、11、12、14 |
| 29 | 28 |
| 30 | 29、19、22、16 |
| 31 | 29、21 |
| 32 | 30、31、24 |
| 33 | 32、16、23、31 |
| 34 | 25、18 |
| 35 | 25、24、26 |
| 36 | 32、33 |
| 37 | 36、23、30 |
| 38 | 37、31 |
| 39 | 27、38 |
| 40 | 39 |
| 41 | 40、26 |
| 42 | 38 |
| 43 | 33、29 |
| 44 | 43、34 |
| 45 | 43、40、33 |
| 46 | 45 |
| 47 | 27、32、34 |
| 48 | 30、33、34、47、41 |
| 49 | 38、43、44、46、47、48 |
| 50 | 34、42、49 |
| 51 | 35、41、48、49、50 |

已做的機械核對僅為：51 個單元的上述前置均存在且在它們之前；尚未確認這份主要前置清單是否完備。

## 9. 研究參考

已參考目錄與代表性內容，並未完成對所有教材的逐頁審閱。以下用於取材、理解教法與比較，不把它們的既有排序當成自動正確的答案。

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Brown 互動版 Rust Book](https://rust-book.cs.brown.edu/)
- [Google Comprehensive Rust](https://google.github.io/comprehensive-rust/)
- [Rustlings](https://rustlings.rust-lang.org/)
- [Rust Book 正體中文版](https://rust-lang.tw/book-tw/)
- [Rust Book 簡體中文版](https://kaisery.github.io/trpl-zh-cn/)
- [Rust 語言聖經](https://github.com/sunface/rust-course)
- [Programming Rust, 2nd Edition](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/)
- [Rust for Rustaceans](https://nostarch.com/rust-rustaceans)

本次送審只要求提出規劃建議；教材撰寫、實驗、依賴安裝與重新排序的實際修改另依後續指示處理。
