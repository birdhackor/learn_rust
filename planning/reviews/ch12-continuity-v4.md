# ORIGINAL CONT12：第12章銜接與契約審閱

本輪對 `ch12-complete-prerequisite-v4` 的原始判斷：六節的主要教學目的、機制、例子及跨節銜接均有正文支持；仍有四項小型的首次使用契約／時序缺口，以及三項可選呈現改善。這是獨立銜接與來源審閱，不能代替基本 Python 讀者的首讀或完整 unsafe 安全驗證。

## 身分、隔離與路線

審閱者為 `chapter01_foundations`，未撰寫第12章任何正文或來源。先前參與第1、8章、11.4，以及14.4／14.5／14.7／14.8，有 Rust 作者背景；本紀錄不能稱為零作者歷史的未提示首讀。依 `AGENTS.md`、clear-tutorial、review-protocol、calibration 與術語指引，核對實際先備、首次契約、機制與用途、例子支持範圍、跨節轉換和實頁。

唯一第12章輸入：`/workspace/work/tutorial-implementation/frozen/ch12-complete-prerequisite-v4`。依 reading-order 讀44個展開 BODY 單位，再核其完整來源與必要前文。C／FFI 的折疊答案在主文路線完成後另讀；頁尾延伸連結不算必要先備。早期理解與未知保存於 `/workspace/work/ch12-continuity-v4/read-checkpoint.md`，後續判斷以追加保留。

未取得 FIRST12 的答案／判斷，未讀 peer12、作者 source-notes／ledgers、root disposition、live 第12章或未授權13–15。root 只提供範圍、一般判準與「另一角色已封存」的協調狀態，沒有提供第12章具體缺口。本輪沒有修改學生內容、凍結原稿、全域設定或他人 review。

### 完整覆蓋

| 範圍 | BODY | 完整來源核對 |
| --- | --- | --- |
| 12.1 GAT | `001-001`–`001-007` | gat module、共用匯出、2個正常／答案 example、2組故意失敗來源／expected、manifest |
| 12.2 variance | `002-001`–`002-007` | variance module、3個正常／答案 example、3組故意失敗來源／expected |
| 12.3 raw pointer | `003-001`–`003-007` | 4個正常／答案 example、缺 unsafe fixture／expected |
| 12.4 contracts | `004-001`–`004-006` | contracts module含6項測試、4個正常／答案 example、4組故意失敗來源／expected |
| 12.5 C 基礎 | `005-001`–`005-009` | main／counts／checks／solution／header、C compile-fail／expected，7份檔案全讀 |
| 12.6 FFI | `006-001`–`006-008` | manifest／build.rs／C header與body／Rust lib與sys／5個正常／答案 example／4組故意失敗來源／expected，19份檔案全讀 |

manifest 列出的71份來源全讀：ch12-unsafe 38份（27 `.rs`、1 manifest、10 expected）、C 基礎7份、FFI 19份、7個 SVG。七幅圖的 XML 與展開包的同byte副本也核對。44 BODY＋71來源＋7個 assets 副本＋manifest／reading-order，共124項初始 SHA，保存於 `input-fingerprint.json`；最後逐項核對未變，見相鄰 `.sha256.json`。

### 實際前文核對

沒有拿章節標題或 root gate 結論當作已教。以下讀取確認具體契約；未讀整份前文的部分不宣稱全章重審。

