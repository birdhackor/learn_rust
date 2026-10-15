# learn_rust 課程規劃 v2

- 日期：2026-10-14；本版為主 agent 讀完 Astra（max）審閱後的判斷與修訂。
- 仍為討論中的課程規劃；此輪工作是更新規劃文件。
- 原始送審稿 [v1](curriculum-plan.md) 與 [Astra 完整報告](curriculum-review-astra.md) 保留不變。
- 15 個大章，全數必修；原本過大的單元拆為短小節，按本文件閱讀順序進行。
- 小節編號是本版新編號。小節可再分段，不代表固定頁數、工時或每個都是大型章節。
- 知識前置表目前為規劃狀態；首次介紹位置尚待正文與真實程式逐項驗收。

## 1. 我的判斷與採納範圍

最需要修正的是：原稿以大標題承擔過多不同理解問題，且把閱讀順序與真正的 API 依賴混在一起。修訂保留完整 Rust 深度，以小成果、使用契約與實際前置減少負擔。

具體取捨：

1. enum／Option／Result 放到安全文字存取前，讓新 API 的回傳型別已有基礎。
2. Vec 在 3 章出現，先於 trait 全貌、完整 lifetime bounds 與外部工具；用已懂操作完成小成果。
3. 基本 trait／derive 與 HashMap 的 Eq／Hash／Borrow 契約在 4 章分節；associated type 移到 Fn／Iterator 前。
4. lifetime 分層：2 章有效性、4 章 annotation、8 章 bounds／'static、10 章 HRTB、12 章 GAT／variance；全部主線必修。
5. type alias 的最低使用在 Display 前；impl Trait 的最低使用在 iterator helper 前。
6. atomic 接在同步機制後，再進 async／HTTP，以延續同一個同步問題並支援測試計數。
7. 先使用 async，再回頭實作 Pin／poll；所選早期例子避免需要尚未教過的 pin!／stream helper。
8. 兩個最終專案保留自然用途，先固定行為與驗收，約千行含測試，不增加第三題或無關功能。

| Astra 項目 | 我的決定 | v2 落實方式 |
| --- | --- | --- |
| N1 | 採納 | 訂明 API 使用契約與實作深度的界線；每段程式有首次介紹位置與已完成前文版本。 |
| N2 | 採納 | 15 大章保留，密集原單元拆成短小節；各小節有一個主要問題與可驗收成果。 |
| N3 | 採納並分段 | 基本 annotation 在 4 章；'static／bounds 在 8 章；HRTB 在 scoped thread 前；GAT／variance 在 12 章，全部必修。 |
| N4 | 按選定 API 採納 | ()、usize、reference 上的 *、Borrow／Eq／Hash、格式化、alias、impl Trait、排序都有預定位置；完整程式出現後再核對真正需要的項目。 |
| N5 | 採納 | I/O 與第三方 API 先做可觀察的小練習，初期 serde 選 owned 模型／from_str，測試工具也先教。 |
| N6 | 採納並選主路線 | bounded JoinSet、可控制的 channel 中止來源、取消後等待收尾；不另加未教的 tokio-util／futures，Pin 使用需要先教。 |
| N7 | 採納 | 所有進階各有有限完整產物、理由與局部驗證；14 章綜整工具，不首次才檢查正確性。 |
| N8 | 採納並具體化 | 固定兩個實戰的功能、錯誤、取消、輸出與驗收規則；約千行含測試，保持範圍。 |
| I1 | 採納 | Vec 與小成果在 3 章；HashMap 在 4 章，先於完整 lifetime bounds／Cargo 生態系。 |
| I2 | 採納 | Cargo 按需要分段，第一章含 crate／workspace／拆檔與基本測試，fmt／clippy 早提供回饋。 |
| I3 | 採納 | 章節由讀者已懂的工作引出新問題，必要解釋留正文。 |
| I4 | 採納 | 第一章 library 分工、4 章 borrowed／owned 選擇、5／8 章錯誤、7 章 CLI 邊界逐步累積設計判斷；14 章綜整。 |

不把每個未列出的標準 trait 都變成一節；只補所選程式、必要推理或診斷真正需要的前提。Astra 的總排序是參考方案，本版採用上述有具體收益的移動與拆分，不把它當唯一正確順序。

## 2. 使用者標準

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

## 3. 完整閱讀順序與階段成果

每個小節先教當下必要意義，再展示程式。每大章有可見成果；高階題以有限產物與理由驗收，不膨脹成另一套通用框架。

### 第 1 章：工具與程式基礎

本章成果：建立、拆分並測試一個數字運算 library。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 1.1 | 起點、環境與學習路線 | 基本 Python 起點；編譯與執行、rustup／rustc／Cargo 的角色與安裝。 |
| 1.2 | 第一個可執行程式 | 先教 main 的最小形式、字串文字與 println! 使用契約；cargo new/check/build/run、基本 manifest 與 edition。 |
| 1.3 | 教材 package、crate 與 workspace | 所在資料夾、binary crate、成員選擇、-p／--locked、Cargo.lock、眼前用到的 workspace 繼承欄位。 |
| 1.4 | 基本型別、變數與運算 | 數字、bool、usize、型別推導、mutable／immutable、shadowing；需要時先教輸出格式。 |
| 1.5 | expression、block 與函式 | 參數、回傳值、()、分號與基本函式組合。 |
| 1.6 | 條件判斷 | if、比較與布林邏輯，沿用前節的數字運算任務。 |
| 1.7 | tuple、array 與 range | 解構、索引與邊界、長度、range 的範圍意義。 |
| 1.8 | 迴圈 | loop／while／for 的操作契約；先在 array／range 上使用。 |
| 1.9 | module、library crate 與 binary crate | mod／use／pub、路徑、拆檔；將已懂的函式移到可重用 library。 |
| 1.10 | 第一個測試與開發回饋 | 數值 assert、#[test]／#[cfg(test)] 的最低使用意義、cargo test/fmt/clippy、定位錯誤；此處不寫泛型 assertion helper。 |

