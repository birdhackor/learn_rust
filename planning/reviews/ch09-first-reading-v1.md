# 第 9 章 v5 原始順序首讀

本紀錄是 AI 模擬首次順序閱讀，不宣稱真人測試。讀者背景：基本 Python，加上本人實際已讀第 1–7 章正文／既有局部補讀、第 8 章 v8 全 66 單位及第 7 章 v13 指定五單位的局部重讀。前章實讀版本與範圍沿用封存 entry；不是完整看過前章各最新版本。不以模型原本的 Rust 知識補答案。

唯一新正文來源：`frozen/ch09-complete-prerequisite-v5/reading-order.txt` 的 27 單位，涵蓋 9.1–9.4。此次只收到範圍與 protocol，未收到已知問題或技術答案；不讀第 8／9 章 peer、TECH、作者說明、outline、disposition、sourcearchives、whole live、外連或第 10 章。tests／fixtures 僅讀當下 BODY 所示片段，closed 提示／答案在工具輸出之前過濾為 marker。

按 gate 一次一單位，每個必要新概念首次保存四題短答、Q2a 機制／Q2b 既存需要、短引文、最短已教推論與必要未知，再開下一單位。既教概念指出前文來源；必要缺口、optional、合理待教、未驗範圍分開。後文若澄清只追加，不改原判。圖片只在正文原位 hook 出現後核同 frozen SVG，若渲染只作單圖原生視圖，不宣稱完整頁面。未實跑範例／技術探測。完成全 27 初判後封存 SHA，停在第 9 章，不開 peer 或第 10 章。

### 001-001 — 共同設定的既存需要

Q1：承接已讀 1.3 workspace／7.1 依賴版本與功能，這節先提出共用設定、功能選擇、編譯設定三件管理工作。引文：「當更多程式一起維護時」。
Q2a（機制）：此導入尚未教新 Cargo 設定語法；不先猜如何共用或選 profile。
Q2b（需要）：多個 package 一起維護，需知道哪些設定共用與這次如何建置；小計材料讓管理問題不再新增資料模型負擔。
Q3：80×2=160，另 120×1，合計 280；命令位置明定 repo 根、package ch09-engineering。
Q4：兩筆具體數字固定後續核對材料；「運費仍屬第一章原來的計價工作」界定示範範圍。
單位末：這是需求導入，新設定合理待下一單位，當下無必要缺口。

原始SHA-256：03e61e8dcf9195a9c7412bcb4e54abb0c416be6ee2abc2b548af90232299e205；visible SHA-256：03e61e8dcf9195a9c7412bcb4e54abb0c416be6ee2abc2b548af90232299e205。

### 001-002 — dependency 繼承與 profile

檢查點 A：共同 dependency。
Q1：根 `[workspace.dependencies]` 先定可繼承來源，成員 `[dependencies]` 再選用；這與已讀 1.3 package 欄位繼承相接。引文：「成員還要明確選擇採用」。
Q2a（機制）：根 path 從 workspace 根起算，`ch01-basics.workspace = true` 才讓該成員依賴它並採共同設定；未列出的成員不因此有依賴。
Q2b（需要）：多成員需在一處維護相同版本／來源，同時明確知道各自用哪些 dependency；此安排減少重複設定而保留選用責任。
Q3：根放 `path = "examples/ch01-basics"`，成員寫 workspace=true；不把 root 節錄覆蓋整份 manifest。
Q4：两段 TOML 分別顯示定義與採用，及與 7.1 成員 path 起點的差別，足以對應。

檢查點 B：profile／overflow。
Q1：profile 是編譯設定組，與 edition 分開；dev 用於普通 build／run，--release 選 release。引文：「不是切換 Rust edition」。
Q2a（機制）：opt-level 常用 0–3，dev 預設 0、release 預設 3；本 repo release 的 overflow-checks=true 使超出整數型別範圍時 panic。根 manifest 才採 profile，成員同名設定被忽略。
Q2b（需要）：反覆編譯與較多最佳化工作有不同需求；明訂溢位檢查避免不同建置掩蓋有限整數錯誤，但仍要維持可表示範圍。引文：「不能只由設定推定效能」。
Q3：在根 `[profile.release]` 設兩欄，再用 --release 選它；不將最佳化或檢查視為自動修正資料。
Q4：根 TOML 與具體 dev／release 預設提供設定—效果對照；沒有效能量測材料，故不聲稱更快。
單位末：共用来源／成員選用與根 profile 是兩條不同管理關係，正文充分，無必要未知或新增問題。

原始SHA-256：00cd2524570ecc61ea662bec5eda957ffe0d483ea3daeaef0c95463e2d8bcb5b；visible SHA-256：00cd2524570ecc61ea662bec5eda957ffe0d483ea3daeaef0c95463e2d8bcb5b。

### 001-003 — 本 package 的可選 feature

Q1：由 7.1 選 dependency 的 derive，轉為成員為自己定義 batch；共用設定／profile 不等於這個能力開關。引文：「替自己的 package 定義」。
Q2a（機制）：`[features]` 的名稱列表定義開關及連帶啟用项；batch=[] 有開關而不帶其他功能，default=[] 預設全不開。--features 選本 package、--all-features 本例開 batch、--no-default-features 本例無另變化；Cargo 通常合併不同使用者要求，非互斥選單。
Q2b（需要）：使用者可選編入批次公開函式，同時保留原單筆能力／計價；功能能同時開，符合多依賴使用者要求被合併的情況。
Q3：在成員 manifest 定義兩欄，再於 cargo 建置命令選 batch；這與 cargo add --features derive 修改 dependency 設定分工不同。
Q4：TOML 與三個旗標逐一對應本例空 default／單一 batch。尚未展示 Rust 如何條件編入函式，合理待下一段，不假定已看實作。
單位末：功能定義→建置選擇→增加能力的需要已清楚，無當下必要缺口。

原始SHA-256：e5e8de9e5480f5592d510d563d15c0b6bbda05c532cbbadba76fe7e55aed9ebb；visible SHA-256：e5e8de9e5480f5592d510d563d15c0b6bbda05c532cbbadba76fe7e55aed9ebb。

### 001-004 — 條件編譯、cfg! 與 target

檢查點 A：排除項目與 bool。
Q1：`#[cfg(feature="batch")]` 承接 1.10 的 cfg(test)，把 manifest feature 與這次編譯的函式／呼叫 block 連接；cfg! 則只給條件 bool。引文：「該函式根本不在這次編譯的公開介面中」。
Q2a（機制）：attribute 保留／排除後方項目或 block；普通 if cfg! 不排除 branch，兩邊仍要名稱／型別檢查。未開 feature 時，定義被排除而呼叫未排除，就找不到 two_orders。
Q2b（需要）：同一 main 要能在有／無批次能力時合法編譯，同時顯示開關狀態；用 attribute 包住 use 與呼叫，用 cfg! 顯示，分別滿足兩個工作。
Q3：一直保留 order_subtotal；batch 啟用才保留 two_orders 及使用 block。未開輸出 160／false，開則 160／true 加 280，來自先前兩筆材料。
Q4：正常 main 與普通 if 的失敗 fixture 使用同材料，只改是否排除呼叫；「false 只讓執行時不進入 branch」足以解釋所列找不到名稱錯誤，未需作者答案。

