# 第 3 章修後局部核對

這不是新的 blind 首讀。以實際讀完第 1～3 章為基礎，只核對本輪指定 ch03-review-fixed-v6 单位；原 primary 首讀／分級／hash 不變。每單位先保存四題要點、引文與必要未知，再開下一個指定單位。不讀其他 review／問題清單、作者 notes／outline、未指定正文／後章、Rust 答案或 sources。既有圖 bytes 未變，本輪只核對原位文字，不重開圖片／整頁；未實跑。

另允許的單一 archive Cargo.toml 將在六單位後補讀：它未在 primary 首讀完整讀過，會獨立記錄当下理解／setting 前註解，不能回填正文初判。

### ch03-v6 / 004-003 首次 move 前先說 String 無 Copy

- Q1：new／push_str／as_str 與 String 管理、&str 借讀角色不變；新增前置「String 沒有 Copy 能力」使末段 moved 的身分在程式前明確。
- Q2 機制：完整 String 交 ownership 給 moved，不能再從 greeting 用它；show 僅借，last use 後可追加。需要：把轉管理與只讀分開，首次看到賦值已能知道原名不可再用。
- Q3：仍得 Hello, Rust／Hello, Rust!／Hello, Rust!，最後改用 moved。本人原 004-003 只保存「示例未再用 greeting，未單靠它證明舊名失效」；修後前置明句已提供答案，不把原證據限制改写成原始缺口。
- Q4：原圖兩箭頭仍只表管理／借用，不是 Copy 錯誤測試或完整布局；bytes 不變，本輪未重看圖／整頁。已知後文 004-004 也教此規則，本輪非盲讀，新增前置關係可核對、無新負擔。

本輪正文 SHA-256：3688dacfaff0c7ef1762b85649c6f1982fc19b025146a105679ded56335e3310。

### ch03-v6 / 004-004 move 與 view 衝突的後續證據

- Q1：开頭改為回指前單位已教的指派；E0382 原名失效、&str Copy 位置長度、E0502 借用衝突都維持原讀關係。
- Q2 機制：move 不另複製全部 heap 內容／雙 owner；追加可能換空間，存留 view 使用與 mutable 存取不能重疊。需要：轉管理用 move，只讀用 as_str，不以本次剛好不換空間逃過契約。
- Q3：second 可讀、first 再讀錯；把 view 最後讀放在 push_str 前可順序進行，已有 2.4 last-use 機制足以理解。
- Q4：與前單位的規則前移銜接清楚，後單位錯例提供具體失敗證據；「不是每次追加必定更換空間」界線仍明示。無新增必要負擔，未實跑。

本輪正文 SHA-256：c7dfc1bf8c2e3b1d3d5cf4f2b1af43be22e22424588408c78309b0efe6f0cd27。

### ch03-v6 / 005-002 char 顯示能力前置與計數單位

- Q1：scalar、char、byte 與可見字形的身分不變；新增「char 也支援…{} 顯示形式」明确此具體型別能力，不推广为未知泛型 T 可印。
- Q2 機制：char 保存一 scalar，UTF-8 1～4 bytes，len 數 bytes；完整 scalar 未必完整字形。需要：切文字先找 byte／scalar 分界，顯示 char 現在可先從定義取得接口依據。
- Q3：A中🦀 的 len8、合法端0／1／4／8；1..4 取中、1..2 截其中一 byte。原首讀在下一單位的完整呼叫／輸出得知 char 可印，修後提前明教；不改原判。
- Q4：特定編碼表不能推广所有中文／符號長度；圖文字仍對應 crab 與三卡範圍，本輪未重看既有圖／整頁。無新增必要負擔／未明界線。

本輪正文 SHA-256：6b55ed58cd8f212671fab918c95d4fd96ecc1306c5c1b7ec97f65ca8272c47f1。

### ch03-v6 / 006-002 Vec 元素空間／空建的範圍限定

- Q1：Vec<T> 身分与各接口仍是本人已讀的模型；現在 heap／連續空間明確限「本節的 i32／String」，Vec::new 是空容器，不預配元素空間。
- Q2 機制：型別不寫長度、同型元素、owned 清理；new 的空建不等於已請求元素空間。需要：不定長結果與 slice 借內容仍有不同管理角色，新增限定避免把容器存在誤當必已配置或推遍未知 T。
- Q3：push三項→get1成功／get9缺→remove0交80→pop交200→借改剩120為130；接口取值 ownership、borrow、None／panic 邊界均維持，不需新流程。
- Q4：remove(len) 的錯例不改；表清楚區分 pop 空 None 與 remove 越界 panic。本輪只支持指定元素型別／接口，其他 T 配置規則未教且不需猜。無新增必要負擔。

本輪正文 SHA-256：4d9240a8052a9362bd501952e6f73e11b88d8e167f31e176f21293c2025cfec5。

