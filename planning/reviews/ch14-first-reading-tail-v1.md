# 第14章14.6–14.8獨立首次閱讀原判

AI輔助閱讀，不是真實Python新手研究。沿自己的actual1–10與已封存FIRST11／FIRST12路線；本入口41 BODY只建立本人的14.1–14.5實際背景，不改其他讀者的prefix原判。尾段23 BODY另作獨立原判，沒有接觸其他第14章審閱答案。

每次CURRENT在工具輸出前closed-filter，不開未讀unit、archive、完整source、作者、同行、ROOT findings、outline、live或外部答案。原位置SVG到當段才看；保存四問、Q2a機制／Q2b原需要與用途、原句、合理推論／自補／未知／回讀、source SHA後才NEXT。不跑Rust，不把獨立SVG當實頁驗證；主文完成前不讀閉合選讀。正體中文敘述，引用/API保留原文字。各輪分開封存，不回填早期分級。

路線／reading-order SHA：

```json
[
  {
    "snapshot": "ch14-complete-prerequisite-v8",
    "reading_order_sha256": "4b2b839b7e2be5fa0cdf177dd88ed66080d092749a38421d696b1410b81d2f71",
    "selected_units": [
      "006-001.md",
      "006-002.md",
      "006-003.md",
      "006-004.md",
      "006-005.md",
      "006-006.md",
      "006-007.md",
      "006-008.md",
      "006-009.md",
      "006-010.md",
      "007-001.md",
      "007-002.md",
      "007-003.md",
      "007-004.md",
      "007-005.md",
      "007-006.md",
      "007-007.md",
      "008-001.md",
      "008-002.md",
      "008-003.md",
      "008-004.md",
      "008-005.md",
      "008-006.md"
    ]
  }
]
```

## ch14-complete-prerequisite-v8 / 006-001（順序1）

SHA256 `2960d5334e2326c9462e579c5bf615ca055f3277696b5b16f99f6e67e7e4a687`。

Q1：Loom與Miri本節分別檢查並行交錯模型／執行中的unsafe責任，接本人actual10.8 worker增加與12.4safe切分，但不能互相代答。Q2a：當前只知Loom小型並行模型、Miri執行解譯，具體怎麼explore/interpret尚未教，不臆測；後文先關係再工具材料。Q2b：原需要已在工具前提出：這次0→2未漏更新不代表其他允許交錯；safecaller修改兩段成功不代表internalunsafe存取皆守契約。不是“有新tool”才反推測試需求。

Q3：普通tests仍核結果，另需材料可呈現交錯或unsafe違反；將分別保存成功／反例界線，當前不推任意程序可跑。Q4 原句「這次沒有遺失更新，其他允許交錯呢？」與「safe 呼叫正常完成，內部的 unsafe 存取是否也守住契約？」具體說尚缺關係；「工具不能替這兩個問題彼此作答。」限定用途。無圖；待後文之機制未知如實留，不算已教complete。

## ch14-complete-prerequisite-v8 / 006-002（順序2）

SHA256 `73b67d281a3d7ba73f82e10b35118d47470371a8addec81f739539692ce0e7c8`。

Q1：lost update沿actual10.8，是各atomic load/store分開所致的邏輯錯，不是non-atomic data race UB。Q2a A讀0/B讀0/A存1/B存1合法，atomic單操作不可拆不讓兩calls合成一次increase；普通thread一次僅看到OS/hardware給那路徑。Q2b 需每worker一次總0→2，尚需排除其他允許交錯，反覆跑沒1也不足證明無。Q3 需分析關係／後續模型而非以一次2pass擔保全paths。Q4 原句「這是邏輯上的更新遺失，不是 data race 的 UB。」和「沒有遇到1仍不足以排除它。」限定邏輯/UB及觀測。

原位SVG SHA95b87282af7e5fb55d7fdcc42761c821021d2666f2e6b928dea7b30b544b20b4已讀source/Inkscape390孤立view。頂「每次重新建立count=0」分叉到獨立兩框；藍甲A讀0→A存1→B讀1→B存2，結果2；橘乙A讀0→B讀0→A存1→B存1，結果1；每框三內向下線只該路徑操作先後，頂分叉表示兩次可能探索，非同一時間兩counter。底只見甲無法排除乙／箭頭該次先後明確。非頻率/工具完整trace、無memory ownership意味。字可辨，未實頁/實跑。新model探索 mechanismまだunknown等下一段；無新增必要問題。

## ch14-complete-prerequisite-v8 / 006-003（順序3）

SHA256 `cf13f5de7efeb24c4d726087a421664abf8727379b2be1a8047bfda1b5d2e63f`。

Loom/model：Q1 Rust並行測試工具，以模型替代同步／threadAPI記操作、安排允許路徑重跑；Q2a loom::model收無引數unit closure，要求Fn()+Send+Sync+'static，本例不捕外部、每輪在body新建0，主動探索交錯/atomic觀察而非sleep撞時間；Q2b 原需要是排除未觀察lost-update，不把前輪結果帶下輪混淆。Q3 共兩worker加協調路徑三modelthread，一atomic各加一次，無loop/random/I/O的小模型；Q4 原句「count 必須每次從0新建」與「Loom 只追蹤接上它的操作。」支持探索reset與覆蓋界線，006-001/002機制未知此處補足不回填。

