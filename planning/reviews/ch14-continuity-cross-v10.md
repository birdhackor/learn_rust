# 第14章CONT：交叉與v10修後覆核

本輪確認原CONT的三項必要問題在v10已補足；TECH與FIRST各一項可選操作提示亦有相稱、位置正確的修正。沒有確認新增必要缺口。這是已知問題提示後的交叉／修後覆核，不是新的獨立首讀或撰寫；原初判的分級、發現時點與SHA保持不變。

## 角色、順序與保護原判

- 仍為第12章作者，沒有寫第14章；有先前actual前文與CONT14全章背景，不稱blind FIRST。本輪不審第12章，也沒有讀或向其他角色傳送任何12 peer／ROOT結果，不擴13／15。
- 自己原 `ch14-continuity-v8.md` SHA256：`4939fe3132b74673964657b3b0d59cce4a15a302ba5735fea0a2ff5dcc1b675c`。main、sidecar與work sealed副本維持0444及原bytes，必要N1／N2／N3、原0 optional均沒有改寫。
- FULL讀peer原稿：TECH `ch14-technical-v8.md`，117行，SHA `93cfeddf35415a2e942cf0b6b99de7429244f1fc5c485d13d5959416b0313423`；FIRSTprefix `ch14-first-reading-prefix-v1.md`，641行，SHA `f75674e841d37f1f10e8ac84362a3ab9531cb927610c6a173d3612cca54e2c5f`；FIRSTtail `ch14-first-reading-tail-v1.md`，267行，SHA `2f907328208261369a43c0cef80c1a6dc19d3a2b70c12d779ea47ac48f1d3274`。初次合併輸出有截斷，已分段補讀到末，未把被截斷內容假稱已讀。
- 先保存自己的peer比較於 `work/tutorial-implementation/continuity-ch14-cross-v10/peer-cross-before-root.md`，SHA `a5a480f186c1139a18f5424610984c6705609a4db725fc8256db8cdf6afb30c1`；其後才讀v10四處修正／archive，最後比對 `ch14-root-disposition-v10.md`。root裁定沒有提前當我的peer比較答案。
- FIRSTprefix使用五份分節原gate，不是whole v8的直接同檔名。我自行按其實際route對應41 units；排除asset絕對snapshot前綴後，6／7／10／11／7 units全與原v8 BODY相同，沒有教材實質版本差異。FIRSTtail的入口41 units只建立那位讀者背景，**不是第二份prefix獨立QA**。
- 本輪唯一修後教材：`/workspace/work/tutorial-implementation/frozen/ch14-review-fixed-v10`。沒有開live14、新版14或作者14紀錄，也沒有以ROOT preview8858代自己的視覺驗證。

## 全部必要與可選項交叉裁定

| 同一缺少關係 | 各自original的發現與限制 | 自己交叉判斷與root最後比對 | v10局部結果 |
| --- | --- | --- | --- |
| expr fragment先用後教 | CONT N1與TECH N1獨立初判同一最低關係。FIRSTprefix 14.2/001-004確實記expr未知，但依固定&distance與E0603推此例匹配成功，列合理待教；下一段才釐清。 | 維持必要、很小的first-use負擔。選定負例已寫`expr`，仍應先給expr匹配expression、&distance屬expression的最低關係；不用提前講完整expr域。FIRST是分級差異，不能算同項必要獨立重現，也不改其原0必要。root採同一最小橋接。 | 002-004新增一句，在private_reading規則與呼叫前；002-005仍講兩輸入／hygiene，沒有倒移完整課。**關閉。** |
| assert第二參數的自訂panic message | 只有CONT N2原初判。TECH／FIRST談Default與invariant足夠，但沒有報此argument角色；不能以它們主線通過代答新assert形式。 | 保留原小型必要操作橋接；已有bool條件檢查、panic與格式文字，欠的是逗號後message只在false時作panic訊息。事後同意不算同行initial發現。root採用。 | 004-003負例前明說`assert!(條件, "訊息")`及false時的訊息，接回derive得到0的違約；不改正常Default100。**關閉。** |
| 兩版隔離快照缺自身lock附件 | CONT N3與TECH N2各自行讀archive並有限probe，都確認空[workspace]不能借rootlock、原locked run先因缺lock失敗。FIRST只有BODY相容性／路徑理解，未讀archive或執行。 | 必要交付缺材，保留來源／行為相容概念本身正確。FIRST這項可重現範圍是未驗，不誣稱它測過卻漏錯。root把既有lock交付archive／發布集合。 | v10加兩份鎖定自身1.0.0／1.1.0的lock，原manifest／library／同實體consumer不改；本輪讀／parse／hash而不重跑。**附件缺口關閉；不新增本輪執行PASS宣告。** |
| `test --example`與run main的工作不同 | TECH O1原可選；CONT／FIRST原未報。actual1.4的target selector與1.10的測試執行器，加原「測試另外借讀零與負數」，可以合理接起。 | 可選澄清有收益：命令各負責main或test harness，避免把兩段預期輸出混讀。不是新旗標或未教測試機制，不升必要；CONT原未發現保持。root亦採可選。 | 001-004首次兩命令前明說run執行main，test跑該example單元測試、不執行普通main。一句即夠，後段仍核wrapper／值域。**採用後足夠。** |
| Just／RngSeed短名稱精確import | FIRSTprefix O1原可選；CONT／TECH原看whole source有use而沒有列BODY入口負擔。原BODY已教兩者用途／契約，並有完整fixture可用。 | 支持可選給精確use，方便不開完整fixture而重建當段；不是property、seed或shrinking機制未教。自己在peer提示後同意，不能回算原CONT發現。root採可選、不升必要。 | 005-005在Just與RngSeed的首次使用前列完整兩路徑及use，與原fixture header逐字一致；其後契約與反例保持。**採用後足夠。** |

