# 第14章原凍結稿：獨立銜接審閱初判

本輪只審閱 `ch14-complete-prerequisite-v8`，不修改教材。結論是 **兩項很小但必要的先教後用銜接，以及一項必要附件缺口待處理**；主要方法的理由、例子範圍和九張圖的對應，其餘已查部分足夠。這不是全套Rust測試通過宣告，也不是全部soundness的證明。

## 獨立性、版本與實際範圍

- 審閱者為第12章作者，沒有撰寫第14章；曾為第12章實讀第6／8／10章的先備，也做過第6／7／8章技術與第10章銜接審閱。這些背景如實保留，本輪**不稱blind FIRST**。
- 原稿唯一來源：`/workspace/work/tutorial-implementation/frozen/ch14-complete-prerequisite-v8`。初判前沒有讀peer14、FIRST14、author14 notes／ledger、root14 findings、live14或新版本。沒有擴讀第13／15章。
- 應用repo的clear-tutorial SKILL、review-protocol與rust-terminology。按實際正文重建先備、角色、轉換、方法理由與例子的支持，不把大綱或名稱出現當成已教。
- manifest實際列 **8頁、64 BODY units、134 selected sources**，不是依root摘要估數。64 unit SHA及134 source SHA均自行核對一致；manifest SHA256為 `b36fa91e3a812dfec05fc3ee7341297637d8e8be0da405e187f6f4a5007be683`。八頁raw SHA保留manifest原記錄，沒有以live原稿重新計算。
- 134份包含68 Rust、16 manifest、6 Cargo.lock、20 expected、9 SVG、5 JSON、10文字材料。Rust含正常範例、library／helpers、tests、derive consumers、練習答案及negative；不能只看主文的正常片段。
- 逐段完整實讀001-001–001-006、002-001–002-007、003-001–003-010、004-001–004-011、005-001–005-007、006-001–006-010、007-001–007-007、008-001–008-006，共64 units。之後比對archive完整Rust（主文片段相同者亦核header、組合與額外tests）、全部manifest／expected；六份lock解析package與依賴關係。未用後文回改首次需要位置。
- 九份原位置 `assets/` SVG和archive source逐byte相同；另保存pair proof。benchmark三份文字報告、profiling三組JSON／stdout／stderr及兩份reproduction都實讀／解析，核數字、frame、source行號及可用source hash。
- work evidence：`/workspace/work/tutorial-implementation/continuity-ch14-v8/`，含fingerprints、asset proof、selected probes、preview build、render script／records／screenshots。初判以後封存為獨立副本及SHA sidecar，不從peer結果反填。

## 實際前文與first-use路徑

以下是必要能力的實際位置，不是把整章標題算先備。先前已讀背景與本輪按需回讀分開；本輪特別完整實讀required **`ch11-timing-prerequisite-v4`全9 units**，不是只看Instant一行。