替代Arc/Atomic：Q1 不是std同型別，只沿已讀10章角色；Q2a LoomArc::new/clone保活同一counter非複製，Atomicnew/load副本/store替換/fetch_add一次RMW返舊，Relaxed同atomic操作不發布其他資料；Q2b 需忠實共享位置與increase規則又接入追蹤。Q3 全在modelbody使用，換stdatomic便未完整追蹤；Q4 原句「名字相似不表示與 std 是相同型別」支援不能僅model外包裝聲稱覆蓋。

模型spawn/join：Q1 Loom模型內另一執行路徑／handle；Q2a spawn FnOnce工作與結果不靠短命借用，join耗handle等Result成果，兩份完成才讀count；Q2b 需要worker各操作與結尾檢查時點正確。Q3 每輪body自己建立／等待，不將ordinaryOSthread當模型對象；Q4 原句「加上執行 model body 的協調路徑，共三條模型 thread。」計數對應。

配置devdep精確0.7.2、空features與ch12path沿7/9已知；獨立integration model不是production實作自動改std/Loom。更多操作／loop可能狀態大增、真實服務不能直接全塞；小模型仍須忠實保持被測關係。無圖／實跑，API最低需要有表支援，無新增必要猜測。

## ch14-complete-prerequisite-v8 / 006-004（順序4）

SHA256 `eff814e5879efefaa991368555058acc1b46cef989581db910fb1ab6e133465f`。

Q1：故意失敗的Loom小模型對分開load/store的每worker增一次要求給反例，隔離正常tests。Q2a model內新0、兩clone同counter、spawn各load舊+1/store，兩join取得結果tuple後Ok/Ok才assert2；worker不正常完成也panic失敗，不能當兩次完整更新。Q2b 原需要是每份增加一次應合2，也需反例可重現，而非只檢test非零退出。Q3 從獨立fixturemanifest／--test命令找到目標，診斷left1/right2是完成後counter vs工作需要；expected／source實檔未開，正文足解。Q4 原句「前圖的乙是一種能解釋1的交錯，不把它冒稱為這次診斷中保存的全部操作trace。」與「下載失敗、找不到Loom或工具自身不能建置，都不是本例反例。」支持counterexample而非頻率／trace／工具錯誤。

合理推論：一個合法模型路徑即可否定分開read/store總正確，無需列全paths；不能據診斷說實際machine失敗概率。這是正文支持的邏輯推論，沒我獨立執行。無圖、新syntax爲既有moveclosure/join/tuplepattern/Ok_/panic全部已有actual背景；無必要未知。

## ch14-complete-prerequisite-v8 / 006-005（順序5）

SHA256 `1322108df86a4b51e809f38f2f4dd2d0c3b6e6245be98fa2456ce3addc57d1f8`。

Q1：同工作模型只改worker增法，fetch_add/RMW沿10.8讀改寫一次的修法，Loom驗同final2。Q2a atomic修改序接兩次RMW，不依各自先load0覆寫；兩join完成再load，其他共享來源／初值／兩份／assert不變，_old只未使用舊值binding。Q2b 原需要是各加一次不漏且最終檢查確已完成，不需發佈其他資料，故仍Relaxed；加SeqCst給兩步並不能代一次RMW。Q3 正常testtarget的正文PASS只說這有限模型本次探索沒違final2，需同保存探索限制；LOOM_MAX_PERMUTATIONS等env可縮範圍，本節宣告無排列/時間/搶占上限用預設。Q4 原句「理由仍包括RMW及兩份join」與「Loom本身也不是完整的Rust／C11記憶體模型，某些弱記憶體行為未涵蓋」界定機制／工具結果。預設探索“模型範圍”不等實際硬體所有輸入算法證明，且沒替Relaxed增加publish。本人未跑，不把作者驗證配置當我觀察。無圖、新env是限制來源已給足角色無需查詢值細節；無必要unknown。

## ch14-complete-prerequisite-v8 / 006-006（順序6）

SHA256 `a34bb10852fa93df1aff74bcbdf4418ecbdfaac8cedb293b01497b9b2ab463a1`。

Miri/MIR：Q1 Rust執行解譯／UB偵測，MIR=Mid-level Intermediate Representation中階中間表示是compiler整理的執行步驟，與Loom交錯模型不同。Q2a 解譯step追配置/存取，支援如use-after-free/outofbounds/uninit/alignment違約可報，沿測試呼叫入safeAPI內部unsafe不省略；Q2b 原需要是split safecaller觀察值正確外，internalraw來源／範圍／alias等是否守約，不是safe字面等sound。Q3 selectedinputs/path檢，不能證任意safeclients無UB，12.4contract/range/privateinvariant仍審；Q4 原句「不是看到caller沒有unsafe就略過它。」與「它檢查的是實際解譯到的輸入與路徑」支持覆蓋與證明界線。前開場mechanism未知此處補足只追加。

nightly/rust-src/setup：Q1 nightly按日發佈，Miri貼compiler內部需對應nightlycomponent；rust-src同版std source供解譯環境。Q2a rustup install指定nightly-2026-10-01/minimal/componentmiri，addsrc，再cargo +date miri setup與version；+只此次command不改主stable1.99；Q2b 需要工具／std環境匹配可重播，不能用隨日變化nightly名或把setup當tests。Q3 setup首次可能download/buildstd，test/run解譯target；固定版本rustc1.101.0-nightly/Miri0.1.0/commit/date為作者給定位非本人安裝。Q4 原句「完成setup並不代表教材測試已通過。」明確工具準備/被測結果分離。cargo miri test/run角色／Cargopackage/target已足，pureRust miri_contracts非Loom模擬排程；未查外源／安裝／執行。無圖／新必要洞。

