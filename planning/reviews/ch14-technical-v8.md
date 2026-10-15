# 第 14 章 TECH ORIGINAL：v8

本輪確認兩個局部必要問題：macro fragment `expr` 在解釋前先用，以及兩份隔離相容性快照缺少讓原 `--locked` 命令可執行的 lock。另列一項可選 Cargo 操作說明。沒有確認核心 macro／公開契約／property／Loom／Miri／benchmark／DHAT 的技術或數據錯誤；這些通過限於原稿宣告的材料、契約與證據，不代表所有輸入、平台或正式發布均驗收。

## 角色、唯一稿與隔離

- 審閱者為 `chapter02_ownership`，不是第14章作者。先前撰寫第2章、4.4–4.5、第5章及9.2–9.4，也有 CONT3／7／8、TECH10 與其交叉背景；這是非作者獨立技術初判，不稱 blind 首讀或真實 Python 新手測試。
- 唯一第14章稿為 `/workspace/work/tutorial-implementation/frozen/ch14-complete-prerequisite-v8`；manifest SHA-256：`b36fa91e3a812dfec05fc3ee7341297637d8e8be0da405e187f6f4a5007be683`。完整8頁64 BODY先按序實讀，保留自己的原需、機制、用途、例子與候選，再全文核134 archived sources。
- 來源包括68 Rust、16 TOML、6 Cargo.lock、20 expected、9 SVG、5 JSON、10文字紀錄；正常 helper／test／全部答案、proc-macro與consumer、isolated compile／runtime／property／Loom／Miri負例、bench與原測量資料都讀。完整列表與九個實讀批次保存在 `source-full-read-coverage.json`，不由綠燈代替先備核對。
- original 封存前未讀 peer14／FIRST14／CONT14、作者14筆記／ledger、root14問題或處置、live14及任何新版本。恢復環境時檔名搜尋顯示同行及root維護檔案存在，沒有開啟內容，也沒有取得其發現或答案；不拿 root gate 當獨立 QA。
- 需要的 actual 前文以實際正文為準：12.3的 raw／allocation／unsafe／provenance；12.4六 BODY與整份 contracts helper；11.4九 BODY的 Duration／經過時間／範圍／測試時鐘。其他1–10沿既有 actual 背景按必要位置回查，含1.4／1.10目標與測試、5錯誤、6closure／iterator、7輸入規則、8ownership／layout、9Cargo設定、10並行／atomic。沒有因標題或目錄認定已教。
- 環境中斷後從自己的 checkpoint 接續，保存已完成64／134、官方選段、有限 probe及原資料核對；收尾親看既有畫面並再次回讀上述必要12／11全文和1的局部。沒有重做未改程式、測量或整套檢查，亦未修改學生正文／source或共享 Cargo／lock／nav。

`work/ch14-technical-v8/` 保存原 BODY逐段筆記、source batches／覆蓋、209份凍結檔案指紋、actual 前文回查、selected 官方契約、自己的隔離 lock／命令／stdout／stderr、原資料核對及實頁圖片。原筆記中「pending」與早期未知不回寫成當時已完成，完成與最終判斷另記於本報告。

## 必要 N1：expr fragment在介紹前先進入規則

- 位置：14.2／002-004，private helper負例，`macro_rules! private_reading { ($value:expr) => { ... } }`；呼叫 `private_reading!(&distance)`。
- 已教：14.1／001-003已教metavariable／fragment角色與 `ident` 匹配identifier；ordinary expression與reference在實際前文充分。002-004也已教 `$crate` 固定macro定義來源、仍受private限制，這些機制不缺。
- 缺少關係：這裡首次改用 `expr`，尚未說它接受一段Rust expression、因此 `&distance` 是合法輸入。它是macro輸入匹配契約，不能從普通expression概念或ident模式自行推出新fragment selector的角色。
- 後文：002-005開頭才明說「輸入模式中的 expr 指匹配一個 Rust expression，而非前節的型別名字 ident」。這能釐清後續hygiene模板，不能使前一負例已先懂。本人在第一次 BODY 留候選、後續 source／canonical 搜尋再確認，沒有同行提示。
- 影響／分級：**必要、增加理解負擔；首次使用時序**。學生要把E0603定位為可見性問題，而不是在尚不知能否接受該輸入時猜匹配規則。不是說整項 `$crate`、private 或hygiene主線無法理解，也不是程式缺import或技術錯誤。
- 最小修正：在002-004規則前補一句「expr 匹配一個Rust expression，例如本例的 &distance」；002-005再回收其與ident的差別即可。不必增加macro變體、測試或章節。