### 第 2 章：struct、ownership 與 reference

本章成果：解釋一個值由誰持有、哪些函式可以借用或修改它。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 2.1 | struct 與資料模型 | 只用已懂的數值欄位建立簡單 struct，先不使用 derive。 |
| 2.2 | 記憶體模型、ownership 與 move | stack／heap 的概念模型、scope、move、數值的 Copy 行為；Copy 的必要能力意義先解釋，診斷只引用已懂部分。 |
| 2.3 | reference 與基本操作 | shared／mutable reference、&／&mut／* 的使用意義，以具體數值與 struct 練習。 |
| 2.4 | borrowing 與 lifetime 的基本意義 | 借用何時仍在使用、最後一次使用、重新借用、reference 有效範圍與 scope；此處不寫 annotation。 |
| 2.5 | impl、method 與 self | Self、self／&self／&mut self、修改欄位與從借用資料取值的限制。 |
| 2.6 | concrete slice | 由 array 借用 &[i32]，區間、長度及借用關係；不先用泛型 &[T] 教。 |

### 第 3 章：enum、泛型、文字與 Vec

本章成果：完成一個記憶體內的文字清單整理器。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 3.1 | enum 與 pattern | 自訂 enum、variant、match、if let、let else、基本解構及本例需要的借用模式。 |
| 3.2 | Option 與 Result | 先以具體型別與自訂錯誤 enum 說明缺值、成功／失敗；先不用 unwrap、? 或 boxed error。 |
| 3.3 | 泛型（generics） | 型別參數、型別推導、必要的 turbofish；再寫完整 `Option<T>`／`Result<T,E>` 與簡單泛型資料模型。 |
| 3.4 | String、str 與 &str | 擁有文字與借用文字、String::new／push_str／as_str 的具體契約，回收 heap 與 borrowing。 |
| 3.5 | UTF-8 與安全文字存取 | byte、Unicode scalar／char、len、已教 range；先 as_str 再使用 str 的安全 get，不先用未教的自動 Deref、.chars() 或 conversion trait。 |
| 3.6 | Vec 與第一個清單成果 | `Vec<T>`、vec!、增刪、索引／get、借用、as_slice；以已懂迴圈完成清單整理，Debug 輸出留下一章。 |

### 第 4 章：trait、HashMap 與基本 lifetime annotation

本章成果：完成可分組／統計的資料模型，能說明回傳 reference 的來源。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 4.1 | trait 與 bound 的基本角色 | 以單一數值操作定義、實作 trait，再用泛型 bound 重用操作；這裡先不放 associated type。 |
| 4.2 | 標準能力與 derive | Clone／Copy／Debug／PartialEq 的用途與行為；先教 derive 契約再用，clone 行為依型別判斷。 |
| 4.3 | HashMap 與鍵的契約 | Eq／Hash 與相等鍵的 hash 一致性；既有 String／數值鍵、Borrow 的具體借用查詢、非 closure 更新；不展示含 ?Sized 的完整 get 簽名。 |
| 4.4 | 回傳 reference 與 annotation | 由哪個輸入借出資料、lifetime parameter、elision／'_、必要 outlives 關係；annotation 不延長存活，並比較回傳 owned value。 |
| 4.5 | 含 reference 的 struct 與 method | 多個借用來源、持有 reference 的資料模型、method elision；不把整套 'static 與進階 lifetime 塞進本節。 |

### 第 5 章：錯誤、格式化與轉換

本章成果：給資料處理函式設計具體錯誤與可讀訊息，能傳遞失敗。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 5.1 | 失敗政策與 panic | 可恢復失敗與程式契約、具體錯誤 enum、unwrap／expect／panic 的明確行為與取捨。 |
| 5.2 | type alias 的最低使用知識 | 先讀懂並寫簡單 type alias；提供後續 fmt::Result／io::Result 必要基礎，不混同 newtype。 |
| 5.3 | Display 與格式化契約 | Formatter<'_>、write!、fmt::Result／fmt::Error、必要生命週期與 trait method；不先用 ?。 |
| 5.4 | 可失敗與不可失敗的型別轉換 | From／Into／TryFrom 與錯誤轉換，選用真正有需要的例子。 |
| 5.5 | 解析與借用式轉換 | FromStr、AsRef、ToString／ToOwned 的實際用途；與已教 Borrow 的借用查詢契約區分。 |
| 5.6 | ? 與錯誤傳遞 | 先以手動傳遞比較，再教 ? 及其錯誤轉換；此章不先用 map_err closure、std::error::Error 或 `Box<dyn Error>`。 |

### 第 6 章：closure、associated type 與 Iterator

本章成果：將清單、文字與統計整理成可解釋的資料處理流程。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 6.1 | closure 捕捉環境 | 借用、修改與 move capture，沿用資料整理的小工作。 |
| 6.2 | associated type | 以一個有限介面建立 associated type 的角色，為 Fn／Iterator 使用提供前提。 |
| 6.3 | Fn 系列與 function pointer | Fn／FnMut／FnOnce、輸入／輸出能力與 function pointer；先教契約再展示 bounds。 |
| 6.4 | impl Trait 的必要使用知識 | 參數與回傳 opaque 型別的意義；本節所用的 lifetime capture 與 edition 2024 規則先解釋，再提供 helper。 |
| 6.5 | Iterator 與 IntoIterator | associated Item、惰性與消耗、iter／iter_mut／into_iter 的 ownership 差別；小型自訂 Iterator。 |
| 6.6 | adaptor、collect 與 FromIterator | map／filter／fold 等先講契約；收集目標、短路／錯誤結果依選用 API 先教。 |
| 6.7 | 文字、collections 與排序的回收 | chars／lines、HashMap entry 的 closure 版本、Ord／PartialOrd／比較 closure、穩定輸出，完成資料整理小成果。 |