FIRSTtail原無確認必要／可選仍保留。其CI估計公式未知與擴大成任意巨大repetitions的上界未知，不用同意別人的有限模型通過就抹去；本章讀時間欄位與100次／6400的有限工作不依賴那些擴大域，沒有據此追加必要問題。

## 自行重建的修正範圍與source指紋

- v10 manifest SHA256：`67b328ca646947734332a557cdf8c78d46594644b57836597f4f70946dbe74f6`。實際 **8頁64 BODY、136 sources**，不是照root摘要作結論。
- 自行逐檔對v8的原134 sources：全部與v10 archive **exact byte相同**；新增恰為 `examples/ch14-api/compatibility/v1/Cargo.lock`、`v1_1/Cargo.lock`。68 Rust、16 manifest、20 expected、9 SVG、5 JSON、10文字材料均不變；lock由6增為8。正常helper／tests／答案／derive consumer／negatives、benchmark三報告及profiles全組都有原source，沒有把未列材料預設算在內。
- 全64 units核v10 manifest SHA。raw差異12 units，其中8份額外差異僅凍結image URL的snapshot前綴；規範化此前綴後，實質只有 **001-004／002-004／004-003／005-005**。四份完整BODY都重新實讀，另回讀各自前後units的入口／收束及相鄰重要過渡；其他內容沿原全64 actual閱讀，沒有冒稱本輪把同byte Rust再跑／整章再首讀。
- 九份v10原位assets與其archive SVG逐byte一致、與v8同byte。BODY圖說與SVG labels／箭頭／數字沒有改，只改image定位前綴。
- 自行proof及完整hash保存 `own-original-fixed-byte-proof.json`、`fixed-lock-material-proof.json`、`first-prefix-staged-to-v8-body-proof.json`。root disposition提到未交此角色的中間v9漏profiles，我沒有讀v9或拿該失敗當本人的實測；v10原134全等的自己證據已含這組profiles。

兩份新增lock內容各只有本地 `api-contract-demo`，lock format4，version分別1.0.0／1.1.0；和各自manifest的package／空workspace對上，[[bin]]仍指同`../consumer.rs`。指紋分別：

- v1：`a996b65d5f81f4a90e00222e1a4391511699b4d65d5e772504ab78bd6148069c`。
- v1_1：`822b0084e099c619fa274e3cb7dc71547a1d23835e7768d48b7bd0d70e228841`。

它們是**v10實際交付的既有lock**，不是我本輪產生。與原CONT probe在own copy產生的lock恰好同byte，仍不能把兩種來源／時點混為一份；原probe exit101→own lock→exit0的執行證據維持原scope，不能冒稱本輪跑了新archive。

## 四處first-use與原主需、例子／圖的局部回收

| 局部閱讀路線 | 必要先備與新增說明 | 回應原需、下文與例子支持 |
| --- | --- | --- |
| 001-003→001-004→001-005 | 先ident／同名兩位置，再一層repetition與三型別；新句只接已教example target和測試執行器，不加Cargo新設定或Rust語法。 | 原需重用decl而保Meters80／Seconds30／Millimeters500與ownership。run看main結果、tests另看0／負數；不從newtype推非負驗值，下unit仍分matching失敗／展開後E0308。新操作句不換方法理由，原expansion圖的compile/runtime角色保持。 |
| 002-003→002-004→002-005 | 已教export找到macro不令裸Reading自動找library；新expr句連1.5 expression與2章shared reference，在private macro matcher前給最低契約。 | $crate固定library trait而不能越private；fixed call&distance先是可接受輸入，再因caller無private權限得E0603。下unit保留caller value30／template value80、一次插入／左右求值與hygiene不同責任。圖沒有被提前搬去掩蓋首次matcher缺口。 |
| 004-002→004-003→004-004 | policy預設100與合法1..1000先定；private new、non_exhaustive、Default介面與derive0已教；新增assert message句在負例前，條件／字串／panic皆已有來源。 | 0使非零條件false，message具體標違約；手動100及Options bool false＋合法BatchLimit default接回所有公開建立路徑守invariant。沒有把訊息字串變新資料政策／比較值，沒有改Default或builder本身的工作。 |
| 005-004→005-005→005-006 | 前文已有守恆／逐key oracle、Config與proptest macro；新增兩use在Just固定元素／RngSeed重播設定前。 | 兩筆0覆寫成1仍是負例；固定值不縮、Vec長度可縮，固定seed／禁保存只限negative。手工4→2→1圖對同一「仍然失敗」判準，回歸保[0,0]而一般策略繼續探索；import句沒有更換domain或把手圖冒稱工具trace。 |

