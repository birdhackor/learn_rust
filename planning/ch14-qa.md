# 第14章 QA 閉環與發布待辦

第14章四份original、TECH／CONT cross與FIRST四單位LOCAL均已封存並由Root全文讀完，review／cross／local閉環。唯一修後學生稿為 `ch14-review-fixed-v10`。Root已完成受影響四段的8張兩裝置實頁，但**完整候選check、實際nightly Miri CI與發布仍待Root**。本整理者曾寫14.1–14.3 macro與14.6 Loom／Miri部分，本次只協調、引用封存報告與Root proof，不能代獨立QA背書或把Root執行／畫面算成本人的新驗證。

## 封存報告與角色

本輪完整讀以下四份original及三份followup，逐份自行核SHA，沒有改任何原報告、未知、分級、時序或副本。工具分段閱讀覆蓋完整行範圍，不以摘要代替FULL；此閱讀為協調引用，不是第5份original。

| 角色／範圍 | 封存path | SHA256 |
| --- | --- | --- |
| 非作者TECH v8（全64 BODY／134 sources） | `planning/reviews/ch14-technical-v8.md` | `93cfeddf35415a2e942cf0b6b99de7429244f1fc5c485d13d5959416b0313423` |
| 非作者CONT v8（全64 BODY／134 sources） | `planning/reviews/ch14-continuity-v8.md` | `4939fe3132b74673964657b3b0d59cce4a15a302ba5735fea0a2ff5dcc1b675c` |
| FIRST prefix原路線（14.1–5，41 BODY） | `planning/reviews/ch14-first-reading-prefix-v1.md` | `f75674e841d37f1f10e8ac84362a3ab9531cb927610c6a173d3612cca54e2c5f` |
| FIRST tail v8（14.6–8，23 BODY） | `planning/reviews/ch14-first-reading-tail-v1.md` | `2f907328208261369a43c0cef80c1a6dc19d3a2b70c12d779ea47ac48f1d3274` |

| 角色／範圍 | 封存path | SHA256 |
| --- | --- | --- |
| TECH CROSS v10 | `planning/reviews/ch14-technical-cross-v10.md` | `0492aa7658803f362631e91e961cc8c801d87d352e1ced97ef641f1b4cb09963` |
| CONT CROSS v10 | `planning/reviews/ch14-continuity-cross-v10.md` | `eb6da4e117bf575a50be43a551b0a708a8f728309b66088cc571006e66213fff` |
| FIRST LOCAL v10（指定4 BODY） | `planning/reviews/ch14-first-reading-local-v10.md` | `31274244881f8e02261df7acd27a691326647500657f6de87dac0cbc236c730d` |

TECH為非14作者，有2、4.4–4.5、5、9.2–9.4作者及其他審閱背景；CONT為非14作者／第12章作者，有相關前文及技術／銜接審閱背景，兩者都不稱blind新手。FIRST prefix是另一位AI順序讀者：當時實讀1–9與局部補讀，按五份分節gate保存41 BODY，沒有實讀10–13；FIRST tail沿自己的actual1–12及14.1–14.5入口背景，尾段23 BODY才是其獨立原判。tail讀者的41單位入口**不是第二份prefix獨立QA**；不把它的入口未知混為FIRST prefix的發現。

兩份專家original皆先獨立封存，Root FULL讀後開cross。各自先保存peer比較，再自核唯一v10／局部實頁，最後另層讀Root disposition；FOLLOWUP不回填original發現。FIRST LOCAL只讀v10指定四單位、每段先存四問後NEXT，未看同行／Root解答，不是新盲讀或第二次獨立重現。

## 處置、原分級與未驗範圍

