# 第 3 章首次順序閱讀原始紀錄（v3）

這是 AI 模擬首次順序閱讀，不是真人測試。目標讀者只懂基本 Python；只使用實際已讀前文，沒有以既有 Rust 知識補答。共用檔案系統不是隔離。本輪只開 reading-order 所列正文，每單位先保存當下四題、短引文與必要未知才開下一個；新圖只在正文原位提到後查看指定手機圖副本。

前文基礎：第 1 章首三頁實際讀 ch01-first-three-v1，其餘七頁讀 ch01-full-v2；另已讀 ch01-review-fixed-v6 指定八個已讀單位的修後核對（不是新盲讀，未回填原判）。第 2 章實際讀完 ch02-visual-fixed-v3 的 31 單位，初判已封存。本輪不改既有第 1–2 章紀錄，也不看第 1 章 v8 未提供修句。

已沿用 clear-tutorial 的 review-protocol/calibration 通用判準；不讀作者 prompt、術語答案、source-notes、ledger、manifest、課綱、他人 reviews、網路或外連 answers。四題：Q1 身分與前文關係；Q2 分開機制／需要；Q3 操作與結果；Q4 示例／圖的證據限度。已教機制只定位來源，新必要概念在首次需要時作答；小節末只補整體關係。必要缺口、選讀改善、合理待教與未驗證分開記。未實跑程式或親看整頁版面，不宣稱相關驗證通過。

凍結包：ch03-prerequisite-v3。reading-order.txt SHA-256：eba50b935aa83743e372eb83ab75a6d3db021bb649589eb2630604c306898297。共 32 單位。

## 001-001 3.1 導入：把不同情況也存成資料

- Q1：此處需要表示報價的三種情況，延續上一章 Order 把單價、數量同時存起來，現在是互異情況。依據：「尚未提供」「有金額」「被拒絕，附上一個代碼」。
- Q2 機制：只先提出讓「情況本身也是資料」，enum 如何寫仍合理待教。需要：若用 0 兼當無報價，無法分辨缺少與合法零元，這個歧義獨立於 Rust。
- Q3：目前能選出要保存哪種報價情況；尚未要求寫程式，所以沒有必要語法缺口。
- Q4：零元反例證明單一數字不足以保存情況；沒有證明後續 enum 語法或存放方式。判定：足夠，具體實現待下文。

正文 SHA-256：e289c0355bde29d993f895228b9c0cf1dc84aaebb571a7ebb1f7856b1cee4fff。

## 001-002 enum／variant 的身分與資料形式

- Q1：enum 是自訂型別；variant 是該型別列出的互斥形態，一個 Quote 值只處於一種。與 2.1 struct 同時具有各欄位不同，這裡先選情況再決定附帶資料。
- Q2 機制：「某個時刻只屬於其中一種 variant」，Missing 無資料，Amount 位置帶 i32，Rejected 命名帶 code；Quote:: 指定型別的形態。需要：解決 001-001 缺少與零元混淆，且拒絕時還需保存代碼。
- Q3：能照表建立 Quote::Missing、Quote::Amount(120)、Quote::Rejected { code: 37 }；定義用 enum Quote、逗號分隔。Missing 沒有金額，不能直接乘數量；如何辨形態仍合理待教。
- Q4：表中三個值證明同一 Quote 型別可有不同資訊形式；不支持所有情況都可當 i32。判定：新概念足夠，操作辨形態尚未要求。

正文 SHA-256：ace8de888b01fe0600ed4f4e821da66974e1e496a45f4cef601bf29ac917b97e。

## 001-003 pattern、match、arm 與完整分支

- Q1：pattern 描述已有值的形狀並可命名其內部資料；match 以 pattern 選工作，每對 pattern => 工作是一個 arm。Amount(amount) 此時拆值，與前單位 Amount(120) 建值方向不同。
- Q2 機制：「只執行第一個符合的分支」，且編譯器檢查涵蓋全部情況；數字可當 pattern，_ 任意符合但不綁名，剩餘分支放末尾。需要：先確認 Quote 形態才安全取其金額／代碼，避免把缺值當金額，也避免漏處理。
- Q3：完整值傳 show_quote 交 ownership（2.2），第二呼叫取 amount=120 只印報價；第三取 code=37。match 結果是選中 arm 的值，同型別要求沿用 1.6 if；遺漏 Missing 的 amount 函式需先決定缺值政策再添 arm。
- Q4：三行輸出支持三次各選一支；E0004 片段支持涵蓋檢查，不支持任意補值就符合需求。判定：足夠；命名欄位解構由「{ code }」與其對應欄位明示，可跟做。