| 凍結前文 | 實讀位置與本章關係 |
| --- | --- |
| `ch01-complete-source-v8` | 1.3 `003-002`／`003-004` 的 package、crate、workspace、TOML繼承；`003-005` 的 -p／--locked／Cargo.lock契約。 |
| `ch02-review-fixed-v5` | 2.6 `006-001`–`005`全文：array、slice、range、mutable view；借array用 `&values[..]`／`&mut values[range]`。 |
| `ch03-review-fixed-v6` | 3.6 `006-001`–`005`全文：Vec來源與 as_slice／as_mut_slice；array caller仍明寫slice。 |
| `ch04-publication-target-fixed-v8` | 4.4 `004-001`–`009`、4.5 `005-001`–`006`全文：elision／placeholder／outlives、來源vs receiver、借用struct。另核4.2已教的Debug／PartialEq／Copy規則。 |
| `ch05-review-fixed-v7` | 5.3 `003-002` 的 `&T: Display`；5.5 `005-006` 的 str.to_owned→String及獨立資料，支持guard顯示與FFI保存文字。 |
| `ch06-review-fixed-v10` | 6.2 `002-001`–`008`全文：associated type／Self::Output／S::Output／泛型impl／equality bound；6.3 `003-005` 函式到function pointer；6.6 `006-004` Vec的條件Debug／PartialEq，6.7 `007-006` tuple能力。 |
| `ch07-review-fixed-v13` | 7.4 `004-002` 的u8、固定array初始化、buffer；`004-005` 的as_bytes與byte slice ToOwned角色。 |
| `ch08-review-fixed-v9` | 8.8 `008-001`–`007`全文：Cell／RefCell／guard／執行期借用／try_borrow_mut／drop；8.4 `004-002`–`003` type outlives；8.3 `003-004` Deref借用coercion；8.5 `005-006` impl lifetime placeholder、`005-007` where clause。 |
| `ch09-review-fixed-v6` | 9.2 `002-001`／`002-002`／`002-004`：build script、host／target、stdout Cargo指令、build-dependencies、OUT_DIR。 |
| `ch10-review-fixed-v15` | 10.1 `001-003`–`004` Send／Sync；10.4 `004-002` scope／spawn／join與borrow；10.2 `002-005` literal型別後綴段。 |
| `ch11-prefix-seven-for-review-v11` | 11.6 `006-009`所選段：byte string literal、byte array reference／byte slice的Debug／PartialEq。這個具體契約不直接教任意owned numeric array的能力。 |

為N1／N3限定實際1–11的選定凍結 BODY，搜尋array→slice與標準容器trait契約。必要候選輸出截斷時再單獨開讀；未搜尋peer或作者筆記。第11.8服務層未被本章使用，沒有為本次擴讀其實作。

## 四題核對：概念、理由、操作與例子

第二題分存機制／性質及需要／用途。小變化只是檢查已教關係，不把我新增的題當成作者原例，也不以答對代替方法理由。

### 12.1：GAT、borrowed view與共同helper

1. **角色／前文。** `001-001`先說6.2的Extract接手來源；現在「保留來源，只借讀內容」，Vec交slice、RefCell交guard。`001-003`定義「帶有泛型參數的 associated type」；View宣告／使用分開，method將當次receiver的lifetime帶入結果，不是永久借用。由正文與actual6.2／4.4支持，已足夠。
2. **機制／性質。** `001-003`先教 `where Self: 'a` 再寫trait：receiver本就要求Self符合'a，GAT明寫條件，讓實作能使用借自self的型別。`001-004`選slice／Ref，不把guard責任消失成reference。`001-005`回 `S::View<'_>` 保留同次receiver關係，bound不承諾未知View可索引／Deref。已足夠。
   **需要／用途。** 原需求是 `001-001`：「把不同視圖型別放進同一份trait契約，而且結果仍能連到每次呼叫借用self的有效範圍。」普通Vec的fn本已可借slice，沒有虛構舊法完全無法借用。GAT讓同一操作保留兩種實作不同的結果與責任，直接回應需求，已足夠。
3. **使用。** trait→impl→helper→具體caller；普通slice最後使用後可push，Ref要drop才解除狀態。is_err及練習as_mut_slice的最低契約在讀測試／答案前教。沒有以未實作標題當先備，已足夠。
4. **例子／圖。** `001-004`同一source `[80,120]` 用完view後push40，再借3項。圖箭頭是順序，a／b只標不同借用；兩種caller及兩種失敗分別支持靜態來源有效性、執行期活躍borrow。練習修改後再push沿同一規則。小變化「guard drop前借mutable」應被RefCell拒絕，理由來自活躍guard。只支持本次view模式，不宣稱所有GAT通用。

### 12.2：subtyping、variance、寫回與callback方向

1. **角色／前文。** `002-001`固定需求：「另一種型別能否滿足全部契約」，不改存活或做數值轉型。`002-002`的outlives接4.5；`002-003`先指明正在看的參數，再分covariant／contravariant／invariant。已足夠。
2. **機制／性質。** mutable reference的外層lifetime可縮，但對T不能任意改換。`002-004`target借reference變數、T是`&'static str`；若能寫短借用就破壞saved原契約。`002-005`Cell的shared介面仍可set，同一寫回理由成立，不因本次body只轉交值而改variance。`002-006`按callback輸入／輸出分別說反向／同向。已足夠。
   **需要／用途。** 保持原合法替換需求：使用者仍可依原有效性／操作能力使用。replace及Cell指出哪份原容器會遭錯誤寫回；callback看使用端提供的資料是否滿足函式要求。不是由新名詞反推需求，已足夠。