| 能力／此處需要判斷的關係 | 已教的actual位置 | 第14章的使用與本輪核對 |
| --- | --- | --- |
| expression／statement／block、函式回傳、有限整數、array／range、use／crate與tests | ch01-complete-source-v8：005-002／005-004、006-002、007、009、010-002；ch02-review-fixed-v5的reference／mutable walk；ch03-review-fixed-v6的Result／Vec | macro產生的是語法，不能把runtime參數與metavariable混在一起；`expr` fragment仍有新的匹配關係，見N1。assert基本bool形式已教，但message形式另見N2。 |
| trait bound、derived Debug／PartialEq／Clone／Copy、HashMap鍵契約與查找修改 | ch04-publication-target-fixed-v8：001、002；本輪回讀003-002–003-005的Eq／Hash、insert、get／Borrow與get_mut | Reading普通泛型先建立，再生成struct／impl；property的i32鍵有Eq／Hash，已有次數加一，缺鍵才insert，沿用已教match借用邊界。 |
| From／Into與prelude、panic格式、AsRef、ToOwned、同型別錯誤的? | ch05-review-fixed-v7：001-003、004-002、005-005／005-006、006；本輪再回讀AsRef與prelude必要關係 | compiler TokenStream和quote結果以既有轉換連接；formatter／panic語法不是新依賴；dhat file_name的Path view可接5.5及7.5。 |
| closure的capture／Fn能力、Iterator逐元素與Vec比較能力 | ch06-review-fixed-v10：001、003、006，既有實讀背景 | Criterion兩層callback的角色當下分開；Loom model是可重複Fn而非消耗唯一材料的FnOnce。borrowed lines保留，per-line owned複本按當次scope收尾。 |
| 文件測試、外部integration caller、原生Path、完整搜尋規則 | ch07-review-fixed-v13：002、005-004；本輪回讀009-002／009-003／009-004 | contains連續片段、ASCII case、空query先拒絕、原行號含空行、owned Report來源獨立，都有已教依據。14.4保留搜尋並新增合法上限，14.7明確縮成已分好行的純計數，不假稱整CLI。 |
| Box／Drop／method查找、dyn Error source、newtype／local impl | ch08-review-fixed-v9：001／002／003、006-003、009-003／009-004；本輪回讀必要Error與newtype原文 | Meters／Seconds／Millimeters不是首次未知型別形式；derive consumers的本地型別可實作外部trait。Miri的pointer不保活Box資料，profile局部Profiler和static配置器的清理角色不同。 |
| workspace設定、profile／release overflow checks、feature／cfg、host與target、build階段 | ch09-review-fixed-v6：本輪回讀001-002–001-004、002-004；002-002的既有actual host背景 | macro crate在編譯時執行，不是consumer runtime；bench profile與profiling是不同概念；non-default／all-feature caller測試沿既有操作。 |
| Send／Sync／Arc、join與atomic／Relaxed／fetch_add、數字suffix | ch10-review-fixed-v15：002-005、004、008各actual；已有完整CONT10及局部交叉背景 | Loom另用自己型別，不拿std操作假裝模型可見；兩次load→store都讀0可合法得到1，屬於邏輯丟失而非data-race UB。suffix在實際canonical10已補，14.5本節亦先說明。 |
| wall elapsed與Duration、timeout觀察不能當保證、量測跨度 | **ch11-timing-prerequisite-v4：001-001–001-009全9 units** | std Instant wall elapsed、Duration非負／from_millis／from_secs已教；paused Tokio clock不是標準Instant。14.7從一次elapsed換重複取樣，設定時間目標不是精確保證，符合原先對時間的限制。 |
| raw pointer資格、safe abstraction／soundness、split的輸入來源與借用契約 | ch12-complete-prerequisite-v4：本輪具體回讀003-002／003-003、004-003／004-004／004-006；其餘12作者背景保留 | raw pointer不保持allocation有效；from_raw_parts_mut由有效原slice、mid檢查、來源與不重疊建立前提，Miri有限路徑不能替代這些理由。14.6的array→mutable slice與safe split在actual12.4已使用；不用未教Pin或第13章。 |

新工具／語法的當地先教：14.1先定義ident／metavariable／token與repeat符號；14.2先教macro_export／$crate／mixed-site hygiene、compile_error與tt；14.3先教compiler TokenStream、AST欄位、span／reject、parse_macro_input、quote插入與.into，以及proc-macro manifest／固定依賴的角色；14.4先教Default／non_exhaustive／builder／pub use／[[bin]]／package；14.5先教Strategy、collection::vec的兩個範圍、proptest syntax／Config／Just／seed／persistence與shrinking；14.6先教Loom API與模型邊界、Miri的解譯／安裝／cfg；14.7先教bench target／harness、Criterion各API、black_box、runner設定、樣本與CI／baseline；14.8先教allocator、global_allocator、static、Profiler與HeapStats、debug環境變數、viewer欄位。除下面兩處形式的次序缺口，這些local-first說明能接到其後的source／tests／答案。

## 必要待處理項目

### N1：14.2的private-helper負例先使用expr fragment，下一unit才教匹配意義