檢查點 B：target／host。
Q1：target 是程式預備執行的平台，host 是執行編譯工具的平台，兩者與 profile 分開。引文：「--release 不會把 Linux 程式變成 Windows 程式」。
Q2a（機制）：crate 內 unix／target_os="windows" 條件依建置 target 產生 bool，未另選 target 通常等於 host；不是執行時重新偵測 OS。
Q2b（需要）：顯示這份建置對哪類平台成立，避免把 release 或將來跨平台建置誤認為改了 host／動態偵測。
Q3：main 將兩個 cfg! bool 保存並印出；Linux 示範為 true／false，其他環境依自身 target。
Q4：可見 bool 程式與明確平台預期對應；本文未教另選 target 命令，當下只觀察既有 target，不必猜跨平台操作。
單位末：feature→attribute 合法介面／cfg! 可觀察 bool→target 條件，關係完整。無 necessary／optional；未實跑 fixture 或平台檢查。

原始SHA-256：ef918852330f6c101c91751460e4befaded840fbba353d4ecd5f394bece7d454；visible SHA-256：ef918852330f6c101c91751460e4befaded840fbba353d4ecd5f394bece7d454。

### 001-005 — feature 組合的測試與 release 結果

Q1：測試也依前單位的 cfg feature 保留／排除對可選函式的使用；一般測試與 batch 測試對應兩種介面。
Q2a（機制）：沿用 1.10 integration test 與前段 cfg：default 留一項，all-features 留兩項；--all-targets 指 library／binary／範例／測試等，非前段 target OS 的集合。引文：「不是指所有作業系統」。
Q2b（需要）：不能只確認 feature 開時 main 可跑，還要各組合合法且 batch 用每筆自己的資料；零數量材料可暴露誤用另一筆數量。release 再核同工作結果，不將最佳化視為換規則。
Q3：依序預設 test、全部功能 test／clippy，再 features batch 的 release build／run；預期小計仍 160／280，產物在共同 target/release。
Q4：兩個 assert 中第二笔不變、第一笔數量由 2 到 0，預期從 280 到 120，支持測試目的；命令旗標與已教 profile／feature 對應。
單位末：正文清楚限定「沒有作速度測量」，我亦只讀命令及預期，未宣稱執行通過。沒有新必要概念或缺口。

原始SHA-256：956fb6d2c5cd05071a9c4badb0f0a25397222b07d8b1c902d6ee5a7f2b6e28bc；visible SHA-256：956fb6d2c5cd05071a9c4badb0f0a25397222b07d8b1c902d6ee5a7f2b6e28bc。

### 001-006 — 9.1 小節末整體關係

Q1：共同 dependency 的定義／採用、feature 的能力開關、profile 的編譯設定、target 的平台條件各有不同責任，不能由其中一項推定另一項。
Q2a（機制）：依已存 001-002–004：root dependency 不自動建立成員依賴；batch 排除批次項目但保留單筆；cfg! false 不排除 branch 的名稱檢查。
Q2b（需要）：多 package 的共同維護與不同建置需要明確界線；這些控制可增加能力而保留同樣資料的計价結果。
Q3：本例 --no-default-features 不另改空 default 列表，仍是預設小計 160／batch=false；下一節的先執行 Rust 程式僅是導向，尚不猜執行契約。
Q4：這段三題練習與 001-002–005 的具體設定／fixture 相合。注意：其答案是 inline 可見文字，我於本單位已看見，沒有宣稱本次在答案未見時獨立作答；前三題的必要機制已在先前單位保存。
小節末：原需→共用設定／可選介面／profile／平台條件→多組建置與固定結果的關係足夠。9.1 無 necessary／optional，未驗實際 Cargo 執行或效能。

原始SHA-256：895588845ccf568eaac8624376841962b07572b5cff7f456c635c8559e8f6d7a；visible SHA-256：895588845ccf568eaac8624376841962b07572b5cff7f456c635c8559e8f6d7a。

### 002-001 — build script 的建置前需要

Q1：build script 是 Cargo 在主要 crate 編譯前執行的另一支 Rust 程式，與執行時 main、9.1 的建置選擇銜接。引文：「它不是 src/main.rs」。
Q2a（機制）：通常位於 package 根 build.rs，先執行並將結果交接下來的 crate 編譯；本導入尚未教交付語法，不自行補答案。
Q2b（需要）：demo／practice 應是這次建置記錄的設定，使每次啟動回報同一值，而不是執行時重新讀環境。
Q3：本節另有 ch09-build-script package，命令從 repo 根跑，不改 9.1 package 的 script 工作。
Q4：demo／practice 提供建置決定→多次執行一致回報的實際目的；細節合理待下一單位，當下無必要缺口。

原始SHA-256：d74f18a5317742049dacd5183fd90b186d536ecd8d0bd77c385aaf79cc303d26；visible SHA-256：d74f18a5317742049dacd5183fd90b186d536ecd8d0bd77c385aaf79cc303d26。

### 002-002 — build script 階段、環境值與 stdout 協定

檢查點 A：執行階段與 env::var。
Q1：package 的 build 欄位指定相對根路徑；Cargo 先編譯 script、在 host 執行，再編譯主要 crate，最後執行成品。env::var 讀的是當時正在執行的 script 環境。引文：「它在 stdout 印出指定格式」。
Q2a（機制）：var 回 Result<String,VarError>，NotPresent 與 NotUnicode(OsString) 分別是不存在／不能成 String；Debug 支持已教 expect。自動發現 build.rs 與明寫 build 欄位都已說明，無需猜 Cargo 入口。
Q2b（需要）：模式需固定在建置中，且缺省可用 demo，但坏 Unicode 不能當缺省；沿用 7.5 OS 字串界線，讓無效配置導致建置失敗。
Q3：match 缺值建 demo，非 Unicode 印 error 後 return；其他文字再以兩個 != 的 && 排除。demo／practice 才交付模式，既有 String／match／return／&& 足夠。
Q4：原位 build-phases 圖依序四框：編譯 build.rs 供 host→script 讀 LESSON_MODE 交 TEACHING_MODE→編譯主要 crate 時 env! 固定文字→成品不重讀模式；三個向下箭頭只表先後，與文字完全對應。

檢查點 B：逐行建置協定。
Q1：script 的 stdout 交 Cargo，並非 return 值交設定，也不是最終程式的使用者輸出。
Q2a（機制）：rerun-if-changed 縮小檔案追蹤、rerun-if-env-changed 追外部 LESSON_MODE 的設定／取消；rustc-env 供後續 crate 編譯，error 使 script 結束後 Cargo 判失敗，即使 main 正常回傳。引文：「這個逐行約定叫建置協定」。
Q2b（需要）：Cargo 需知道何時重做與要交哪些固定值；先限制模式名稱，避免外部換行被當新協定行。追外部配置而不追 Cargo 自己提供的 TARGET，分工明寫。
Q3：script 先印兩項重跑條件、驗模式，再印三項 rustc-env；錯誤分支不會走到交有效模式，協定 cargo:: 格式由固定工具鏈支持。
Q4：四列指令表與實際 println! 一一相合；error 指令而非 process 失敗也能拒絕建置的界線已直接說清。