## ch14-complete-prerequisite-v8 / 006-007（順序7）

SHA256 `35452d4e356b27e6d8a785b12cbdcf7bc9ce34637623fd40267b8624bee1f403`。

Q1：三test用同公開split_i32_at_mut，不複製unsafe實作，將actual12.4契約特定路徑交ordinary/Miri各檢。Q2a 中間mid2修改left0+20/right0+5後borrow塊結束核原array100/120/45/60；端點0／len與emptyVec僅讀非empty元素，超長mid3回None且原值沒變。原長0仍合法nonnull/aligned、不讀onepast；機制沿12.4。Q2b 需要覆蓋普通結果之外的邊界內unsafe存取責任，端點/空材料具理由非裝飾；同helper同target防換實作混淆。

Q3 ordinary先結果，再Miri相同target3路徑，作者呈現通過支持指定輸入結果與所檢UB未找到，沒有生成更多輸入。Q4 原句「沒有複製另一份unsafe實作。」／「它沒有增加新輸入、也沒有涵蓋所有unsafe用法」限制證據；完整soundness全前提另審。未跑，未據正文pass稱自己驗證；無圖／首次語法新洞，borrowlife/VecasmutSlice/expect/None來源均原actual已讀。

## ch14-complete-prerequisite-v8 / 006-008（順序8）

SHA256 `7b2dc644aee8625d5cfdc9a342035acf83000f8d95fc6a083b12882732b526d1`。

存取負例：Q1 rawpointer不保活來源沿12.3；use-after-free本例明確storage已失效，不需實驗aliasmodel。Q2a Boxnew80與&rawconst *owner指同i32，drop接手Box釋放allocation，pointer資訊還在但後derefUB；unsafeblock只允操作、不復活storage。Q2b 原需要是驗證Miri能辨被釋放後讀的違約，不能以可能印80支持正確。Q3 只解譯fixture，診斷*deref與allocation/free點關聯；編號變化不影響同份已釋放資料事實。Q4 原句「unsafe block只有責任標記，沒有恢復已釋放的storage。」與「不需先推導完整的實驗性aliasing模型。」限定結論。

cfg(miri)/not：Q1 Miri爲被解釋crate設置條件，not反條件；Q2a ordinarycfgnotmiri保compile_error拒正常artifact，Miri則能解釋此故意UB；Q2b 需此反例僅工具解釋不在普通test/host運行，防誤當normal；Q3 isolatedbinary無deps/workspace/lock，與miri run targetcommand預期nonzero，但download/toolbuild錯誤非memory反例；Q4 原句「僅供Miri解譯的fixture」明示scope，本人連Miri也未執行。cfg規則9.1與compile_error14.2前文已教，新miri條件/反轉在這裏足據。

SVG SHA009b71a12c190640044ee3daad10abd924fc2489425305db7e735711197db367已source/Inkscape390孤立view：上藍ownerBox保活→右綠i32/80實線；中黃pointer不保活另折線亦到同綠框（非第二allocation）；垂直線drop(owner)表示時間前後。下黃舊pointer資訊仍在→右橘已釋放不能存取虛線，虛線是失效指向非可用讀。底舊資訊不延長／*pointerUB，字辨認無實頁聲稱。無新必要／optional洞，無猜結果normal80。

## ch14-complete-prerequisite-v8 / 006-009（順序9）

SHA256 `601a4e5be5a0628522c8f8119aaed494ffea4c7beeadd7a8e7a4f886caba55b8`。

Q1：工具診斷與可得結論表覈對不同責任，Miri UB vs unsupported operation兩類不能混同。Q2a UB=所選路徑觸發工具檢查違約；unsupported=工具不支援操作，不即UB；noReport=此解釋未找到。Miri多平臺API／externalC不在一般能力，不窮盡thread；Loom小模型探索才問交錯。Q2b 原需要是用證據回答正確關係，防成功／錯誤字樣代替範圍，FFI類型owner/free需另審。

Q3 ordinarythreadpass僅一次assert；Loom1存在counterexample；fetchaddpass模型本次未反例；Miri正常3inputs未檢UB；freeddiagnosticstorage已破壞，normal表象無法修。這些各存settings/範圍/反例，不能由Miri未報證明所有C或safeclient。Q4 原句「不能直接當作程式有UB。」／「沒有報告也只代表這次解譯未找到」與table精確對應。無圖、未執行；supported/unsupported邊界明確不需查外部名單完成本節pureRust例，無新增必要洞。

## ch14-complete-prerequisite-v8 / 006-010（順序10）

SHA256 `6dad6cfa3c90ea73eb682d47a561c27cbd539d0877ed09eb3653ff255cb3df63`。

Loom練習：Q1 將同RMW理由搬不同增量；Q2a model每次80，worker fetchadd20/5，同atomic修改序各保留增量，join兩份才final105；Q2b 需兩工作成果完整而不是若前例1恰好成立；Q3 原trace/次數不變只值變，可用同正常modeltarget，無新flag；Q4 原句「第一題把一次更新與工作完成的理由帶到另一組材料」支持推廣理由，非對所有算法證明。