## 未報問題的主要通過項交叉覆核

peer們的正面理由需回到原問題，而不只彼此說PASS；以下與我的original表相符，未用TECH執行範圍補成自己執行：

- **macro／derive分工**：Reading普通泛型已完成借讀，重複在宣告；macro產生struct與impl而derive利用caller既有struct資訊，只加impl。名稱／visibility／其他derive、private alias i32及零／負值的例子實際發揮這份分工，不捏造普通函式或原macro無法完成工作的障礙。
- **公開API**：private newtype、手動有效Default、owned builder、typed Error source、pub use與text feature各有局部責任；合法上限與原行資料、same physical consumer／false、source與behavior compatibility能具體判別。lock是交付操作缺口，不能把它升成這些概念設計本身錯誤；同caller結果不包辦任意公有trait／欄位變更。
- **property**：總量與per-key直接掃原輸入是獨立判準；composition的0＝0＋0仍可能通過，不能替代兩個強些的測試。shrink保留失敗、seed重播／具體regression與合法域不同責任；固定±1000不是小域策略的隨機覆蓋。
- **Loom／Miri**：兩次atomic load/store仍可能漏更新，model每輪fresh state與mock操作、RMW＋兩份join的理由先於PASS；Miri只解譯所選輸入／path，safe caller內unsafe仍需actual12契約。TECH fixed-nightly／Loom selected probes是他的追加正面技術證據，非我跑過；FIRSTtail只依BODY和孤立SVG理解，也不是desktop／mobile驗收。兩者都未宣稱全部soundness或全weak-memory model。
- **效能兩步**：純count相同Result／slice／query／case、每行一次與獨立固定答案，Criterion iter／output Drop／setup outside及重複samples範圍明確；兩輪CI／baseline含義不等CPU佔比、單次min-max或全部CLI倍率。dhat scope內100次真實計數與6400、PP來源main18→repeat12→count21→Clone、total／peak／current責任不同，零新增配置不抹原heap。三角色皆能接到原問題，原CONT／TECH讀raw records與FIRST只BODY的驗證邊界不同，不能混爲三次資料復現。

## 修後實際頁面與限定驗證

使用自己的v10 preview：由全64 current BODY按順序重組、只去重H1及重寫同byteasset URL，保留closed fold。複用自有v8已保存的CSS，explicit extra_css；CSS SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`，瀏覽器實際載入一致。`zensical build` strict本輪exit0、No issues found、0.32s。

我親看 **1280×1000桌面／390×844手機**的四個受影響段落，共8張；另親看兩尺寸的原位macro-hygiene與property-shrinking，共4張，**最終12張**。record不是以bbox取代view_image：

- run／test句在兩命令前，手機正常換行仍完整說main與單元測試責任；下文0／負數與matching失敗過渡可讀。初次capture selector太泛選到較早main段，已只重取這兩張，final records明確排除錯位置，不把它算受影響段落驗證。
- expr句在private matcher前，手機可讀expr→expression→&distance，之後matcher／caller／E0603相鄰，再到hygiene；沒有需要展開答案才取得說明。
- assert message句在derive Default負例前，手機完整可見false／panic訊息及同例條件，正常Self100緊接；沒有被header遮住必要文字。
- import句完整列兩個source與use，手機inline code可換行，隨後Just／seed table的介面角色清楚；不需從超長程式碼行猜路徑。普通code blocks仍使用其原水平捲動容器，本輪未逐行測每份長code的交互。
- hygiene圖的綠色側線仍指caller Seconds30，藍template value保Meters80，結果(80,30)非兩份80；shrinking手圖4／2失敗、1通過的判準與保兩項反例標籤均清楚，不需放大。實載SVG SHA與v10 archive吻合，沒有額外必要圖／視覺修正。

own work：`/workspace/work/tutorial-implementation/continuity-ch14-cross-v10/`。`screenshots/final-selected-view-records.json`、`render-local.py`／corrected selector script、current preview與build log保存；先前original九圖19capture不回算成v10全九圖重看。本輪新視覺只指上述12張，不把ROOT preview8858或TECH原22張算本人。

## 未做與收束

本輪**沒有重跑Rust／Cargo tests／negative、bench、Miri／Loom或dhat**，未操作外部DHAT viewer，也沒有native執行故意UB；工具執行僅限資料parse／hash／diff與自有preview build／Chromium覆核。原Rust／SVG bytes不變，不因四段文字與新增archive locks重跑；舊工具結果仍維持原先的scope。

沒有修改學生正文／source、root Cargo／lock／nav、原reviews或任何第12章檔案。root disposition的五項採用與自己的先存peer裁定相符，中間版本／發布allowlist屬root處置，不冒稱我驗過正式發布、dark theme、其他平臺或真人新手。四段最小說明沒有引入新先備負擔，兩份lock補足實際交付；本輪原判保護、交叉與修後local完成，封存source／report SHA後STOP。