| 項目與原出處 | 修後關係／保留界線 |
| --- | --- |
| expr fragment：TECH N1、CONT N1各自原必要 | 002-004首次private matcher前說expr匹配已教Rust expression，&distance就是本例輸入；不搬完整fragment課。FIRST prefix當時一般規則未知、依固定呼叫及E0603理解private，下一段釐清，仍原必要0；不把這份分級差異改成第三次必要發現。 |
| assert第二訊息參數：CONT N2原必要 | 004-003錯Default負例前教條件false時的自訂panic訊息，沿既有bool／panic。TECH及FIRST original沒有報該argument首用；cross後採納不能回算他們獨立發現，Default／invariant主線原通過仍有效。 |
| 兩版隔離compatibility缺自身lock：TECH N2、CONT N3原必要 | 原v8提供空workspace與locked命令卻未附兩lock。兩角色各自copy原材料跑locked101，僅在自己的副本建lock後run0／同callerfalse；不是API相容邏輯錯。v10加入既有兩lock，原manifest／library／same physical consumer不改。FIRST未讀附件或執行，不能把概念通過當locked交付已驗。 |
| test選example與main：TECH O1原可選 | 001-004首對命令前明說run執行main、test --example跑該target單元測試。既有selector／harness背景可推，故不升成新測試機制必要洞；CONT／FIRST原未報保持。 |
| Just／RngSeed精確imports：FIRST prefix O1原可選 | 005-005首用前給完整namespace及use，對原fixture header。原用途／生成／seed／shrinking已懂，缺的是自行建檔的入口定位；TECH／CONT原完整source看過use而未列BODY負擔，後見同意不回填原發現。 |
| FIRST tail原無確認必要／可選，仍有未知 | 信賴區間估計公式、超出有限100次的巨大repetitions上界仍未知；讀欄位及100次實驗不依賴它們，不把unknown全算PASS或強制升必要。tail入口僅背景，原23單位範圍不擴為另一份prefix驗收。 |

修後四段的首次使用、相鄰過渡與用途由兩cross各自實查，原Rust／圖／測量資料不變。TECH完整回讀四material及鄰段共12 BODY、相關完整source；兩個新lock各做 `cargo metadata --locked --offline --no-deps --format-version 1` exit0，僅驗解析，沒有重run／generate-lockfile。CONT實讀四段及相鄰過渡、parse/hash兩lock，沒有重跑Rust；FIRST LOCAL只四BODY／孤立圖，完整sources、兩lock操作不在其範圍。

原及修後視覺分開：TECH original九圖／兩表22張1280×1200／390×1200，cross四段加property圖10張；CONT original九圖含手機derive尾共19張1280×1000／390×844，cross四段加hygiene／shrinking12張。FIRST prefix四張native SVG、tail五張Inkscape390孤立SVG，LOCAL一張同byte shrinking圖；沒有Chromium實頁或Rust。以上均為各角色自述的實見範圍，本整理者沒有重看或操作這些畫面，不能轉成自身執行、全站視覺或真人Python讀者成果。

## 工具、數據與有限證據

TECH original全文64 BODY／134 archives，固定API及original raw資料；有限own工作區有16個命令結果，包含版本／lock準備，不等16學生tests。它實跑derive consumer兩test、bool編譯負例、property固定seed覆寫反例、Loom split反例與RMW模型、兩版compatibility控制、固定nightly Miri 3正常＋1owner答案＋1freed負例。CONT original亦全文64／134及原測量資料，但Rust實跑只兩版compatibility疑點；其他工具／bench／DHAT沒有重跑。兩cross未重跑模型、Miri、benchmark／DHAT或全workspace，不能以Root後續check回填為其執行。

工具主張仍有限：Loom只追接入mock API的小模型、每輪fresh state及選定預設設定，反例1／RMW後2不證全Rust／C11弱記憶體或所有真機；safe façade所呼unsafe的完整來源／範圍／借用契約仍須審，Miri只檢所選輸入／path，沒有報UB不證全部soundness、FFI或任意平台。故意freed fixture只允Miri解譯，**NEVER host native run／test**，保持隔離workspace及cfg guard，不能為追crash放進正常alltargets。

Criterion原兩輪只支持相同合成64／256短行、相同count答案及計時邊界下borrowed中心估計較小；CI不是單call min/max、baseline是同id歷史、共享CPU／outlier與環境限制保留。沒有數據把時間差拆成配置、byte複製、free各占多少，更不支持整CLI倍率／所有輸入。DHAT三組JSON及紀錄只支持原profile scope／同工作量、clone累計25600blocks／326400bytes／peak15／end0，borrow零新增、64行答案6400／81600／15／0；零新增不等原heap不存在，PP量不等CPU比例。專家核原reports／frames／source hash，不代表重現全部historical rootlock／工具環境或親自操作外部DHAT viewer；本協調亦未重跑測量。

Root另外實際跑一次Miri檢查script，原raw `/workspace/work/tutorial-implementation/root-miri-script-check.log`（SHA `1a2bad8b574efd9fd13d9dc16ad4bb8276980092a77d9aea00126fc0dfce57b9`）顯示3個miri_contracts及1個miri_owner_solution正常通過，script末行確認故意UB命中指定freed allocation。這是Root的selected原結果，不是本整理者或cross的新執行，亦不等GitHub nightly Miri CI成功。保留raw，不為文字修正再次執行。