3. **使用。** generic helper先描述方向；compile-fail不執行失效讀取。不能加反向bound後仍說原契約不變。練習get原shared reference再建新Cell，不改原Cell的T。已足夠。
4. **例子／圖。** Cell圖是假設錯誤替換如何破壞static契約，禁止符號不是UB輸出。literal caller是操作示範，generic body才核對一般bound。小變化「長效輸出給短效使用者」可行、反向不可行，只用已教outlives及caller需求推得。已足夠。

### 12.3：raw表示、檢查、存取與來源資格

1. **角色／前文。** `003-001`明說reference已能修改數值，raw用來理解安全抽象／外部介面責任。`003-002`分型別星號與運算式dereference；mut不是隨時可寫。`003-003`raw borrow operator、null／is_null／align_of先教。基本角色足夠，Copy契約另列N4。
2. **機制／性質。** `003-004`unsafe不關掉borrow檢查／不做runtime驗證，UB不等於必然crash；來源、初始化、alignment、aliasing權限各自確認。`003-005`add以元素位移，同allocation／one-past／isize／overflow限制先教；`003-006`provenance不等於地址數字。已足夠。
   **需要／用途。** 要分開建立／檢查／讀寫，後文才能放責任進API。non-null不能證aligned／initialized／有效，地址不能取得來源資格；每個限定都有當下判斷用途，不要求完整provenance模型。已足夠。
3. **使用。** 來源是scope內已初始化i32，raw期間不交錯其他路徑；array先教as_mut_ptr／add，one-past只形成不讀，raw後才回索引。首次caller／assert有N1／N3，其餘selected API在code前教。
4. **例子／圖。** 80→100只支持本次讀寫；120→140能對到同allocation第二項，one-past沒有第四格。練習第三項30→50沿同一規則，add(3)不是第三項。安全理由仍是條件，不依成功stdout。已足夠。

### 12.4：unsafe fn、安全抽象、分割與unsafe marker impl

1. **角色／前文。** `004-002` fn宣告的caller責任與body內unsafe責任分開，edition2024警告先教；`004-003`定義safe abstraction／soundness，不把Err當UB；`004-005` unsafe trait／impl接10.1，marker無methods仍有契約。已足夠。
2. **機制／性質。** read_i32只交Copy數字，不接配置；split先查mid，再算len−mid／add。reference提供有效來源／alignment／初始化／獨佔，兩段不重疊才能同時借。`004-004`from_raw_parts_mut不配置／驗證，零長仍non-null／aligned；空Vec只add(0)、不dereference。ReadOnlyI32保存真實reference與同源private pointer，全部公有操作唯讀；Send／Sync分別論證移交／共享。已足夠。
   **需要／用途。** `004-001`要「看見安全責任如何放進介面與實作，練習審查」。mid／reference／不重疊處理不同前提；borrow／private不變條件防caller組錯來源。沒有說日常必須手寫unsafe，標準safe API的存在已交代。已足夠。