## 必要 N2：兩份相容性快照的locked命令無法從交付材料重現

- 位置：14.4／004-009；兩份獨立manifest都有空 `[workspace]`，原命令各為 `cargo run --manifest-path examples/ch14-api/compatibility/v1/Cargo.toml --locked` 及對應v1_1。
- 已教：前文已有Cargo.lock／`--locked`；14.4也明教另一workspace、獨立lock和manifest目錄。same physical consumer、私有欄位、Default保持false、source／behavior相容的對照本身清楚且正確。
- 交付缺項：v8 archive列出兩份manifest／library和共用consumer，沒有兩份快照的Cargo.lock或先建立各自lock的步驟。外層root lock不能代這兩個隔離workspace，與正文對故意失敗fixture保存獨立lock的安排不同。
- 本人有限實跑：在只複製原材料的isolated own workspace，兩個原命令加 `--offline` 均exit101，診斷是 `cannot create the lock file ... because --locked was passed to prevent this`，不是caller不相容。只在自己的副本各generate-lockfile後，同一source及同一caller再run，兩者exit0／assert通過／stdout相同 `未修改的caller：ignore_case=false`。原snapshot／學生檔案未改。
- 影響／分級：**必要、阻礙這一步操作；可重現交付／工具契約**。讀者照正文不能得到所承諾比較，且容易把工具準備失敗當相容性失敗。這是指定凍結材料的問題，不擴稱未讀live repo的實際狀態，也不把邏輯相容性示範判錯。
- 最小修正：保存兩個snapshot的獨立lock，或在第一次 `--locked` run之前教並執行各自的lock建立步驟。前者維持已有locked操作，不新增學生API；無需再跑benchmark或修改consumer。

## 可選 O1：說明test選example，是跑其測試執行器

14.1／001-004第一次並列run與 `cargo test -p ch14-macros --example reading_many --locked`，14.2亦沿用。actual1.4已說 `--example 名稱` 按檔名選範例target；actual1.10已教cargo test／cfg(test)／測試執行器，不需在main逐一呼叫tests。這些已教關係加上「測試另外借讀零與負數」，足以推得選reading_many中的測試；這不是全新旗標或未教測試機制。

可在第一次命令前加一句「run執行main；test選這個example的單元測試，不再執行其普通main」，讓不同命令的責任更直接。**可選：操作目的與閱讀一致性**。原 BODY 的selector候選經actual1回讀後降為此有限建議，保留早期未知，不把canonical沒有 `cargo test --example` 這個字串自動升成先備缺口；與N2的實際無法執行分開。

## 每節主要需要、先備、機制與例子範圍

下面核「機制／性質」與「原需／用途」兩項，再列例子真正支持的範圍；不是只看新名字或算出數字就通過。