檢查點 C：script 的 host／主要 crate target。
Q1：9.1 cfg! 看「正在編譯的這個 crate」；script crate 要供 host 執行，所以它的 windows bool 與主要 crate target_os 是不同來源。
Q2a（機制）：CARGO_CFG_TARGET_OS 由 Cargo 執行 script 時供給，expect 的依據限本例只由 Cargo 啟動；script 的 cfg! 描述 host，不能替代這份 target 資料。
Q2b（需要）：將來替別的平台建置，host／target 可不同；保留兩項資料能看出來源差異，而非由本機 bool 誤設主要程式平台。
Q3：var 取得 target_os，cfg! 取得 host_windows，各自交 TEACHING_TARGET_OS／TEACHING_HOST_WINDOWS；不猜跨平台建置操作。
Q4：Linux 同平台只會讓兩者碰巧相同，正文的不同來源解釋比相同輸出本身提供更多依據。
單位末：新機制的時點／輸入政策／協定／平台來源都有必要前提，無 necessary／optional。只看單圖原生渲染，未實跑或看整頁。SVG SHA：1a69b9cb08e010bb93d8827c459a3dbdb6df9d5a4da404bc9924211a913a569f；PNG SHA：6c48a5f929d2d2939cc125ece8217467c0db003664280c22ce903807cd5e12b6。

原始SHA-256：c510aa0f3324061e58eef83463132a8d9f65e556623ede9515a8c66083da9930；visible SHA-256：c510aa0f3324061e58eef83463132a8d9f65e556623ede9515a8c66083da9930。

### 002-003 — env! 固定值與 Bash 的命令環境

檢查點 A：已教 env! 的建置結果。
Q1：script 的 rustc-env→library 的 env!→binary 顯示，完成上段的交接。7.8 env! 與 8.4 &'static str 已讀，來源在本段明指。
Q2a（機制）：env! 編譯時取值並給 &'static str，不在每次呼叫重讀；缺值／非 Unicode 為編譯錯誤。host_windows_text 回 true／false 的文字，不自動解析 bool。引文：「沒有暗中把它解析成 bool」。
Q2b（需要）：成品需一直回報自己那次建置設定；若改為每次啟動讀新設定才應在主要程式用 env::var，兩種需要不混用。
Q3：三函式分別交模式、target OS、host Windows 文字；binary 只顯示，不重新讀 LESSON_MODE。
Q4：script 已供的三個名字與 env! 鍵逐一相同，可辨值的交付及型別，無隱藏轉換。

檢查點 B：改建置／執行環境。
Q1：Bash `NAME=value command` 只傳給這次命令及子程序，與永久 shell 配置分開。引文：「不是永久修改 shell 設定」。
Q2a（機制）：demo cargo run→practice cargo run 會觸發已教的環境追蹤、script 重跑及主要 crate 重編；直接 binary 不經 Cargo，runtime 的 demo 無法改已嵌入的 practice。
Q2b（需要）：用相同模式值在兩個時點改動，可驗證建置固定值，而不是只看輸出猜來源；wrong 配置還需在建置時拒絕。
Q3：依所列順序兩次 Cargo run、一次直接 target/debug binary，預期 demo→practice→practice；wrong cargo check 是 script cargo::error 拒絕，改回 demo 不需刪 target。
Q4：正文對三次模式輸出及 wrong 診斷都有具體預期，且 -vv／輸入不變時可能沿用結果，解釋看不到 script stdout 的觀察界線。
單位末：時點、追蹤、固定文字與重跑的因果足夠；無 necessary／optional，僅文字驗收預期，未實跑。

原始SHA-256：65e1f8b92a4017f996dc2b1237caf98d4a9a8096c0388f0dcade29b18b026b88；visible SHA-256：65e1f8b92a4017f996dc2b1237caf98d4a9a8096c0388f0dcade29b18b026b88。

### 002-004 — 生成檔案與 build-dependencies 的界線

Q1：OUT_DIR 是 Cargo 供 script 的建置輸出位置；build-dependencies 供 host script，dependencies 供主要 crate，承接兩支程式的分工。
Q2a（機制）：需生成編譯輸入時寫 OUT_DIR，但可能保留前次檔案；兩組依賴即使同 package 也不自動互通。引文：「不能假設每次都是空目錄」。
Q2b（需要）：生成檔案要有明確工作位置，避免改 source；script 若要第三方 crate，應按使用者／編譯平台列依賴。本例只交文字及用標準庫，當下並不需要這兩項擴充。
Q3：有檔案才在 OUT_DIR 建輸出，有 script 第三方 crate 才列 build-dependencies；本例不加依賴、不引入讀生成檔案 macro。
Q4：正文直接限定本例未生成檔案，故缺少生成／讀檔完整例不是当前必要缺口；分類與已讀 manifest／host 階段足夠推得位置。
單位末：合理擴充界線已教，沒有為當下工作增添必須猜的步驟；無必要／可選問題。

原始SHA-256：38407671c10f0985d2b05ce4c1a1c9312eb64a65112287bae5fc736996a07fb7；visible SHA-256：38407671c10f0985d2b05ce4c1a1c9312eb64a65112287bae5fc736996a07fb7。

### 002-005 — 9.2 建置識別與小節末

Q1：package 名稱由 Cargo 自動提供，模式／target 由 script→library 提供，example 只整合顯示；三項都是識別成品建置的資料。
Q2a（機制）：新增選用鍵 CARGO_PKG_NAME 的供應時點已明寫，沿用 env! 取編譯時 &'static str；模式／target 函式結果與 format! 都已教，無額外 API 猜測。
Q2b（需要）：回報問題需附名稱、建置模式與 target；用編譯固定值才能識別成品，改 runtime env::var 會錯把使用者啟動時環境當建置。
Q3：最短已教做法：env!("CARGO_PKG_NAME") 取名称、build_mode／target_os 取兩值，用 `package={}; mode={}; target={}` 的 format! 合成 String 後顯示。未建檔／未實跑。
Q4：practice／linux 的具體識別例與三步說明足以對照組合結果；完成答案是 closed marker，未開。
小節末：外部值驗證→協定交付→編譯固定→執行顯示完整，與下一節 runtime 檔案工作有明確时點界線。9.2 無 necessary／optional；未知實際本機輸出屬未驗，非前提缺失。

原始SHA-256：5c3a424096c9d540001206cb730b45e0fe79466ab89dd3a0ed00468e0a88d08b；visible SHA-256：8642f3423145b024c880da5a0510b729c09a8fe7738f6e9ac9f06833d724d4a3。

### 003-001 — 目錄清單與失敗保存的需求

Q1：由 7.5 單一路徑擴為根目錄及子目錄的一般檔案清單，原生路徑、bytes 長度與修改時間供後續內容工作；本節尚不讀內容／判重複。
Q2a（機制）：先給錯誤政策：根設定不成立無法開始，個別項目失敗保原因但繼續。引文：「没有列出可能被誤認為不存在」。具體走訪 API／資料型別尚待下段。
Q2b（需要）：目錄中檔案可能不可處理，只有成功清單不足以表示實際工作結果；需同時區別失敗與不存在。
Q3：輸入改目錄、包含隱藏子目錄檔案；在另 ch09-files package 工作，命令仍從 repo 根跑。不預先猜走訪／metadata 的 API。
Q4：root 無法開始與某檔失败仍繼續的對比提供先前已教 Result／集合的應用目標；機制合理待教，當下無必要缺口。

原始SHA-256：7809f7400d6502ee277e0c5195b377f8b40ff1e2497596ddbd6619b305dc83c7；visible SHA-256：7809f7400d6502ee277e0c5195b377f8b40ff1e2497596ddbd6619b305dc83c7。

### 003-002 — symbolic link 與走訪範圍

