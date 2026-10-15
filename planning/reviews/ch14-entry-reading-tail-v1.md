# 第14章尾段所需1–5背景補讀

AI輔助閱讀，不是真實Python新手研究。沿自己的actual1–10與已封存FIRST11／FIRST12路線；本入口41 BODY只建立本人的14.1–14.5實際背景，不改其他讀者的prefix原判。尾段23 BODY另作獨立原判，沒有接觸其他第14章審閱答案。

每次CURRENT在工具輸出前closed-filter，不開未讀unit、archive、完整source、作者、同行、ROOT findings、outline、live或外部答案。原位置SVG到當段才看；保存四問、Q2a機制／Q2b原需要與用途、原句、合理推論／自補／未知／回讀、source SHA後才NEXT。不跑Rust，不把獨立SVG當實頁驗證；主文完成前不讀閉合選讀。正體中文敘述，引用/API保留原文字。各輪分開封存，不回填早期分級。

路線／reading-order SHA：

```json
[
  {
    "snapshot": "ch14-complete-prerequisite-v8",
    "reading_order_sha256": "4b2b839b7e2be5fa0cdf177dd88ed66080d092749a38421d696b1410b81d2f71",
    "selected_units": [
      "001-001.md",
      "001-002.md",
      "001-003.md",
      "001-004.md",
      "001-005.md",
      "001-006.md",
      "002-001.md",
      "002-002.md",
      "002-003.md",
      "002-004.md",
      "002-005.md",
      "002-006.md",
      "002-007.md",
      "003-001.md",
      "003-002.md",
      "003-003.md",
      "003-004.md",
      "003-005.md",
      "003-006.md",
      "003-007.md",
      "003-008.md",
      "003-009.md",
      "003-010.md",
      "004-001.md",
      "004-002.md",
      "004-003.md",
      "004-004.md",
      "004-005.md",
      "004-006.md",
      "004-007.md",
      "004-008.md",
      "004-009.md",
      "004-010.md",
      "004-011.md",
      "005-001.md",
      "005-002.md",
      "005-003.md",
      "005-004.md",
      "005-005.md",
      "005-006.md",
      "005-007.md"
    ]
  }
]
```

## ch14-complete-prerequisite-v8 / 001-001（順序1）

SHA256 `9c83a8c1775f2923dbc43af1b43487a5698e361cb61337051c2fd94bf5d05731`。

背景補讀Q1：declarative macro按輸入語法產相同宣告，接7.9型別函數/8.9單位newtype與既用println/vec，現在自己定義。Q2a 此段只給方法角色，macro_rules語法/匹配展開尚未教，不以title當答案。Q2b 原需要多單位各保型別身分、共同i32借讀介面而少維護重複struct/impl樣板，不是把所有單位變同一T或宏使運行更快。Q3 先普通Rust答案再宏產相同宣告，package/import沿既有路線，未實跑。Q4 已給共同材料/保留性質支持學習目的，不證何種macro規則可實作/任何語法皆可展。原句「保留各自的型別身分」「少維護幾份相同的 struct／impl 樣板」。目前macro定義/生成操作未知待BODY，背景補讀非他人prefix盲讀驗收。

## ch14-complete-prerequisite-v8 / 001-002（順序2）

SHA256 `55a9771d0ef656dcb75529f02fe5f6ed2f91f9c3031a71a2a136c400179f9d1c`。

背景Q1 Reading共trait借i32、read_value泛型沿4.1，Meters/Seconds不同newtype沿8.9；型別在examplelocal可impl外trait。Q2a &self不移wrapper、.0 Copyi32，DebugEq觀原值無Copy整wrapper；兩普通impl已可讀80/30。Q2b 原需要保單位邊界/owner又避免更多樣板，普通函數已重用讀邏輯，剩重複是struct/impl宣告非運算不能做；Reading不保混單位加/互傳。Q3 callerborrow兩值後仍原Meters80/Seconds30，可由assert/Debug核相同owner與型；宏後要保此行為。Q4 兩型同介面展示公共宣告形式、單位仍異，不用數值80/30證trait新增能力或宏提速。原句「重複的是…型別與實作宣告」「改變的是宣告的來源」。當下樣板/借讀與目的足夠，宏匹配未教保留。

## ch14-complete-prerequisite-v8 / 001-003（順序3）

SHA256 `2629afd6b6da2f7d38d7610244c714938eb9f1873c9916c730d1a3ca0ab6210c`。

首用expansion/macro_rules規則/ident metavariable/token：Q1 compile時按規則產Rust語法仍型/ownercheck，非runtime文字造型；token識別字/標點/literal、metavariable存語法非let值。Q2a macro_rules!定名不在名尾!、呼叫name!；($name:ident)=>模板;匹配identifier、兩位置代同名struct/impl，定義在呼叫之前當前example名稱順序，非先eval再普通match值。Q2b 原需要重用輸入名對應宣告/保型別與借讀，函數可處sliceVec數量不等資料亦能做但不是這個宣告需求；宏省樣板代價追輸入/展開更難讀，非必macro計數。Q3 先useReading，兩callMeters/Seconds產同手寫答案，mainreadValue借80/30且wrapper仍在/noCopy/no換算；跨crate名稱下一節未讀。Q4 SVG SHA256 `c67dbdbf122f64ace7860e3ad0eecf059b68d589b72bdcd15b3a0cba19739f2b`source+390已看：三藍inputMeters(不是值)→ident匹配模板nameMeters→struct/impl同名，兩compile箭頭，第三編後執行→綠借Meters80回80原仍在；箭頭時序非耗時。圖與兩型main支持宣告來源變/行爲保，非同型或全部syntax合法；未run/實頁。原句「匹配的是這些語法」「代價…追蹤輸入與展開」。各新概念角色/需足夠，entry非prefix新blindgrade。

## ch14-complete-prerequisite-v8 / 001-004（順序4）

SHA256 `84a5956042e803a733766f748d844037c0a5fed90eee632fafa5a9480964588d`。