Miri練習：Q1 將validstorage時期讀與i32Copy獨立值分離；Q2a owner仍活時*pointer Copy80，dropowner後只用普通i32副本，舊raw不讀，搬正常integrationtarget；Q2b 原需要仍是拿值用同時storage存取守有效契約，非等待UBcrash修；Q3 Miri可核正確selectedpath，正常不留故意UB；Q4 原句「不要把故意UB留在普通target嘗試crash。」和「工具結果是契約審查的證據」限定驗證行爲，我沒寫／跑。

14.6節末已連原工作0→2／safe分段修改需要，與模型共享、RMW／join及storage/alias前提。兩閉合完整參考程式／命令都未讀，外連也未開。模型限制／支持範圍／onepath證據已正文明確，不把局部可懂=一切方法充分。無確認必要或optional問題；完整弱memory與alias模型是明示界線非本例洞。

## ch14-complete-prerequisite-v8 / 007-001（順序11）

SHA256 `35197ae807fc82e61ead6a4ef018ed8eb49770af8ff044959c48108277778d56`。

Q1：benchmark基準量測固定工作／材料重複執行，記範圍結果判差異，接11.4Instant但非單次elapsed即證優劣。Q2a 本段只有方法位置，Criterion mechanism後教；相同輸入答案先核，再量重複，其他程序／排程／當次材料都可能影響。Q2b 具體原需要先給：只借讀文字計匹配行數時，每行owned判斷複本增加多少工作；7.9原先爲表示/保存清楚，這裏不是本來就需owned結果。

Q3 ch14-performance後選2候選同工作，先覈對答案後Criterion，有工具結果要連範圍再判。Q4 原句「只需要借讀文字、計算符合行數時，每行先建立複本，會增加多少工作？」與「先核對答案，再引入Criterion」支持需要／方法順序。無圖，量工具/具體函數未知待後文，不稱已會性能調優；無必要洞。

## ch14-complete-prerequisite-v8 / 007-002（順序12）

SHA256 `fbef0a736fc554a850cbd86b0d344be9e83524b017d631f196348bda5086ba50`。

Q1：microbenchmark小範圍基準量測，只在已有owned行清單做case-sensitive非空query的匹配行數，與整CLI/I/O/Match/JSON不同。Q2a count_borrowed每String.as_str shared借再contains，count_cloned每行clone新ownedString再contains、迴圈末drop；兩者借原query/lines，Resultusize/String，emptyquery先Err，成功不保存hit。Q2b 原需要是相同純計數工作是否有每行復制額外成本，要先持工作／材料/結果等同再談時間，不能因borrow會少copy而憑名字給固定倍率。

Q3 five材料RustRust/rust/中有Rust/Trust/empty，Rust2符合行非次數3，rust2含Trust子片段，Python0/無資料0/emptyqueryErr；原輸入兩方法均保留。Q4 原句「一行符合就算1，不數出現幾次」與「時間差仍須量測」支持工作規範與source-extra-work vsmeasurement界線。APIasstr/clone/contains/Result都actual教過，新增micro範圍已說明；原需要不ownedresult所以borrow合法，本段無需改變library契約。無圖/實跑，無確認必要unknown。

## ch14-complete-prerequisite-v8 / 007-003（順序13）

SHA256 `67372de6eb3b87be4dc2f47d74925484a2428d4110e34a4e51a7ee1bf8a6a7d4`。

Q1：合成workload固定分佈而非代表所有真筆記，建立計時前輸入／oracle依據；helper與tests沿已學fnVecStringfor。Q2a index%4==0選每4首RustRustnotes15byte、其餘Pythonnotes12byte，64/256rows各16/64hit；same slice給兩法；預期按rules獨立核而非只比較outputs。Q2b 原需要是量同工作複本成本，把行數／長度／hit比例可追蹤，並避免兩錯誤答案相等當正確。

Q3 固定1/4hits／總內容816/3264bytes，VecString結構額外未含；3integrationtest片段case/empty/knownworkload，debug/release均正確，不設millisecondassert。release沿9.1而非忽略正確性。Q4 原句「不能只檢查兩份錯誤答案恰好相同」及「答案正確與時間是否改善，是不同檢查。」支持先正確後time。64與256巡訪4倍不意味着時間必恰4倍，作者沒做該推斷，真實各種字長／hit比例／大檔未涵蓋。無圖／實跑，新%若記憶不足可從明給0/4/8語義讀本例，不僞稱新增未知（actual1.7已讀運算背景）。無必要洞。

## ch14-complete-prerequisite-v8 / 007-004（順序14）

SHA256 `0fa67584ac3ed9b05ac2194a20acdd8e5a7af9efc0da5afdcce4a28feffb3492`。

Criterion/benchtarget：Q1 stable用benchmarklibrary重複工作收耗時分析，bench專可執行target非example；Q2a devdep0.8.2/lock解析，關default選cargo_bench_support不平行async圖表，[[bench]]name/默認benches/name.rs及harnessfalse關Rustrunner由macromain替；Q2b 需重複可靠計量此純計數而非library自動決定答案／代表材料。Q3 默認elapsed沿Instant，OS暫停仍流時非CPU指令累計；Q4 原句「沒有替我們決定正確答案或代表輸入。」限定管理者責任。