Q1：symbolic link 是保存另一位置資訊的檔案系統項目，目標可為檔案／目錄／不存在；它不同於一般檔案與目錄。引文：「追蹤 link 表示改去處理指向的目標」。
Q2a（機制）：目錄 link 可指回上層造成走訪循環；本例 root link 視為設定錯誤，樹內 link／特殊項目不追蹤而略過。不是目錄不能推得一般檔案，後面需明選 is_file。
Q2b（需要）：掃描使用者選定的一份目錄樹，避免入口悄悄換到別棵樹與 link 引出的循環；將政策允許略過與真正項目失敗分清。
Q3：root 要普通目錄；一般檔案記錄，含隱藏的子目錄往內，內部 link／其他特殊項目略過，無需找其目標。
Q4：五列位置—處理—范围表足以對照入口停止、內部略過與成功記錄；當下不用未教 API 猜實作。
單位末：檔案分類与工作範圍的關係明確，無 necessary／optional。檔案系統循環的原因由本段教出，不以第 8 章 Rc 循環補答案。

原始SHA-256：faa03a39296e61112c98ebf217afea6911ce6ddf824173a0ce21c35144b23d50；visible SHA-256：faa03a39296e61112c98ebf217afea6911ce6ddf824173a0ce21c35144b23d50。

### 003-003 — metadata 觀察與成功／失敗資料

檢查點 A：metadata 查詢。
Q1：metadata 是檔案系統的類別／長度／時間資料，不是檔案內容或鎖；PathBuf 擁有路徑文字不擁有磁碟檔案，承接 7.5。引文：「它不是檔案內容，也不是替檔案上鎖」。
Q2a（機制）：std::fs::metadata 追最後 link，symlink_metadata 查該項目本身，都回 io::Result<Metadata>；file_type().is_symlink、is_dir、is_file 分辨政策類別。
Q2b（需要）：拒絕 root link 及只接受一般檔案，需觀察項目本身，不把目標或「非目錄」誤判一般檔案。
Q3：observe 用 symlink_metadata，再判 is_file，取得 modified，任何查詢／時間失敗回 Io，非一般檔案回 NotRegular；即使先前看過也再觀察。
Q4：表格方法與 observe 分支直接對照，说明位置可能改變而需重新查詢；沒有把觀察視為鎖定。

檢查點 B：Snapshot 的值。
Q1：len 是 u64 byte 長度，modified 是 SystemTime 時間點，兩者組成這次 Snapshot。
Q2a（機制）：u64 是無號有限 64-bit 值，不自动与索引用 usize 互換；SystemTime 有 Debug／Clone／Copy／PartialEq，可副本与比較，但相同長度／時間不證明相同內容。引文：「名字不承諾一致的檔案快照」。
Q2b（需要）：後續內容工作先保存可比較的觀察值，且避免把 bytes 誤當文字數／索引，或把 metadata 相等當內容驗證。
Q3：observe 成功回兩個欄位；本節只保存／比較時間，不需猜秒數換算或時區操作。
Q4：Snapshot 的 derive 與欄位型別符合上述特性；std::time::SystemTime 来源明給，節錄未列 use 的部分可由已教 use 規則匯入，不需新 API。

檢查點 C：資料／原因分開。
Q1：FileRecord 持 owned 路徑與 Snapshot，ScanReport 分 files／issues；RootError 是工作不能開始，ScanIssue 是個別失敗。
Q2a（機制）：沿用 enum／Option／Display：ScanCause 保存 Io、Walk 原錯誤或明確類別；issue path 可能 None、action 為已讀 8.4 的 static literal。walker Error 具體型別與 Debug／Display 已說，走訪 API 合理待下段。
Q2b（需要）：不可用 root 不能被成功空清單冒充；個別不可處理仍保存原因，路徑未能取得也可記錄。
Q3：可由現有 struct／enum 讀出每種结果的資料，Io／Walk 用已有 write! 原樣顯示原因，固定類別給明確文字。
Q4：資料宣告與 observe 的失敗分支互相支持；NonUnicode 尚未在此操作使用，暂不猜其後續處理。
單位末：觀察契約、型別、錯誤層次足夠，無 necessary／optional。節錄的 fs／Path／io／fmt 名稱來源皆已在本段或第 5／7 章教過 use，未把省略 imports 誤認新 API 缺口。

原始SHA-256：c0d259d1c0f4c9211c76b44edb1006d7c2c11deae5d7c398a9bbaf81993f079d；visible SHA-256：c0d259d1c0f4c9211c76b44edb1006d7c2c11deae5d7c398a9bbaf81993f079d。

### 003-004 — WalkDir 的進度與 Option 的零或一份資料

檢查點 A：走訪設定／項目。
Q1：walkdir 管逐層進入與進度，我們保項目政策及資料；WalkDir 有已讀 IntoIterator，每個 Item 是 Result<DirEntry,Error>，與先前一般 iterator 的資料可為錯誤相接。
Q2a（機制）：new 接原生 root；follow_links(false) 控內部，follow_root_links(false) 控 root，後者預設 true。validate_root 先查不追 link 的 metadata，但不鎖檔案系統；Err 後可繼續其他項目，打不開的目錄內容无法取得。引文：「只寫前者不能完成我們的 root 政策」。
Q2b（需要）：讓共同 walker 保進度，自己明訂不追 link、保存失敗；穩定目錄整理仍要區分 root 檢查與 walker 開關，不能過度承諾同時替換防護。
Q3：根 symlink_metadata→拒 link→拒非目錄；walker 兩 false。entry.file_type 初篩、path 借用觀察、into_path 消耗成 owned 路徑；結果後 sort_by，不推定走訪次序。
Q4：manifest 列 runtime walkdir／測試 tempfile；root 分支、兩開關的預設與 ownership 表提供足夠操作依据，無未教 builder ownership 猜測。

檢查點 B：Option 操作。
Q1：error.path 的 Option<&Path> 要成為可長久保存的 Option<PathBuf>；與 iterator map 都沿用 closure，但只處理零或一份。
Q2a（機制）：is_none 借 Option 给 None bool；Option::map 接手 Option，Some 才 FnOnce callback 並包新 Some，None 不呼叫。引文：「沒有丟棄原始 walkdir 錯誤」。
Q2b（需要）：路徑有時不存在不能造假；有時借用錯誤內資料，要 owned 副本才能在 issue 移入原錯誤後保留。
Q3：`error.path().map(|path: &Path| path.to_path_buf())` 只接手暫時 Option，path 副本不再借原錯誤；依已讀最后使用借用規則可再保存原 Error。
Q4：明給 callback 参数型別／結果及 7.8 to_path_buf 來源，不需未知轉換；is_none 的具體路徑表達使用合理待下一段。
單位末：遍歷、分類、borrow→owned、副本與錯誤保存鏈充分，無 necessary／optional。

原始SHA-256：7df243165ab210cbbebb4ab8caa82facf7eb6c11773b26901ecccc30f2fc58ba；visible SHA-256：7df243165ab210cbbebb4ab8caa82facf7eb6c11773b26901ecccc30f2fc58ba。

### 003-005 — 原生路徑身分、collect_entries 與 partial move