正文 SHA-256：2204b86a6afb03683555dd52cf2485e414abb2561d3ae7ec0ca2548176b95128。

## 001-004 shared-reference 的 enum pattern 解構

- Q1：此處 match 的材料是 &Quote，仍用同樣 variant 外形，內部 amount/code 改是 &i32。這是新解構規則，引用本身與 * 取目標值沿用 2.3。
- Q2 機制：「Rust 會借用符合的內部資料」；*amount 讀 Copy i32 不搬走 Quote，借用有效範圍仍服從第 2 章。需要：呼叫者還要原報價，可多次檢查而保留 ownership。
- Q3：用 is_available(&quote)；Missing/Rejected 得 false，Amount 依 *amount >= 0；Rejected 先印 *code 再尾值 false。每 arm 都 bool，符合前單位 match 結果規則。
- Q4：兩次同筆 Amount 得 true 支持 shared 借用後可再用；其他 assertions 支持這兩種 false，沒有涵蓋負金額。判定：足夠；不能誤說 match 一律消耗目標。

正文 SHA-256：8caf992adec477fc17a2b3c0ea2df6abdb3655284da9c79c11fc7d4dd81fee68。

## 001-005 if let 與 let else 的成功範圍

- Q1：if let 是只關心單一 pattern 的條件工作；let else 是先排除失敗再留下成功綁名的宣告。沿用 001-004 的 shared 解構，amount 都是 &i32。
- Q2 機制：if let 成功才進 body，名稱限 body；let else 成功後名稱供後面 scope 用，失敗 else「必須離開目前這段流程」，本例 return; 结束 unit 函式。需要：前者只顯示有金額，後者保證後續加運費時有金額。
- Q3：Amount(120) 先印 120，再 show_amount 印 140；Missing 印略過並 return，不加 20。let else 外末尾分號結束宣告；if let 沒有全面涵蓋檢查，不能取代需分別處理每種情況的 match。
- Q4：三行輸出支持兩種成功範圍與 Missing 早退；簡化 Quote 明示只含 Amount/Missing，不把 Rejected 處理偷偷省去。判定：足夠，借用不延長 owner 生命仍沿用 2.4。

正文 SHA-256：ebb8057dd18ba8df281aea1bcce89f9aeca1036edce55a0c5ff92208e1817946。

## 001-006 3.1 小節末：Shipment 練習

- Q1：Shipment 與 Quote 同為 enum，Waiting 無資料、Ready 有命名 count；全部必要形態與 shared 解構已在 001-002～004 教過。
- Q2 機制／需要：&Shipment match 的 Ready count 是 &i32，*count 回傳 Copy 數字；Waiting 的 0 是可出貨統計，不會抹去原資料的等待情況，避免與節首缺值混淆。
- Q3：可寫兩個 arm，Waiting => 0、Ready { count } => *count；測 Waiting 0、Ready 3、同筆 Ready 再 3，assert_eq! 只比較回傳數字，沿用 1.10。
- Q4：練習可檢查分支結果及未消耗同筆資料，沒有驗證任意數量或整個 enum 可直接相等比較。未開外連參考、未執行答案。小節整體：情況由 enum 保留，pattern 分辨並取資料，參數借用決定能否再用。無必要缺口。

正文 SHA-256：31f69fd0b92c39dc0fa1951a428d2bd39c6d0611add8c031295d039e5b080b86。

## 002-001 3.2 導入：Option 與 Result

- Q1：Option、Result 是標準函式庫已提供的 enum，與前節自訂 Quote 同類；本輪只先放具體數字。
- Q2 機制：Option 表示「答案或沒有答案」，Result 表示「答案或失敗原因」；其具體 variant、型別寫法尚合理待教。需要：常见形態可使用既有型別，不必每次取專屬情況名。
- Q3：能依工作需求選缺值或帶原因的失敗，尚未要求寫宣告或呼叫。
- Q4：導入區分兩種需求，未證明互相可代換。判定：足夠；後續需教各形態與數字內容對應。

正文 SHA-256：b149a94427c82ceaa61caac3daf942a96593e01258d896bc88f9813538e55a66。

## 002-002 Some、None 與名稱可用範圍

- Q1：Some 是 Option 有值的 variant，None 是無值的 variant；不能當成普通數字或與 0 混同。enum／pattern 機制沿用 3.1。
- Q2 機制：「自動帶入一般程式可用的範圍」，所以直接寫 Some/None、不加自訂 Quote:: 或下載套件；Some 內數字可供推導型別。需要：保留是否有金額，操作前先處理缺值。
- Q3：可用 Some(120) 或 if bool 的 Some／None 初始化同一變數，再用 Some(amount) 與 None arms 取值／決定缺值政策。兩 if 分支同為可能有數字的結果，符合 1.6 同型別要求。
- Q4：文字建立路徑支持選兩種 variant；未提供完整程式與型別標註，當前尚未要求，合理待教。判定：足夠。