### ch03-v6 / 006-003 元素轉交與 Vec 借用衝突複核

- Q1：Vec<String>／&String 的兩種角色與本人 v3 理解一致；Vec 自身無 Copy，即使 i32 元素能 Copy，仍不等於 Vec 能 Copy。
- Q2 機制：push 接 message、pop 交 taken，get 借元素；push 可能換存放區，Some 包 reference 不解除 borrowing。需要：區分讀元素與取回 owned 元素，避免舊名／存留借用與修改衝突。
- Q3：正常路徑印 Rust、len0；錯例先借 first、push 後仍讀則 E0502，应先完成 last use 再改。沒有因排版或泛型字樣需要額外技術答案。
- Q4：正常／錯例證據與「不能依賴…恰好沒有更換」界線仍足夠；未驗配置細節／程式。無新增必要負擔／未明界線。

本輪正文 SHA-256：6dedb66d897ebca64d5acdd758309023f344ef15978d19bac03e21ed1560a535。

### ch03-v6 / 006-004 過濾結果與測試複核

- Q1：&[i32] 借原材料、Vec<i32> 擁新結果，以及 library list 分工，與本人 v3 initial 相同。
- Q2 機制：position<len 才索引、i32 Copy、>=0 才 push、每次加一、尾值交 Vec。需要：按原順序保留非負與重複值，不改原輸入；Vec 只提供可變長表示，不改筛选規則。
- Q3：本例 [120,0,80]，重複測例 [80,80,0]，空／全負都空；只比取出數字與長度，符合已教 assertions，未要求整 Vec 的新能力。
- Q4：兩個展示測試與正文「共六項」界線維持；其餘四個文字測試仍未在指定正文展示，未另讀 sources 或實跑，不能據此宣稱測試通過。無新增必要負擔。

本輪正文 SHA-256：c92f6babe3fab5f0acbc966e732e378b4aa736ef0996ac3ccb618bb2d7b59846。

## 指定補讀：archive Cargo.toml（不是正文修後盲讀）

本輪第一次完整開這份 archive Cargo.toml，路徑為 ch03-review-fixed-v6/sources/examples/ch03-data/Cargo.toml；只開此一指定設定檔，未開相鄰 Rust sources／答案。此前 primary 首讀沒有讀過完整 manifest，以下理解不回填 primary。

既有 package 設定：
- Q1：[package] 的 name="ch03-data" 是 package 身分；四項 .workspace=true 要繼承根共同值，沿用本人 1.3 與 v6 已讀根／成員模型。
- Q2 機制：共同值由根提供、成員明選繼承，name 留自己的。需要：命令 -p ch03-data 按名稱選這個 package，版號／edition／最低版本／publish 共用配置。
- Q3：可以對上第 3 章 -p 名稱；這次只開成員 archive，沒有重讀此凍結版根設定，不宣稱具體繼承值或完整 repo 建置已核對。
- Q4：第1～6行支持身分與繼承選擇；它不是全部 workspace／依賴／lockfile 的展示，無必要理解缺口。

註解與單項 Clippy 提醒：
- Q1：第8行先明教 TOML 開頭 # 是註解；第9行説明 [lints.clippy] 管這個 package 的提醒級別，manual_map 是一項提醒名，不是已教 Rust 函式。
- Q2 機制：第10～11行说它建議簡化 match，"allow" 只讓此項不報提醒、其他檢查照常。需要：「保留本章已教的 match」，所以控制這一提醒而不必現在學未給出的簡化寫法；Clippy 與測試／編譯作用差異沿用 1.10。
- Q3：第12～13行按註解可讀為此 package 抑制 manual_map；註解在設定前提供當下用途，沒有要求先查提醒列表／實際替代方法。
- Q4：本檔支持作者指定的提醒政策與註解不改設定；未實跑驗證工具是否接受／實際報什麼，也未猜 manual_map 全部觸發形式。判定：補讀理解足夠，沒有必要新未知；不是替 primary 補回一項原缺口。

補讀 Cargo.toml SHA-256：633e1d3caf2ea700e3d6cb4495696f93e054532907ec43243221721a9c5f5ed7。

## 本輪局部核對封存

指定 6 個正文單位已依序保存理解／引文／本輪 hash；沒有新增必要負擔或未明界線。此為已讀 1～3 章後的局部修後核對，不是新首讀，不支持未開單位與整章的全面重驗。

String 無 Copy／char 顯示能力前置、Vec 配置限定均能理解；單一 Cargo.toml 的指定完整補讀另記，未回填 primary。沒有開任何 Rust 答案／sources。

既有圖只核對原位文字、未重看圖或整頁；不宣稱手機／桌面或實跑通過，未誤見後章／他人 review／作者 notes。原 initial 四份檔案與原第1章 follow-up 前綴已核對未變。完成後停止，等待下一個指定任務。