檢查點 A：原生 key／報告政策。
Q1：核心 PathBuf 保原生路徑，display 只供人讀；借用 Path 可查 PathBuf key，承接 4.3 Borrow／Eq／Hash 與 6.7 Ord。引文：「沒有查磁碟證明是同一個實體檔案」。
Q2a（機制）：原生路徑組成比較／排序，不以有損 display 文字重建路徑；本次報告只收 to_str 成功。最短前文推論：後接已教 Option::is_none，加上非 Unicode 政策，足以讀出 None 表示不能表示成文字，當下未用 Some 內容，無需額外型別猜測。
Q2b（需要）：不同原生資料可能顯示同文字，需保真正位置；無法供本報告表示的檔案要記 NonUnicode 並繼續，不能悄悄換檔名。
Q3：scan 先驗 root，collect_entries 逐筆保 Walk 錯誤後 continue；一般檔案才檢查文字表示／observe，再 into_path 存 FileRecord，最後按 Path::cmp 排序。
Q4：程式中三條 issues 分支分別保走訪、路徑表示、觀察失敗；最後成功檔案排序，而非丟棄 Err。IntoIterator／Option map／sort_by 機制已於前段保存，不重述。

檢查點 B：partial move。
Q1：main 的 for 接手 report.files，但 issues 欄位獨立，仍能借用；report 整值已不完整。
Q2a（機制）：只移走部分欄位，其餘未移欄位可用，不能再交完整 report。引文：「這叫 partial move」。連接已讀 8.2 非 Copy 欄位 move／有 Drop 型別的限制：本段 ScanReport 未實作 Drop，例子不改寫先前限制。
Q2b（需要）：消耗成功清單來顯示，再看失敗列表與完整旗標，不必複製兩組資料或仍把 report 當完整值。
Q3：for report.files 後，for &report.issues，再 issues.as_slice().is_empty；根 Err 顯示設定失败，不當零成功。
Q4：預期隱藏 third 6 bytes、另外兩檔各 5（含換行）且完整 true，與包含隱藏目錄政策一致；改根一般檔案則 NotDirectory。
單位末：walker→保原因→owned 路徑記錄→原生排序→元件顯示，與 root／項目政策相合；「沒有包裝成完整 CLI 結束狀態」明訂範圍。無必要／可選問題，未看 fixtures 或實跑。

原始SHA-256：64a5e5f7c8991646960421f14e5305fa73908705c288e8b2b759865ae2e9992b；visible SHA-256：64a5e5f7c8991646960421f14e5305fa73908705c288e8b2b759865ae2e9992b。

### 003-006 — 受控 fixture 的平台與根政策

檢查點 A：fixture 新 API。
Q1：沿用 7.8 TempDir 持 owner 到工作完成；create_dir、Unix symlink／OsStringExt 用於造特定材料，不改正式走訪介面。
Q2a（機制）：create_dir 只建最後一層且需已存父目錄；symlink 回 io::Result<()> 建原位置指示；OsStringExt 的 from_vec 接手 Unix 原生 bytes，255 不合法 UTF-8。#[cfg(unix)] 可由 9.1 cfg 條件規則推得只編入 Unix。
Q2b（需要）：測 root／內部 link 與不可 Unicode 檔名，需要確實具有這些性質的材料；不假設所有平台都能用同套造檔方式。
Q3：在已有 TempDir 下逐層造目錄，Unix 測試才造 link／bytes 檔名，掃描完才清理；此單位未示完整測試碼，我未開 source。
Q4：契約的路徑／回傳型別與 255 例明確，非 Unicode 政策不依人讀 display 判斷。

檢查點 B：collect_entries 驗收。
Q1：正式 scan 要合法目錄 root；直接給收集元件的 iterator 材料可以來自一般檔案／不存在路徑，兩層不混用。
Q2a（機制）：WalkDir 以檔案 root 給一筆，缺 root 給 Err；Vec 的 Ok／Err／Ok 形狀滿足先前 IntoIterator 約束，錯誤後继续的機制已在 collect_entries 保存。
Q2b（需要）：穩定驗證中段錯誤保存及後段成功，避免依登入者權限猜檔案一定不可讀。
Q3：受控三筆交 collect_entries，再核 files／issues；其餘列出的政策由 scan tests 驗，所列 --test scan 命令沿用已教 integration test 選擇。
Q4：正文明說「不是把正式 scan 的 root 驗證關掉」，消除元件材料與入口政策看似衝突；僅核文字契約，未見／執行那些完整 tests。
單位末：fixture 的平台、進度與政策目的足夠，無 necessary／optional。

原始SHA-256：dfb894ead6d34c854c0a49b1e6f1365f9f6385728ddf136f9db542ccf404a530；visible SHA-256：dfb894ead6d34c854c0a49b1e6f1365f9f6385728ddf136f9db542ccf404a530。

### 003-007 — 9.3 原生 key 練習與小節末

Q1：成功 FileRecord 可轉成 HashMap<PathBuf,u64>，但 ScanReport 的 issues 仍是完整性判斷資料；查一筆成功與整次成功不同。
Q2a（機制）：依已存 003-005 的 Borrow<Path>、Eq／Hash 與 partial move，移 record.path 作 key 後仍能取 snapshot.len 這個 Copy 值；查詢借 &Path 而不 display 再重建。
Q2b（需要）：以真正原生位置查 byte 長度，保身分一致性，同時不讓單筆命中掩盖失敗。引文：「不能因查到 5 就認定整次掃描沒有失敗」。
Q3：最短已教操作：逐 records insert(path,snapshot.len)，查同一 root 下 nested/second.txt 的 &Path，預期 Some(&5)，另回報 report.issues.len。查詢位置需帶原掃描 root（可沿 7.8 fixture 的 join 形成），不是假定相對檔名本身就是保存 key。
Q4：第二檔 5 bytes 與前單位固定材料相合；完整可執行答案為 marker，外連僅看到連結文字，未開。
小節末：候選分類→原生身分→每項觀察→成功／失敗清單→完整性報告足夠。metadata 成功不保證之後可讀，下一段內容工作合理待教。9.3 無 necessary／optional，未看完整 source／tests／fixtures 或外部。

原始SHA-256：11d44e0a582dea62ae378834ab2a58098105dbd1aff1df920c2d1e81e1836c47；visible SHA-256：ce4f8ff3c9be7658315165bf030b2b3ed1e2d3dcfa3580408a60389dccd2b7f1。

### 004-001 — 大小、摘要與內容確認的分工

Q1：由 9.3 路徑／觀察資料，進到不同路徑的完整 bytes 是否相同；長度分組、hash 摘要、直接比較分三層，不混 metadata 與內容證據。
Q2a（機制）：此導入先明訂篩除契約：不同長度可排除，同長不能；不同摘要可排除，相同摘要仍候選；最後直接比完整 bytes。摘要如何產生的具體機制／API 尚待後段，當下尚未要求操作。引文：「摘要相同仍是候選」。
Q2b（需要）：讓讀取／直接比較集中較小候選，避免一開始所有檔案兩兩重讀；單獨一份長度無配對，先排除可減工作。
Q3：先按已觀察長度分組，留下可配對項再摘要，最後內容確認；本節有限元件，不猜後續完整 CLI／部分失敗政策。
Q4：AAA／BBB 都同長而不同 bytes、單份 6-byte 無對手，具体支持前兩層工作需要；聲稱的是工作分配，非特定速度量測。
單位末：新摘要的選用理由及不能判相同的界線已明，機制待下一段，不将同 hash 當證明。無當下必要／可選問題。

原始SHA-256：68c9975db3d45664b5fad04aff3a8a3eef54b9737acff04888611d59ff312fdb；visible SHA-256：68c9975db3d45664b5fad04aff3a8a3eef54b9737acff04888611d59ff312fdb。