### 第 7 章：Cargo 生態系、I/O 與第一個 CLI

本章成果：完成一個規模較小、可安裝執行的文字搜尋／統計 CLI。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 7.1 | 外部 dependency 與 docs.rs | crates.io、cargo add、dependency 名稱／crate 路徑、dependencies／dev-dependencies、版本／lock／update／基本 features；此工具節不技術依賴完整 lifetime。 |
| 7.2 | 跨檔測試與文件 | integration／doc tests、文件註解、cargo doc；沿用 1.10 的日常回饋。 |
| 7.3 | 文字檔與具體 I/O 錯誤 | read_to_string、Read／Write 的最低契約、io::Result／io::Error、UTF-8 失敗。 |
| 7.4 | bytes、buffer 與串流 I/O | EOF、短讀取／短寫入、有效長度、需要時的 BufRead；自訂 Read 測試若使用，先教其實作。 |
| 7.5 | path、OS 字串與 CLI 輸入輸出 | Path／PathBuf、OsStr／OsString、args_os、工作目錄、stdout／stderr／結束狀態、display 的輸出物件與失真限制。 |
| 7.6 | clap 的小練習 | Parser、選用 attributes／features、try_parse_from 與實際回傳錯誤；先驗證 library，再談 process 結束行為。 |
| 7.7 | JSON 與 serde 的小練習 | JSON 型別、Serialize／Deserialize、derive、serde_json 的選用函式與錯誤；初期只用 owned 模型與 from_str，不展示 DeserializeOwned／from_reader 的 HRTB 定義。 |
| 7.8 | fixture 與 CLI 驗收 API | tempfile、std::process::Command／Output、臨時檔／資料夾與退出狀態的具體契約；核心測試避免依賴特權或真實外部服務。 |
| 7.9 | 第一個完整 CLI 小專案 | 整合已教的參數、檔案、Iterator、具體錯誤、文字／JSON 與測試；仍不使用 `Box<dyn Error>`。 |

### 第 8 章：smart pointer、資源與進階型別