正文 SHA-256：52032f96a5b4364fd8c73821de5f829f0de62f8f16a5e7ff9513f9be0806714e。

## 002-003 Ok、Err 與先檢查再除法

- Q1：Result 的 Ok 保存成功值，Err 保存失敗資料；DivideError 是自行設計的 enum，ZeroDenominator 是此例唯一原因。與 Option None 不帶原因不同。
- Q2 機制：「Err…不會自動顯示錯誤或停止程式」，接收方 match 選處理；if 零分支建立 Err，只有非零分支執行 /。需要：已知 1.4 零除非法，可保留原因而不用假成功数字。
- Q3：for 的 2 得 Ok(6)，0 得 Err，分別印商與原因；Ok/Err 可直接使用由本完整示例的無前綴寫法取得，不必補自訂定義。Some/None 前兩段印 120／缺值，沿用前單位。
- Q4：四行輸出支持指定材料的分支，沒證明所有 i32 邊界都安全；正文已限小整數、只建零除政策。判定：足夠，不把 Err 視為 panic。

正文 SHA-256：7e1693b568b99958742b1bdd1bedd6994f26896729f1ac1f95f1f7694f92145c。

## 002-004 選 Option／Result 的資訊需要

- Q1：兩種標準 enum 的形態已在 002-002／003 教過，此表補接收方要回答的問題。
- Q2 機制／需要：None 不帶原因，Err 可帶原因；需不需要原因決定選哪種。「沒有值」本身不等於崩潰，沿用 Err 由 match 處理。
- Q3：只需成功時工作可用 3.1 if let Some／Ok；若原因影響結果，明写 Err 工作，不能只略過。
- Q4：比較表支持資訊需求選擇，未聲稱所有失敗都可用 None。判定：足夠，沒有新必要前提。

正文 SHA-256：7fd17a2274138123e9c8241d6c376ef4fd0cf6fa4126870e7773601bc66535ef。

## 002-005 3.2 小節末：零與負數練習

- Q1：AmountError／Negative 可照 002-003 的 DivideError／ZeroDenominator 定義；Result 結構已教。
- Q2 機制／需要：非負建立 Ok(amount)，負數 Err(AmountError::Negative)，原因供 match 處理；這樣合法 0 不會與失敗混淆。
- Q3：可用 for amount in [0, -5]，先依 amount < 0 選形態，再選 arm 印接受 0／拒絕負數。本題先留在迴圈、不要求尚未教的回傳型別。
- Q4：兩項材料驗證零成功及負數失敗，未驗全部範圍；未開參考答案。小節整體：Option 保留有無，Result 還保留失敗資料，兩者都需接收方決定工作。無必要缺口；完整型別合理待下一節。

正文 SHA-256：99ac6e6741fbe37c98441a3cf50e4b91b248f9f845df3ca9be8398844fb220bd。

## 003-001 3.3 泛型與型別參數

- Q1：泛型讓定義使用代表型別的名稱，使用時填具體型別；型別參數代表 i32/bool，不是值參數接到的 120/80。
- Q2 機制：「先替要填入的型別取名稱，在使用時決定具體型別」。需要：Some／None 的有無形狀可保存數字，也可保存 bool 設定，不必各寫不同形狀。
- Q3：能分辨改的是內容型別而非外框；實際型別參數宣告和填入語法尚合理待教。
- Q4：設定有無例子支持同形狀不同內容的需求，未支持任意未知型別皆能加總等操作。判定：足夠。

正文 SHA-256：77ae574ccd666231889380ca2bb8abc88b62c8e485634f84aafdaf17dc77bf93。

## 003-002 Holder<T> 與 hold<T> 的型別對應

- Q1：Holder<T> 是 value:T 的泛型 struct；T 是定義處取的型別名稱。Holder<i32>／Holder<bool> 是不同具體型別，沿用 2.1 struct 欄位。
- Q2 機制：角括號填型別，hold<T> 的參數 value:T 與回傳 Holder<T> 共用同一 T；不允許同個已定型盒子任意換數字／bool。需要：保存任意所選型別時保留輸入與欄位一致的關係。
- Q3：可寫定義後以具體 i32/bool 使用；函式只把收到的值交進盒子，Copy／move 仍由該型別决定（2.2）。未知 T 不可假定能乘或列印。
- Q4：文字支持型別一致及泛型不取消 ownership，未支持「盒子」必在 heap。判定：足夠，完整推導／呼叫程式待下單位。