## 唯一v10、Root範圍與實頁proof

唯一 whole `/workspace/work/tutorial-implementation/frozen/ch14-review-fixed-v10`：8頁64 BODY、136 sources、9 SVG；manifest SHA `67b328ca646947734332a557cdf8c78d46594644b57836597f4f70946dbe74f6`。本整理只核manifest計數／SHA並引用兩cross及Root逐byte證據：原134來源exact，包含68 Rust、16 TOML、20 expected、9 SVG、5 JSON、10文字材料與6原lock；新增只有compatibility v1／v1_1兩lock，lock總數8。新lock SHA分別 `a996b65d5f81f4a90e00222e1a4391511699b4d65d5e772504ab78bd6148069c`、`822b0084e099c619fa274e3cb7dc71547a1d23835e7768d48b7bd0d70e228841`。

Root `/workspace/work/tutorial-implementation/ch14-minimal-fix-scope.json`（SHA `22e9c509122a9c73e9a0589f9d8f4c2c051c3938ca40bac4c2261382c3ff602c`）列四material BODY：001-004／002-004／004-003／005-005，其餘八個raw差異只是凍結asset絕對目錄；沒有Rust／測量／九圖內容變更。中間v9曾漏10份profile附件，Root source集合assert攔下，未交審閱者；**v9不是QA快照或release輸入**。v10補完整old134＋2locks，兩專家用自身proof核這一完整集合。

Root `/workspace/work/tutorial-implementation/root14-minimal-fix-views-proof.json`（SHA `005fde1eaa701cf742899ed0820bd48bf3c8766778ea6d27327d179af559b124`）記8頁preview／136 current source exact、Root親看受影響四段8畫面，1280×844／390×844；新句在code／命令前可讀，無整頁overflow，長code在區塊內橫捲。proof也保留預覽TOML／docs_dir恢復失敗及最終strict build0，不拿早期失敗當PASS；未聲稱所有表格method都不斷行或全站完整視覺。這些是Root證據，本整理者只實際讀引用檔，沒有自稱親看其PNG。

`ch14-root-disposition-v10.md` 文末「cross/local／實頁待完成」是當時狀態，原文不改。本整理追加目前四original＋三followup閉環及Root局部實頁完成的狀態；正式候選、CI與公開確認仍待處理。

## 發布待辦與signoffs

- Root FULL核本協調QA及signoffs，確認版本／封存report paths與SHA。
- **第13章完成publish後**，Root才stage第14章候選及CI／script；本整理不stage、不執行候選工具，不改學生／root Cargo／nav／lock／workflow／Git index。
- Root完成固定v10候選全套check、學生byte／依賴／原判hash保護，實際確認固定nightly Miri CI與Pages發布，另追加commit／Actions／公開HTTP／SVG驗證。現在沒有上述完整候選或遠端成功的證據。

`/workspace/work/tutorial-implementation/release14-signoffs.json` 的 `closed: true` **只指review／cross／local閉環**；四original、三followup逐hash核對，candidate／nightly Miri CI／publication保持pending。此整理不讀第15章新稿或第13章peers；完成後停止。

## Root正式候選完整驗證

Root已全文核全部original／followup與本彙整。release14-candidate只納入已審1–14章98小節＋首頁，本章144學生檔working／INDEX／candidate均精確符合v10，七封存report SHA保持。Cargo只新增八個已審members與候選自己的lock，不含15；Miri workflow／script及操作文件一併納入。

首次完整check退出101，raw保留release14-check-attempt-enospc.log：磁碟ENOSPC伴linker SIGBUS。僅刪完整檢查已結束的共用generated debug快取，保全source、測量profiles、rawlog與screenshots，不碰作者工作target；清理proof另存。重做本地完整驗證將dev／test除錯符號設0、關incremental，opt-level與debug_assertions保持；未更改教材profile、原測量資料或CI普通profile。完整check退出0：strict、fmt、all-target Clippy、all-target及doc tests通過，442個test target結果／261項tests零失敗，158指定negative、C正常3與指定compile-fail1均通過。普通benchmark target僅Cargo test驗收模式，不重作原Criterion／DHAT測量。

候選另真執行固定nightly-2026-10-01的check-miri.sh退出0：實際setup sysroot後，3個miri_contracts與1個owner答案通過，隔離故意UB命中指定freed allocation。正常host未執行故意UB；raw為release14-miri-check.log，Miri PASS只支持選定輸入路徑。遠端CI也新增實際安裝固定miri／rust-src及執行此script；公開部署／遠端成功仍待確認。