3. **使用。** # Safety角色與caller表先於fn；E0499只是兩份索引borrow的有限示例，不宣稱必重疊。wrapper介面／不變條件先於source；thread沿actual10.4，兩個join完成再判結果。field取副本缺N4，直接arraycaller缺N1，其餘支持保留。
4. **例子／圖。** 原indices0–3對range0..2／2..4，回原數列100／45證改原位置；練習mid3／len與tests的0／len／空Vec／invalid mid／wrapper各有固定答案。`004-006`明說六項PASS不能證完整soundness；constructor破壞同源時，unsafe block未變也得重審。小變化mid>len應先Err且資料未改，沿已有branch即可推得。

### 12.5：獨立C程式建立FFI所需契約

1. **角色／前文。** `005-001`要讀對方表示／存取／清理，先獨立C，不假設會C。`005-002`宣告／型別／初始化／header／preprocessor／array；`003`定義／prototype／pointer；`004` pointer+len／for；`005` NUL vs buffer；`006` malloc／free；`007` printf／macro；`008` compiler／linker依需求引入。sizeof具體語法解釋在首code後，列N2。
2. **機制／性質。** array傳參數轉第一項pointer，不帶len；C*沒有borrow規則。for逐項訪問，signed overflow限制明說；len0可NULL但不讀元素。NUL掃描vs按len處理零值分開；malloc先NULL檢查再初始化，free後只讀saved數字；prototype沒有body、object還須link。已足夠。
   **需要／用途。** 原工作是改一值／加總／分字串範圍／獨立配置與釋放。pointer+len解決接收端不知道有效項數；NUL區分string與byte資料；逐項複製與唯一free解決獨立修改／cleanup；linker合body成程式。各項回應工作，不只列C語法，已足夠。
3. **使用。** C-specific for的初始化／條件／index +=1、if／comparison／void／main狀態在source前說；printf格式／PRId32先解釋。旗標表／mkdir -p／./先於命令；relativeheader與故意缺len在negative前教。C沒有Rustunsafe關卡，編譯不能證合法存取。除N2外足夠。
4. **例子／圖。** amount100、sum240、strlen4vsstorage5、三byte／兩非零、owned240分別核不同關係。圖第五格是NUL，buffer中零不停止。練習改malloc副本80→100，原240／新260，只free一次，可由規則推得。checks未實際模擬malloc失敗，固定case沒有概括所有輸入。

### 12.6：ABI、建置、buffer與兩種C string ownership

1. **角色／前文。** `006-001`固定本地／單向同步／無callback／三種資料關係。`002` ABI／extern／unsafe extern的宣告與呼叫責任分開，private sys供wrapper；`003`cc是buildscript函式庫，不是compiler，archive／symbol／連結資訊有各自角色；`004`C struct／repr(C)；`005`CString／CStr／c_char；`006`Greeting／配對free，先於各code說。已足夠。
2. **機制／性質。** 同target的uint32_t↔u32；repr(C)對齊結果，CountError只在Rust；usize→u32用TryFrom，不截斷，?失敗不進C。CString拒內部NUL，C只同步借、不保存／free。Greeting只接自己工廠五byte配置，from_ptr前有NUL／可讀不變條件，to_str另驗UTF8，Drop只呼叫此C配置的free。已足夠。
   **需要／用途。** safe caller用本地C，同時逐項核型別／呼叫／borrow／cleanup，不承諾C更快。version驗連上body；buffer只交數值；Rust-owned string只借；C-owned配置由專用owner free。不是都在heap就交Box／CString；各安排處理不同責任，已足夠。