- 原位置：`14-02-macro-hygiene.md`的「$crate固定定義來源，仍保留可見性邊界」，凍結 `002-004.md:63`：`($value:expr)`，同unit正常展開／negative都用這份規則；archive `src/hygiene.rs`及`macro_private_helper.rs`一致。
- 當下已教：1.5普通expression；14.1的 `$name:ident` 是指定匹配語法種類；14.2本unit的$crate固定定義crate、不能取消private。尚未教的是**expr這個fragment specifier匹配expression，並非ident或runtime型別**。
- 後文位置：`002-005.md:5`才明說「輸入模式中的expr指匹配一個Rust expression，而非前節的型別名字ident」。它能釐清，但不能回算成第一次看到private_reading規則時已教。
- 當時理解：可以從既有private規則判E0603；但讀規則的新匹配種類要先猜「expr是expression縮寫」，才能知道`&distance`爲何屬於可接的輸入。不是缺完整macro parser，而是已選擇的輸入形式缺最低關係。
- 分級：**必要／時序／增加理解負擔**，不否定$crate與private的主要理由。最小修正是在private_reading之前先加一句expr匹配已教expression及本例&distance；下unit保留hygiene所需的兩輸入說明。
- 來源核對：Rust Reference `macros-by-example.html`的Metavariables明確區別expr、ident與tt；canon1–12和14.1至此沒有較早的macro expr匹配說明。本輪自行發現，未受peer／root舊項提示；沒有改稿或修後驗證。

### N2：14.4的Default負例首次用assert自訂message，尚缺一句操作橋接

- 原位置：`14-04-api-compatibility.md`的「Default也要產生有效值」，凍結 `004-003.md:80`及`fixtures/runtime-fail/derived_default.rs`：`assert!(limit.0 >= 1, "預設必須符合非零規則")`。相應expected直接依這段message識別失敗。
- 當下已教：1.10 `assert!(條件)`在false時panic；5.1 `panic!`格式文字；本unit已教derive Default會得到usize 0、違反非零invariant。這些足以理解錯誤default，但**assert第二個參數是失敗時的自訂panic message，而不是比較的另一個預期值**尚未說明。
- 自行查漏：對actual1–10、11的existing canonical packages／prefix、12之BODY查assert optional文字形式，沒有找到先前已教message的段落。14.5之後講property assertion附訊息，不能前移作此處先備。
- 分級：**必要／first-use操作／增加很小理解負擔**，不是主要Default概念阻礙。最小修正是在負例前明說`assert!(條件, "訊息")`只在條件false時panic並帶這份訊息；沿用既有格式文字即可，不需要新增大段macro課。
- 來源核對：std1.99 `macro.assert.html`的Custom Messages說明此第二形式及失敗時求值。正文「assert指出default違反規則」說明檢查目的，但沒有交代這個新argument的角色。自行初判，未提示；未改稿／未重跑此熟悉的panic案例。

### N3：14.4兩份隔離相容性快照的locked命令缺自身lock附件

- 原位置：`004-009.md:98–99`要求兩個 `cargo run --manifest-path ... --locked`，接着承諾兩次都顯示`未修改的caller：ignore_case=false`並通過同assert。
- 實際附件：v8 manifest的134 sources包含`compatibility/v1`與`v1_1`各自Cargo.toml／src/lib.rs以及共用consumer.rs；**沒有兩份相容性Cargo.lock**。六份selected lock全部屬於其他negative workspace。兩份manifest末尾都是獨立`[workspace]`，不能借用外層root lock。
- 已教部分：1.3已用locked固定既有lock；14.2已教隔離workspace／manifest-path；14.4已解釋[[bin]]的相對path。因此學習者知道如何選兩版，但收到此凍結附件仍無法完成承諾的操作；正文也沒有先教生成這兩份lock的步驟。
- 相稱實驗：只把這五份相容性材料原byte複製到自己的work；從repo cwd／Rust1.99環境分別執行原locked run。兩次均exit101，報`cannot create the lock file ... because --locked was passed to prevent this`，發生在編譯與caller assertion以前。日誌保存；第一次在/workspace誤無toolchain的呼叫未作教材證據，正式probe均從有固定toolchain的repo cwd重做。
- 最小對照：只在自己的副本各做`cargo generate-lockfile --offline`，再原locked run，兩次exit0且都輸出相同false，source沒有改。這支持「缺自身lock」，沒有變成比較程式邏輯的另一項錯誤。
- 分級：**必要／附件與可操作性／阻斷本小實驗**。先核對兩份lock是否確實交付讀者並納入canonical frozen/archive；若選不提供，則在locked命令之前先教一次準備步驟。不要求改SemVer內容或重跑所有Rust。
- 範圍限制：沒有去live14／公開repo猜它另有lock；本項精確指v8提供的附件不足，不能宣稱任何未讀位置也必然缺檔。Cargo Book的`cargo-run` Manifest Options明確missing lock會令--locked退出。獨立初判，未受peer／root提示；教材未修改。