本章成果：解釋擁有、借用、共享、動態 dispatch 與錯誤邊界的選擇。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 8.1 | Sized、DST 與 Box | 大小、間接存放、str／slice 的大小限制、?Sized、Box、遞迴資料與 heap 配置。 |
| 8.2 | Drop 與資源收尾 | 自訂 Drop、scope 收尾及 std::mem::drop 使用契約；初期選已教的自訂數值／文字模型。 |
| 8.3 | Deref 與 coercion | associated Target、借用介面、所選自動轉換與 method 查找；回收前面明寫 as_str／as_slice 的經驗。 |
| 8.4 | lifetime bounds 與 'static | `T:'a`、'a:'b、&'static T 與 `T:'static`；在 Error／Any／thread 真正需要前教清楚。 |
| 8.5 | dyn 與 trait object | dyn 相容性、動態 dispatch、Box<dyn Trait> 與具體返回形式的取捨。 |
| 8.6 | 完整 std::error::Error | Debug／Display、source、&(dyn Error + 'static)、錯誤鏈與 `Box<dyn Error>`；轉換邊界沿用 5 章。 |
| 8.7 | Rc 與 Weak | 共享 ownership、clone 的具體語意、循環與 Weak 升級。 |
| 8.8 | Cell 與 RefCell | interior mutability、borrow guard 與失敗；先教回傳 guard 與 Deref 行為，再組合共享資料。 |
| 8.9 | 進階型別與 trait 設計 | type alias 深入、newtype、coherence／orphan rules、運算子 trait；逐項用有限介面練習。 |
| 8.10 | Any 與 downcast | type identity、concrete downcast、'static 限制與 payload 的實際意義。 |

### 第 9 章：Cargo 工程設定與檔案處理套件

本章成果：完成可單獨測試的遍歷、hash 與內容比較元件。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 9.1 | workspace、profiles、features 與 cfg | 共用依賴／設定、release、條件編譯、target 與平台差異；回收已使用的欄位與 attributes。 |
| 9.2 | build.rs 與建置協定 | Rust build script、Cargo 訊息、環境與 target；此處先用已懂的 Rust 工作，不先展示未教 C 原始碼。 |
| 9.3 | walkdir 與檔案身分 | iterator item／錯誤、metadata、symbolic link 政策、PathBuf 身分、非 Unicode 報告策略。 |
| 9.4 | blake3、分塊 hash 與內容確認 | digest 契約、短讀取、大小篩選、真正內容分組、檔案可觀察變動、排序與位元組指標；小練習不交付整個最終答案。 |

### 第 10 章：thread、同步與 atomic

本章成果：完成可收尾的並行工作，解釋同步保證與限制。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 10.1 | 並行模型、Send 與 Sync | ownership 跨 thread、Send／Sync、`T:'static` 的實際限制，回收 8.4。 |
| 10.2 | thread、join 與 panic payload | 建立、等待、普通失敗與 `Box<dyn Any + Send + 'static>`；丟棄 handle 與完成工作分清。 |
| 10.3 | HRTB 與 callback | for<'a> 的呼叫關係，以已懂 closure／reference 建立動機；不與 GAT／variance 合併驗收。 |
| 10.4 | scoped thread | scope 的借用／收尾契約，必要 HRTB／scope lifetime 簽名已教。 |
| 10.5 | channel 與協作收尾 | 傳送、接收、關閉、失敗與工作生命週期。 |
| 10.6 | Arc 與 Mutex | 共享資料、lock guard、poisoning、持有與釋放範圍。 |
| 10.7 | RwLock 與同步設計 | 鎖的範圍、blocking／死鎖、不同 guard 的契約；對小工作比較選擇。 |
| 10.8 | atomic 與 memory ordering | 先單一計數，再區分跨操作同步；ordering 保證與未保證的內容。必要局部驗證與理由在本節完成。 |

### 第 11 章：async、HTTP 與受控測試

本章成果：完成有上限、可取消與收尾的批次 HTTP 檢查元件。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 11.1 | Future、IntoFuture、async／await 與 runtime | 等待模型、Output、Tokio runtime／features、blocking 工作；先教使用，poll／Pin 型別簽名留 13 章。 |
| 11.2 | async task 的生命週期 | spawn／join、Send／'static、JoinError、JoinHandle 與 JoinSet 的持有／丟棄語意。 |
| 11.3 | 有上限的工作排程 | 主路線採 JoinSet 維持上限內的 task，完成一項才補下一項；必要 Tokio mpsc 等 channel API 先教。 |
| 11.4 | 時間與 timeout | Duration／Instant、deadline 範圍、timeout 回傳結果與受控驗證。 |
| 11.5 | select!、取消與收尾 | 取消安全、guard 不跨不必要等待、停止排新工作、abort 與等待完成；主路線以已教 Tokio channel 提供可控制中止來源。 |
| 11.6 | HTTP 的必要基礎 | URL／scheme、method、headers／body、狀態碼、redirect、transport failure；不假設 Python 起點已懂網路協定。 |
| 11.7 | reqwest 與單一請求 | 共享 client、request／response、feature、URL 解析、狀態分類、耗時與期限；先做單一可測試工作。 |
| 11.8 | 受控本機服務與整合測試 | 採 axum 的最小 Router／State／Response 契約與 Tokio TcpListener；動態 port、啟動完成、回應控制、同步計數、shutdown 與等待收尾全部先教。 |

### 第 12 章：進階 lifetime、unsafe 與 FFI

本章成果：用有限介面解釋借用限制，完成有安全契約的小型 wrapper。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 12.1 | GAT 與借用產物 | associated type 隨借用變化；一個有限的借出內部資料介面，不實作通用容器。 |
| 12.2 | variance 與合法替換 | 以 reference、function pointer、Cell 等已懂型別解釋替換與限制，與 GAT 分開驗收。 |
| 12.3 | raw pointer 與有效性 | 來源、範圍、初始化、alignment、aliasing／provenance、必要標準 API；先用單 thread 的小模型。 |
| 12.4 | unsafe 與 soundness | 安全契約、有限 safe wrapper、局部驗證與逐項 safety 理由；跨 thread 的資料競爭／unsafe Send／Sync 題需先完成 10 章必要部分。 |
| 12.5 | FFI 所需的 C 基礎 | C 的最小型別、函式、pointer、字串／buffer 與編譯連結概念；本節先教再給 C 程式。 |
| 12.6 | FFI、ABI 與 ownership 邊界 | repr／資料布局、字串與 buffer、呼叫與釋放責任；實際 cc crate／建置指令 API 在此教，寫最小完整安全 wrapper。 |

### 第 13 章：Pin、Future 與 executor 內部

本章成果：實作有限 Future，能追蹤 Pending、喚醒及再次 poll。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 13.1 | self-reference 與地址穩定性 | 連回 async 使用，分清 lifetime、move 與地址穩定的問題。 |
| 13.2 | Pin 與 Unpin | 安全使用、Drop 契約；選用的 PhantomPinned／projection 等必要 API 先教。 |
| 13.3 | Poll、Context、Waker 與 Wake | 先建立狀態與喚醒關係，再教簽名；採 Wake 的安全建立途徑。 |
| 13.4 | 手動實作 Future | 有限狀態的 Future 與需要理由的局部驗證，避免一次塞入全套 runtime。 |
| 13.5 | 簡易 executor | 明確限定支援範圍，追蹤 poll／等待／喚醒；與 11.1 的使用模型接合。 |

### 第 14 章：macro、API 設計、驗證與效能

本章成果：能評估公開介面、驗證方法與量測結果。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 14.1 | macro_rules! | 輸入模式、展開與有限完整例子；與早期 macro 使用經驗接合。 |
| 14.2 | hygiene 與錯誤位置 | 作用域、名稱與診斷，保留與前節相同的小任務。 |
| 14.3 | procedural macro 與 derive 實作 | proc-macro crate、TokenStream、選用 syn／quote 的最低 API、輸入輸出與錯誤；先教再寫，骨架不藏 parser 前置。 |
| 14.4 | library API 與相容性 | 回收早期分工、錯誤與 newtype 的設計；pub 契約、feature、版本、發布前檢查。 |
| 14.5 | property testing | 小模型的性質與產生器；所選 crate API 在使用前教，不用重複的實作鏡像驗收。 |
| 14.6 | 並行與 unsafe 驗證工具 | Loom 的模型／替代型別、Miri 的分開固定 nightly 與限制；回收先前局部驗證，工具通過不代替理由。 |
| 14.7 | benchmark 與公平量測 | release、正確性、工作量、量測範圍、必要 Criterion 等 API。 |
| 14.8 | profiling 與改善 | 配置／複製成本、profile 證據、改善前後公平比較；不以名詞或一次跑得快當結論。 |

### 第 15 章：最終練功：引導實作與完整參考版

本章成果：完成兩個各約千行、可用且可驗證的工具。

| 小節 | 主題 | 教學範圍與限制 |
| --- | --- | --- |
| 15.1 | 實戰共同入口與規則 | 先給需求、操作與資料、里程碑／驗收；所有必要 API 來自已完成前文，約千行包括測試且不累加快照。 |
| 15.2 | A1 收集檔案 | 根目錄／link／路徑政策、檔案資料與項目錯誤；讀者自行實作，提供分層提示。 |
| 15.3 | A2 判定重複內容 | 大小篩選、串流 hash、候選桶與真正內容分組；驗證短讀取及跨 buffer 情況。 |
| 15.4 | A3 報告與部分失敗 | 穩定排序、重複內容位元組指標、文字／JSON 與不完整掃描。 |
| 15.5 | A4 CLI 與驗收 | 接上已教 clap、結束狀態、fixture 與完整核心／CLI 測試。 |
| 15.6 | A 完整可用參考版 | 獨立 package、全部功能、錯誤與限制、建置／安裝／使用、測試與設計理由；不換成主線未教的高手寫法。 |
| 15.7 | B1 單一網址 | 固定請求／redirect／成功／耗時政策，先完成有分類的單筆結果。 |
| 15.8 | B2 依序批次 | 保留輸入身分、處理錯誤與混合清單，先不用並行隱藏結果模型問題。 |
| 15.9 | B3 有上限並行 | 共用 client、上限內的 task、收集與穩定結果，受控服務驗證。 |
| 15.10 | B4 停止新提交與完整收尾 | 保存整批停止理由及逐筆結果、取消後等待全部已提交工作、保留未提交輸入；受控中止與真 Ctrl+C 示範。 |
| 15.11 | B5 CLI、報告與完整可用參考版 | 引導接上正式 CLI、穩定文字／JSON及完整退出優先序，再提供獨立可用 package、建置／使用／測試與設計理由。 |

## 4. 作者使用的前置核對規則

### 使用契約與內部實作

- 使用具體型別的 API 前，必須理解輸入／輸出、借用或取得哪些值、必要限制、失敗／副作用，以及呼叫後哪些值仍可使用。
- 使用 trait 能力或 derive 前，教清能力與本次條件；實作該 trait 或展示泛型簽名時，簽名中的 bounds／associated types／lifetime 也必須已教。
- 當下不需要的內部實作可以後學，例如先使用 println!／derive／async，後學 macro／poll；不能省去必要的操作契約。
- 第三方內部私有的 Box／unsafe 不自動要求呼叫者先學；若出現在教學接口、helper、診斷或推理中，就必須先教。
- 不用黑箱 helper 把未教的必要前提藏起來。外部連結不是前置已完成的證據。
- 診斷引用選當下需要且已教的片段；不貼整段未知內容後要求讀者忽略。

### 每段程式的必填紀錄

| 欄位 | 要保存的內容 |
| --- | --- |
| 程式與用途 | 真實原始碼／片段／測試位置、此段要讓讀者能做或解釋什麼 |
| 使用的能力 | 語法、型別、API、bounds、錯誤、設定與診斷 |
| 首次教學證據 | 本單元較前位置或已完成前文的真實位置與版本 |
| 使用契約 | 必要輸入／輸出、borrow／move、限制、失敗／副作用 |
| 狀態 | 待寫／已教待驗／已完成與覆核；待寫不能解除作者阻擋 |

### 預定首次介紹位置

下表是責任分配，不是已完成的白名單。實際版本、程式、API 與測試選定後要補真實位置，完整核對每段範例。

| API／概念 | 預定位置 | 本版選擇與限制 |
| --- | --- | --- |
| main／println!／基本 manifest | 1.2 | 最小程式、文字形式、輸出與執行；變數插值到 1.4 再教。 |
| -p／--locked／workspace 繼承 | 1.3 | 正在選哪個 package、鎖定解析的意義與眼前欄位；不把 --locked 等同所有平台結果相同。 |
| #[test]／#[cfg(test)]／assert_eq! | 1.10 | 僅數字等已知值的測試使用契約；若引用泛型 bounds 診斷，等 4／6 章相應知識。 |
| Copy 必要意義／derive Copy, Clone | 2.2／4.2 | 數值 Copy 與自訂 struct 不自動 Copy；Clone 不一律深層複製。 |
| &／&mut／*／重新借用 | 2.3–2.4 | 建立與使用 reference、修改範圍、借用最後一次使用。 |
| Option／Result／泛型語法 | 3.2–3.3 | 先具體角色再型別參數；unwrap 需到 5.1，? 需到 5.6。 |
| String／UTF-8 安全存取 | 3.4–3.5 | new／push_str／as_str／get 等選用接口；Option／Result 已在前，chars 到 6.7。 |
| Vec／vec!／get／as_slice | 3.6 | 建立、借用、消耗、回傳 reference 的有效性；Debug 到 4.2，Iterator adaptor 到 6。 |
| Debug／PartialEq／Clone／Copy | 4.2 | 能力與 derive 條件，不先展示未知 Formatter 簽名。 |
| Eq／Hash／Borrow／HashMap::get | 4.3 | String／str 的具體借用查詢與 hash 契約；泛型 get 簽名含 ?Sized 必須等 8.1。 |
| lifetime parameter／'_／outlives | 4.4–4.5 | 回傳 reference 與含 reference 的型別；必要 bounds 先教，深入 'static 在 8.4。 |
| type alias／Formatter／write!／fmt::Result | 5.2–5.3 | 格式化 input／output／失敗契約；5.3 不先用 ?。 |
| From／TryFrom／FromStr／AsRef／? | 5.4–5.6 | 分段教轉換、解析與傳遞；map_err 等 closure 寫法等 6。 |
| associated type／Fn／impl Trait | 6.2–6.4 | 各自的操作與必要簽名；edition 2024 的 capture 若需要在借用 helper 前教。 |
| iter／iter_mut／into_iter／collect | 6.5–6.6 | Item、借用／消耗、惰性、目標型別與錯誤結果。 |
| chars／lines／Ord／排序 closure | 6.7 | 文字處理、比較契約與穩定輸出；HashMap closure entry 此時才使用。 |
| io::Result／Read／Write／短讀寫 | 7.3–7.4 | buffer 有效長度、EOF、錯誤；Read 的泛型 wrapper／自訂測試先教實作契約。 |
| PathBuf／OsString／path display | 7.5 | 原生路徑身分與文字顯示分開；第一次選用 metadata／SystemTime 比較在 9.3–9.4 前教。 |
| clap／serde／JSON | 7.6–7.7 | 固定選用函式／attributes／features；owned 模型、from_str。from_reader／DeserializeOwned 完整 HRTB 定義需到 10.3 之後另教。 |
| tempfile／Command／Output | 7.8 | fixture 生命週期、process 狀態與輸出；進階工具不代替先備。 |
| Sized／DST／Drop／Deref | 8.1–8.3 | 先各自解釋能力與安全使用，Deref Target 依賴 6.2。 |
| 'static／dyn／Error::source | 8.4–8.6 | trait object bound 與回傳借用分清；`Box<dyn Error>` 僅此後。 |
| Rc／Weak／Cell／RefCell／Any | 8.7–8.10 | clone、guard、內部可變與 downcast 的具體接口。 |
| build.rs／target／cc | 9.2／12.6 | 9.2 只用已懂 Rust 建置工作；cc 編譯 API 與 C source 在 C 基礎完成後使用。 |
| walkdir／blake3／metadata | 9.3–9.4 | link／錯誤／路徑、hash 狀態與 digest、短讀取、變動與內容確認。 |
| Send／Sync／thread Result／HRTB | 10.1–10.4 | Box／dyn／Any／'static 在 8；完整 scope 簽名只在 HRTB 之後。 |
| Arc／guard／atomic Ordering | 10.6–10.8 | 鎖範圍、可見性與 ordering；測試不以多次執行無錯代替同步理由。 |
| Future／IntoFuture／Tokio runtime | 11.1 | 先教等待與 Output 使用契約；不提前貼帶 Pin 的 poll 定義。 |
| JoinSet／Tokio mpsc／timeout／select! | 11.2–11.5 | 上限內建立 task、錯誤、期限、取消安全；abort 後等待並收齊結果。 |
| HTTP／reqwest／Duration／Instant | 11.4／11.6–11.7 | headers／body、status／transport、client、redirect、測量與期限的範圍。 |
| axum／TcpListener／State／response／shutdown | 11.8 | 動態 port、啟動完成、控制回應、同步計數與收尾；只教測試服務實際用的 API，若展示 Service 泛型簽名需先補相應角色。 |
| GAT／variance／raw pointer／FFI | 12 | 各自有限產物；C／ABI／記憶體布局與必要 C 標準接口在使用前教。 |
| Pin／PhantomPinned／Poll／Wake | 13 | 地址、Drop 與喚醒；選用 projection 等接口先教，不先用隱藏 unsafe helper。 |
| proc_macro／syn／quote／proptest／Loom／Miri／Criterion | 14 | 選用 API、工具鏈、設定、限制與局部目的各自先教，再給程式或命令。 |

## 5. 兩個最終專案：已選定的核心契約

每題約 800–1,200 行手寫 Rust，包含程式、測試、fixture 與必要測試服務。只計完整參考版一份；不累加階段快照、不計生成碼／依賴／lockfile／教材文字。這是範圍尺度，尚無實作行數；以功能完整、清楚與負擔適中為準。

### A：重複檔案搜尋器

| 項目 | 本版決定 |
| --- | --- |
| 輸入與對象 | 單一根目錄，遍歷一般檔案與子目錄，包含隱藏目錄；不追蹤 symbolic link；根路徑為 link／不是目錄時回報設定錯誤 |
| 判定 | 先按大小縮小候選，串流 blake3，再以真正 bytes 比較劃分內容相同的群組；不能把同 hash 桶直接當成一組 |
| 報告 | 每組至少兩個不同路徑；路徑／群組穩定排序；文字與 JSON 表達相同結果 |
| 數字意義 | 每組重複內容位元組為 (路徑數 − 1) × 長度；依路徑計算，包含 hard link 的多個路徑，不承諾實體可回收磁碟空間 |
| 部分失敗 | 項目不可讀、不可處理時記錄動作與原因，繼續可讀項目，明確標示掃描不完整 |
| 路徑 | 核心保留 PathBuf 身分。最小完成版接受可表示為 Unicode 的路徑；不符合者列項目錯誤並跳過，錯誤另有可辨識的 debug 顯示而不拿顯示字串作身分 |
| 檔案變動 | 盡力而為的觀察；讀取前後的可觀察 metadata 變動／讀取失敗列項目錯誤，不承諾一致快照，也不把未偵測到變動說成絕不會變動 |
| 結束狀態 | 0：完整完成（含無重複）；1：有項目失敗／不完整；2：輸入／根設定／報告輸出等致命錯誤 |
| 額外功能 | 刪除、同步、背景監控、精確實體回收與一致快照保留為延伸，不加入完成版 |

驗收資料：空檔、同內容不同檔名、同大小不同內容、跨 buffer、短讀取、子目錄、可控制項目錯誤、link 政策、非 Unicode 策略、部分失敗、穩定排序、文字／JSON 一致。平台不能建立的 fixture 使用明示條件與已教的可控制失敗案例；不要求讀者取得特權。

使用 crate：clap、serde／serde_json、walkdir、blake3；fixture 使用已教 tempfile。主要為同步工具，範圍內不另加多 thread hash。

### B：async 網址檢查器

| 項目 | 本版決定 |
| --- | --- |
| 輸入 | 每行一個 URL，跳過空白行；接受 http／https，保留原輸入行與索引，重複網址仍是不同項目。無效 URL 列輸入項目失敗；空清單為設定錯誤 |
| 請求 | GET，最多追蹤 5 次 redirect；最終 2xx 算檢查成功，其他已收到的 HTTP status 保留為 HTTP 結果，不混同 transport failure |
| 耗時 | 從項目開始送請求至取得最終 response headers，包含 redirect；工作到此完成，不下載完整 body |
| 期限 | 每項工作有可設定期限（預設 10 秒）；主路線以已教 Tokio timeout 包住取得 headers 的完整工作，不額外加入整批期限 |
| 並行 | 正整數上限（預設 8）；JoinSet 中只維持上限內的工作，完成一個才補下一個，不先建立無限制 task 再擋請求 |
| 中止 | 收到中止後停止排新工作，取消執行中工作並等待收尾；保留已完成結果，剩下分為取消／未開始，不算網路失敗 |
| 結果 | 區分成功、HTTP 未通過、輸入錯誤、transport failure、timeout、task failure、取消與未開始；依輸入順序報告，文字摘要與 JSON 同一結果模型 |
| 結束狀態 | 0：全部檢查成功；1：有項目未通過；2：設定／讀輸入／輸出等致命錯誤；130：使用者中止且報告成功。致命錯誤優先於中止，其次才一般未通過 |
| 驗收服務 | axum 最小本機服務，動態 port、受控正常／錯誤／redirect／延遲回應與同步計數；以可控制事件驗證上限與取消，不用真實公共網站狀態當固定答案 |
| 額外功能 | 爬蟲、認證、重試、HEAD fallback、資料庫、長期監控與 dashboard 保留為延伸 |

使用 crate：Tokio、reqwest、clap、serde／serde_json；測試支援 axum、已教 fixture 與 Tokio channel。controller／test server 的生命週期先在 11 章教清楚，不能只提供未知的 start_test_server helper。

### 引導正文與參考版

每個里程碑：可觀察需求與輸入／預期結果 → 必要介面或起始檔 → 讀者動手 → 分層提示 → 驗收與理由回收 → 下一步。

- 每階段一個主要問題；先讓讀者理解資料流與 ownership，骨架不藏核心演算法。
- 分層提示先給思考方向，再給已教能力與局部結構；完整答案另有清楚入口。
- 每階段保留可執行完成快照，與對應引導位置一致；未完成起始版明示待完成範圍。
- 完整參考版是獨立、完整可用 package，包含功能、錯誤政策、限制、建置／安裝／使用、測試與設計理由。
- 提示、測試、參考版不使用主線未教過的另一套寫法。

## 6. 閱讀順序、API 依賴與作者排程

閱讀順序依第 3 節；全部內容都是必修。作者阻擋條件只依其程式／教學實際需要且已有可引用完成稿的前置；全課程必修不表示每個程式都要用全部技術。

### 可成立的平行波次

| 前置完成條件 | 可釋出的作者工作 | 不能提前使用的內容 |
| --- | --- | --- |
| 1–4 章必要稿已確認 | 5 章錯誤、6 章 closure／Iterator、8.1 Box／8.2 Drop、7.1–7.2 工具可分工 | 錯誤分支不先用 map_err closure；Drop 不先用 File；工具例子不先呼叫未知第三方 API |
| 6.2 已完成 | 8.3 Deref 可釋出 | associated Target 必須引用已完成解釋，不依正在寫的 trait 小節猜測 |
| 5／6 與必要 7 章小節完成 | I/O、clap、JSON、fixture 可以按實際入口分工，最後整合 7.9 | JSON 不提前展示 HRTB 的 DeserializeOwned 定義；測試接口都要有來源 |
| 必要 8 章與工程前置完成 | 9.3–9.4 檔案 API、10 章 thread、12.1–12.3 進階借用／單 thread pointer、14.1–14.3 macro 可分工 | 12.4 跨 thread／資料競爭等須等 10 章實際前置；C code／cc 要等 12.5–12.6 |
| thread／runtime 與分支前置完成 | async 排程／取消與 HTTP 單一請求可依最小接口分工；11.8 等兩側必要稿整合 | test server 不先用未教 atomic、IntoFuture、Router／State／shutdown |
| 9.4、11.8、共同工具與API規範分別就緒 | A 與 B 分別獨立 package 撰寫，可不同時間啟動 | A 不需 reqwest；B 不需 walkdir；兩者都有實際已教測試、CLI、JSON與錯誤來源 |

所有作者取得前文版本、實際介紹位置與允許使用的能力清單。新增概念須先在本單元解釋；若又依賴未完成稿，就等待或換例子。根 manifest／版本／features／nav 與共享設定由主 agent 整合。

### 小節的主要範例前置

以下只記規劃中選定範例的主要前置，不是完整白名單。全部狀態目前為待寫／待驗，機械拓樸檢查不代替實際程式與第一次閱讀驗收。

| 小節 | 主要前置 |
| --- | --- |
| 1.1 | 無 |
| 1.2 | 1.1 |
| 1.3 | 1.2 |
| 1.4 | 1.2 |
| 1.5 | 1.4 |
| 1.6 | 1.5 |
| 1.7 | 1.6 |
| 1.8 | 1.7 |
| 1.9 | 1.8、1.3 |
| 1.10 | 1.9 |
| 2.1 | 1.10 |
| 2.2 | 2.1 |
| 2.3 | 2.2 |
| 2.4 | 2.3 |
| 2.5 | 2.4 |
| 2.6 | 2.4、1.7 |
| 3.1 | 2.5 |
| 3.2 | 3.1 |
| 3.3 | 3.2 |
| 3.4 | 3.3、2.6 |
| 3.5 | 3.4、3.2 |
| 3.6 | 3.5、3.3、2.6、1.8 |
| 4.1 | 3.6、2.5、3.3 |
| 4.2 | 4.1 |
| 4.3 | 4.2、3.6、3.4 |
| 4.4 | 4.1、2.4、3.4 |
| 4.5 | 4.4、2.5 |
| 5.1 | 4.5、3.2 |
| 5.2 | 5.1、3.3 |
| 5.3 | 5.2、4.4、4.1 |
| 5.4 | 5.3 |
| 5.5 | 5.4、4.3 |
| 5.6 | 5.5 |
| 6.1 | 4.5、2.4 |
| 6.2 | 4.1 |
| 6.3 | 6.1、6.2 |
| 6.4 | 6.3、4.4 |
| 6.5 | 6.2、6.4、3.6 |
| 6.6 | 6.5、6.3、5.6 |
| 6.7 | 6.6、4.3、3.5 |
| 7.1 | 1.3、1.9 |
| 7.2 | 1.10、4.2 |
| 7.3 | 5.6、3.5 |
| 7.4 | 7.3、2.6、6.2 |
| 7.5 | 7.4、5.3、6.5 |
| 7.6 | 7.1、7.5、4.2 |
| 7.7 | 7.1、7.3、4.5、6.2 |
| 7.8 | 7.1、7.2、7.5 |
| 7.9 | 7.6、7.7、7.8、6.7 |
| 8.1 | 4.1、3.6、2.6 |
| 8.2 | 8.1、4.1 |
| 8.3 | 8.2、6.2 |
| 8.4 | 4.5、4.1 |
| 8.5 | 8.1、8.3、8.4、6.2、6.4 |
| 8.6 | 8.5、5.6 |
| 8.7 | 8.2、4.2、3.2 |
| 8.8 | 8.3、8.7 |
| 8.9 | 6.2、5.2、8.5 |
| 8.10 | 8.4、8.5 |
| 9.1 | 7.1、7.2 |
| 9.2 | 9.1、7.5 |
| 9.3 | 7.5、6.7、7.1 |
| 9.4 | 9.3、7.4、6.7、5.6 |
| 10.1 | 8.4、8.8、6.3 |
| 10.2 | 10.1、8.10、5.6 |
| 10.3 | 6.3、4.5 |
| 10.4 | 10.2、10.3 |
| 10.5 | 10.2 |
| 10.6 | 10.5、8.8 |
| 10.7 | 10.6 |
| 10.8 | 10.7 |
| 11.1 | 6.2、6.4、10.7、7.1 |
| 11.2 | 11.1、10.2 |
| 11.3 | 11.2、10.5 |
| 11.4 | 11.1、5.6 |
| 11.5 | 11.3、11.4、10.6 |
| 11.6 | 7.9 |
| 11.7 | 11.6、11.1、11.4、7.7 |
| 11.8 | 11.7、10.8、11.5、7.8、10.3 |
| 12.1 | 6.2、4.5、8.8 |
| 12.2 | 4.5、8.8、6.3 |
| 12.3 | 12.2、8.1、8.3 |
| 12.4 | 12.3、10.8 |
| 12.5 | 12.4、9.2 |
| 12.6 | 12.5、8.2 |
| 13.1 | 12.4、11.5、8.2 |
| 13.2 | 13.1 |
| 13.3 | 13.2、6.2、10.6 |
| 13.4 | 13.3 |
| 13.5 | 13.4 |
| 14.1 | 7.9、8.9 |
| 14.2 | 14.1 |
| 14.3 | 14.2、9.2、6.2 |
| 14.4 | 8.9、8.6、9.1、7.9 |
| 14.5 | 7.9、7.8、4.3 |
| 14.6 | 10.8、12.4、9.1、7.1 |
| 14.7 | 9.1、11.4、7.9、6.3 |
| 14.8 | 14.7 |
| 15.1 | 7.9、7.8、9.1 |
| 15.2 | 15.1、9.3、7.8 |
| 15.3 | 15.2、9.4 |
| 15.4 | 15.3、7.7、6.7 |
| 15.5 | 15.4、7.6、7.8 |
| 15.6 | 15.5、8.6、14.4 |
| 15.7 | 15.1、11.7、11.8 |
| 15.8 | 15.7 |
| 15.9 | 15.8、11.3 |
| 15.10 | 15.9、11.5、11.8 |
| 15.11 | 15.10、7.6–7.9、8.6、14.4 |

分派前依真正的代表性程式與使用契約更新此表；整合時按讀者順序核對。前置修訂後只追蹤確實引用其能力的後續，不把工作量優先於理解。

## 7. 驗收與目前證據範圍

- 每小節有自己的問題、可見結果或可解釋限制；每大章有成果回收。
- 進階 HRTB／GAT／variance 各驗一個借用關係；atomic 有同步理由；unsafe 有逐項安全契約；FFI 有最小可建置介面；Pin／Future 有可追蹤狀態；macro 有輸入／展開／診斷；API／效能有公平的判斷依據。
- 正文、程式、helper、題目、提示、答案、測試、manifest、診斷全部使用同一前置規則。
- 教材來源與範例 package 使用真實來源引用，階段版本不串入後章功能。
- 技術建置與測試、教學順序理解與真實版可用性分別驗證；作者自查與獨立首讀不混稱。
- 這份 v2 是規劃修訂。尚無完整正文或最終程式，因此目前不能宣稱 API 全部閉合、行數已量測、所有讀者都好學，或 v2 已再次由 Astra 審過。

## 8. 來源與後續檔案

- [v1 原始規劃](curriculum-plan.md)、[Astra（max）完整審閱](curriculum-review-astra.md)、[當時的協調者整理](curriculum-review-followup.md)。
- Rust 標準能力、Error／lifetime、thread／HRTB、serde／Tokio 等技術查證來源見 Astra 報告第 9 節。
- 本版額外核對：[axum serve](https://docs.rs/axum/latest/axum/fn.serve.html)、[axum graceful shutdown](https://docs.rs/axum/latest/axum/serve/struct.WithGracefulShutdown.html)、[tempfile](https://docs.rs/tempfile/latest/tempfile/)、[Tokio mpsc Receiver](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Receiver.html)。它們支持選用最小測試服務／fixture／中止接口的方向；實際版本／features 與每個接口仍在教材寫作前固定。
- 原教材研究來源保留在 v1 第 9 節。本版不把任何既有教材排序當作自動正確。