正文 SHA-256：639d67c4974d864e644b45060877d539f4574d7afac6208c692fa61c2450193a。

## 003-003 泛型推導與 turbofish

- Q1：::<i32> 是呼叫時給型別參數的 turbofish 語法；後面的 (80) 給值參數。變數 :Holder<i32> 是另一條已定型需求，推導沿用 1.4。
- Q2 機制：hold(120) 可由輸入／接收型別推導 T，hold::<i32>(80) 直接指定；省寫不等於無型別。需要：推導不足或想明示選擇時使用，足夠時可省。
- Q3：number 是 Holder<i32> 值120，ready 是 Holder<bool> 值true，explicit 值80；Holder { value } 欄位 shorthand 已在 2.5 教過。只列印各已確定型別的欄位，未對未知 T 列印。
- Q4：三項 assertion／兩行輸出支持同一保存定義供不同型別使用，未允許同一盒子換型別。判定：足夠。

正文 SHA-256：128f6a5ee5ba23b9cf20151921da01daf5441fc16debfd1cfc25ef70b52ccff6。

## 003-004 共用 T 與兩個型別參數

- Q1：Pair 的 first／second 都寫同一 T，是同筆兩欄同型別要求；若改 Pair<T, U> 則是兩個可各選的型別參數。
- Q2 機制／需要：重用參數名稱建立相等關係，逗號宣告不同參數可保留不同型別。需先決定資料模型要同型或異型，才能選修输入或修模型，不能只為過編譯改需求。
- Q3：120 與 true 建原 Pair 不符，E0308；同型需求改其中一值，異型需求改 first:T、second:U 的定義。
- Q4：錯例支持同 T 約束，並不支持泛型禁止異型欄位。判定：足夠，兩參數修法有完整欄位對應。

正文 SHA-256：689a589a02dd5a86ef1bc8a7ae0f9b4736f7cdc4f884c2890055b57383b71e4f。

## 003-005 Option<T>／Result<T, E> 的完整型別

- Q1：Option<T> 的 T 是 Some 內容；Result<T,E> 的 T 是 Ok 內容、E 是 Err 內容。這把 3.2 形態接到本節型別參數，不是把成功／錯誤值混成同型別。
- Q2 機制：具體參數在簽名／標註確定內容型別；None 無內容線索，:Option<i32> 提供線索。需要：可把原工作拆成回傳結果的函式，讓呼叫方知道成功與失敗各收到什麼。
- Q3：divide(i32,i32) 回 Result<i32,DivideError>，零回 Err，非零回整數商 Ok；main 分別處理缺報價／120／true／6／零錯誤。前節 if Some 可供推導，單獨 None 需標註，兩路依據清楚。
- Q4：示例支持具體成功、失敗型別與兩種 Option 內容；不驗邊界除法，也未教直接比較整個結果。判定：足夠。

正文 SHA-256：9f8700d73207cecfbc30743839f2a403dd9b3046add04c3704743ba07e71c42f。

## 003-006 3.3 小節末：Pair<T, U> 練習

- Q1：Pair<T,U> 与 pair<T,U> 延用 003-002～004，first:T、second:U 是兩種角色的對應。
- Q2 機制／需要：參數 first:T、second:U 與回傳 Pair<T,U> 保留對應；可保存 i32 金額與 bool 可用性，不必强迫兩欄同型。
- Q3：可用 Pair { first, second } 回傳，接收為 Pair<i32,bool> 並 assert_eq! first=120、assert! second，分別列印已知型別欄位；沒有要求未知 T 的列印能力。
- Q4：練習檢查欄位保存，不支持執行期數字充當型別參數；未開答案。小節整體：定義的型別名稱在使用位置確定，可保留形狀與欄位／結果關係；完整 Option／Result 簽名已能讀。無必要缺口。

正文 SHA-256：6b935f0dc7c12e53858b1ca231d0f79a85d9a5ca2558786692f39b497dab5ae8。

## 004-001 3.4 導入：可改變文字的管理者

- Q1：從固定 println! 文字，轉為可接合與追加的一份文字資料；owner 與借用身分仍沿用第 2 章。
- Q2 機制：目前只提出管理整份文字與只讀的區別，具體文字型別待教。需要：顯示之後還要追加，須避免只讀函式接走整份 ownership。
- Q3：當前能描述接兩段、顯示、追加的工作順序，尚未要求新語法。
- Q4：工作情境支持區分擁有／借用的需要，不支持任何特定文字型別。判定：足夠，具體建立／追加合理待下文。

正文 SHA-256：ae1d3fdea12cf49ba147b716f10ff6077ebc92211928345339b80eeb7442009c。