本輪沒有另列可選建議。N1／N2是局部很小的必要橋接，N3先處理交付材料；不把它們累積成整章完全未教。

## 主要通過項：四題與當下支持界線

以下保存自己的回答；Q2分開記機制／性質與需要／用途，不能拿編譯PASS代答方法理由。Q1角色與Q3所需／交付合併在焦點；Q4仍對照原承諾，沒有擅自提高成一般框架或全平臺證明。

| 位置／Q1與Q3焦點 | Q2機制／性質：自己的回答與來源 | Q2需要／用途：原需與最短連接 | Q4例子支持及判定 |
| --- | --- | --- | --- |
| 001-002–003：Reading與單規則macro | `($name:ident)`把同名插入struct與impl，之後才runtime借讀；生成Rust仍受type／ownership檢查。 | 001-002明確重複的是「型別與實作宣告」，普通函式已處理借讀；保持80／30與兩個非Copy wrapper，重用宣告template纔回應該需。 | 手寫／macro同main、同結果，graph區分階段；wrong unit仍E0308。不能推出函式／泛型原來不能重用資料邏輯。兩個Q2與Q4已足夠。 |
| 001-004–006：repeat／空清單 | `+`至少一份、`*`零或多份、`?`可選尾comma；匹配與展開對應同層重複。 | 增加已教Millimeters500而不再手寫一組；空清單練習只改接納域，雙側+都需換*。 | bad name、empty+與wrong unit分屬匹配和型別錯誤；solution零個／兩個呼叫並存，支持當前有限模式，不推一般nested匹配。已足夠。 |
| 002-003–004：scope／macro_export／$crate／private | textual先定義；export在crate root可用；$crate固定定義來源但保留visibility。 | exported模板在caller使用，bare Reading會依caller缺名／另名而錯；固定library契約可減少caller導入責任。私有helper仍不能被caller展開直接用。 | caller另有無關Reading也回傳80／30，E0405與E0603角色分開；普通public函式在library內可呼叫private，macro不能偷渡此權限。主體已足夠，expr先用另報N1。 |
| 002-005：hygiene／once／order | caller輸入的value保留來源，template let value另有上下文；左let→tuple左右次序，模板每輸入只插一次才支持once。 | 同名時仍要借到Meters80與Seconds30，並避免輸入取得reference的副作用重複；不是hygiene天然讓所有宏只求值一次。 | 普通函式對照(80,30)，calls2與order[1,2]各查不同關係；圖側邊箭頭回caller而非template。已足夠，未要求這份模板覆蓋所有名稱空間。 |
| 002-006–007：fallback／compile_error | rule按順序首個匹配，後續型別錯誤不回落；tt捕獲剩餘輸入，compile_error給當下診斷。 | 不合名字的呼叫要具體說明錯誤，而非看到難懂通用matcher失敗；duplicate Meters仍是展開後E0428。 | 自訂錯誤、重複宣告負例與reverse-result練習均限定當前規則，未聲稱解決所有hygiene問題。已足夠。 |
| 003-001–004：derive分工／TokenStream／AST | 已有struct declaration交入，syn拆出ident／params／data／fields，derive只加items；host macro與consumer分crate。 | 原需是「使用已有宣告資訊」增加Reading，保留caller的名字、visibility、其他derive；既有macro_rules仍可工作，未捏造其能力障礙。 | 3packages與manifest符合實際分工；形狀只有非泛型單tuple欄位，未假裝處理任意Rust AST。已足夠。 |
| 003-005–007：reject／quote／entry | parse失敗變診斷，span指位置，quote以#name放Ident語法，proc_macro2結果into compiler流，早拒不支援形狀。 | 重用原名字建立impl而非把source stringify再猜；保留原聲明，並把錯誤在相應編譯階段指出。 | pipeline圖把兩個TokenStream區別、Rust型別檢查與runtime80分開；entry前介面都已介紹。兩Q2已足夠。 |
| 003-008–010：consumer／有限驗證 | alias仍是i32，private field在同crate的generated impl可讀；bool形狀接受，回傳i32時才E0308。 | 使用者只選derive並保留自己的型別身份；別讓工具重造public struct或按寫出來的欄位字樣判i32。 | 零／負數、private alias、五種negative與Millimeters練習實際發揮分工；絕對ch14_macros路徑與不支持重命名依賴明列。已足夠，不要求一般derive框架。 |
| 004-002–003：public契約／newtype／Default／non_exhaustive | private usize經new限制1..1000；Copy有效值仍有效，Default手選100；external enum必須兜底。 | 所有公開建立路徑都保留invariant；預設建立也不能偷渡0。擴充variant時需先給caller未來入口，事後加標記仍breaking。 | 0／1001拒絕、1／1000接受、預設100、private及external match負例均有明確判斷。Default主要概念／理由足夠，新assert message橋另報N2。 |
| 004-004–008：builder／typed Error／search／pub use | mut self builder接手設定交回；Error::source借原io原因；query先驗，owned Hit保存原行；private module以pub use穩定入口。 | 保留原ASCII／片段規則而增加合法上限；caller依公開型別分類失敗，不需猜Display文字，representation只借資料。 | consumer cap2保留原1／3行，PermissionDenied source不寫死OS全文；no-default無text入口但核心工作不變。tests／練習cap1只用已教API，已足夠。 |
| 004-009–011：兩種compatibility／SemVer／package | 同實體consumer分別編譯1.0與1.1；private新增不強迫literal補欄位，預設false需留；package本機解包編譯與registry發布不同。 | 問的是「原caller不改能否繼續編譯與滿足約定」；編譯能力不能替行爲default作保。列包能看真正交付的src／test／資料與nested邊界。 | 固定caller的assert支持這一項行爲，不證明任意API。1.x示範與repo0.1版本規則分開，publish=false不等網站不能發布。概念關係足夠；執行附件N3另待處理；本輪未實跑package歸檔。 |
| 005-001–003：properties／策略／獨立oracle | HashMap加次數；策略描述生成與縮減，不是馬上生成Vec；總數保存與每鍵direct count不同，64爲case數。 | 找分組遺失資料而非只比兩個可能一起錯的實現；固定原輸入規則，oracle直接巡訪原slice而不重用Map計數演算法。 | counts例、正數／守恆與per-key測試支持當前[-3,3]與長度0..31；輸入外固定±1000另外回歸，不混爲隨機覆蓋。已足夠。 |
| 005-004–007：shrink／replay／regression／composition | insert1覆寫重複，Just0仍可縮Vec長度；seed／固定version讓此fixture可重現；shrink保留失敗而非數學全局唯一最小。 | 找到第二次同鍵覆寫的易讀反例，再存普通regression避免具體問題消失；composition用加法關係補充，不能代替direct oracle。 | 手工4→2→1圖明說非工具trace，fixtureexpected[0,0]屬工具紀錄；always-zero可滿足composition，故主文不替換兩條較強屬性。已足夠，本輪未重跑proptest失敗fixture。 |
| 006-001–004：Loom重建／lost update／fetch_add | 兩個load／store各atomic仍可都讀0；Loom包裝替代std進入model，callback每輪重建Arc／counter；RMW一次fetch_add不同於兩步。 | 原需每worker增加一次最終2，單次host觀察不能排除另一允許交錯；模型探索提供額外反例。 | 手工甲2／乙1非頻率亦非工具完整trace，bad fixture僅模型失敗；solution80+20+5=105保持一次RMW。已足夠，PASS只對模型、參數、工具限制。 |
| 006-005–010：Miri解譯／safe wrapper／UB限制 | 解譯MIR檢查實際執行中的allocation／權限等；safe caller所進inner unsafe亦解釋；Box結束而raw資訊不延長storage。 | 12.4的全前提理由仍需審查，有限測試補查具體0／len／empty／invalid paths；不是Miri通過即可宣告safe abstraction對所有client sound。 | 三測試核safe split原資料與拒絕不變；UB fixture顯式只允許cfg(miri)，清理後dereference是錯誤點，ordinary編譯被compile_error拒絕。記憶體圖／read-before-drop答案足夠，不native跑UB，未把FFI／Loom一起假稱Miri已驗證。 |
| 007-001–004：same work／microbenchmark／callback邊界 | 兩法同Result、同slice／query、每行只算1；clone candidate完整生命週期在iter內，準備／assert在外；bench harness false由Criterion提供main。 | 已分好行的純計數只借讀即可，不需要owned副本；減少工作是否省時間要量，不從clone名稱猜倍率。兩層callback分開管理者與每次routine。 | 64／256各16／64、case與空query測試排除錯誤快版；邊界圖不是並行兩方法，black_box只best effort不關閉最佳化。已足夠。 |
| 007-005–007：warmup／samples／CI／baseline／setup練習 | warm反覆資料可cache；30批各含多次呼叫，iter是完整清單；time中心／95%CI非三次最快中慢；change比同id舊baseline。 | 一次elapsed不穩定，要保存input／source／環境／profile與重複範圍；原兩輪同source的regressed提示不能反推程式碼修改因果。 | 八格time均與兩輪raw報告匹配（三位小數舍入）；練習新問題把建置／收尾移入iter，新id只證新範圍能量，不將差值精確分解或推CLI倍率。已足夠，本輪未重跑bench。 |
| 008-001–003：repeat／allocator／static／profile scope | 256×100逐次真實count，累加6400；每binary唯一global_allocator，static不靠scope Drop，局部Profiler build→get→drop；借讀追蹤前材料不算新allocation。 | benchmark總量不知道配置出處；要問每行clone是否實際配置以及改善是否取消這項工作，得追蹤對應資源／位置而非猜CPU時間。 | 單獨repeat tests6／0限定答案，圖顯示原清單仍由main擁有；零profile不等零heap，trace本身不納使用者統計。兩Q2已足夠。 |
| 008-004–006：profile數字／source／viewer／改善 | JSON同PP聚合25600次326400 bytes；source main18→repeat12→count21→String Clone；total／peak15／curr0不同分母，snapshot在drop前，end是停止追蹤。 | 保持答案6400／材料／100次，定位只爲contains建立的candidate再改as_str；真實owned Report責任未被悄悄刪掉。 | 三JSON／stdout／stderr數字與source hashes、frame行號吻合；改善零root有原非零對照。64行練習1600／6400blocks／81600bytes而peak仍15支持總量與同時存量分開。與原正常bench只連接有限證據，不把dhat時間當CPU熱點／倍率。已足夠；未親自加載外部DHAT viewer。 |