3. **使用。** cc1.8.0與selected methods／build-dependencies先於配置／build.rs；host建置與target object分開。checked_len／conversion、CString API／Error最低角色／strcmp、Greeting介面／Drop先於片段；題目先補to_bytes／with_nul，to_owned沿前文。arraycaller／rawfield仍有N1／N4，其餘足夠。
4. **例子／圖。** version1來自C；buffer `[10,0,20]`零1／非零2與len0分支分開；label true／false／內部NUL分開；Greeting借view後保存獨立String，drop後仍能用String。圖追同C配置→CStr借用→String副本→pairedfree，沒有把from_ptr當複製。小變化with_nul多一零、string比較仍沿終止約定，正文已解釋範圍差異。未證明任意library／ABI／callback安全，換library要重核。

## 獨立必要項：最小首次使用補強

四項均非編譯錯誤或已證unsafe錯誤。依使用者「每個example只能用已教東西」列待修；主要機制已足夠的結論保留。

### N1：array reference→slice coercion未在首次caller前教

- **位置與原句。** `003-005`說「先透過已教的 `&mut [i32]` 借用整個array」，接 `let view: &mut [i32] = &mut values;`。知道slice及已有 `&mut values[..]`，不能自動證明直接arrayreference會轉slice。
- **同類後續。** raw_solution、safe_split／unsafe_solution、contracts tests直接交 `&mut values`；`006-004` buffer_count及FFI tests交 `&bytes`／`&[]`／固定arrayliteral到slice介面。合併為同一關係，不拆分計數。
- **已教／缺少。** actual2.6／3.6用range或as_slice；8.3是Vec Deref、8.5是T→dyn，未替arraycase教契約。Reference Unsized Coercions另列 `[T; n]` 到 `[T]`，確認是特定介面能力。
- **影響／分級。** 程式合法，但讀者須猜何時arrayref變slice，或錯套Deref／dyn機制。必要的先備／首次使用負擔，程度低，不否定raw安全條件。
- **最小修正。** 首code前教最低arrayref→slice契約，或相關caller都改已教 `&mut values[..]`／`&bytes[..]`；空輸入明確typed空array range或空Vec.as_slice。無需新trait旁支。
- **背景／後文／處置。** 先前14.5作者工作曾收到arraycoercion修正，保留這個背景；第12章首次位置／來源／先備由本輪核對，沒有12具體提示。凍結正文沒有補arraycase。未改、未做修後驗證。

### N2：sizeof這份語法／單位計算在第一個Cblock後說明

- **位置。** `005-002`第一段arraycode用 `size_t len = sizeof values / sizeof values[0];`；「sizeof是C大小查詢運算子，結果型別size_t……商才是項數」在block後。
- **已教／缺少。** 前面 `sizeof(char)=1`有Cbyte單位，不是名字全未出現；object形式、整array／一項結果、商的角色尚未先教。
- **影響／分級。** 後段立即釐清，非整節缺sizeof；嚴格首次code規則仍要求先看陌生運算再回補。必要而小型的時序負擔。
- **修正／來源／處置。** 把現有sizeof段搬到block前即可，不需擴寫。逐序來源自行發現；後文已釐清而時序問題保留。未改、未做修後驗證。

### N3：whole numeric array的Debug／PartialEq需在首次assert前交代

- **位置。** `003-005`先 `assert_eq!(values, [80,140,40])`；`004-004`再Debug整array；答案／tests重複同類。
- **原始未知→查證。** 工作紀錄先標unknown：4.2教trait／assert需求，尚不能證standardarray的impl。actual1–11定向核出Vec／tuple各自能力與11.6 bytearrayreference契約，沒有whole `[i32; N]`。最後讀std array條件列表／Debug impl才裁定；不倒改為首讀已知。
- **影響／分級。** 程式合法；不能由i32有trait推每種容器都有。沒有最低標準array能力，讀者只能由能編譯反推。按嚴格example規則列必要的小型介面介紹，不要求教impl內部。
- **最小修正。** 第一份assert前一句說本例數值array提供Debug／PartialEq，等長且各位置相等才相等，Debug列元素；可補一般元素條件，不需手寫trait／derive。
- **來源／處置。** 獨立初判後的正式API查證，未讀peer。未改、未加test、未做修後驗證。