## 004-002 String、str／&str 與 UTF-8 保存角色

- Q1：String 是可直接使用的標準 owned 文字型別；str 是 UTF-8 內容型別，&str 是 shared 文字借用。&str 的開始位置／byte 長度類似 2.6 切片，但指文字內容。
- Q2 機制：String 管理長度可變文字，mutable owner 可追加；&str 不接 ownership／回收。文字保存為 UTF-8 byte 序列，一 byte=8 bits；Unicode 跨語言表示、UTF-8 編成序列。需要：只讀文字不必交管理責任，追加則需可變的 owned 文字。
- Q3：雙引號 "Rust" 直接提供 &str，程式保存內容，不另建 String；能區分要持有修改還是只讀。建立／追加方法及每字幾 bytes 尚未要求，合理待教。
- Q4：文字支持型別身分不同；沒有證明 byte 長度等於看見的字數，也不支持 shared &str 可改內容。判定：足夠。

正文 SHA-256：024e9fcbb78f61673a24a41bd79b202ed99b3a2c261c8f7d15614bba04a61c33。

## 004-003 String 接口、heap 管理與同份內容的圖

接口與管理首次需要：
- Q1：String::new 是建立空 owned 文字的 associated function；push_str 是追加 &str 的 mutable method，as_str 是讀原內容的 shared method。receiver／associated 區分沿用 2.5。
- Q2 機制：push_str 追加內容、回 unit、不接手片段；as_str 回借用整段 &str、不另建文字。非空 String 管理 heap 空間，不足時請求空間，owner 結束歸還（2.2）；需要：建立可變問候語，顯示只需只讀內容並保留後續追加能力。
- Q3：new → 追加 "Hello, "／"Rust" → show → 追加 ending="!" → show，得到兩行 Hello, Rust／Hello, Rust!；show 借用用完後可改，沿用 2.4。末段 moved.as_str() 仍顯示後者；此示例沒再讀 greeting，當下不靠它證明原名不可用。
- Q4：正文明示 String／&str 的 {} 顯示內容；三行輸出支持相應內容，不是完整空間布局或所有配置行為的驗證。

原位圖：
- Q1：已看指定 343×404 手機圖副本；藍框 greeting:String 是管理／清理者，黃框 heap:Hello,Rust 是唯一文字內容，綠框 show 的 text:&str 是讀者。
- Q2 機制／需要：上藍向下黃箭頭「管理」，下綠向上黃箭頭「借用」；兩箭頭同到一份內容，區分管理責任與讀取入口，免誤認 show 另持一份文字。
- Q3：圖對應第一次 show，只讀借用尚在用時不可衝突修改；show 結束後追加可行，由文字與 2.4 推得，不由圖的箭頭判斷時序。
- Q4：圖支持同份內容與兩種角色，不支持完整 String 欄位布局。當前必要關係足夠，未看整頁版面。

圖 SVG SHA-256：1984156e753c7124dbe2776290db8736244056395c4397c63e8b30b04cc79b27。
圖 PNG SHA-256：5e43f6037dee5a653f08a46337b3e207075783678aa4c2ee85cb911ab844ef67。

正文 SHA-256：17aaa2a55fcf99040a1d0b68caf33c6ed134f55598b06b8cfdd7f26182a4c2c4。

## 004-004 String 的 move 與存留 view 的修改衝突

- Q1：此處明示「String 沒有 Copy 能力」，補清前單位末段：greeting 交 ownership 到 moved，原名不可再用；原 004-003 當下證據限制保留。
- Q2 機制：move 轉管理責任，不另複製整份 heap 內容／留雙 owner；&str 可 Copy 的是位置長度，不是內容。需要：只读選借用，轉管理才交完整 String，避免誤以為普通賦值獲獨立文字。
- Q3：錯例 second 可用、first 後讀 E0382；view 若最後讀在 push_str 後，shared 借用與 mutable 存取重疊，E0502。把 view 最後讀提前即可，沿用 2.4。
- Q4：追加可能換空間，正文區分「可能」與「每次必定」；兩錯例支持 move／借用限制，不能靠本次剛好不換空間放寬契約。判定：足夠，前單位未證明事項已在本單位明教。

正文 SHA-256：970f2ecdcc13f42f7a879c27c2fe6a9f9002ebc613ff6f65db2f17900495d4a9。

## 004-005 3.4 小節末：append_label 練習