兩層callback：Q1 Criteriondefault建管理者，bench_function(id,callback)mut借管理者給callback mutableBencher，iter接FnMut()->O工作；Q2a outer|bencher|給工具，inner||每次借同lines不consume，iter反覆calls整批elapsed幷包括O收尾／loop成本；Q2b 需每次相同完整計數與同邊界而非偷偷減少工作。Q3 prep/make_workload與正確assert在iter外，countclone內部candidate分配/contains/drop皆iter內，borrow完整調用同邊界；Q4 原句「輸出O的收尾也在計時內。」與「不是憑空得到沒有量測負擔的硬體指令時間。」支持範圍。

black_box：Q1 std::hint泛型函數ownedin/out samevalue，best-effort compilerhint；Q2a 對referenceinputs與Resultoutput減憑常數消除簡化，不關release、不clonewholeVec，不保證具體機器指令；Q2b 需量確實消費結果與輸入的工作而非被恆量化簡，不能把hint當需求邏輯。Q3 sharedsliceref/queryrefCopy仍可重複，outputResult正常Okusize；Q4 原句「不能證明編譯器保留某一套機器指令。」界定hint。

原位SVG SHA868acfb53dffef6f3e5bb43754d0067bbbcad17a9ae8827420cb3a6d4b0f1bea source/Inkscape390view：灰框iter外準備同清單＋先核行數，向下箭頭到各自反覆量；藍iter內借讀巡訪→contains→行數，橘iter內owned每行clone→contains、每行副本收尾→行數，兩框之間無連線，非先後同時跑。底相同答案範圍清楚；無ownership箭頭。字可辨非實頁。新必要API各有最低契約，無洞；closedsolutiontarget僅名被列，未讀。

## ch14-complete-prerequisite-v8 / 007-005（順序15）

SHA256 `539d4096d682b2eaaac25d162b130bfa411f468772221d1a783273f2fd607842`。

warmup/cache/samples：Q1 暖身先重複同負載；cache快取保存近期資料可能較近取而非每次遠memory；samples整批估單call，30份非30次。Q2a warmup500ms非0，sample_size30至少10，measure2s非0可因work延長，withoutplots只禁圖不變routine；Q2b 需可追蹤重複工作測量噪聲/分佈，但重用清單不可當首次新檔讀成本。Q3 小練習設置非全任務通用精度，methodsowned管理者→新配置可串；Q4 原句「不能直接當作每次都初次讀入新檔案的成本。」／「30份樣本不是只呼叫30次。」支持material與samples界線。

criterion_group/main：Q1 兩macros生成runner/main；Q2a group name命名runner、config提供剛教配置、targets選收&mutCriterion函數，mainmacro供main/處理Cargo選擇再runner；Q2b 需把量程序與工作管理者連接並取代harnessfalse的defaultmain，非手添anothermain。Q3 compare_counts交工具後iter每次計工作，group不自動新增不同input；Q4 原句「不是要自己補另一個main。」限定組成。

cargo bench：Q1 build/runbench，benchprofile默認繼承release有優化，本repo仍overflowchecks；Q2a --bench目標、--no-run只build，settings9.1已知不復述；Q2b 需確認量測target能編譯與真正測量兩個步驟，不把build當time結果。Q3 兩commands分別no-run/run、locked保解析版本；Q4 原句「不算已量測」明確證據。無圖／執行。新cache基本性質給足本例作用，無需CPU內部推論；無必要unknown。

## ch14-complete-prerequisite-v8 / 007-006（順序16）

SHA256 `db1b9c6a310b0456720c5c14cd502f550a277ee480036d5fbdf942afae517b7b`。

confidence interval：Q1 time[左中右]的中爲每完整iteration時間估計，左右信賴區間界線非min/median/max；Q2a 從此批samples/工具方法表達不確定，默認95%是符合前提反覆整體取樣估計時這種interval長期涵蓋目標值約95%，不是95%單call在內；Q2b 原需要是量結果變動與差異範圍，不用單times猜保證等待。Q3 µs微秒、ms1000µs，完整64/256行＋Result；本輪borrow中心各小但same method兩輪會變，cloned64第二區間寬；Q4 原句「也不保證使用者下次等待多久。」／「環境或輸入改變時，原區間更不能直接移用。」限制統計與實際等待。區間估計具體公式未給，本例讀取含義不需自行推導，我不猜。

outlier/baseline/change：Q1 outlier偏多數樣本觀察；baseline前同id資料；Q2a change/improved/regressed同id比歷史不比borrow/cloned，same source第二輪regressed非代碼修改致退步；Q2b 需公平保存變動、兩輪與環境而非挑快／刪異常／假因果。Q3 borrowed64與cloned64各自己的baseline，不由某change給兩方法倍率；Q4 原句「不能據此判定某項程式修改造成退步，因為我們根本沒有改程式。」支持因果邊界。

CPU／scope：Q1 CPU=central processing unit中央處理器執行運算；Q2a 共享Linux/x86Xeon配額、1.99/Criterion0.8.2/opt3+overflowchecks，兩輪同source，其他CPU工作可影響elapsed，結果通常targetcriterion；Q2b 需記錄版本profile/input/boundary/env/source才能理解重跑限制；Q3 數據支持本合成純計數去副本工作，不分配/bytecopy/free各多少、更非CLI速度／allinputs／hard時限tests；Q4 原句「仍沒有指出時間差中配置、複製bytes或釋放各占多少」限定歸因。所有外連reports/reproduction未開、沒實跑；無圖。輸入/workequivalence與need先明確，因此方法需要可評非僅局部懂數。無確認必要/optional洞。

## ch14-complete-prerequisite-v8 / 007-007（順序17）