這些通過項的判斷依據是當下的原問題、明示角色／工作與可追溯的操作／結果，不是「有兩個例子所以通過」。小變化只用已教關係核對：空macro改*仍不生成型別；caller另有Reading也不改$crate；tuple欄位alias與private不重造宣告；default改true可編譯卻違反原caller；property總量不足證明每鍵；atomic兩步不等RMW；profile最長行沒變時peak不跟總行數成比例；這些理解可由正文／圖／actual前文達到，沒有把自設情境當作者實測。

## 圖、原位置與呈現負擔

獨立preview只組合v8 BODY、保留原順序／折疊，換image URL以載入同byte SVG；未提前搬圖或補正文。明確載入canonical `ch09-review-fixed-v6/sources/docs/stylesheets/tutorial.css`，其餘用同Zensical theme；`zensical build` strict在本輪exit0、No issues found。實際Chromium原位 **1280×1000桌面、390×844手機**各九圖，18主capture加手機derive尾段，共19張，已逐張親看；render records顯示原image皆載入、無頁面水平overflow，DOM數字只作輔助，不代替親看。

| 圖／原位置 | 親看必須區分的對應與支持 |
| --- | --- |
| macro-expansion／001-003 | Meters是輸入名字→template同名兩位置→struct與impl→編譯後borrow80，四框／三個箭頭階段明確；手機read_value行完整，無須放大。 |
| macro-hygiene／002-005 | caller value保存Seconds30、template value保存Meters借用；綠色側邊箭頭回右輸入，中央arrow是template來源，最後80／30不是兩個80。對應原文同拼字不同context，不拿顏色替代名字。 |
| derive-pipeline／003-006 | compiler TokenStream→syn AST→quote proc_macro2→into compiler→Rust檢查→consumer80，兩個流名稱與階段必要字可讀。手機圖較長，按原位正常上下捲並另看尾段，底部結果未被固定header遮住。 |
| property-shrinking／005-005 | 4零與2零仍總1而失敗，1零總1成功，保留2零；手工標籤及箭頭試更短材料清楚，不誤認完整工具trace或ownership。 |
| loom-paths／006-002 | 每次fresh count0分成兩次可能路徑，A／B讀存次序分別得2／1；不是同一時間兩counter／不是概率，結果分母與兩條path文字足以判別。 |
| miri-storage／006-008 | 上半Box owner／raw pointer同指80，清理後舊資訊仍在但虛線失效，vertical是drop先後；raw pointer不保活資料，圖下UB與原操作相鄰。 |
| benchmark-boundary／007-004 | gray準備／正確性outside，blue／orange各自iter內巡訪完整列表；orange含clone／收尾，題前說明兩方法分別反覆，不能把兩框讀成同時執行。 |
| profile-boundary／008-003 | 256行先有、build開始、100完整count、get snapshot、drop停止寫JSON、之後印數與輸入收尾；原清單仍存在，與零新配置關係可見。 |
| profile-source／008-004 | main18→repeat12→count21→String Clone箭頭是呼叫；教材四層和raw JSON frame一致，最後25600blocks／326400bytes只算同一來源，未重複相加框架。 |