- Q1：output:&mut String 是可改的借用，label:&str 是只讀片段；兩者 owner 都留在呼叫方／原資料，沿用 2.3、004-003。
- Q2 機制／需要：output.push_str(label) 對借到的原文字追加、回 unit；mutable 存取由 &mut String 提供。需把結果保存在呼叫者文字中，不能在函式內另建同名 output。
- Q3：main 的 mutable 空 String 兩次借用傳入 "Rust"／" 教學"，呼叫結束後顯示 Rust 教學；函式無回傳新容器的需要。
- Q4：結果文字支持兩片段追加，未驗不同長度／配置；未開答案。小節整體：String 管理可變內容，&str 借讀，&mut String 借改；借用用完才允許衝突工作。無必要缺口，byte 分界合理待下一節。

正文 SHA-256：65323eb67809a08749982a61d9dcfdb86936633fd86bab12f4df0100a3686160。

## 005-001 3.5 導入：文字切取的分界

- Q1：要從 A中🦀 取「中」，不是只借整段；延續 004-002 的 UTF-8 bytes 與 2.6 範圍切片。
- Q2 機制：一項文字「不一定只占一個 byte」，不能切其內部。需要：照看見的第幾字任意用數字，可能不對應有效文字範圍。
- Q3：當下知道先找實際 byte 分界再切，計算本材料邊界／安全接口尚待教，未要求先猜。
- Q4：導入支持 byte 與可見項目不能直接等同，未說各項實際宽度。判定：足夠。

正文 SHA-256：e1d67b68cbbb49b5a7089eb4a89506081a00f4b1a758a319b4cdb10a344ff343。

## 005-002 scalar／char、byte 長度與 UTF-8 分界圖

單位首次需要：
- Q1：Unicode scalar 是表示文字的基本數值，Rust char 保存一個 scalar、單引號 literal；雙引號仍是一段 &str。可見字形可能由多個 scalar 組成。
- Q2 機制：一 scalar 編成 1～4 bytes；str.len() 得 usize 的 byte 數，不是 scalar／字形數。需要：要界定儲存範圍並完整切取，必須知道數的單位，而不是把看見的字數當位置。
- Q3：本材料 A／中／🦀 寬度 1／3／4，總 len=8，合法分界 0／1／4／8；取中用 1..4，不含終點。byte 值由表提供，不需自行推 UTF-8 編碼算法。
- Q4：「不是所有中文或所有符號的通用長度規則」限制了此表的證據；結合重音例說明完整 scalar 不保證完整可見字形。判定：足夠。

原位圖：
- Q1：已看指定 343×584 手機副本；藍 A 卡 0..1／值65／位置0，綠中卡 1..4／值228 184 173／位置1 2 3，黃 crab 卡 4..8／值240 159 166 128／位置4 5 6 7。crab 對應正文 🦀。
- Q2 機制／需要：各卡列下一分界1／4／結尾8，標題總列0、1、4、8；下綠框完整 1..4，下紅框 1..2 的終點2落在 scalar 內，對照顯示只在長度內仍不足。
- Q3：可按圖定位中全三 bytes 和蟹全四 bytes；range 終點不在結果中，8 可以作尾端分界而不是 byte 位置8。
- Q4：圖各標記與正文表一致，支持本字串合法／非法切點，不提供任意文字字形分段算法。必要關係足夠。

圖 SVG SHA-256：bf40798d5647ee4d3082b0efbfb769540c90751792d1796593e022c5129806fb。
圖 PNG SHA-256：79a148373eaad5ea1eaf12e1dac22036a6a818718250600b530b2bcffa4cf460。

正文 SHA-256：5376e25f97d81d19591ff1ac533755696618bf90d4de06ac746383db8cbea578。

## 005-003 str.get 的 Option<&str> 結果

- Q1：get 接 byte range，回 Option<&str>；Some 的 part 是借原文字的 &str，None 無可讀 part。完整型別可由 3.3 泛型讀出。
- Q2 機制：需 start≤end、兩端≤len 且都在 scalar 分界；有效回借用，失敗回 None、不因範圍失敗 panic／新建 String。需要：把不合法範圍留作可處理結果，避免直接假定任意 byte 切點可用。
- Q3：String 先 as_str() 再 get；本例 0..1／1..4／4..8 得 A／中／🦀，1..2／4..9 得 None，8..8 得 Some 空文字；2..2 雖空仍不合法由正文得知。char 的 {} 顯示能力由首行完整呼叫及輸出直接示範。
- Q4：顯示六組範圍支持完整／切斷／超界／空成功的差別，None 不告訴失敗原因；borrow 只在 show_range 內顯示，未要求外返借用。判定：足夠。

正文 SHA-256：665835fd3119ac1eb95d213070b7fc6736ad943432a6fb9156a5aff1710e7139。

## 005-004 直接 range 索引的失敗方式