重連恢復註記：本次續讀時，own state／ledger 已開 20 單位至 004-002，原紀錄只保存 19 單位至 004-001。先以原紀錄恢復實際已教背景，保留既存 bytes／原判；僅重新取得這個已開但未存的當前單位，先過濾再核原 visible hash，不推進 gate。下方 004-002 為本次重取正文後當下保存的理解，不宣稱找回上次尚未保存的心中答案。未讀下一單位。

### 004-002 — 大小候選的 borrowed FileRecord 與排序

本單位是重連後重新取得已開但未保存的同 hash 正文，再保存當下理解；原 19 單位未改。
Q1：map 以 u64 長度分類，bucket 保存借自 report.files 的 &FileRecord；map 接手與 rows 排序都沒有接管原記錄或讀取內容。引文：「它不複製檔案內容，也不接管原 FileRecord」。
Q2a（機制）：u64 的 Eq／Hash 支持鍵，Ord 支持數值排序；get_mut 更新已有 bucket，沒有則插入一筆。into_iter／collect 接手 map 的 entries，再以 row.0 排序；sort_by_key 的 mutable、key: Ord 與 unit 結果已在實讀 6.7 教過（由本人 entry 記錄核對），不另猜新 API。
Q2b（需要）：同大小才有可能成對，先留下至少兩筆的大小組讓後续內容工作集中；map 任意走訪順序不能當固定報表，排序後才顯示。失敗清單仍保留，避免候選成功掩蓋掃描失敗。
Q3：5、5、6 入 map 得兩個 bucket，rows 按大小排列，候選數>=2 才顯示，故只輸出 5 bytes 的候選數=2；另顯示 issues.len=0。references 仍依賴 report，不把消耗 by_size 當成消耗 report。
Q4：型別註記、兩條 bucket 分支及最後兩個 println 支持上述結果；「還沒讀內容」限定候選的證據層級。未執行 size_candidates。
單位末：分類→移出 map entries→依數值鍵排序→篩至少兩筆→另保失敗數，靠已教 HashMap／borrow／排序即可理解；無必要未知或新增 necessary／optional。

原始SHA-256：25523e7c7d6d8b79335d3717441d965895339aea3270d73c53899a96ad594833；visible SHA-256：25523e7c7d6d8b79335d3717441d965895339aea3270d73c53899a96ad594833。

### 004-003 — digest、collision 與依序更新的摘要狀態

Q1：此處 hash 是任意 bytes→固定長度 digest 的計算；BLAKE3 的 Hash 是摘要值型別，與已讀 4.3 的 std::hash::Hash 鍵 trait 分開。引文：「它們同名但角色不同」。
Q2a（機制）：相同 bytes 必有同 digest，不同 bytes 可能 collision；Hasher 累積依序 update 的 bytes，finalize shared 借狀態取得當時 32-byte Hash，不消耗／清空狀態。摘要值的 Eq／Hash 可作 map key，但相等只證明摘要相等。引文：「不能替代本次『真正相同內容』的判準」。
Q2b（需要）：同長候選仍可能內容不同，用固定摘要進一步縮小後續直接比較工作；維持 collision 限定才能避免把篩選工具當內容證明。分批 update 使讀取 buffer 重用，不需把全部內容存 Vec。
Q3：new 狀態→反覆 Read→Interrupted 重試／其他 Err 傳回→count=0 才 finalize；其他批次只 update 有效 `buffer[..count]`。沿已讀 7.4 短讀規則，即使未滿 4096 也不結束，尾部舊 bytes 不加入。
Q4：「ABC 再 DE」與「一次 ABCDE」是同一有序輸入，具體支持分批機制；空來源與非 UTF-8 都合法，因工作沒有文字解碼。API 表列接收方式／回傳值，足以理解程式，不需要 BLAKE3 內部演算法猜測。
单位末／最短前文推論：節錄省略 use，名稱來源可直接由表中 blake3::Hasher／Hash 及已讀 std::io／Read 推得；不是額外未知 API。map 鍵資料相等與原內容相等已清楚區分，後续檔案／群組保存合理待下一單位。無 necessary／optional；未跑摘要值或 source。

原始SHA-256：a85a0b279323c3ebd13274fa0d23581189418fd606b1b34a24e5f39a4a84c6a8；visible SHA-256：a85a0b279323c3ebd13274fa0d23581189418fd606b1b34a24e5f39a4a84c6a8。

### 004-004 — 填齊比較段、slice 相等與受控來源

檢查點 A：fill_chunk／完整序列比較。
Q1：read 的任意批次切分與內容序列不同；fill_chunk 先把每邊接成同容量的段，equal_readers 才比較同位置的有效 slices。引文：「切分是讀取行為，完整序列才是內容」。
Q2a（機制）：filled 累積，read 只借尚未填入的後段，到滿或非空後段回 0 才停；slice PartialEq 要長度及每格 byte 皆同。先排除不等，再在 left_count=0 時確認兩份都空；I/O Err 保 Left／Right，Interrupted 重試。引文：「I/O 失敗不是 Ok(false)」。
Q2b（需要）：短讀不同不能被誤報為內容不同，故先消除批次切分差異；不能讀則沒有確認答案，需保失敗來源，不能造出 false。
Q3：ABCDE 上限 3 與 1：四格圖先成 ABCD／ABCD，尾段 E／E，最後兩空才 true；實作每段 4096，邏輯相同。最短推論：只有填齊或抵 EOF 後，段長差異才代表剩餘內容長度不同。
Q4（原位圖）：正文 hook 後核同 frozen SVG 原生渲染：上框左 ABC|D|E、右 A|B|C|D|E 的 | 是 read 切分；第一向下箭頭到各自填 4 格或 EOF，第二箭頭到第 1 段 ABCD=ABCD／第 2 段 E=E。文字補足左第二次只能讀 D 的剩餘一格，圖與 buffer 切片一致。單圖可辨，未看完整頁面。

檢查點 B：assert! 與 fixture 驗收。
Q1：assert! 是已知 bool 結果的檢查；ShortReader／事件失敗來源沿用實读 7.4 及 7v13 局部，供摘要／比較同時驗收。
Q2a（機制）：assert!(condition) true 繼續、false panic，帶 ! 要求原條件 false；short_reader 複製 byte slice 成 owned Vec，正上限限制每次量，零上限直接 panic。引文：「不是處理使用者配置錯誤的介面」。
Q2b（需要）：固定兩種切分要真正確認比較為 true、分批 digest 等於一次 update；拒零上限避免來源在仍有資料時一直以零交付造成不合法的測試設定。
Q3：main 先建兩個獨立來源供比較，再建新來源供 hash；前組已讀完不拿來當完整 hash 輸入。事件來源 Interrupted→R→PermissionDenied，預期重試後保錯誤而非摘要／內容答案。
Q4：可見 main 的 assert!／assert_eq! 與三事件實作可直接核對；正文另外列 9001 bytes／空來源／改末 byte／短尾段的 tests 目的，但未見完整 tests，未聲稱實驗通過。
圖 SHA-256：`ef3f5de75db2c695eec994cac60568da088faee1a9e792f08c69ceb6acc34491`；原生 PNG：`b6c91606406421066cbb8dd94da08eb573db43ca9778ebe9d9af9c386e9ffab4`。
單位末：讀取批次→填齊位置段→slice 比較→相等／不同／不能確認三種结果清楚；所有必要新性質有文字及材料支持，無 necessary／optional，未實跑。