### N4：raw pointer值的Copy與referent／ownership尚未明確銜接

- **必要位置。** `004-005` `ReadOnlyI32::read(&self)`用 `read_i32(self.pointer)` 按值交field；`006-006` as_c_str亦將self.pointer交from_ptr，Drop交foreign free。合法的field副本依賴rawpointer Copy，不是移出非Copyownedfield。
- **已教／缺少。** 前文Copy／sharedreferenceCopy與pointer不接配置已教；`003-002`–`006`未明教兩種rawpointer值Copy。尤其 `*mut T` 和mutable reference不同，不能由星號mut推得。官方兩份Copyimpl確認不要求 `T: Copy`。
- **影響／分級。** 關鍵ownership操作需猜move／借用／Copy，可能混同複製pointer與複製原資料／增加free責任。必要的小型Copy契約，實作本身沒有這個錯誤。
- **最小修正。** raw基本性質或wrapper首取field前說：`*const T`／`*mut T` 值可Copy，只複製pointer資訊，不複製T、不接ownership／增加free資格；mutable raw pointer可複製也不放寬衝突寫入／有效性要求。無需新negativecase或allocator細節。
- **來源／處置。** BODY後回查field取值先備時自行提出，再查官方；無peer提示。未改、未做修後驗證。

## 可選呈現項

### O1：12.1入口粗體符號實際未解析

`001-001` closing delimiter緊接「不需要」，desktop／mobile entry畫面均顯示兩端字面 `**`。核心語意與後文仍充分，列可選標記修正。closing後空白或另起段能讓入口問題正確突出，不涉及新概念。

### O2：FFI圖type／value名稱可對齊cleanup

`ffi-owner.svg`寫 `drop(Greeting)`，程式真正操作 `drop(greeting)`。初框Greeting可合理讀成owner型別標籤，不判無法追同配置。可選初框標 `greeting: Greeting`、清理框寫 `drop(greeting)`，更直接對回一個ownedvalue，減少回找程式的名稱轉換。無需每框不同名稱；收益是value／type邊界一致。

### O3：桌面GAT末框被浮動「回到頂部」局部遮擋

desktop原位的「讀到3項」右側與chip重疊。有限UI補查：同凍結頁、1280×800、圖頂y約180，停留2200ms後仍有chip（x575.5／y728／114×40）；見 `desktop-gat-borrows-idle-context.png`／`top-chip-probe.json`。

mobile無同樣遮擋，正文／code已能得知3項，其他圖未觀察到同類遮擋。因此可選閱讀路徑改善，非GAT機制缺漏。可讓浮動chip避讓必要圖字或相稱調整版面；不能因可再捲就宣稱原位無遮擋。本輪不改全域theme，未測其他scroll位置／裝置／發布站。

## 實際圖文、表格、折疊與過渡

隔離預覽 `/workspace/work/ch12-continuity-v4/preview` 只含授權六頁：有序44units合併，僅移除合成的重複H1、把同byte SVG的凍結URL換相鄰URL，不重排正文／code／圖／折疊。`render-inputs.json`保存轉換。沿repo theme與Markdownextensions建置，nav只六頁；沒有讀live12。最終build實際exit0、No issues found。初次workconfig TOML nav語法失敗修在work副本，原稿未變。

Chromium viewport 1280×800、390×844。84張畫面全親看：七SVG兩種尺寸context＋element28張、全部14tables兩種尺寸28張、六頁entry＋transition24張、兩份展開answer兩種尺寸4張。另加O3停留補查，共85張。尺寸／bounds／SHA在 `visuals/manifest.json`及probe JSON；不是只統計檔案存在。