- Q1：&text[start..end] 是直接借文字區段的形式，沿用 2.6 range 外形，但有效條件用前單位 UTF-8 規則；不是 get。
- Q2 機制：合法借一段，不合法 panic；單一整數索引不能取第幾 char／文字 byte。需要：未確認的分界應選可處理 None 的 get，而不能忽略失敗。
- Q3：先 text.as_str()，再取 1..2 切中內部，panic「not a char boundary」，沒有正常 partial 顯示；這段沒有 Some／None 可 match。
- Q4：錯例支持此材料的 panic，未證明所有位置失敗；不同失敗行為正文直接對照 get。判定：足夠。

正文 SHA-256：82e37cb98ee0158125b37ee383e4a93bb8a8283e7699c06ee536c8fec3679121。

## 005-005 3.5 小節末：區段 byte 長度結果練習

- Q1：回 Option<usize> 保存取得的長度／未取得，與前單位 Option<&str> 借文字不同；usize 是 owned Copy 數字。
- Q2 機制／需要：match text.get(start..end)，Some(part) 回 Some(part.len())，None 回 None。保留形態可區別合法空段的 0 與範圍失敗，回數字也不需把借用交到外層。
- Q3：1..4 為 Some(3)、1..2 為 None、8..8 為 Some(0)；可取出數字再檢查，或依 arm 印要求結果。先在本檔寫函式即可；library 分工沿用 1.9，不必先開參考。
- Q4：三組材料驗證完整／非法／空成功，不能代表全部分界；未開參考函式或呼叫答案。小節整體：byte 位置须同時滿足範圍與 scalar 分界，get 用 Option 保留成功與失敗，完整 scalar 不等於完整字形。無必要缺口。

正文 SHA-256：5eb2f2f9b44dece809cb6a3d0984e5d2fcec11df87c7a90f5e4dfd2924b61f51。

## 006-001 3.6 導入：不定長度的保留結果

- Q1：新工作是按順序保留數列非負項；array 固定長度與 slice 只借段資料已在 1.7／2.6 教過。
- Q2 機制：當前只提出可增加項目的容器，接口待教。需要：保留項數隨輸入而變，不能先假定固定 N；本材料應三項，其他材料可能四項或零項。
- Q3：可先判出本材料应保留 120、0、80；如何動態存入未要求先猜。
- Q4：數列支持過濾需要，未支持特定新容器操作。判定：足夠。

正文 SHA-256：d0a7ef65ebb5958b958b243c8313d70e0f1d987888230687e90654c312473355。

## 006-002 Vec 型別與清單接口

Vec 身分／建立：
- Q1：Vec<T> 是標準同型元素清單，型別不寫長度，擁有 heap 連續空間中的元素；與固定 array／借內容的 slice 不同。
- Q2 機制：長度可變，Vec 結束也結束其持有元素，按元素型別清理。需要：保存不定項數的結果；Vec::new 空建，vec![...] 依逗號值清單建初值，不是 array。
- Q3：:Vec<i32> 或後續 push 可供 T 推導；本例 new 後加入三項，vec![120,80,200] 也得三項。不得在 Vec<i32> 混 bool。
- Q4：示例支持建立与元素數 len():usize，未支持 len 是文字 bytes 或元素型別任意混用。判定：足夠。

增減與位置：
- Q1：push 加末尾、pop 取末尾、remove 按索引取項，是 Vec 的 mutable methods；移除交回 owned T。
- Q2 機制：push 收值、回 unit，i32 Copy／String move；pop 有項 Some(T)、空 None，remove 越界 panic 並非 Option，移除後後項前補且順序保留。需要：按任務新增／收回值，須選對失敗契約。
- Q3：[80,120,200] remove(0) 交80、剩[120,200]，pop 交200、剩[120]；remove(len) 無對應項、錯例 panic「removal index」。
- Q4：正常追蹤與錯例支持指定位置變化／越界，不能把相同索引當永久同項，空 pop 的 None 契約由表明示。判定：足夠。

清單視圖與安全 get：
- Q1：as_slice／as_mut_slice 分別借整列 &[T]／&mut[T]（切片機制沿用 2.6）；slice.get(position) 新回 Option<&T>，與文字 get(byte range) 的參數／結果不同。
- Q2 機制：get 合法借元素，不合法 None，不移除／交 owned T；視圖不接 Vec ownership，可寫視圖只改已有項。需要：沿用 slice 函式且安全處理可能越界的位置。
- Q3：get(1) Some(&120)，*value 讀 Copy i32；get(9) None。剩[120] 的 writable[0]+=10 改到130，最後使用後 owner 可讀；有效直接 [0] Copy 数字但越界仍 panic，沿用 2.6。
- Q4：輸出支持借读／借改与上述索引，不支持 get 自動擁有元素或追加容量。判定：足夠，接口契約逐項明示。