14.4沒有圖時，private field／公開new／default／builder／pub use的角色已有具體源碼、參數表與caller對照，相容性同實體consumer也明確，無額外必須靠想像的空間位置；不據此要求裝飾性architecture圖。其操作附件問題另報N3。正文長code保持正常可橫捲容器；本輪確認必要SVG基本理解不依賴zoom，沒有冒稱所有全站code行已做手機交互驗收。

## 實際實驗、正式來源與未驗證

- **本輪Rust實跑只限相容性疑點**：兩個原byte副本在lock缺席時原run均exit101；各副本offline產生自己lock後再run均exit0，完全共用consumer，stdout同false。`compatibility-locked-probe.log`、`compatibility-v1_1-locked-probe.log`、`compatibility-after-local-lock-probe.json`保存正式範圍，未修改frozen或student資料。
- **未重跑**一般Rust全suite、negative集合、property shrink、Loom、Miri、Criterion benches、dhat profiling、clippy／doc／cargo package；沒有把教材已有exit0／root QA說成自己跑過。本輪沒有原生執行任何故意UB，也沒有強制cfg(miri)繞開guard；Miri工具cache沒有作爲作者答案來源打開。
- **附件核實**：完整讀三benchmark stdout與reproduction，核取樣stderr、profile、input bytes816／3264、兩輪限制及四捨五入表；核三profile JSON的PP／frames／bytes／blocks、stdout／stderr一致。performance六份selected teaching source hash、profiling六份均與record匹配；record中shared Cargo.lock／rust-toolchain SHA未在134 sources中，未獨立重算或聲稱歷史環境復現。
- **正式規則核實**：Rust Reference [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)的fragment、首匹配／textual／export／$crate關係；[Procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html)的entry／different crate／derive appended items；std1.99 [assert](https://doc.rust-lang.org/std/macro.assert.html) Custom Messages與[black_box](https://doc.rust-lang.org/std/hint/fn.black_box.html) best effort；Cargo Book [cargo-run](https://doc.rust-lang.org/cargo/commands/cargo-run.html) missing lock規則。外部正式材料在正文全64讀完後補查，不據外部答案回算當時已教。
- **固定crate source抽核**：本機registry criterion0.8.2 `bencher.rs:48–95`的FnMut／output Drop／range成本、`lib.rs:1212–1218`兩層callback與default confidence0.95；loom0.7.2 `model.rs:231` Fn+Send+Sync+'static及Builder的外部限制；dhat0.3.3 `lib.rs:179–185`舊配置忽略／realloc例外、build活躍限制、`HeapStats`欄位／get前提。這些支持教材當下使用的有限契約，未要求學生讀依賴內部實現。
- 未親自操作外部DHAT viewer，故只確認JSON可解析／frame與數字能支持敘述，不把record中的`viewer_load_verified`當本人的視覺證據。獨立視覺只指上述9 SVG原位19capture，未驗證dark theme、其他平臺或其他viewport。
- 未進行實際Python程度真人讀者測驗，無法把本輪作者背景審閱的可理解性視爲真人首讀結果。三項必要問題沒有修後稿，本輪只給原initial判斷並封存，等待root安排cross。