背景首用repetition：Q1 $(...)圈重複語法，+至少一/*零或多/?零或一是macro操作符非算術加或Result問號。Q2a matcher $($name:ident),+逗號分至少1名字、$(,)?單尾逗可選；template同層$(struct/impl)+每匹配name展一組，不nested，不runtimeVec存名。Q2b 原需要一次列三單位仍各獨立類型、少重複宣告定義，數量變化在compile選原始碼非函數處理多數據理由。Q3 readingTypesMetersSecondsMillimeters尾comma→三struct/impl，read80/30/500仍借且原assert可用；負/零亦可讀，newtype不暗帶非負驗證沿8.9。Q4 新第三名字/可選comma明見匹配與展開對應，不能推未知嵌套規則/值validate/消耗，未test命令。原句「輸入與樣板保持相同的重複層次」「只產生型別與取值介面」。當段操作/用途/邊界夠，背景非他人驗收。

## ch14-complete-prerequisite-v8 / 001-005（順序5）

SHA256 `070b966f175618c0bf143e9300bb5a089132ed83dbe96ae8d12834e0d2acfb31`。

背景Q1 兩故意失敗分宏匹配與展後普通Rust型別契約，接ident/+與8.9newtype界線。Q2a 80literal非ident→無規則match/noRulesExpected；+空call不接受→unexpectedEnd。Meters/Seconds各合法產struct，但distanceValue要&Meters給&Seconds→E0308，共Reading不取消特定型需求。Q2b 原需要保各單位型身分並知道錯在哪階段，不見macro錯就改輸入或以共同read能力混距離時間。Q3 小fixture分別只保struct模板隔matching，或兩合法名字接錯函數，診斷只正文聲明未己編譯。Q4 有意移1條件能依規則推失敗原因，不支持所有宏輸入合法/代碼已安全或compiler不做展後檢查。原句「前者是呼叫語法能否匹配」「後者…滿足型別契約」。當段原因足夠，未讀完整source/peer。

## ch14-complete-prerequisite-v8 / 001-006（順序6）

SHA256 `901b953c0b8925147f8d74fa658400f10b22db5e1d5a2bd0413a5bfba3c835b4`。

14.1背景末Q1 空清單是已教重複次數規則小變化，不新資料模型/型別。Q2a matcher/template同+改*，零name即零struct/impl；optionalcomma一次使單一comma也合法空，不能碎impl或假型。Q2b 原需要同模板保型身分/借讀介面且容空名字輸入，少宣告樣板目的由普通版對照已接，不拿宏「多筆」能力貶函數。Q3 空call再MetersSeconds→80/30原值保，兩處同層operator維持匹配展開；closed完整答案/外連都未讀/未實跑。Q4 正文重複規則足以推空/逗空/原兩型結果，非任vec宏完整實作或所有macro域；下一節名稱來源未知待實讀。原句「名字重複零次、可選逗號出現一次」「普通函式…多筆同型資料」。entry當下已知宣告來源如何改且原需要維持，無必須自補。

## ch14-complete-prerequisite-v8 / 002-001（順序7）

SHA256 `6eddfdaa2ea39d0b8c4879bf108e101bb078899e8086f846ea77e410e22b7514`。

背景Q1 14.2將library模板交其他crate，新增hygiene模組；既有struct/Reading生成已知，名字域規則待教。Q2a 尚只問題：template名字找library或caller、同名locals是否改第二源；不以hygiene名稱猜所有token隔離。Q2b 原需要跨crate可重用模板且一次借不同型兩源得80/30，不讓caller同名影響錯讀來源；非純多筆同型函數不能處理理由。Q3 library/template和caller分開才需名稱來源追蹤，先不改前例/不讀完整答案。Q4 開場能定位兩類scope需要，未能斷言macro所有名字衛生或caller遮蔽無效。原句「模板裡的名字應找 library 還是 caller」「第二份來源變成第一份」。新規則未知保存，entry非prefix首讀驗收。

## ch14-complete-prerequisite-v8 / 002-002（順序8）

SHA256 `f4fedc40f20c4ee5e4f4c086c39510638d3d2461b0a7a7ef2610f66ea9c8dc6b`。

背景Q1 readPairValues普通genericfn接T/U各Reading允不同型兩借→兩i32，readInternal私有helper/librarypubmodule沿1.9。Q2a same-modulepub函數可callprivate，再交結果外caller；外crate不能直接private，先usecrateReading與pubmodhygiene明來源。Q2b 原借兩資料工作普通函數已做且不移owner，後宏用來觀察生成程式尋名/權限，不因fn能力不足發明macro用途。Q3 borrowdistance/waiting→80/30，各型獨立bound，macro未來不能默認繞過private可見性。Q4 小函數分工明確支持原借工作與內部/外部可見區別，未給跨crate宏尋名規則，不猜callsite能用private；未run。原句「不是因為普通函式無法借讀兩份資料」「外部 crate 不能直接使用」。當下已知/需要足夠。

## ch14-complete-prerequisite-v8 / 002-003（順序9）

SHA256 `5482418773a987cd30549f8757370d9b8e917540cc1f138fe3b0b434ea7334c8`。

背景首用textualscope/macro_export/pathscope：Q1 本地按文字先後，跨crate選exportmacro以defcrate根public路徑找，沿use/crate路徑非全template名字library。Q2a localcall須def後包圍scope，前call不能findmacro未matcher；export即定義hygienemodule也根use/qualifiedcall。裸Reading別名契約仍caller處需要，在example只importmacro無Reading→展後E0405，macro自身已找到。Q2b 原需要library可重用宏且辨macro位置與模板名稱來源兩問題，export不代尋trait；不由missingReading反說macroexport無效。Q3 callerimport根macro或qualifiedcall即可定位它，還須使生成程式name合法，正確固定librarytrait方法下一段待。Q4 三diagnosticstage與外example具體支持差別，不推全部識別字同域/自動library所有name。原句「export 只處理 macro 自身可從哪裡找到」「裸 Reading」。當前scope與失敗cause清楚，需名稱對應補後。

## ch14-complete-prerequisite-v8 / 002-004（順序10）

SHA256 `f71b296d7634c6bdcf31e58aae321370ef01687e77f432b48294a28c4ed6ee71`。

背景首用$crate/可見性：Q1 特殊metavariable是defmacrocrate非callercrate或runtime參數；固定trait與helper名稱來源，不給caller私有權。Q2a impl$crateReading避caller同名unrelatedtrait，生成MetersSeconds可滿libraryreadValuebound；privateReadingexpansion直接$cratehygienereadInternal仍E0603，普通pub函數內部call則有權。Q2b 原需要跨crate模板穩定用其作者介面，caller免importtrait/避免同名換契約，也需保private分工，不把尋到名字等於可用。Q3 exportmacrocaller只usemacro/readValue，仍80/30原value保；可用template改call公有$crateReadValue，moduleitem需完整modulepath。Q4 同名trait不同method+library普通bound形成真正來源對照，非僅數字相同；負例scope權限據完整path/私有已知，不推$crate全權。原句「不是 caller 的 crate 起點」「沒有把 caller 變成 library 的私有實作區塊」。局部新fragment expr先出於privateRead matcher，本文未明解這種類別，從&distance及已給E0603可知此輸入已展開，猜它匹配expression但完整最低規則仍待下段；名稱/權限主線可判，不假稱已教expr或回算他人prefix問題。

## ch14-complete-prerequisite-v8 / 002-005（順序11）

SHA256 `3e5075a0f2b7fc865b567514f748dfbf31601b7772272f0122e14b2379aaf0fa`。

背景明教expr/hygiene/mixed-site/block/eval次數：Q1 expr匹配Rustexpression產sharedref非ident，hygiene以來源ctx解析名，mixedsite此模板localsdefsite、輸入callersite、裸Reading其他名callsite/$crate可定；補002004當時猜測，不回填。Q2a $left先let模板value，$right拼value仍callerwaiting；外brace模板、內產普通block包let+tuple，左helperread後右expr，兩輸入各插一次才eval1次，hygiene不保此性質/生成item名不自unique。Q2b 原需要不把第二源換成第一並可追讀取動作/順序，普通fn仍可借二源，macro在此觀察尋名而非能力超越。Q3 caller存waitingrefvalue，普通/宏80,30，blockinputs各calls+1→2；left-righttrace[1,2]較單total更對目的，重插expr會重eval。Q4 SVG SHA256 `ee1710566e1eba0c5c906261dea8945b0d35bfdfa9a3c288cfa632642b7c4aec`source+390已看：頂綠callervalueSeconds30右邊折線直入紫讀右30，藍模板value左Metersref中下箭頭到紫讀左80，再依來源到綠80/30；兩拼法以來源標記分，箭頭是name來源非move/exec次序，evalorder另正文支持。例只有限兩Reading/所示expressions不保所有宏等價普通fn；同module同Meters重宣仍衝突。原句「hygiene 本身不保證只求值一次」「插入的輸入片段保留 caller 的上下文」。圖字可讀未實頁，當段機制/需要足夠，未run。

## ch14-complete-prerequisite-v8 / 002-006（順序12）

SHA256 `b9428f2b860f8fae12bce7ca9ec6907e073e97a5627a191f2d5435e204cc8928`。

背景首用compile_error/fallback/tt/規則次序：Q1 compileError預先可用macro文字→compile診斷建置失敗非panic/成功值，tt tokenTree是單token或成對括group。Q2a firstidentrule再fallback $other:tt*零或多，firstmatch即用template嵌readingType再展，80fallbackcustommessage；Metersmatch後E0428重宣不能退fallback。Q2b 原需要直接說輸入shape與判哪階段出錯，不見macro位置就猜pattern；查名/展後Reading來源/權限/custom/重型別分。Q3 localfixturepath相對其manifest上兩層非cwd，emptyworkspace隔故意example，--manifest-path選toml/--example選該package target、locked已有；不讀.expected/archive/實跑。Q4 各單因對照支持診斷定位，不能外crate找不到當輸入拒成功；compileErrorexplain不代unsafe/type任保。原句「不會因生成程式失敗就回退」「不能拿找不到外部 crate當成成功拒絕輸入」。每新規則/操作用途明，entry未知expr已在002005補足，無新增必要guess。

## ch14-complete-prerequisite-v8 / 002-007（順序13）

SHA256 `c54b43e8324acef5b22773e0b5c6b9d97351cbc26b5ee30a9233cc9491e434c2`。

14.2背景末Q1 reverse宏復用expr/block/let與普通read函數，反結果tuple不反輸入取得順序。Q2a 先leftlet再rightlet each1，組right/leftread→30,80，但runtime取得source記錄1,2；只把兩帶副作用expr反展會2,1，hygiene不自保eval1/order。Q2b 原需要保兩源/次數/順序時可改交回排列，原wrapperowner仍可借讀，展示名字來源與動作區別非宏比fn能力必要。Q3 局部macro不新closure/trait，closedanswer不讀也可沿規則重構；Vecpush順序trace比count2更有據。Q4 反結果與trace不同對象合理小變化支持用途/機制，不能生成item名字auto唯一/越private；整節textual/path/$crate/mixedsite/eval與diagnostic問題已連。原句「依原順序各取得一次 reference」「把兩份輸入連同副作用一起反過來」。procedural macro/parser只是下節待教，未預讀外鏈。

## ch14-complete-prerequisite-v8 / 003-001（順序14）

SHA256 `a8737d1ec638a5959db46353693ff75d8a11b73400ad41e0dc7e0fdc2fd2ccc5`。

背景Q1 derive新分工：caller已定struct，工具僅補Readingimpl，接前macro產struct+impl與既有derive使用。Q2a 當前只知讀宣告資訊產impl，proc/token/parser具體尚未教，不用derive名假懂AST。Q2b 原需要caller在自己宣告旁選能力、保自決名字/欄可見/其他derive，仍同80/30借讀；手寫/前模板原能做，差在利用現有宣告，不是runtime動態造型。Q3 原owner與readValue契約要保，derive實作/包裝定義待後。Q4 開場把方法分工與同材料需求連起，不支持任意struct皆可derive或名稱沒限制。原句「struct 已由使用者寫好」「已有的宣告資訊」。新mechanism未知留後，entry非prefix新QA。

## ch14-complete-prerequisite-v8 / 003-002（順序15）

SHA256 `e89ff9ede5050542f5669cab126b75c307073ea98e025e58d4fa69e68c399c49`。

背景首用procedural macro/ReadingValue/TokenStream/三形式：Q1 compile執行Rust程式讀語法→回新語法，非runtimeMetersvalue；procTokenStream是tokens串名/標點/括group不是Vec數據，entryinput/return都此型。Q2a derive編譯器交型別宣告、保原加items，所以只returnReadingimpl不得重struct；ReadingValue工具名非Readingtrait。functionlike括內syntax→取代呼叫，attribute參數+標item→取代item，此節只derive不先假懂其他操作。Q2b 原需要利用既有宣告/選能力、可用Rustif/match檢語法後產impl，與模式模板不同分工非提升borrow能力。Q3 caller#[derive工具]，entry接tokens檢/產後compiler仍處理output，沒有input實例或自動值validation；骨架/parser待後。Q4 表格角色足以防derive複宣與工具trait混名，不證任意型自動合格或另外兩形式實作已學。原句「derive 保留原宣告，將產生的 items 加在後面」「不是裝著 Meters 值的 Vec」。當下概念/用途足夠。

## ch14-complete-prerequisite-v8 / 003-003（順序16）

SHA256 `cc622b72d35e652ac1b9eaff7f53eb2ad148b5c159efc0d2e644df9e7b4120b7`。

背景首用proc-macrocrate/package分工/synquote：Q1 specialcrate為host編譯工具，consumer另crate不可同crate套derive；rootpubfn entry CompilerTokenStream→same型，runtime僅產impl不重parser。Q2a [lib]proc-macrotrue設定target非dependency，proc_macrocompiler提供不registry；syn tokens→可Rustfield/enum讀語法樹，quote插入語法→tokens。syn精確版只derive/parsing/proc-macro，quotedefault接compiler轉換，非full全部AST。Q2b 原需要工具與原trait/runtimeconsumer分清角色及來源，用既有宣告資訊形成impl/consumer可保自struct，避免錯放為正常dependency或runtimeparser。Q3 三packages：traitlib/derivehost/consumer依賴二者，pathrelativeownmanifest與workspace/精確版本已有；不實際讀manifest之外source或安裝套件。Q4 配置表和角色說明支持host/target與來源，不證parse已完成/所有Rustitems可處理或同crate自用；確切API下一段待。原句「不能在定義它的同一個 crate 套用」「最後執行…並不再次執行 parser」。當段必要材料已足夠。

## ch14-complete-prerequisite-v8 / 003-004（順序17）

SHA256 `4d1674a6e2eaa5d18eb18328ec1cc6d58fda28629137fb748deb5a438823f896`。

背景首用AST/DeriveInput/Punctuated與檢查分工：Q1 AST全名抽象語法樹將decl名字/generics/data/fields結構化非自己切source字符串，model型synDeriveInput。Q2a ident是Ident供quote/診斷，paramsEmpty、DataStruct、FieldsUnnamed、unnamed.len1才支持無generic單tuple；Punctuated存節點+分隔，len數節點非逗號，不當Vec，後matchborrow取shared不移/cloneAST。Q2b 原需要讀已宣資訊但只生成固定self0->i32的有限impl，macro核name/shape而field是否可i32交Rust，typealias可行不靠文字拼i32斷型；避免語法工具冒充typesolver。Q3 shape錯提診斷，合格name插新impl，caller保型別/fieldvisibility；具體parse/Error/quote流程待後，不操作AST。Q4 欄位表+1欄契約支持範圍/分工，不保所有tuple/genstruct/型文字同就行，未syn外文查資料。原句「第0欄是否真能直接交出 i32，交給 Rust 型別檢查」「len 計節點」。各新存取最低意義夠當前，entry相同路線保未知時點。

## ch14-complete-prerequisite-v8 / 003-005（順序18）

SHA256 `e07773dc092b73d5727ca0931c0ec260d5ee5e1876e95797c59b0e5abba6ea90`。

背景首用parseMacroInput/as規則/Span/SynError/兩TokenStream：Q1 parsermacro將compilerTS→DeriveInput，span是source位置非runtime範圍，SynError帶位置訊息轉compileErrortokens；procMacro2TS ecosystem與compilerTS名字同型不同。Q2a input須變數、asDeriveInput是macrogrammar非numeric cast，successmodel/failureearlyreturn compilerTS故所在fn須returnTS非i32；形狀合法另檢查。Errornew ident.span/message→intoCompileError消Error得procMacro2TS→From/Into轉換compilerTS，名字import來源明給，不直接需procMacro2path依賴。Q2b 原需要語法解析錯/自訂有限形狀拒皆定位宣告並交compiler，不把Result/字符串panic當derive產物；source名字span使caller知道哪decl。Q3 rejectborrowmodel/str(Display已知)，entry匹配錯時returnhelper，quoteoutput下段同轉換；不改AST/clone。Q4 minimum接口說明支持兩錯機制和確切來源，不證任何可parse皆shape合或所有同名TS等型，無外部syn資料/執行。原句「不是數字轉型」「提早從所在函式回傳 compile_error! 的 tokens」。新轉換與目的已清楚。

## ch14-complete-prerequisite-v8 / 003-006（順序19）

SHA256 `e399e7636645271e5feb5f6cb03b188f1067a134b9522b810d18055290d3745d`。

恢復時核自己的 state：saved18/current003-006；回讀同一未保存 CURRENT 以恢復原句，未取下一段，不宣稱另一次盲讀。

Q1：quote! 是編譯期產生 Rust 語法 tokens 的模板；#name 取普通 Rust 變數的內容，ToTokens 是交出語法 tokens 的 trait，Ident 已具備此能力。不是 macro_rules 的 $name，也不是執行期字串格式化。Q2a：借 &model.ident，quote 插入名字並生成 impl；結果為 proc_macro2::TokenStream，以 .into() 配合 entry 回傳型別轉成 compiler TokenStream。輸出名字在 caller 解析，因此指定 ::ch14_macros::Reading；:: 由外部 crate 名稱出發。Q2b：原需要是已有使用者的 struct 宣告，借其名字增加正確 library trait 實作，避免再抄 struct／impl；也要避免 caller 同名 Reading 改變來源。這份有限工具採固定 dependency 名稱契約，沒有自動尋找改名後 crate。

Q3：使用者保留 ch14-macros 依賴名字、宣告符合外形；工具借 Ident 即可插入，不需轉 String 或自己實作 ToTokens。得到 impl 語法而非 Meters(80) 值，原 struct 保留，後續 ordinary read_value 使用增加的能力。Q4：原句「Ident 已實作它，所以我們不用手寫這份 trait，也不用先把名字轉成 String。」及「procedural macro 的輸出會在使用者那邊解析名字，不能直接套用 macro_rules 的 `$crate`。」支持機制與契約邊界；不是所有 renamed dependency 都受支援。

原位 SVG SHA ad2dbe412df2915c13f2b4473f2c709705b98c7d9a3237833f2034e560a8fb9d 已讀並以 Inkscape 390px 孤立渲染／view：四藍框依次 compiler TokenStream＋struct Meters、syn AST（ident Meters／參數0／tuple 欄位1）、quote proc_macro2 TokenStream＋#name、compiler TokenStream＋原宣告後增加 impl；第3→4線旁 .into()，第4→綠框線上「Rust 檢查通過後」。綠框才 value=Meters(80)／read_value→80。各向下線為解析／交付，未把語法階段當 runtime 值的移動。字在此孤立圖可辨，未看實頁。未知：改名自動尋找明確不提供，非必要洞；本段無新增必要問題。

## ch14-complete-prerequisite-v8 / 003-007（順序20）

SHA256 `2b35cfb875de8b7bd74bf0c8dce7addf0d302b8c6f07005656a22743e9505a8b`。

Q1：#[proc_macro_derive(ReadingValue)] 將 pub entry 登記成 consumer 可用的 derive 名稱；derive_reading 是實作函式名字，consumer 不直接呼叫它。前幾段的 parse／shape 檢查／quote／TokenStream 轉換現在接成單一入口。Q2a：先 parse 成 model，依序查泛型參數、Data::Struct、Fields::Unnamed 及 len==1；任一不合便 return reject 的診斷 tokens，合格才生成 impl。match 借 AST 而非移走，所以末尾仍借 model.ident。Q2b：原需要是隻支援預定宣告外形，把不能生成正確固定模板的輸入在使用處拒絕，再替既有型別增添 Reading 能力；不是藉 runtime Err 或 panic 來篩值。

Q3：需 proc-macro crate 中的 pub TokenStream→TokenStream entry 與前文匯入／helper。符合單欄無泛型 tuple struct 會得到 Reading impl；不符則停止本次生成、交編譯診斷。固定 self.0→i32 的型別是否吻合仍由 Rust 檢查。Q4：原句「consumer 會用 ReadingValue，不是呼叫 derive_reading。」和「每個 return reject 都交出編譯診斷、結束這次生成」限定兩種名字與錯誤階段；完整骨架的借用 match 與最後 #name 對應前文各元件，未增加新操作猜測。本段無圖；沒實跑，沒有由骨架宣稱所有輸入皆正確。未知／必要問題：無新增。

## ch14-complete-prerequisite-v8 / 003-008（順序21）

SHA256 `5e519dac2f714fe3db8e089123e1e7526db41925cc29d4d8b2366c3683e45a4d`。

Q1：consumer 以 use 匯入 ReadingValue，在自己寫的 Meters／Seconds 上與 Debug、PartialEq 一起 derive；ordinary example 仍呼叫 library read_value。這是前段工具和 runtime 使用串接。Q2a：compiler 先跑 derive，generated impl 位於型別所在 module，可存取其私有欄位；self.0 Copy 的是 i32，不使 wrapper 本身 Copy。型別 alias 保留 i32 身分，所以不靠欄位文字 ==i32 判斷。Q2b：需要在使用者自行保有宣告、可見性和其他 derive 選擇時，省掉重寫 Reading impl，保持借讀與原 wrapper／原 i32 值（含零負數）。

Q3：consumer 依賴與匯入 derive，符合外形，example 借 &distance/&waiting 得80/30且原值仍可 assert；測試零／負數和 PrivateReading(RawReading) 私有欄位補核有限邊界。commands 的 -p／--example／--locked 沿既有背景，不執行。Q4：原句「只看到輸出不能說 parser 在 runtime 被呼叫了。」和「借讀從 i32 欄位 Copy 出數字，不表示整個 wrapper 自動有 Copy」支持階段與所有權界線；兩項測試是作者呈現的實際展開後行為，不是我跑過，也不是直接普通 test 呼叫 compiler entry。合理推論：私有欄位不需暴露給 example，impl 同 module 已有權限；正文明確支持。無圖、無新增未知／必要問題。

## ch14-complete-prerequisite-v8 / 003-009（順序22）

SHA256 `67c1038210d190f2e287a19df834a0297176280549c18f9d1cc55c91c09c4cb2`。

Q1：五份負面 fixture 用來區分 parse、工具自訂形狀拒絕、生成後 Rust 型別檢查，接前面 entry 的分支。Q2a：Holder<T>、enum、named fields、two fields 先解析成功再 reject；Flag(bool) 是一欄 tuple、無泛型，會生成 self.0→i32，Rust 因 bool 與 trait 返回 i32 不合報 E0308。Q2b：需要確定工具只承諾有限外形，且失敗確實來自欲核對的階段，避免以缺依賴或 parser 工具自身不能編譯代替「拒絕 bool」。

Q3：fixture 用隔離 workspace，兩個 path 以自身 manifest 為基準；--manifest-path/--example 指到需要的 target，預期失敗。辨認 custom compile_error 訊息與 expected i32, found bool／E0308，而不是僅見命令退出失敗。Q4：原句「parser 已能讀這份宣告，增加泛型支援也不會讓 bool 自動變成 i32。」及「缺 dependency 或 parser 工具自己無法編譯，不能當成成功驗證『拒絕 bool』。」（後一處單引號是本人標示；正文原片語爲「不能當成成功驗證『拒絕 bool』」的意義，逐字採實際原句「缺 dependency 或 parser 工具自己無法編譯，不能當成成功驗證「拒絕 bool」。」）支持階段邊界。未讀 .expected／來源，未執行；表與 bool 程式能解釋有限工具的行爲，不能由此推出所有合法 Rust 宣告都接受。無圖／新增未知。

## ch14-complete-prerequisite-v8 / 003-010（順序23）

SHA256 `3584d074d68863401e273030588bfbb69921142913c30f38b2b3bc022189d2d6`。

Q1：小節末第三單位練習檢驗現有 derive 的重用，不增加 parser 規則；相同 read_value 可以使用由手寫、macro_rules 或 derive 產生的 Reading impl。Q2a：Millimeters 的名字由 model.ident 經 #name 插入；RawReading 是 i32 alias，shape 檢查無需辨識字面，生成後交 Rust 型別檢查；負數仍是允許的 i32。私有欄位由同 module generated impl 存取。Q2b：原需要是在使用者已有宣告的前提減少 impl 重抄，仍保持不同單位的型別身分、原值與借讀契約，沒有非負或公開欄位需求。

Q3：自宣告 Millimeters(RawReading)，derive Debug/PartialEq/ReadingValue，借現有 Meters80 與新值-5，得到(80,-5)且原值仍可使用；不用改 entry、欄位 pub 或 parser。Q4：原句「這三個答案分別來自名字插入、Rust 型別檢查與只借讀 i32 的需要。」把需要與機制對應；「改變的是 impl 的來源：手寫、macro_rules 模板，或讀取已有宣告的 derive。」支持小節總結而非 runtime 處理換工具。閉合完整參考程式維持 CLOSED、外連未開、未實作／實跑。14.3已可分 compiler輸入→AST姓名／形狀→impl tokens→Rust檢查→runtime值；固定crate名、無泛型單欄外形是明示界線，無必要新洞。

## ch14-complete-prerequisite-v8 / 004-001（順序24）

SHA256 `a6bb3b6227a8f6236b8b013f4e2d037e34f1b15eb41e425cdcd0f359d448c45e`。

Q1：API 展開為 application programming interface，本節是把7.9已有搜尋library交給其他程式，公開名稱／型別以及輸入、結果、失敗、ownership約定構成介面；pub只表可見。Q2a：本段尚未選實作，僅定義公開契約的材料與package ch14-api／Rust名稱 ch14_api。Q2b：原需要是caller可選大小寫與最多幾筆，取原行文字；內部改檔或新增設定後，舊合法caller仍能使用且搜尋意義不變。這個維護需求先於後續方案，不從新增wrapper反推。

Q3：既有7.9背景是設定／純搜尋／報告／I/O分工與非空連續片段搜尋；接下來要加有效上限與可維護公有介面。Q4：原句「加上pub只能讓名稱可見，不會自動說清『成功』代表什麼。」此處『』為引用呈現，逐字核心原片語是「加上pub只能讓名稱可見」與「新需要是有效的結果上限與可維護的公有介面」。支持可見性／語意契約分離，未聲稱任何既有配置皆相容。無圖；實際詳細interface、validation等待後文，非自行猜設計。

## ch14-complete-prerequisite-v8 / 004-002（順序25）

SHA256 `20c074b87350924f64721085ac42a5bdcd2dce0a810f3a1cb274c35c8236e1ce`。

Q1：本段契約表是之後型別與測試的標的，不是已由 pub／private／Default 保證。Q2a：此時機制尚待實作；明定借 query全文Options，回傳擁有文字Vec<Hit>；限制1..1000，預設case-sensitive/100，首次符合行保存且最多limit，line1空行佔號，ASCII折疊，Error分類。Q2b：caller原本需要有效上限、固定預設、保留原行／順序、不被接手輸入，能判斷EmptyQuery與Read原因，不解析display文字；空query先於I/O拒絕，零hits正常成功。不是由新封裝物反推出這些需要。

Q3：合法limit與非空query按選項搜尋，取得ownedHit文本；IO失敗借讀保留io::Error，ASCII條件不適用Ä/ä。後續type與tests須支援這些可依賴行為。Q4：原句「這些約定來自目前的工作需求；後面的型別與測試要支持它們。」及「不能把其中一項當成『整份API一定相容』的證明。」後一引號僅呈現，原核心「private欄位、Default與feature都有各自的作用」支持界線。沒有圖；我已從 actual7.9知道ASCII搜尋、ownedhit/IOboundary，新的可用limit與compatibility仍待mechanism。無必要不清，保留後續驗證。

## ch14-complete-prerequisite-v8 / 004-003（順序26）

SHA256 `74967d01068b52a77311d79609a2703b8a6bf7a07c6a3a99064cdd4d47b91602`。

新 invariant：Q1 是 BatchLimit 每個有效值均1..1000的不變條件，沿8.9 newtype但加範圍規則；Q2a private usize欄位阻外部直接建構／修改，new驗0與>1000，Copy/Clone有效值仍有效；Q2b caller需可靠結果上限，不能零或過大，所有入口都要守，不是隻有某次new。Q3 new回Result並保存Zero／TooLarge原值，get(&self) Copy usize；後續search可依limit保證限制。Q4 原句「建構、複製與預設入口都要保留這項條件，不能只驗證某一個new呼叫。」支持全入口需要；private不保證crate作者自己永不犯錯。

新 #[non_exhaustive] enum：Q1 是整體標記未來可加variant的attribute；Q2a 定義crate內仍能窮舉，外部match必須兜底，已知variant仍可外部建構；Q2b error類別將來可能增加，但現有caller的分類要預留處理未知分支。Q3 caller可辨Zero／TooLarge，同時留其餘分支；不是讓enum或variant私有。Q4 原句「這項enum標記不會把它們變成私有。」限定效果，尚待external程式實際對照。

新 Default／derive(Default)：Q1 std::default::Default是標準prelude的預設值trait，Sized，fn default()->Self無receiver/input也無Result。Q2a derive按每欄Default組成，不呼new；usize0、boolfalse。BatchLimit需手寫Self(100)，保合法常數；Q2b caller需固定有效預設，省自己選數字的負擔，且預設數值本身影響搜尋筆數，並非只能編譯就相容。Q3 caller BatchLimit::default 得100；直接derive錯誤fixture可編譯但assert runtime失敗，不能用來證明語法不合法。Q4 原句「它不會呼叫我們的驗證new。」與「將來把100改成10，即使所有caller仍可編譯，得到的結果數量也可能改變。」支持驗證與語意相容分別。文件///與expect固定2已教，無新洞；未實跑fixture、無圖。

## ch14-complete-prerequisite-v8 / 004-004（順序27）

SHA256 `1c4068b2a960f74995e9b38cb662acd15b26050e166879d72b8e751b99b372e3`。

Q1：Options將case與有效BatchLimit封裝，公開選設定／getter的方法而保留欄位安排；新 mut self 是 owned self 的可修改binding，非&mut self。Q2a：deriveDefault用boolfalse與BatchLimit手寫100；with_*接手整份Options，函式內改某欄後回Self，結果可再交下一method；with_limit輸入BatchLimit而非任意usize保不變條件。getter shared借讀並Copy bool或BatchLimit。Q2b：caller需要自預設配置大小寫／上限，且不必依賴內部欄位、struct literal或mutable reference；library需日後可加狀態但仍守公開method／預設契約。

Q3：Options::default().with_ignore_case(...).with_limit(validLimit)是逐次move→return串接；caller若保留舊變量不能再當未move使用，此沿ownedSelf背景。得到新設定供search借，getter值不承諾佈局／欄位名字或順序。Q4 原句「mut讓函式內這個self binding可修改；它沒有把參數變成 `&mut self`，caller仍交出原值。」及「不是藏住欄位就能隨意改回傳型別、成功條件或公開Clone／Copy能力。」直接支持mechanism與維護界線。無圖；新mutself首用與Default組成已有解釋，無需猜是否手動Borrow／clone。無新必要洞。

## ch14-complete-prerequisite-v8 / 004-005（順序28）

SHA256 `0db0913f84263222b60f39efc6bdec048fb26d679cc0865eac9ef57eb2c5edfb`。

Q1：SearchError分類與source沿8.6，Hit是公開被動結果，search／read_matches沿7.9而加驗證與limit。Q2a：EmptyQuery無source，Read擁有io::Error並借出dyn Error來源（外層借self而非static reference）；Display只提供本層context。search先validate，再lines/enumerate從原index+1編號，case選ASCII折疊比對，push原line owned String，len達BatchLimit即break；read_matches也先validate後I/O並借contents給search。Q2b：caller需依typed error做動作與沿source報告，不能依診斷文字解析；成功需owned原文／原行號／先符合的有限結果，零筆正常；不同入口不應改變空query規則。

Q3：借query全文Options或Path，不接手caller輸入，回Result<Vec<Hit>,SearchError>；即使read_matches局部contents結束，owned結果仍可用。external error match需兜底，Hit的公有line_number/text允許structliteral，日後增加欄位會影響caller，與BatchLimit驗證封裝選擇不同。Q4 原句「Display文字供人閱讀，typed variant與原原因才供程式分類。」及「結果擁有文字，輸入結束後仍有效。」支持角色分離；程式validate在fsread之前且break在push之後對應契約表，非僅憑pub推斷。Hit無驗證invariant的被動選擇不等於所有型別應公開。無圖／實跑／新增必要洞；既有borrow/error/sliceText機制無需重述。

## ch14-complete-prerequisite-v8 / 004-006（順序29）

SHA256 `1f7601022ececc2d4d36c86b6c05f826cc09d32c5659cf52626f2b7ae86fa1a1`。

Q1：re-export（重導出）是pub use把既有公有item增設另一可用公有路徑，與私有module內部安排分離；Options仍同型別，非複製／新wrapper。Q2a：mod options私有，pub use options::Options暴露root ch14_api::Options但不讓options路徑公開；root同樣重導出其餘型別／fn。cfg(feature="text")同時控制module與render_text存在，未啟用仍有搜尋。Q2b：caller需要穩定名稱入口，library需可搬內部檔案；可選文字報告服務有些caller需用，核心search不應依賴它。

Q3：default啟用text時，render_text借&[Hit]返owned String，每筆行號:原文字與LF沿7.9；無text則不能import此fn。保持root路徑和操作契約可容內部搬動，但已公有舊module路徑的caller也有依賴，不能靠新增rootexport免除刪舊路徑的breaking影響。Q4 原句「Options仍是同一個型別，沒有複製定義或建立alias wrapper」與「新增重導出不會讓移除舊路徑自動成為相容修改。」支持identity與兼容界線。feature/cfg已有actual9.1背景，本段只新增text選擇，manifest與renderer細節待後文；無圖／新增必要洞。

## ch14-complete-prerequisite-v8 / 004-007（順序30）

SHA256 `cf29c6932ac65250dd4ba95fd1cdeb3e81f623913681c4a2a0c753227ae6cd74`。

feature首用本節：Q1 是text可選報告能力，同時feature名稱／default皆public承諾；Q2a cfg保module/export與callerblock，default=[text]一般啟用，text=[]不連帶別功能；Cargo在需同一化依賴建置合併請求。renderer借hits逐行format與push_str回String。Q2b caller可能需原行文字報告，但search規則／ownership／有效預設應不受開關改變，兩依賴者選功能後原合法工作仍成立。Q3 no-default保前四行search結果，無報告；features text重新取得；all-features此例同啟唯一text。取消feature名/default能力可能破壞caller，合併功能不能用互斥搜尋規則。Q4 原句「它沒有修改搜尋規則、Options預設或結果ownership。」與「兩個依賴者同時選功能時，原合法工作應仍能成立。」支持獨立能力，不由compile狀態獨斷相容。

外部錯誤分類：Q1 publicerrors用nonexhaustive兜底／typedcause接前文；Q2a new0/1001回相應variant，emptyquery先Err，自己建SearchError::Read(PermissionDenied)再source借原因／match&error借typed.kind；Q2b 需可控觀察公開錯誤保存與分類，避免真實權限不可控制以及OSDisplay文字當契約。Q3 caller有Err(other)未來分支，source trait用use取得method，已教；renderer及search都只借，Options與hits仍可讀。Q4 原句「不是靠真實檔案權限碰運氣」與「測試比較typed種類，不把作業系統文字當固定契約。」對比證據界線。表現原行1/3、第三行小寫、第四符合但限2而不保存，有原data與output相互對應。未實跑／圖，無新增未知。

## ch14-complete-prerequisite-v8 / 004-008（順序31）

SHA256 `c334aa6c24285758b4013bc9902c9fcb53c2b91d4ef6343751ef2f7f11133ecf`。

Q1：真正跨crate負例驗證private constructor、nonexhaustive外部分支、private module邊界。空[workspace]與--manifest-path在14.2已經actual學到，本段雖標新增但不是本人首次，不改前判。Q2a：BatchLimit(0)繞私有字段→E0423；LimitError外部窮舉→E0004，同定義crate不限制；options::Options私有路徑→E0603，rootexport仍可用。獨立package用path依賴真library，自己的workspace/lock隔離預期失敗targets，命令明選manifest與example。Q2b：caller不能繞驗證、需未來error支路、應用穩定公有入口；測試必須真的跨crate，否則看不到外部規則。

Q3：前面正用new／Err(other)／ch14_api::Options；三badsource預期nonzero和目標錯誤，不是應修成通過的一般test。Q4 原句「把enum搬進同一main裡，就測不到本次的外部規則。」與「預期非零結束與對應錯誤，不是應修成綠燈的一般測試」支持驗證範圍。BODY片段可理解；未開sourcearchive／Cargo.lock／fixture實檔、未執行。無圖、無必要新問題；同工具重複介紹只是背景無需當新知缺口。

## ch14-complete-prerequisite-v8 / 004-009（順序32）

SHA256 `3916dadce417c6e634f2119295468d894e10380fbb5e957b7b75d7b32f478c96`。

相容二項：Q1 source compatibility是舊合法caller不改程式可否編譯，behavior compatibility是結果仍守原約定；Q2a 同一實體consumer.rs對1.0.0/1.1.0庫（只privatebool→多privatelimit）編譯／執行同assert，defaultfalse與getter保持；Q2b 原需要是internal改動不迫caller改碼、不暗改搜尋意義。Q3 增private欄位不用補literal；改defaulttrue仍可編譯但assert失敗，公有欄位新增反可能缺欄compile失敗。Q4 原句「這只驗證這份caller用到的契約，不代表任意API改動都已被證明相容。」限定樣本。我沒實跑，不能將正文『如實跑快照』據為自己測試。

[[bin]]：Q1 manifest binary target設定；Q2a name定Cargo目標、path以manifest目錄起點，兩version共../consumer.rs，同package借同library名；Q2b 需要同一份caller免同步改寫掩蓋compatibility差異，不必只src/main.rs。Q3 manifest-path選version，各跑同assert；Q4 原句「不是把caller同步改寫成適應新版」支持比較控制；不開manifest實檔／來源。

SemVer：Q1 semantic versioning語意化版本major.minor.patch表示維護承諾；Q2a 1.x後patch修錯回既有契約、minor加相容能力、major不相容；Cargo不證明程式，0.x依最左非零部分，0.1→0.2跨界，0.1.0→0.1.1通常同範圍，0.0.x各patch可跨界。Q2b 依賴者需要從version知道承諾而不是看語法成功猜行為；刪fn／必填參數／預設100→10／feature移除都要按原契約判。Q3 exhaustiveenum加variant使外部matchbreak，事後標nonexhaustive也break；從一開始標可減來源風險，仍需說明新增失敗語意。Q4 原句「版本號是在表達維護者的承諾，Cargo不會自動證明程式符合它。」與案例表限定規則；不把所有0.x互相相容、也不把pubprivate單項當全證明。無圖，必要concept依據夠，無新增未知。

## ch14-complete-prerequisite-v8 / 004-010（順序33）

SHA256 `aca3da144a764c0dd962328fd45bcc49b69b55b219b94f4f9d35f19c5f0a518f`。

既有tests/doc/clippy：Q1 integrationtest作真正externalcrate核公有入口；Q2a 正常/no-default/all-features各組合，負例獨立；package根cwd使Path fixtures/note.txt指到材料，doc建立非doctest驗證；Q2b 需邊界/default/ASCII/owned行/zero/空query優先/source/text皆受caller證據支持，最低rust-version與platform需實際核非自己機器偶成；Q3 normaltests不應被故意失敗targets混入；Q4 原句「文件建立沿7.2，不代替doctests或caller測試。」明確檢查各角色。未實際跑。

新 cargo package：Q1 是本機組.crate／解包編譯，非registry上傳；Q2a --list只列檔不產artifact，--allow-dirty準未commit內容參與本機，不意味着已commit/publish；nestedpackage snapshots/negative不屬本package，但共用consumer.rs仍外層可列。Q2b 需確定交付包包含src/examples/tests／requiredfixtures與邊界，而不只工作樹可編譯。Q3 list核範圍，package得到target/package/ch14-api-0.1.0.crate並解包compile；registry另需metadata/support/releasenotes，publishfalse保留。Q4 原句「不上傳到registry。」和「發布教材網站和上傳library到registry是兩項工作。」限定操作結果；從BODY得命令用途，不稱確實產檔／pass。無圖／新增未知。

## ch14-complete-prerequisite-v8 / 004-011（順序34）

SHA256 `d74a2716171a05c628d2ea95b4a38617f8f171b05b2122c761f2dbc9b6dbf416`。

Q1：小節末把caller明選新limit與修改既有default分開，比較同搜尋資料契約的影響。Q2a new1合法、Optionsdefaultcasefalse／100再with_limit1，search對原行enumerate後push首個符合並達限break；Python仍佔原行1，所以Hit行2而非保存列表序號1。Q2b caller需選1筆但不改變原行資料規則；維護者需增加可選能力而不暗改未選設定的舊caller行為。Q3 查Rust結果ownedRust第一行/line2；assert可核，原default100保持；default改1影響所有未明選者而操作隻影響選者。Q4 原句「前者改未選設定的行為；後者讓原預設與原caller保留。」支持兼容差異，不從“都可限制1”混爲同效果。完整參考程式外連／命令未開未跑。

14.4節末：可見API之外，invariant全入口、Default具體值、typedfailure/source、ownedresults、root路徑、feature和公開能力各自有承諾；private安排／nonexhaustive／SemVer／測試各能支援部分需要，不能單獨證明整API相容。從actualBODY可連回caller有效上限／搜尋意義／維護需要，未見必要缺口；版本/platform/package具體檢查尚屬技術輪而非本人驗證。

## ch14-complete-prerequisite-v8 / 005-001（順序35）

SHA256 `3ebb7eb3106c59762b4634c7645b027907e2fbfebc2c2553e671df17c605b7d2`。

Q1：property-based testing以性質為基礎，先寫一類合法材料應成立性質，再工具生成多筆驗；本節proptest沿4.3 HashMap同鍵累加，鍵從名稱改小i32便於讀generation/shrink。Q2a mechanism後續才展示；此處明定必須自定合法域／正確答案，工具不能由implementation自知需要，有限測試不窮盡。Q2b 原需要先給：增加長度／重複／鍵組合變化，同時有獨立於被測答案的判準——每筆都計、計給正確key；不是「能生成随机」反推需求。

Q3 將先聲明性質／域再生成逐份檢查，失敗待縮反例；已有fixedRust2/Python1tests不廢，但材料範圍增加。Q4 原句「不依賴被測程式答案的判準」與「工具不會從實作自行知道需求，也不保證有限測試看過全部輸入。」支持獨立oracle與finite界線。無圖；如何提供獨立判準與生成策略尚待當後段，當前不猜。

## ch14-complete-prerequisite-v8 / 005-002（順序36）

SHA256 `ec9916abd231f4bb0d186e65928c8d732a4851ff8793d8123276724473833727`。

oracle：Q1 判定依據來自原輸入計數規則；Q2a [0,2,0,-3]→0兩次、2/−3一、其他無group，總4；空清單map空總0。檢正次數／總和與每key逐筆相等數兩性質，後者不再同算法建HashMap；Q2b 要抓漏/多算／錯鍵而不讓答案與自己比。Q3 fixedsample先讀懂結果，再propertytests換更多材料；onlytotal仍會把次數分錯key。Q4 原句「單看總和仍可能把次數分到錯鍵。」／「預期用原清單直接逐筆計數，不再建立另一個同算法HashMap。」支持獨立性界線。

counts.values新API：Q1 shared借map返回所有valueiterator，本type Item=&usize無固定順序；Q2a for循環Copy*deref相加，總量對frequencies不超原長度；Q2b 要驗全部次數與輸入長度一致，無需排序。Q3 total_count只借本節計數，map仍可用，count_of缺key回0不插0羣組，和真羣組positive性質區分。Q4 原句「total_count只借讀本節計數結果」限定總和無overflow論據，不擅稱任意人爲usize巨大valuesmap都可安全加。frequencies get_mut/insert/i32EqHashCopy沿4.3，無新借用猜測。無圖、未實跑、無必要未知。

## ch14-complete-prerequisite-v8 / 005-003（順序37）

SHA256 `65dc3d6044c68943b6cebccdc7e7e3aeb940ec55a5c35ee42c4fd0808004315e`。

Strategy/vec：Q1 是生成與縮減的規則，非已生成值；Q2a -3i32..4定i32域−3..3，vec(element,size)回VecStrategy，0usize..32長0..31，每case Vec<i32>；Q2b 需多種長度／重複／key但始終合法，可縮簡單；Q3 每份材料給計數性質驗，範圍小不會overflow；Q4 原句「不是直接建立一份Vec。」支持descriptor/result分離。literal後綴先前actual10v14/v15已學，此處3i32/3_i32/0usize一致，不新洞。

proptest!/values in：Q1 macro接受測試聲明生成真正Cargo test；Q2a in是macro輸入形式，用strategy生成值，多次runbody失敗再縮，不普通fn typedparameter；Q2b 需要將材料族連接逐份性質驗，而非只建vec不用；Q3 values每次取得新Vec供body，最終仍是真test；Q4 原句「不是普通函式參數的Rust型別宣告。」限定syntax。

Config/attribute：Q1 Config::with_cases(64)目標64成功新輸入case、其他默認；#![proptest_config(config)]是block開頭macro識別設置；Q2a 提供blockConfig非任意crateRustattribute，cases非Cargo test數；Q2b 需界定此例有限材料量以可控說明；Q3 無須手數64個test名稱，其他可選fork/timeout不用；Q4 原句「cases指輸入案例，不是Cargo列出的test數量。」直接支持，未猜具體默認數。

prop_assert/eq：Q1 runner接受的case失敗assert；Q2a 條件／相等失敗交runner繼續縮減、最終test失敗，eq取得expression值本例Copyusize；Q2b 需判性質失敗且能縮反例，不能依一般assert語義猜owned數據會保留；Q3 本節onlyusize爲所比value，沒有靠eq保VecStringowner；Q4 原句「不成立就把case失敗交回runner，使它能繼續縮減」支持失敗流程。

devdep/std feature爲7.1/9.1已知，只要求1.12.0實際lock定、defaultfeaturesfalse/std提供所選能力，合併可能另啓功能；未開lock/external完整prelude，不虛稱全集教過。無圖／執行，新選interface的需求與結果各有正文依據。

## ch14-complete-prerequisite-v8 / 005-004（順序38）

SHA256 `69cb9f3be4956b654b659b5a78ac52fb7d517c4916ffa6cf60009657d16bccc2`。

Q1：兩property把總數/positivegroup與逐key獨立oracle實施，剛學macro配置現在用於真正test。Q2a 第一frequencies後借counts.values正數／total==len；第二對−3..3掃原Vec以==直接計數，count_of比較，七key覆蓋生成域。兩項各Config64成功新cases，生成可重複；保存失敗重播／縮嘗試不算成功數。Q2b 需既不漏數量也不把次數分錯鍵；0次算給1 total仍正，需要keycheck；overwrite repeated1會少總，需要conservation。不是任意一項pass便全需求完成。

Q3 每case獨立Vec借給frequency，ownedmap借給helpers，eq只Copyusize；Cargo顯示2tests並不止2輸入，64case非64獨特材料或窮盡。Q4 原句「預期算法只讀原輸入與相等規則，沒有呼叫另一份HashMap更新法。」及「多項性質分別回答需求的一部分，不把某一項通過當成整份規格完成。」支持oracle與測試界線。有限域涵蓋sevenkeys+positive+總關係可核這批結果，不能推所有i32/所有長度均證明。無圖，未運行作者預期pass。無新必要unknown。

## ch14-complete-prerequisite-v8 / 005-005（順序39）

SHA256 `e0a84ae5da9b38e4beb1551240926b07875609f8a40ce66c333fb3dce61cfa2f`。

Just：Q1 固定值strategy建構，Clone+Debug要求，每次0且不縮value；Q2a 外vec長0..31仍可刪項，故專攻重複key而非全−3..3域；Q2b 需要把覆寫錯誤縮到第二次同鍵，不先分析長清單；Q3 faulty每筆insert1使[0,0]總1≠長2，正常library保持原法；Q4 原句「這是針對重複鍵的策略，不是前面完整的−3到3鍵域。」限定材料。

seed/persistence：Q1 RNG是random number generator，seed序列起始資料，RngSeed::Fixed enum收u64；failure persistence是保存失敗供重播的安排。Q2a Config字段mutable指派fixed20261014與OptionNone不裝保存器，fixture不生成新記錄，一般test仍default；Q2b 需要可重播故意失敗演示，隔離正常隨機/保存配置；Q3 同版本策略設定可重播，改版序列不承諾，無自訂保存器所需；Q4 原句「在相同版本、策略與設定下能重播，不保證改版本後序列仍相同。」支持邊界，沒lookup預設保存器細節。

shrinking：Q1 失敗後按strategy試更簡短／小候選，重驗同性質；Q2a 必須留在合法域且仍失敗，通過候選非反例；Q2b 原需要是易理解「相同鍵再次被覆寫」的最小關注材料，不是僅印31筆失敗。Q3 作者固定版輸出[0,0]，圖手示4→2→1，前2總1≠len失敗，1總1==len通過，保留2。Q4 原句「沒有聲稱工具實際逐步恰好走4→2→1。」與「不是任意問題都得到唯一、數學上最小輸入的保證。」限定trace與minimal。

原位SVG SHA3504ea9bb7f1ab23579b22fde41b36b9d220c3385bbb86a78d232887ce0b9eb7已讀並Inkscape390孤立view：標題縮短仍失敗／每次insert1；兩紅框[0,0,0,0]1≠4、[0,0]1≠2；兩向下線「試更短材料」／「再試縮成一筆」到綠框[0]1=1；底字一筆不呈現錯誤、留兩筆反例。不是ownership線，字可辨不稱實頁pass。命令預期失敗不跑，無新增必要未知。

## ch14-complete-prerequisite-v8 / 005-006（順序40）

SHA256 `19c1ab4aeaf14b75e38547ef4e3d30fec4c7a96399ab1cd5c1774d6feffc0031`。

Q1：regression test回歸測試保存已知具體反例，與property持續探新材料並存。Q2a 修回同鍵累加，普通test固定[0,0]核一組／0兩次／總2；一般proptest保存seed於常見proptest-regressions先重播再生成，不必每次輸入完整數據。Q2b 原需要是避免工具／策略更動後已知錯誤材料消失，而不是以有限綠燈證明域外。具體case跨生成策略仍在，seed重建因策略變可能不同。

Q3 正常另fixed空／兩0／±1000；目前random只有−3..3長0..31，更大契約須增策略／性質。保存seed記錄可和source版本保存，負例隔離，package全部6tests與property2cases64各層區分。Q4 原句「它們存的是重建材料所需的seed，不是一定與版本／策略無關的完整資料。」及「而非把這批綠燈當成域外證明。」支持反例／seed／domain三界線。下方練習尚未讀，6總數僅作者預告不猜其內容；無圖／未執行，無新增必要unknown。

## ch14-complete-prerequisite-v8 / 005-007（順序41）

SHA256 `b65e8582fcddc24f239e1b78f3353af6c9454a4d7b53405f0e9703013a58ddef`。

Q1：節末可加性property檢兩份同域清單的計數組合，搭配原守恆與逐key精準oracle。Q2a 兩Vec分別frequencies，把元素Copy經push into joined不move原清單，for−3..3查joined==left+right，最多62不會overflow。Q2b 原需要仍是所有原資料計對key；組合性提供另一違反類型，不從等號漂亮推完整正確。Q3 eachkey應countsum，但all0錯誤滿足0=0+0，所以此test不能代替total／精準計數；前段tests保留。Q4 原句「性質的價值來自它能攔下哪類需求違反，而不只是寫成等號就看起來完整。」明確回需要；練習完整參考外連與所有延伸未開，未寫測試／實跑。

14.5已連先需（多材料、每筆不漏／不錯鍵）→合法strategy→獨立oracle／互補性質→失敗縮減→固定反例+seed保存→持續探索。tiny策略／finitecases／版本重播／手工圖／minimal字樣各有界線，沒有以局部可讀代替方法需要。無新增必要／可選問題。

## 封存

已完成 ch14-complete-prerequisite-v8 的14.1–14.5前五頁全部41 BODY，順序001-001至005-007，每段保存原文SHA／當時四問（Q2機制與原需要分開）後才取下一段。這是本人後續14.6–14.8首讀的指定背景補讀，不是其他讀者prefix首讀驗收；沒有閱讀其紀錄或任何peer／作者／ROOT處置。41/41，CURRENT=null。原位四張SVG皆到當段才讀source並孤立Inkscape390渲染view；不是實頁／Chromium版面驗證，無Rust或Cargo實跑。???閉合完整參考程式／答案皆經預先filter未展開，外部解答與其他來源未開。

必要理解：macro_rules宣告生成與原值借讀分階段；跨crate名稱來源、mixed-site hygiene、$crate与private權限分開；derive保留caller宣告，AST名字／外形生成impl，caller解析路徑採明示固定依賴名契約。API由有效上限／原搜尋資料／錯誤／ownership需求先定，invariant全入口、Default、private配置／重導出、feature／公開能力各支援部分契約；來源相容與行為相容及SemVer不混作一個編譯綠燈。Property先建立獨立oracle／合法域，再generation／shrinking／固定regression与seed，有限域與手工圖不作域外或trace證明。

未知時序保留：002-004先見expr fragment時由&distance作合理推論、尚缺名稱契約，002-005正文明确介紹後才追加補足；沒有回填早判。固定crate改名自動尋找、全AST／全proptest API、全alias／platform驗證均未教亦不在此有限需求；無確認必要或可選教材缺口。局部看懂和原工作需要已各自核對。無新技術驗收主張，舊1–12與11各原SHA不更動。現在可在本入口封存後開始新tail14 gate，未讀14.6–14.8 BODY。