SHA256 `0dc89e19b1a5e04ca365671b819fe2dc3a56a9b426d05d3b70f515556dcac220`。

Q1：節末新need是建行清單→計數→臨時清單收尾整段，而非已built slice純count；新bench/newid獨立邊界。Q2a iter內black_box(rows)再make_workload/count_borrowed/block末drop，iter外另已知材料oracle，64/256分佈／runner不變。Q2b 需求若包括準備必須把build和cleanup也計，不能不同工作時間直接叫算法勝負；後profiling才問內部在哪裏，不從benchmark總差猜比重。

Q3 setup_and_borrowed/rows記新範圍，作者一輪2.401/8.911µs只確認可量邊界，非精確原purecount差=buildcost；答案CLOSED，報告/延伸未開。Q4 原句「這與前面已建立slice的純計數需求不同。」與「不把它與前面純計數的差值解讀成精確的建置成本分解。」把need和 evidence範圍明確連結。沒有自行畫產品圖/exec，能用當前說明畫範圍：iter內新VecStringowner及drop，非borrow只call。14.7已連samework correctness→sameinputs/boundary→repeatedsamples/統計→限制／newneed變boundary，非單clone名字／單pass／較小中心推廣全部；必要concept首用契約足，無確認洞。profiling細節尚未讀，不用後文補此判。

## ch14-complete-prerequisite-v8 / 008-001（順序18）

SHA256 `ed6c5722ee1ea35adf1593cbfb8c36ca35d4561d998f476067871285ea13fef3`。

Q1：profiling執行時收選定資源／工作位置與量；本節allocationprofile記heap配置數/size/來源callchain，與CPU profile不同，接14.7不能從總time拆內部cost。Q2a 當前只給資源選擇，還未工具機制；記錄配置不是判哪個fn最多CPUtime。Q2b 具體原問題先立：count_cloned是否每行新heap，borrow是否去此配置且答案相同；同rows/query/countwork，不讓少完成工作冒改善。

Q3 ch14-profiling獨立package沿同材料，後需actualprofile證據與sameanswer核，不拿時間估計替配置證據。Q4 原句「它回答『記錄到哪些配置』，不回答哪個函式占了最多CPU時間」此句引用引號呈現，逐字核心「記錄heap配置的數量、大小與來源呼叫鏈」及「找CPU熱點需要對應的CPU profile資料。」支援範圍。無圖；tool如何捕獲配置待下段，unknown保留，未先找其他profile資料。

## ch14-complete-prerequisite-v8 / 008-002（順序19）

SHA256 `5382271e16b4149b5f35c9ce6783ba200c1978b7fa6b23b1c3effa1df237245d`。

Q1：repeat_counts安排有限重複同計數，連接真實14.7functions而非複製另實現；dhat只預告profiledependency機制待後。Q2a clonedbool選countclone/borrow，每次sharedsameinput/query 經black_box，Resultexpect固定非空之後Copyusize加總；100×256行每次64共6400，非一次答案乘100假工作。Q2b profile前需工作次數／範圍／正確答案確定，才能歸配置改善而非少跑；材料不同test2hits×3=6、reps0/empty0補邊界。

Q3 兩integrationtests debug/release只結果，無time或allocationthreshold；實際profile獨立example進程，devdepdhat0.3.3/lock版本沿7.1。Q4 原句「沒有只執行一次卻把答案乘100冒充做了100次工作。」與「此時只核對重複工作的答案」支持samework與階段。數值100／6400顯然在usize範圍；helper若任意極大repetitions加總上界此段未討論，但此有限實驗不需該擴大域、暫不據此判必要問題。無圖/執行；dhat實現與heap記錄仍未知待當段。

## ch14-complete-prerequisite-v8 / 008-003（順序20）

SHA256 `a22abc840a5d804f621ba010cd42b553b29aae1d690e4d19c7b826bde9c34876`。

allocator/global/static：Q1 配置器取得/釋放heap；dhatAlloc包systemallocator追蹤，本例實驗性；#[global_allocator]選本可執行global唯一配置器；static是全程序item非main每次let。Q2a 名稱:type=初值;，不staticmut，type須Sync（工具滿足），退出不依scopeDrop，Alloc無字段struct同名值不new/括號；Q2b 需攔截配置與建立工具值，又與局部追蹤期限分工。Q3 各example自有一個選擇，不把14.7bench自動變dhat；Q4 原句「它也不在程式退出時靠scope的Drop收尾。」／「与這個static各有角色」支持生命週期區別，新syntax/typevalue兩用足教，無借static擴生命的推斷。

Profiler API：Q1 builder默認heap尚未追、file_name接Pathview保存配置、build接手始追返局部Profiler；Q2a 活躍再buildpanic，HeapStatsget活躍heap下owned數字snapshot否則panic非Result；drop停止寫JSON/stderr；Q2b 需明確所選count範圍／report輸出，不讓準備/打印配置混入。Q3 beforebuild已備list且只借不擴容，get在drop前，snapshot owned數drop後可印，assert/stdout/listdrop在追外、工具活動不算用戶配置。Q4 原句「材料仍然在heap上」與「不算本次新增配置」支持zero不是總memory，006001未知機制到此補足（此處關聯008001，前編號只是早先工具未知，不回填）。