正文 SHA-256：f670973dc13d9ed7f0b4e08b1b3668d1684da2f8dca44603783309247fda27ac。

## 006-003 清單／元素的 ownership 與 reference 存留

- Q1：Vec<String> 管理元素 String，push 接其 ownership、pop 交回 taken；get 得 &String 只是借用。整個 Vec 也「沒有 Copy 能力」，即使元素 i32 可 Copy。
- Q2 機制：message → messages → taken 轉管理，未另複製文字；push 可能搬元素到更大空間，借出的元素 reference 使用期間不可衝突修改，Option 包裹也不解除 borrowing。需要：清楚分開取回 owned 元素與查看借用，免用舊 owner 或失效入口。
- Q3：let Some(taken)=pop else return 成功後印 Rust、len0，沿用 3.1 let else；錯例 first:Option<&i32> 在 push 後再讀，E0502，应先完成 last use 再 push。
- Q4：正常输出支持转交与清空；錯例支持存留借用的限制，不能依賴本次配置空間剛好足夠。判定：足夠；元素 Copy 不推成 Vec Copy 的反例已明示。

正文 SHA-256：5a864d8353b41ac454aa25e9e943a926e99db05a6afdadf73ba2783abf537fc2。

## 006-004 keep_nonnegative 成果與規則測試

- Q1：輸入 &[i32] 借材料，輸出 Vec<i32> 擁新結果；所有必要操作已在第 1～2 章及 006-002～003 教過，library list 路徑沿用 1.9。
- Q2 機制／需要：while 先 position<len 再索引，Copy i32；>=0 才 push 到另一清單，position+=1，尾值 kept 交出結果 ownership。依讀取順序保留值與重複次數，滿足不改原材料／不排序的工作。
- Q3：例子出 [120,0,80]、len3；測試另確認 [80,-5,80,0] → [80,80,0]，空 array 無迴圈、全負無 push，都 len0。for 0..len 的索引已由上界確認有效。
- Q4：長度与逐元素 assertions 會辨順序、重複、零及空結果，不只看項數；原材料 -5 的檢查配合 shared 輸入契約。正文稱 library 共六測試，其中四項文字測試未在本單位展示，未親看／實跑，僅知宣告，不據此聲稱通過。判定：當前理解足夠。

正文 SHA-256：a38c4861b4e7c0c83db9bb722c63d2c48dc756192f086d0eb34a2138cf12bdc0。

## 006-005 3.6 小節末與全章收束：上限練習

- Q1：capped_nonnegative 延續前單位借輸入、交新 Vec 的模型；新增的是非負 ceiling 的工作規則，不是新 Rust 概念。
- Q2 機制／需要：先 value>=0，再按 value>ceiling 選 ceiling／value 並 push，可沿用 1.6 if expression；先篩負數才能保留需求，不能把負數截零後留下。
- Q3：Vec 初值 [120,-5,0,80]，as_slice 借入、ceiling100，結果 [100,0,80]、長度3，原首項120；可逐數與長度 assertion，不需直接比較整個 Vec。
- Q4：材料驗證略負／截大／保留零與較小值，未驗任意上限與重複输入；未開答案。小節關係：Vec 管理動態結果，slice 借讀原資料，篩選與截上限由既有流程決定。全章：enum 保留情況，泛型替換內容型別，owner／reference 區分管理與讀寫；比較／複製／顯示能力明確契約尚合理待下一章。無必要缺口。

正文 SHA-256：7b78fd9021377a9e507f233796ff67f2ccc5bef94eb142682da6b8961f714810。

## 正文初判封存

32 個正文單位全部依 reading-order 順序完成，逐單位保存後才讀下一單位；已核對所有正文 SHA-256。新圖只在 004-003／005-002 原位開指定手機圖副本，兩圖的關係、標記、範圍與資產 hash 已記錄。未誤見後文、未開作者或他人紀錄、未讀參考答案／外連；尚未補選讀／互看。

本轮沒有具體必要缺口，也沒有需另報的選讀改善。004-003 的末段未單獨證明原名稱不可用，當下證據限制已保存；004-004 其後明教 String 無 Copy 與 move 後舊名失效，只在後文追加釐清，未改前判。導入處暫缺具體語法均在實際操作前教齊；最後預告 trait 等能力契約不構成當前必要前提。

未驗範圍：程式與命令實跑、外連 library／example 的檔案和未展示測試、完整手機／桌面頁面、實際真人學習效果。此紀錄僅是受已讀內容約束的 AI 首次理解證據，不能視為上述驗證通過。