原始SHA-256：5e3a743c9678d1aacc3cc0686d6d04db682d85c361d6769b0a866ca2fd3a7a55；visible SHA-256：5e3a743c9678d1aacc3cc0686d6d04db682d85c361d6769b0a866ca2fd3a7a55。

### 004-005 — File handle 觀察、失敗位置與有限保證

Q1：正確的 Read 不代表磁碟資料固定；File::metadata 查已開 File，observe 查原生路徑，兩者各與掃描 Snapshot 對照。引文：「shared reference 限制的是這份 Rust 借用，沒有替檔案系統所有操作上鎖」。這承接 9.3 的 metadata／Snapshot，而非把借用當檔案鎖。
Q2a（機制）：open_checked 先查路徑、開啟，再以路徑及 handle 驗證類別／len／modified；hash_file／equal_files 在成功讀取後再驗證。FileIssue 複製確切失敗者路徑，保存 action 與 Io／Observation／Changed 原因；左／右讀取 Err 各指原來源。引文：「就算 bytes 比較得到 false，也完成兩邊讀取後的觀察」。
Q2b（需要）：檔案可能在掃描與讀取間變動，不能只信舊 metadata；成功／不同的內容答案需經本節約定的觀察，不能將讀取失敗或可觀察改變當正常 false。保留確切位置與動作，才能辨認哪一份候選在哪個階段失敗。
Q3：hash 只有 open／摘要／後觀察均通過才交 digest；比較要两份 open 通過、equal_readers 有 bool、兩份後觀察通過，才交該 bool。任何 ?／match Err 都傳 FileIssue，不以固定 example 的 expect 改成一般檔案政策。
Q4：可見 wrapper 每個回傳點與 FileCause／FileIssue Display 支持原因／位置保留；固定 first／second 同 digest 後仍比較 true，third 比較 false，沒有省略內容確認。未實跑 example 或讀完整 source。
單位末／最短推論：正文說兩次查詢間仍可替換路徑，足以讀出「不是原子操作」指這些觀察不是一次不可插入變化的整體；不需要另猜同步機制。時間精度與改後恢復例也限制 len／modified 證據。範圍只為穩定目錄的盡力觀察，不保證一致快照、往後不變或惡意替換防護；必要界線已教，無 necessary／optional。

原始SHA-256：e0549e6c1cb483cbe17a05cf7868c30e2077851a3cc02d34e53517cfb7178bcb；visible SHA-256：e0549e6c1cb483cbe17a05cf7868c30e2077851a3cc02d34e53517cfb7178bcb。

### 004-006 — tuple 候選鍵與 owned records

Q1：候選 map 以 (長度, digest) 作 key，bucket 擁有 FileRecord；DigestBuckets 只縮短同一 map 型別名稱。這與 004-002 的 borrowed 大小展示不同，輸入 records 在此被接手。
Q2a（機制）：tuple 的兩欄各有 Eq／Hash 才提供鍵能力，兩欄都同才同桶；hash_file 成功後才移 record 入 bucket。引文：「第一次 FileIssue 就返回，沒有把之前的局部 map 當完整成功」。type alias、filter 借 Item 判斷／保留 owned Item、partial move 都已教。
Q2b（需要）：同長同摘要才需在同桶做真正內容確認；錯誤時不能聲稱所有候選已形成完整結果。owned bucket 讓下一階段持有記錄，不依賴這份輸入 Vec 的借用。
Q3：固定 scan 結果接手 files.into_iter，filter 保 len=5 兩筆，collect 交 hash_candidates；它取得相同 digest、插同桶，得到 buckets.len=1／候選數2。report.issues 未移仍可看，map 任意順序沒有變成固定結果排序。
Q4：型別、key 組成與兩条插入分支直接支持分組機制；末句「目前只形成候選，尚未判定重複」避免把此結果誤當內容確認。example 刻意只給已知兩份固定材料，不宣稱它是任意輸入的完整大小篩選工具。
單位末：大小／digest→同鍵桶→等待內容確認的關係足夠；異長不進同桶、同摘要仍可能 collision，並保留掃描失敗數。無必要未知或 necessary／optional，後段如何分真正內容組合理待教；未實跑。

原始SHA-256：50b738f26b9b484cde5b501480dda8c11e64145bed1891155d1a8f3a622c29fa；visible SHA-256：50b738f26b9b484cde5b501480dda8c11e64145bed1891155d1a8f3a622c29fa。

### 004-007 — 候選桶內的真正群組與失敗重試界線

Q1：同摘要桶還可有不同內容，refine_bucket 需建立各真正群組；ContentGroup 借輸入 slice 的 FileRecord，回傳借用不接手或複製檔案。引文：「上框是候選桶，下框才是真正內容群組」。lifetime elision 與 &FileRecord 可複製 reference 已教。
Q2a（機制）：先以原生 PathBuf→unit map 記已處理路徑；新路徑依次和每組代表 equal_files，相同才加入，全部 false 才建立新組。首位仍以 hash_file 完整讀取／觀察，丟棄 digest 而不拿它判內容；遇 FileIssue 的 ? 返回 Err，不交半成品群組。
Q2b（需要）：只比桶第一份 AAA 會漏掉兩份 BBB，故每種內容需要代表；重複同路徑不是另一份檔案，且 singleton 沒比較對手仍需驗可讀性。引文：「不是假設 caller 剛才一定已讀完」。排序後使有限元件的有效輸入有固定同平台順序。
Q3：a 建 A 組，b2 與 a 不同建 B 組；b1 與 a 不同後再與 b2 相同才入 B。各組都非空，先排序 members，再以各組首位路徑排序 groups；只改 references 排列。完整工具若重試，需保存錯誤並移除確切失敗路徑，候選數確實減少才有限進展。
Q4（原位圖）：hook 後核同 frozen SVG 原生圖：上框手工同 digest 的 a:AAA／b2:BBB／b1:BBB（皆3 bytes），向下到 b1 先比 a 不同、再比 b2 相同，再到群組1 a／群組2 b1,b2；圖中文字、兩箭頭與程式搜尋各代表相合。這是受控同摘要安排，未假稱找到 BLAKE3 真實 collision。
圖 SHA-256：`3cd8b006d9184d9b6001ec567bc8154ad811d0466cbb0595c47dec9aad27fe82`；原生 PNG：`9b9bf0e43cd5081e4230511d601ef05198cce97c0ab0b201086b1f0a0709f615`。
單位末：候選→原生位置去重→各組代表的直接內容確認→borrowed 真正群組→報告時再篩至少兩路徑，關係完整。單元 Err 不等於不同內容，也不證明其他項目沒問題；完整繼續政策明留練功專案，不是此有限元件當下前提。無 necessary／optional，未實跑／看完整頁面。

原始SHA-256：98dc8cabf1000c03cd02d9f64cc972907de54f5c15b73c97f075fe3e9a18647b；visible SHA-256：98dc8cabf1000c03cd02d9f64cc972907de54f5c15b73c97f075fe3e9a18647b。

### 004-008 — 按路徑的內容指標與受檢整數運算

檢查點 A：指標／hard link。
Q1：真正相同內容組依不同路徑計數；hard link 是同一實際檔案的另一名字，與先前 symbolic link 的指向資訊不同。
Q2a（機制）：保一份後的額外份數為 path_count−1，再乘每份 len；兩個 hard-link 路徑仍各算一路徑。引文：「不是實體磁碟可回收量，也不是刪除建議」。
Q2b（需要）：提供可解讀的重複內容數字，同時限制用途，避免把按路徑統計誤當可刪除／節省實體空間的證據；當下不用辨識 hard link API。
Q3：三份3 bytes 為6、兩份為3；零／一路徑先回0，空 bytes 的乘積為0。不用既有所有權規則推測磁碟實體分享。
Q4：兩個小數字例與 hard-link 反例足以顯示公式及有限用途，沒有要求實際刪檔。