| 範圍／來源中的原需要 | 身分與實際依賴／首次介紹 | 機制與用途的支持；例子與界線 |
| --- | --- | --- |
| 14.1／001-001–006：不同單位wrapper共用取i32介面，少重寫宣告 | 4的trait／tuple struct、8.9的newtype、1的宣告／test已學。metavariable／ident／token／expansion及repetition +/*/?／可選comma在本節先解釋再code。 | 普通Reading/read_value仍負責共用runtime取值，macro重用struct／impl宣告；未將函式說成不能處理多筆。80／30／500保各型別／owned來源，不假定Copy或非負驗證；輸入不匹配與展開後E0308分開，空清單答案只把已教+改*。 |
| 14.2／002-001–007：library提供模板，caller兩份來源各借讀，名字不被模板let置換 | 普通read_pair_values及private helper先固定工作；textual／path scope、macro_export根、mixed-site hygiene、$crate、公有helper、tt fallback／first rule與compile_error的角色依序教。expr先用的N1保留。 | caller同名Reading與caller value／template value各有來源，80／30保持；輸入各插一次才求值一次，與hygiene不是同一保證。計數2與tests的order[1,2]支持實際求值次數／順序，reverse答案先保存兩借用再交(30,80)。privateE0603／duplicateE0428與失配診斷不混；隔離crate真跨邊界。 |
| 14.3／003-001–010：保留原struct／visibility／derive選擇，只增加Reading impl | host編譯工具與consumer三package／path／proc-macro設定先教。TokenStream、AST、Synselectedfeatures／DeriveInput／ident／generics／Data／Fields／Punctuated、parse_macro_input!、syn Error／span／compile tokens、quote!／#name／ToTokens／兩stream的into先定角色。 | derive利用已存在宣告的資訊，不重生成原struct；shape checks拒泛型／非struct／named／多欄位，bool由Rust型別檢查拒。aliasi32／private欄位／負值保留，runtime借讀80／30；consumer unit tests2項與boolE0308有限probe一致。::ch14_macros固定crate名稱與dependency改名限制已寫，沒把它說成$crate。 |
| 14.4／004-001–011：caller選case與有效上限，保留原行號／owned文本，library可增加內部狀態 | 7.9的片段／ASCII規則、5/8的具體Error/source、9.1 feature合併先備實際可用。non_exhaustive內外差別、Default／prelude／derive與new不同、mut self ownedbuilder／getter、公有re-export、feature additive、isolated跨crate與[[bin]]共用caller皆在使用前教。 | domain／failure order固定後才code；limit1回原row2，不重新編號。Defaultderive0隔離負例不是合法預設；private範圍與external wildcard分開。先把1.0／1.1當兩份快照label觀察同callerfalse，再教SemVer的major/minor/patch與0.x界線；版本號承諾不自動證明相容。只證本caller／既有能力，不包任意private欄位改動。N2仍待修。 |
| 14.5／005-001–007：不同長度／key／重複次數，用獨立判準檢查frequency | 6 iterator／closure、4HashMap、1tests可用；values借讀、Strategy生成與縮減、Configcases64、宏語法／prop assertions、Just／CloneDebug／fixedseed／persistence／regression各在新用法前教。 | total conservation與逐key直接scan檢查不同關係，oracle不複製implementation；生成域有限，64新成功案例不等兩個test或全部可能輸入。insert1 overwrite的固定負例實得[0,0]／1vs2；手圖4→2→1明標手工，不冒稱工具trace／數學唯一最小值。composition只0=0+0不足，逐鍵答案補其真正用途；域外±1000仍有固定例。 |
| 14.6／006-001–010：普通PASS未排其他交錯或unsafe存取違約 | actual12.3全raw基礎及12.4全contract／helper支持來源、endpoint、合法borrow；10模型支持RMW／join。Loom mock Arc／Atomic／thread／model Fn+Send+Sync+static與每次重建state、Miri MIR／工具鏈／sysroot／選定操作各先解釋。 | 兩條手工交錯示2或1，不是native頻率／data race。Loom只有自己的同步型別進模型，3個threads、無loops／外I/O，split counterexample1與修RMW2各有probe；PASS只及本模型與設定，弱memory／外部資源未全覆蓋。Miri只所選路徑，setup不等PASS，dangling只Miri解譯／cfg屏障，不進normalworkspace；safe split及owner答案支持正常有限材料，不由4個Miri tests證全部soundness。 |
| 14.7／007-001–007：只需借讀計符合行數，量每行clone增加的工作 | actual11.4九 BODY已教Duration／elapsed與範圍；3String／as_str與7.9contains／搜尋規則、6迴圈、5Result可用。Criterion／Bencher iter FnMut與resultdrop、black_box best effort、bench harnessfalse／runner／warmup／sampling／CI各先契約再code。 | 同材料／同case／非空query／每行一次命中與Result，64／256行、hit1/4、bytes816／3264固定工作量。準備／預期在iter外，clone／contains／該副本Drop在內；兩輪真報告支持這兩組合成材料中心估計較小，有outliers／shared CPU限制，未證全部輸入或CLI倍速。setup exercise改量另一需要，不與舊時間相減當algorithm勝負。 |
| 14.8／008-001–006：同count工作每行是否真有heap配置，回source定位再移除副本 | 回收14.7完整function／資料。dhat tool allocator／global_allocator／static unit value、Profiler builder／start／get snapshot／Drop／JSON、HeapStats四欄、PP／backtrace／viewer欄位在使用與讀數前教。 | 100次全256行eachhit64→6400；原資料在profile前建且仍存活，追蹤scope只count，停止後才print／清理。真profile clone25600blocks/326400bytes、peak15/end0與borrow0；三source行18→12→21連到StringClone，rawframe逐一對上。64行答案仍100次→1600hits／6400blocks／81600bytes／peak15，累計與峰值不混。零新增配置不等零heap，配置百分比不等CPUtime，最佳化可省配置故不是語意testgate。 |

小變化皆依原已教規則：空／可選comma與宣告數量；交換已借來源卻保持取得順序；只改default為true仍可編譯但caller assertion失敗；頻率一筆不暴露覆寫而兩筆暴露；split兩端空段不能索引；準備移到iter內改工作範圍；256→64使累計減至1/4但temporary最大行長15不變。這些支持遷移與用途，沒有把審閱者新增需求冒充作者原承諾。

沒有確認需補的必要圖缺席：14.4的private／new／default及同caller對照可由有限來源／錯誤／table判；非完整structlayout／ABI課，SemVer不要求先證所有公開能力。每節都有實際任務與最小操作，缺失N1／N2未被整章主線清楚或有限綠燈蓋掉。

## 固定版本的selected官方契約

std／Reference／Cargo使用 `https://doc.rust-lang.org/1.99.0/`：macros-by-example的metavariables／transcribing／repetitions／hygiene；procedural derive與其hygiene；non_exhaustive外部限制；Default／HashMap::values／black_box；Cargo test選目標與工作目錄、package／allow-dirty／list、SemVer change categories與private欄位。原HTML、URL／SHA及真正選段保存於official目錄；不是下載整頁就宣稱全官方文件讀完。

同版crate source selected：Syn2.0.119的selectedfeatures／parse_macro_input早退／Errornew與to_compile_error；Quote1.0.47的ToTokens／Ident與token append，proc-macro2 1.0.107的兩stream From；Proptest1.12.0的prop_assert_eq借值與錯誤回傳、Configcases／seed／persistence、Just不縮減；Criterion0.8.2的iter計時／FnMut輸出Drop、warmup／sample／measurement／confidence；dhat0.3.3的allocator記錄範圍、Profilerstart／Drop、get snapshot／totals／peak／PP；Loom0.7.2的model／Builder limits、mockspawn／join與C11/weak-memory限制。保存source檔SHA與行範圍，不把registry其他版本或未選API當學生先備。

Miri README固定到實際nightly的Rust commit `21b707e3f`，實讀tool用途／UB類型／有限性／安裝與cfg(miri)。fixed `nightly-2026-10-01` 實際compiler為1.101.0-nightly（commit日期2026-09-30），Miri0.1.0同commit；不把nightly日期混成stable1.99版本。MIRI_SYSROOT只使用root明授權的工具cache，沒有開該author目錄其他證據／報告。

Loom與Miri的充分性由模型／記憶體契約及原條件判；一次host值、某次模型PASS、沒有遇到UB均不能證沒有其他違約。normal split的non-null／alignment／初始化／同allocation／exclusive及borrow來源由actual12契約支持，mid檢查只接手範圍／減法；不是把Miri PASS當理由。

## 實際有限執行與未驗範圍

隔離own workspace只從v8複製原package；ch12依賴用指定contracts freeze，whole helper已核，為精確編譯複製的其他ch12材料明列「不聲稱本輪全文審閱」。normal workspace不含isolated failpackages。自生成own lock107個packages（含local），SHA-256 `418f074befa26285b2420fe8ae1c7481288529f3400f12e9a3c46aa09df8df8e`；不借根lock或稱它重現author benchmark的整份dependency graph。

`run-probes.py`／`probe-results.json`共16個命令結果，完整argv／cwd／目的／stdout／stderr／exit保留；其中包括工具版本與三次own lock準備，不把它們算學生tests：

- 兩份compatibility：原locked命令各101（N2）；own copy各建lock後run各0，same caller assert與相同stdout。只準備自己的副本，沒有補到原稿。
- derive consumer unit tests：2項PASS；同版Syn／Quote正常編譯。bool跨cratecheck101／E0308在derive展開，確認不是shape拒絕或依賴不存在。其他derive負例全文核但未執行，不以這一個代表全部。
- 固定seed的property負例：exit101，實讀minimal `[0,0]`、left1/right2、successes0，對應覆寫而非build錯。未再跑正常property套件或重新取隨機樣本。
- Loom split負例101／left1/right2；RMW normal target1test PASS。LOOM_MAX_PERMUTATIONS／DURATION／PREEMPTIONS／BRANCHES當次未設定；仍保留工具預設模型bounds，不說所有平台所有memory情況全遍歷。
- 固定nightly Miri：miri_contracts3test PASS（不重疊修改、端點／空storage、invalidmid未改）；miri_owner_solution1test PASS（先Copy80再drop）。dangling fixture只 `cargo +nightly-2026-10-01 miri run` exit1，診斷實指 `*pointer`／source allocation／drop deallocation，含預期 `has been freed, so this pointer is dangling`。**從未native run或test該UB負例**。

host為x86_64-unknown-linux-gnu、stable1.99.0；自己的target與lock、選定tool source versions保存。未跑全workspace／普通all-target Clippy／全部正常例、全部compile/runtime負例、另一平台、所有property／Loom models／Miri paths或完整benchmark／DHAT測量；root後續候選檢查不回算本人執行。所有tool failure、logical assertion failure、compile error與Miri UB不同類型，原 expected各按自身責任核。

## 實測原資料的核對

Criterion完整run1／run2／exercise原文字與reproduction JSON的stdout相同，selected source SHA與凍結package吻合。自己從raw bounds／中心值按三位小數四捨五入，正文兩輪borrowed64為1.188／1.214、cloned64為1.992／2.151，borrowed256為4.877／4.852、cloned256為7.953／8.399μs；exercise含setup為2.401／8.911μs。各interval亦匹配原報告，沒有偷換95% confidence interval成個別呼叫min/max或median。CHANGE是同id前次baseline，不能拿borrowed與cloned的change百分比作方法比較；兩輪都保存，不挑快的一次。

DHAT三raw JSON自己decode PP／frames與數值：cloned total25600blocks／326400bytes／globalpeak15／end0；borrowed0／0／0／0且無PP；solution6400／81600／15／0。JSON SHA、stdout／stderr、reproduction及source SHA一致；main18→repeat_counts12→count_cloned21確實是對應呼叫／clone行。此程式snapshot時已沒有temporary String，因此current0與profile end0相同；這是本例時序，不把兩欄概念一概等同。

64×15＋192×12＝3264bytes，每遍256clone，重複100得到326400／25600；只保當前行temporary，所以peak15而不是總和。profile原資料仍存活但配置在開始前，故零新增不否定其heap占用。原測量共享host／release與debug=1／工具開銷、最佳化允許省略等限制保留；無法用DHAT耗時扣出每次cloneCPU時間，也不能把count範圍外工作忽略成完整CLI加速。

`raw-data-verification.json`記真source／raw hashes與數值、bounds／rounding。root Cargo.lock及rust-toolchain的reproduction記錄SHA不在本章archive，本輪沒有讀live root去補「已核」，明留未驗；author另存外部viewer證據路徑未開。沒有重新跑benchmark或DHAT測量，亦沒有宣稱自己已操作外部DHAT viewer。

## 九圖與兩表的實際頁面

由原64 BODY按順序重組自己的same-theme隔離preview，只去重H1／替換asset URL；沿先前明授權已複製的公開CSS，explicit `extra_css`，不開live14或同行頁面。HTTP實載CSS SHA及每個SVG bytes核原凍結稿。Chromium的1280×1200／390×1200各九圖加兩資料表，共 **22個畫面親自view_image實看**；不是只看SVG source或bbox。

| 原位圖／必要對應 | 本人親看結果與教學範圍 |
| --- | --- |
| macro-expansion | Meters名字輸入→compiletime ident→struct／Reading impl→runtime借Meters80，三階段與保owner可辨；不把macro當runtime造型別。 |
| macro-hygiene | 綠caller value借Seconds30沿右線、藍template value保Meters左借80，紫tuple分左／右；同拼法不同來源與結果(80,30)清楚，箭頭不當求值時間軸。 |
| derive-pipeline | compiler stream→Syn AST名字／零generic／一tuplefield→Quote另一stream→into回compiler保原struct增加impl→consumer80；前四步與綠runtime框、兩stream名稱不混。 |
| property-shrinking | 手工4／2仍失敗而1通過，紅／綠各有完整判準；保[0,0]是此overwrite反例，不是實跑trace或通用最小值算法。 |
| loom-paths | 各路新count0，左A讀0存1／B讀1存2；右兩讀0再各存1；結果2／1及「只看甲無法排乙」與圖說都清楚。 |
| miri-storage | Box owner與raw pointer指同80、只有owner保活；drop後虛線到已釋放位置與UB讀取不同於上半正常箭頭，資料資訊沒有延長allocation。 |
| benchmark-boundary | 灰iter外準備／核答，藍借讀與橘clone／contains／副本Drop各在iter內，說明各自反覆量而非同時跑；兩種方法仍同count答案。 |
| profile-boundary | main預建256行仍存活、builder start→count100／get snapshot→drop stopwriteJSON→print／後續資料清理，橘追蹤區段與灰原資料分開。 |
| profile-source | allocation_cloned::main18→repeat_counts12→count_cloned21→StringClone四層／省略allocator框架，與25600／326400同PP可沿鏈回查；不把它當前圖的時間線或CPU比例。 |

desktop圖寬390或400px，mobile約343px，原24px標字縮後約20.6px，必要框／箭頭／文字不需放大。derive圖較高但完整手機畫面仍容納必要鏈；兩表的欄名／兩輪與兩方法／units／數值在手機可正常換行，沒有確認必要字被裁切。相鄰圖說把同來源／不同來源／箭頭用途說清楚。沒有確認新增必要圖或視覺修正；未逐個code block水平捲動、所有fold／下載／外部links／整頁footer做全站驗收。

`visual-metadata.json`保存viewport／URL／bbox／alt／相鄰原句／實載asset SHA；preview建置只支持本地重組可建，不代正式strict網站／發布。外部DHAT viewer操作未親看，原JSON／source核查與教學圖的原位核查已分列。

## 原初判封存與停止

必要N1是最小首次介紹時序、必要N2是原快照的可執行交付，均保留已教部分、最小原句、實際影響與修改建議；O1可選有具體閱讀收益，不升成新測試機制。其他通過項保持條件、物件與證據範圍；未做檢查不填成PASS，也不自動列缺陷。

本報告、SHA sidecars與work immutable original副本封存前再次核209份原v8檔案未變。有限probe／工具cache／復原後圖像實看均有自己的紀錄；尚無交叉／修後覆核。完成本輪即STOP，不讀peer14或其他未授權scope、不修改學生或共享檔案，等待root讀完原稿後另行安排。