| 圖／BODY | 實際對象與連接核對 | 判斷 |
| --- | --- | --- |
| gat-borrows／`001-004` | 同source；第一borrow2項，用完push40後再借3項；a／b非固定長短，箭頭是順序。 | 關係充分，desktop末框遮擋另記O3。 |
| cell-variance／`002-005` | 兩shared視圖是同Cell；假想短效寫回破壞static契約，禁止符號清楚。 | 充分，不是執行UB。 |
| raw-array／`003-005` | 同array三格，start／add1在第一／二格起點；add3只端點；120→140，沒有第四個可讀元素。 | 桌面／手機標記直接可讀。 |
| safe-split／`004-004` | 原0–3indices對left0..2／right2..4；原資料成borrow，修改80→100、40→45。 | 支持不重疊／同源，不只數值相同。 |
| c-string-buffer／`005-005` | 同Rust字串四字元＋第5格NUL；strlen4／sizeof5。另三byte buffer中零不停止、非零2。 | NUL／NULL／字元0不混同，手機數值／箭頭清楚。 |
| c-compile-link／`005-008` | header分到兩source，各cc-c成object，兩object＋stdlib經linker合demo。 | header不代body，source／object／執行檔充分對應。 |
| ffi-owner／`006-006` | 同C配置借CStr／&str→複製獨立String→配對free→只剩String。 | ownership／borrow／複製／cleanup充分，拼寫另記O2。 |

較寬圖手機縮成約16–18px文字，親看必要標記仍可辨認；較窄的split／C／FFI圖亦可直接讀。整頁scrollWidth為desktop1265、mobile375（viewport含15px scrollbar），未觀察整頁橫向溢出；長code使用區塊內水平捲動。答案全部source已另讀，不因單張畫面就稱全長source同時顯示。

14張契約表在手機換行，但欄位／genericangletype／對應可讀；Rusttype參數未被HTML吞。頁首依序承接borrowed關聯型別→合法替換→raw責任→安全介面→C來源→跨語言wrapper。頁尾收束已會的工作，再交代下一問題，沒有要求選讀才完成必要理解。12.4對14.6只是後續查驗工具角色，不把本節契約推遲；12.6清楚限定本地單向同步，未偷用callback／unwind。

C／FFI答案在題後折疊，展開顯示真實source／命令／結果。malloc複製、CStrbyte範圍、獨立owned保存已在主文教，答案不補必要API。12.3／12.4的外鏈參考不作補洞；本輪另讀凍結答案，保留N1／N3同類問題。

## 技術補查與證據界線

只為N1／N3／N4讀selected正式資料，不把下載全文當已讀：

- `https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions`：Unsized Coercions簡介，array→slice與T→dyn條目。
- `https://doc.rust-lang.org/std/primitive.array.html`：一般trait能力列表、Debug條件impl。
- `https://doc.rust-lang.org/std/primitive.pointer.html`：`*const T`／`*mut T` 的Copyimpl、無 `T: Copy` 要求。保存std頁標1.99.0，符合課程工具鏈；未聲稱全頁API／nightly功能已讀。

網頁與所選文字存 `/workspace/work/ch12-continuity-v4/official`。這是後續技術查證，不替學生當時正文補教。沒有Rust／C runtime probe，沒有重跑workspace／同byte整套；沒有執行故意UB、無效pointer或doublefree。判定可由來源與介面確認，不以測試成功放行缺說明。

本輪獨立靜態核對支持：所展示來源／存取安排、split不重疊、wrapper同源唯讀、CString只借、Greeting配對free、checked length及ABI／佈局責任。不是Miri驗證、任意safe client形式證明、所有compiler／target／外部C library可用性；也不是基本Python目標讀者實驗。實頁為exact frozen isolate preview，未驗GitHub Pages發布。

封存先核124項輸入未變與七SVG副本一致，再保存本文件同byte `.initial.md`及 `.sha256.json`。SHA含完整輸入映射、review、工作原始紀錄、預覽與實看截圖。cross前未讀同行結論；後續交叉或修稿應另存追加，不修改這份ORIGINAL。