檢查點 B：TryFromIntError／checked_mul／checked_add。
Q1：len:u64 與 path_count:usize 不自動同型，額外路徑數先經已讀 TryFrom；新選的標準 TryFromIntError 保存不可表示的原因，非先前自訂 Count 錯誤。
Q2a（機制）：checked_mul／add 按值接數字，可表示結果回 Some，超範圍回 None，不繞回；MetricError 分 Count 與 Overflow。引文：「來源數字無法由目的整數型別表示」。兩個轉換／運算的失敗不同。
Q2b（需要）：重複量及累計仍是有限 u64，不能因一般算式看似合理就讓溢位數字冒充真結果；需將表示或計算失敗交 caller 判斷，而非只靠 9.1 profile 的 panic。
Q3：先 path_count<2 避免減法下溢，再 try_from(path_count−1)，再 checked_mul；累計獨立 checked_add。轉換失敗先回 Count，運算 None 才 Overflow；因此不把所有錯誤說成「沒有重複」。
Q4：可見分支明列所有正常／失敗回傳，Debug 的前提亦已交代；可由前文 Option／Result／match 讀出處理流程，沒有新必要猜測。
單位末：指標含義→型別表示→乘法／累加→明確錯誤政策完整，無 necessary／optional；未實跑或看後段答案。

原始SHA-256：408b540144a83f23e27fbda6c1eb77777b25c538e831bf10044a05476f37f439；visible SHA-256：408b540144a83f23e27fbda6c1eb77777b25c538e831bf10044a05476f37f439。

### 004-009 — 受控同摘要練習、fixture 新選 API 與小節末

檢查點 A：練習與既有元件銜接。
Q1：此練習把同摘要候選與真正內容群組分開；固定 Hash 只是手工測試鍵，不是 AAA／BBB 真摘要，延續 004-007 的群組責任。
Q2a（機制）：Hasher::new().finalize 的空輸入摘要已有契約，Copy 可重用鍵；用三個 FileRecord 放同一 Vec bucket 再 refine_bucket，輸出借原候選，TempDir 及候選記錄須留到確認结束。
Q2b（需要）：不必找到真 collision 也能穩定驗出「只比第一代表」的缺陷；依真正兩組算量，避免把同摘要三筆直接誤算為6 bytes。引文：「只作測試鍵」。
Q3：最短已教做法：fs::write 造無 LF 的 AAA／BBB／BBB，observe 建 records；一次同鍵 bucket 存三筆 Vec，refine_bucket 後得 a 與 b1,b2，再各用 duplicate_bytes／add_duplicate_bytes 得0+3=3。仍需處理各元件 Result，不將 Err 當少一筆重複。
Q4：四步列出 owners／材料／鍵／排序／指標，可用已讀 API 完成，不必猜新函式；closed 完整答案未開，未建檔／實跑。

檢查點 B：新增 fixture 選用與有限驗證。
Q1：u64::MAX 提供整數界線，fs::hard_link 提供 Unix 的同實際檔案兩名字材料；都只為驗收前段規則。
Q2a（機制）：MAX 是 associated constant 不是函式；hard_link 接原／新路徑回 io::Result<()>，新名字指同一實際檔案。沿前文 associated constant／fs 路徑與平台 cfg 規則即可讀出用法。
Q2b（需要）：直接用有限最大數驗溢位，免巨大檔案；hard-link 材料驗按路徑指標，不冒稱實體空間量。引文：「沒有把平台未跑的結果算已驗證」。
Q3：可用 duplicate_bytes(MAX,3) 或 add_duplicate_bytes(MAX,1) 對照 checked 運算的 Overflow；兩個3-byte hard-link 路徑計3。讀完再 fs::write 改長度，再 verify_unchanged，只驗觀察可揭露變動。
Q4：正文明確區分可控制的前後觀察測試與同步修改每次 read；列出 streams／files／groups 驗收目的，未展開完整 tests，外連只看到文字未開。
小節末：大小筛候選→digest 再分類→完整 bytes 確認→原生路徑群組／有限指標→成功與失敗驗收，四題整體可答。短讀不當差異，碰撞不當相同，Err 不當正常答案，觀察不當一致快照；無必要未知或 necessary／optional。第9章正文至此讀完，未擴第10章／peer／archive。

原始SHA-256：8cc51c44ac5d562a965346f4f8da8d486829c840e481bb697cbf1f3ef05f77f0；visible SHA-256：7cf6a4c99fc81749adb415453aa7f41bbcf09c54650daee3686d6c403f4c09da。

### 9.4 小節末四題整體關係補記

Q1：掃描 FileRecord 的原生路徑／Snapshot 是來源記錄，大小／digest 是候選鍵，borrowed ContentGroup 才保存真正相同內容的分組。
Q2a（機制）：有效前段更新摘要、填齊段比較完整 bytes、前後 metadata 觀察、各组代表比對及 checked 整數運算，各自保留已教的有限契約；沒有把一次 map 命中或觀察成功當完整內容證明。
Q2b（需要）：為降低無須比較的工作，又能保留真正內容判準、失敗者與按路徑指標，先分層篩選再確認，避免漏掉同桶第二種重複內容或錯報成功。
Q3：大小候選→hash_candidates→逐桶 refine_bucket→只報至少兩不同路徑的組→duplicate_bytes／安全累加；有限元件的 Err 先交 caller，完整繼續政策留後續練功，不猜實作。
Q4：ABCDE 異切分圖支持完整序列比較，手工 A／B／B 圖及練習支持各代表分組與總量3 bytes；實際命令、完整 tests 與答案皆未拿來當已驗證證據。

## 原始首讀封存

唯一新範圍為 ch09-complete-prerequisite-v5 的全4頁／27 BODY 單位，沿 reading-order 順序存四題／Q2兩面、引文、未知與 hash 後才看下一單位。重連前已存19個單位的34357 bytes 前綴原樣保留，其 SHA-256 為 `7a111dc282254670ddefbe6e9655305b6b5afaf8cb94c04fc7065cea63aaf915`。004-002 已開未存的重取時點與限制明記於當下紀錄，沒有回填失去的心中答案。

原始判斷：necessary 0、optional 0。共用 dependency／profile／features、建置 script 的時點與協定、目錄走訪政策／原生身分／每項原因、大小／摘要／完整內容證據與有限觀察／指標，皆能從實讀前文與當下正文建立原需→改變→用途，不需作者或 peer 答案。

過濾 ledger 記3個 Markdown closed fold、0個 HTML closed fold；完整答案內容未見。BODY 實際指到3個原位 SVG：build-phases、short-read-comparison、candidate-content-groups，均在原位後核同 frozen 單圖原生渲染。未因範圍通知的可能圖數而搜尋其他圖。

通過範圍為上述 BODY 的理解、可見片段／已教最短推論與3張原位圖关系。未讀 archives／完整 source／完整 tests或fixture檔／author／peer／live／外連；未實跑 Rust／Cargo／錯誤探測，也未親看完整桌面或手機頁面。實際編譯結果、平台覆蓋、效能及整頁版面仍未驗，不算必要前提缺失。第9章全原始理解完成後封存，停下，不讀第10章或互看資料。