HeapStats：Q1 four public numberfields量不同職責；Q2a total_blocks u64累次配置，total_bytes u64累requestsbytes同空間再取重加，max_bytes usize同時livepeak，curr_bytes usizeget時live；Q2b 需解釋大量短命copy可total高而peak小，不誤稱整個進程RSS/allocator所有管理成本。Q3 可保存數字報告與查callchain；Q4 原句「總量、同時峰值與目前存量回答不同問題」支持解釋。

SVG SHA68ae7c3c8e56b64dd6c3a329f74453e651c4a2318778a847f1db236d890f9c6d source/Inkscape390view：灰main256行/核64，線下橘build追/100完整count/原list借不擴/get snapshot，第二線下綠drop停寫JSON/印數/list後cleanup；兩線執行先後非owner移送，灰owner在橘仍活，底零新配置≠零heap。字可辨非fullpage。未實際工具運行／查看原source，API最低用據明確，無必要洞。

## ch14-complete-prerequisite-v8 / 008-004（順序21）

SHA256 `5e472f5379ae983ef0f047df7cbe8d5adc0775e0b73c71702488755e99aef8d3`。

debug information／env：Q1 除錯資訊是compiler保存來源位置供tool，不等dev無優化；Q2a CARGO_PROFILE_RELEASE_DEBUG=1只前置此shellcommand對應releasedebug1，不改rootmanifest，仍release；Q2b 需profile指回source而保量程式優化設置可追蹤。Q3 cloned／borrowed各example同256×100scope，僅模式/filename變，targetrelative以當前repo cwd爲起點；Q4 原句「它不把opt-level改成dev的0。」支持debuginfo與optimization分離。具體符號內部未教不需要。

report完整性：Q1 ProfilerDrop停止寫檔會stderr診斷不返ioResult；Q2a get前drop後printstats，行數assert6400不證json已寫，需本次path／stderr／實際Load檢文件，缺檔/parseerror不能拿舊檔；Q2b 需profile證據可讀是這次數據，而非正確stdout充當報告。Q3 clonedtargetjson與borrowed獨立記錄，兩模式同正確答案；Q4 原句「行數輸出正確，不等於JSON檔案已完整保存」限定Drop成功假設。本人未載檔或viewer，這些都是BODY教的後續操作非已驗。

量與歸因：Q1 表是作者固定日期/版本/releaseDebug1實觀察，非type語意保證。Q2a 256×100=25600blocks，每遍64×15+192×12=3264bytes×100=326400，當前candidate逐行free故peak15/end0；borrowzero新增但原256Strings/Vec仍活。Q2b 原需要先6400同工作，再實allocation/source證據確認copy額外工作去除；不能靠源碼強制假設編譯器一定全配置。Q3 compiler可消合配置、tool/config影響統計，不設普通test exactallocation門檻；Q4 原句「這些是這份程式的觀察，不是所有版本都必須產生的配置次數。」限制推廣。

backtrace/PP/viewerfields：Q1 backtrace呼叫鏈紀錄指出哪些fn到配置，PP=program point同來源聚合程式位置非每次新節點；Q2a Load本次JSON/SortTotalbytes按累bytes；Total100%分母所追配置非CPU；At t-gmax追蹤整體peak15/1不含前list，At t-end追停存量0非main所有owner結束；Allocated at鏈先近allocator框後main，從main往前找repeat/countclone/StringClone，同筆量不能每框再加。Q2b 需具體來源位置連接推測與記錄，而非挑slowname猜熱點；Q3 borroweremptyroot0無憑空熱點，viewerelapsed/lifetime非CPU耗時分配，tracker自己影響time。Q4 原句「不要把每個框架各列的同一筆量加總成多份配置。」支持鏈與計數關係。

原位SVG SHA2aa142ed3bb90bd7d42e544733712d8168c75fe286f085a96d8d749867f69627 source/Inkscape390view：灰allocation_cloned::main example18→藍repeat_counts profiling12→橘count_cloned performance21→橘StringClone（更近配置框架省略），三下線是調用關係非先scope時間線或owner移動，底同來源25600blocks/326400bytes非各框四份。來源與BODY line.clone片段相連，不開actualsource；字辨非實頁。新必要API/解釋均據足；無確認洞，尚未自己實證性能/配置。

## ch14-complete-prerequisite-v8 / 008-005（順序22）

SHA256 `b12a84c5590bb826f3379d3115654da768283fbf0e7d0e4a1aa41b83f84cbc67`。

Q1：allocation evidence與normal benchmark各自回答一部分，samecountwork纔可接，不把profile儀器耗時混入performance歸因。Q2a countclone僅contains用candidate當行drop，借str去複本仍完整遍歷／querycase／100times；benchmark無dhat量完整countelapsed，profile有tracker/debug觀配置受自身記錄callchain擾時。Q2b 原需要是找可移除的配置同時完成同答案/owner/failure規則，而非所有clone都少用、少input當優化。

Q3 兩證據支持此合成短行計數：實record有多餘配置、borrow無此新增且前正常time估較小；不能相減儀器time推cloneCPU比，也不能從allocationtree猜CPU，非所有輸入／CLI倍率。若7.9要輸入結束後ownedreport，則ownedcopy有責任，borrow不能悄換失效。Q4 原句「那份owned資料有工作責任」與「改善要保留輸入、答案、ownership與失敗規則，才有比較的意義。」明確 need先於低allocation偏好，連borrow機制與原用途。無圖／實跑，本段足回應方法需要而非局部語法；無新增未知。

## ch14-complete-prerequisite-v8 / 008-006（順序23）

SHA256 `59ccf5422506b22c604b1ba892d4e4975010a85c1ea37eb818a1921e85f438a7`。

Q1：節末64行練習區分累積配置量與同時存量，仍同count_cloned／query／100次，僅材料規模變。Q2a：分佈16符合行×100=1600，64×100=6400blocks；每遍16×15+48×12=816bytes，累計81600。每行短命candidate結束，最長仍15，所以峯值未縮、snapshot0。Q2b：原需要是說明新材料的用量關係與證據範圍，不能一句少bytes同時指總量／峯值／目前存量，也不能以較少行的配置次數叫演算法改善。

Q3：scope仍先準備、活躍時get、drop後輸出、新檔dhat-solution，不覆蓋舊報告。作者呈現數支持此固定版關係，工具／輸入變需重跑保存實際報告再判。Q4：原句「少了巡訪總量，但每次最長複本沒有變短」及「此題讓累計量與同時存量分開」支持累計／存量分離。完整參考答案程式維持CLOSED；折疊之後的普通BODY數值實際可見且已讀，不據此聲稱展開或自己執行。profiles／viewer／外部連結未開。

14.8已連原問題（每行heap新複本是否消失且答案仍相同）→固定同工作／oracle→allocator與Profiler範圍→owned數字／報告完整性→來源呼叫鏈→正常benchmark互補→ownership用途界線。新增static、無欄位struct同名值、global_allocator、builder/get/drop、debug環境值、PP／viewer欄位皆首用有最低契約。沒有完整API或技術答案查證。無確認必要或可選缺口；008-002一般極大repetitions加總上界未討論仍保留未知，有限100例不受影響。

保存時點追加：第一次checkpoint工具回exec-server transport disconnected，沒有推斷已保存；恢復後先核自己的state確為saved22/current008-006、SHA仍59ccf5422506b22c604b1ba892d4e4975010a85c1ea37eb818a1921e85f438a7，才重新保存本已獨立寫出的理解，未取其他BODY或答案。第14.8完成後停止，不讀13／15。

## 封存

本原初判已完成 ch14-complete-prerequisite-v8 的14.6–14.8三頁23 BODY：006-001…006-010共10、007-001…007-007共7、008-001…008-006共6。每個CURRENT預先closed-filter，在保存四問／Q2a機制及Q2b原需要／原句／合理推論／未知／回讀与raw SHA後才NEXT；23條單位標頭與23條SHA256紀錄，state saved23/current=null。入口14.1–14.5的41 BODY先獨立背景封存，未冒稱其他讀者prefix盲讀驗收。沿本人actual1–12及新入口，未讀13／15，未接觸peer14、作者、ROOT findings、修稿、outline、archive／完整source、live或外部答案；原sourcecase只取BODY片段。

原位五SVG（Loom路徑、Miri storage、benchmark範圍、profile範圍、profile來源鏈）均到該CURRENT才讀SVG／SHA並孤立Inkscape390渲染view；具體框／名稱／線／標记关系當段保存。這不是完整實頁或Chromium版面驗收。三單位四份閉合參考答案（006-010兩份、007-007一份、008-006一份）全部未展開；折疊以外普通BODY預期結果仍按實際可見內容閱讀。外連fixture／report／viewer／reference未開。未跑Rust／Cargo／Miri／Loom／Criterion／dhat，不據ROOT或正文PASS宣稱自己實跑。

目前可依正文解釋的能力：14.6把原需要（並行各更新不漏、safe body內存取守約）各接Loom替代同步／reset每輪的小模型、RMW＋join，以及Miri的MIR解譯／所選storage路徑。反例、有限模型未反例、單次解譯未找到UB、unsupported operation各可支持結論不同。14.7先核同計數規則／獨立預期、合成材料及迭代／setup／cleanup計時邊界，再辨樣本、信賴區間、baseline与共享CPU環境；不同工作量和不同邊界不能直接稱演算法勝負。14.8以相同100次巡訪／答案配合allocator／局部Profiler起訖、owned snapshot、報告完整性及PP來源鏈確認可移除的判斷複本。零新增配置不等零heap，累計量不等峰值／現在存量；配置證據不作CPU占比，而owned報告若有存活用途不可悄換借用。局部讀懂與原方法需要已分別核對。

分級：沒有確認必要或可選教材缺口。保留未知而不補查：007-006信賴區間具體估計公式未教，本頁讀欄位與限界不需公式；008-002 public helper若擴成任意極大repetitions，累加上界未討論，本實驗有限100／6400不受影響，不把該扩大域猜測判成已確認問題。完整弱記憶體／aliasing模型、Miri不支援平台與外部C、全部工具API與硬體／所有輸入也都未驗，屬明示範圍而非本例需求缺口。初見機制未知後文補足只在當時追加，無回填。

保存中斷：最後008-006第一次checkpoint遇exec-server transport disconnected，恢復先核state saved22/current008-006及同SHA，才保存本已讀的理解，未跳段；現在已確認完整。少量早段本人短記夾用的外語詞（如006-002的mechanismまだunknown）意指当時機制尚待後文，這裏僅追加正體說明，原判文字／分類／時序不回寫。舊FIRST10／11入口、prefix、middle、tail／FIRST12與本14入口封存SHA已只按bytes核對皆未變；沒有重讀同byte前文或改學生檔。

此處封存original並STOP；不取第13／15章或任何修後／同行內容。
