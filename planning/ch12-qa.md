# 第12章 QA 閉環與發布待辦

三份original、兩份cross與FIRST指定六單位LOCAL已封存。**Root指定實頁、共享navigation.top取消、候選完整C／Rust check及公開部署驗證均已完成**；commit、Actions與86頁HTTP證據見文末。以下保留各角色封存當時的待辦與限制，不將後續驗證回填成原判已完成。本文件是協調彙整，不算新獨立QA，也不修改任何original。

## 封存報告

三份original共用唯一 `ch12-complete-prerequisite-v4`，六頁44 BODY／71來源，但可見路線不同。FIRST是AI輔助順序讀者模擬，答案維持閉合，只看當時BODY和遇到的孤立SVG；TECH／CONT未寫第12章，卻有各自已聲明的其他章作者／工具背景。TECH另有較早12.3／12.4背景，CONT的arraycoercion曾在14.5作者工作遇同類問題；不稱完全零提示的新手blind讀。

| 角色／範圍 | 封存path | SHA256 |
| --- | --- | --- |
| FIRST whole（六頁44單位） | `planning/reviews/ch12-first-reading-v1.md` | `acebae2a5843d0a64275b88b76e57487bdae506e0d8879b235018f14723e0a7c` |
| TECH original v4（44 BODY／71 sources） | `planning/reviews/ch12-technical-v4.md` | `18a4a920e1158beed7f355095036b386699dd9381fbf58b1ed755cd564ab0b7d` |
| CONT original v4（44 BODY／71 sources） | `planning/reviews/ch12-continuity-v4.md` | `1cd8d6348fca32e65aabdf771ec2033a62409b6081249af28ec123d44f032045` |

followup保留後見時點，不補算original獨立發現：

| 角色／範圍 | 封存path | SHA256 |
| --- | --- | --- |
| TECH cross v5 | `planning/reviews/ch12-technical-cross-v5.md` | `deb9b7a9cf0e67f98f1762612f6ab6a7f2fba409744f29cdfa7e1949ff807d7a` |
| CONT cross v5 | `planning/reviews/ch12-continuity-cross-v5.md` | `6884d59a01ba4da862964a716029c88051ba7d9735b30fe6360f8fb438cc6489` |
| FIRST LOCAL v5（六單位） | `planning/reviews/ch12-first-reading-local-v5.md` | `8b3f0136e17cbd29f73e36a98e925e4e3cbe0e68fff4449ff9da00796de8c36d` |

original各自先封存，才授權互看／修後。TECH／CONT先全文互讀原判，保存自己的pre-rootcross，再核unique v5／受影響實頁，最後才另層讀root處置。root派工已透露候選修改方向，cross明記有提示，不稱新blind重現。FIRST LOCAL仍逐CURRENT過濾答案／選讀，保存四問、Q2a機制／Q2b原需要、原句／unknown才NEXT，只讀指定六單位，不看peer／root處置／archive；不是新whole original。

## 九個合併項目與目前處置

依原缺少關係合併，不因兩角色報raw Copy就重複計數：5項必要、4項可選。其中8項正文／source／SVG／markup修正已由cross／local確認；第9項topchip只確認本地候選控制，root共享／發布設定仍待完成。採用方向和發布完成是不同狀態。

| 項目與原發現／分級 | 最小修正、修後證據與保留界線 |
| --- | --- |
| raw pointer Copy：TECH N1／CONT N4原必要 | 12.3首次正常code前明教兩rawpointer值Copy、無T:Copy要求；複資訊不複referent、不延源、不增ownership／free權限，不推mutable reference亦Copy。後wrapper從borrowed self按值取field可接前文。兩份原判是同一關係的獨立發現，非兩項修正。 |
| arrayref→slice：CONT N1原必要 | 12.3第一array caller前教預期slice型別處可轉shared／mutable arrayref、借同元素、長度來自array，與Derefcoercion分清；保留normal／tests／答案code。TECH原未報，cross採納後不回填命中。 |
| numeric array Debug／PartialEq：CONT N3先unknown後原必要 | 第一wholearray assert前教本例i32array能力、等長逐位置比較、Debug列元素，不由i32能力概括所有容器。CONT unknown→actual／官方核對的時序保留；TECH原未報，tests能過不等已教。 |
| sizeof時序：CONT N2原必要 | 將既有object形式／size_t／wholebytes除itembytes→項數／固定型別不讀值的說明移到第一Cblock前。原段後面已解釋，不能說整節沒教；FIRST讀完同單位理解仍不回填第一次code前已教。TECH原未列，cross採小型時序補強。 |
| Utf8Error PartialEq：TECH N2原必要 | 最後owner test改先expect固定ASCII取得&str，再比文字；不再以整Result默用E的PartialEq。Debug／expect／字串比較有實際前文，仍驗第一份free不影響第二份。CONT原全source閱讀卻漏最後assert的E能力，cross採納保留漏查；FIRST未讀archive，此test不在其BODY可見範圍。 |
| strlen計長／判停止：TECH O1原可選 | 正文及SVG明標上箭頭是計入長度四項，判停止仍須讀有效NUL，長度不計零；下箭頭仍len3走訪含零buffer。原validNUL契約與4／5／3／2皆正確，未升成C安全錯誤。CONT原未報、FIRST原圖讀成strlen走4，後見理解不補算original。 |
| GAT粗體：CONT O1原可選 | closing delimiter後空白，兩尺寸實頁真正突出同trait／每次self借用的原問題，無字面**。不改教學需求，TECH原未報。 |
| FFI圖value／type：CONT O2原可選 | 初框greeting: Greeting、cleanup drop(greeting)對齊source，保同C配置／借view／獨立String／配對free。原圖可追同owner，修拼寫只降低回找負擔，不升成ownership機制缺漏。 |
| GAT浮動回頂chip：CONT O3原可選 | CONT1280×800原位停留2200ms仍遮末框；TECH原1280×1200未見、FIRST孤立SVG不構成反證。TECH18／CONT32修後畫面在各自local config移除navigation.top後末框完整，僅本地控制結果。root處置採發布時取消的方向；共享設定／root actual／release仍pending。 |

三角色主要PASS保留：GAT保不同結果型別和每call借用，不說Vec普通view做不到；variance依使用端全部契約、寫回可破static，function pointer輸入／輸出分方向；raw建立／檢查／存取分、nonnull／地址不代有效來源，one-past只形成不讀；safe split由輸入／mid／不重疊range分責任、唯讀wrapper以全接口分Send／Sync；C pointer+len、NUL／buffer零、malloc唯一free、prototype／body／object／linker分；FFI三種ownership、同targetABI／repr(C)／checkedlen與配對cleanup各有目的和契約。小型先備缺口不抹去這些已成立理由；正常輸出／有限tests不證完整soundness。

FIRST各入口未知在相應後文釐清，原cc設定method精確receiver／return未列、完整raw aliasing模型和後續工具範圍仍按當時限度。temporary設定鏈沒有用未知簽名推owned轉移，兩cross未把此unknown升必要。FIRST original未確認必要／可選問題，分級及恢復時點保留，不用cross反寫其原判為已抓到新契約。

## 各角色證據、漏查與未驗範圍

- FIRST original44正文／七遇到的孤立SVG，未開答案、完整archive／source或外部解答，沒有Rust／C／實頁。LOCAL只讀001-001、003-002、003-005、005-002、005-005、006-006六BODY，三圖於CURRENT才讀／孤立390view；無新增確認必要／可選，不稱網站或完整source驗收。
- TECH original完整44／71，16有限命令包括1項GAT＋6項FFI library tests、C正常／獨立配置輸出、所選compile／link負例；22張1280×1200／390×1200圖／closed-open答案。不是所有normaltarget／全負例／C checks／全workspace。cross核17受影響／相鄰BODY、三source差異與18張1280×800／390×844實頁，沒有新Rust／C執行；原未報arraycoercion、arraytraits、sizeof時序和CONT呈現問題的時點不回填。
- CONT original完整44／71、85張1280×800／390×844實看，零Rust／C probes。cross核五material／相鄰FFI圖／完整改lib、32張文字／三圖context-element／C-FFI closed-open答案；漏Utf8Error PartialEq及strlen精確性在cross追加，未算original命中。

**CONT原CSS限制追加披露，不改舊bytes。** 原v4preview雖列extra_css，未copy tutorial.css，85張仍是真實所見，不能叫全部公開stylesheet實載。cross補copy同byte通用CSS（唯一details font-size:inherit）、HTTP200與15px字級，再親看兩份closed／open答案；同時僅在local取消navigation.top。這不使原85張事後變成完整style驗收，不宣global UI已改。TECH各自使用已授權CSS副本；FIRST只看SVG自身style。

兩cross沒有新blind或真人學習實驗，未驗全站每個表／fold／scroll位置／darkmode／GitHub Pages，也未本輪重跑Rust／C／Miri／benchmark或host故意UB。無invalidpointer／doublefree／ABImismatch危險probe；mallocfail、巨大buffer、其他target／compiler／crosscompile、任意C library或所有safe client的soundness未驗。不互借角色執行或以有限結果擴大結論。

## 固定學生版本與定向執行

唯一修後來源 `/workspace/work/tutorial-implementation/frozen/ch12-review-fixed-v5`，六頁44 BODY／71來源、7 SVG。manifest SHA `018ddb72f2de3d26e4428d9ed79a2eeec6240332662ab963f28f141ecd5bdad2`。兩cross自核v4→v5為68來源exact；只變一份FFI lib末test、c-string-buffer.svg及ffi-owner.svg文字／desc。正常C／headers／ABI宣告／build.rs／Cargo／expected和所有unsafe本體不變。

44 BODY有10個rawhash變動；僅正規化凍結資產根路徑後，material只五項001-001／003-002／003-005／005-002／005-005，其餘39同內容。006-006BODY只資產URL換根，但載入的新owner SVG另核，不能因正文同內容略過圖。v5 lib SHA `0739016349249bf5ffe4e008a99a7d8606b572a66e19a12b4faba301c3a5ff2a`；Cstring SVG `da375b3c849136eff7ae922089b1251b1273ef0c47e11a3970367dca02030627`；FFIowner SVG `e493ca38f3061d00e643a0a216fa79e116288eb5e64f17169c985a184523d300`。

root執行依 `planning/reviews/ch12-root-disposition-v5.md`（目前SHA `8f4d2e26e474de7b459e61c36546e362001df01195554f2408740a7feb1ebd64`）：定向fmt、改owner那一項FFI test（1項exit0、其餘5項filtered）、lib／tests Clippy exit0。這是root聲明，兩cross未讀rawlogs也未自跑，不算彙整者新PASS。原TECH的六FFI tests只適用v4；未把它們冒稱v5全套。C與候選完整check、root sharedUI與live仍待實際執行。

CONT cross封存時引用root處置的SHA `9bac0a52c3b3e1b0a8f49568fcba9f2ef827c9559f5c7ce72413e0dd974d7746`；現在「處／長度」正體校準後指紋不同，逐項處置與pending狀態相同。本彙整用當前指紋，不回改cross的當時引用或bytes。

root處置文尾的cross／local尚待是當時狀態，原文不改。本彙整追加如今review／cross／local閉環；root修後實頁及發布工作仍pending。FIRST local只覆指定六BODY，不替末test或取消globaltop作執行／UIsignoff。

## 下一步

- Root全文核六份封存報告、本彙整與signoffs JSON，完成自己的修後實頁／navigation.top共享發布配置核對。
- Root建立與v5學生byte相符的發布候選，做C及候選完整check、source／依賴／原判hash核對；本任務未staging或執行候選工具。
- 候選通過後依使用者既有授權發布，另記Actions、公開HTTP／SVG與live驗證。現在無第12章候選全套PASS、發布commit、Actions成功或公開頁面證據。

`/workspace/work/tutorial-implementation/release12-signoffs.json` 的 `closed: true` **只指review／cross／local閉環**；root實頁、共享UI、候選、C完整check、發布及live欄位均pending。六份original／followup以相對repo path與SHA保存，snapshot及root處置另保hash。本整理只是新planning檔與work JSON，不變學生、staging、original、全域或其他章scope。

## Root後續局部實頁及共享UI（2026-10-14）

Root已全文讀三original、三followup及本彙整。global working設定已移除navigation.top，11候選captured後才改，未影響已提交11版。自己current六頁preview strict退出0，同byteCSS實載與三指定SVG HTTP200/hash精確符合v5。Root親看兩尺寸共14處文字／GAT停留畫面，以及六張GAT／Cstring／FFIowner context加一張桌面FFI full element，合計21張指定實頁。新契約均在所用code前，markup粗體正確；兩圖標籤完整、無碰撞。GAT頂180左右停2300ms，桌面1280×800與手機390×844最後讀到3項仍完整，DOM top component為0。長code保持自己的橫捲，不是頁面溢出。桌面FFI圖footer不在context屏內，另讀full element才確認末框；未把局部叫whole site／所有scroll位置驗收。capture第一次matcher逐位置不存在，改為原文字每個位置；這是工具定位失敗，未改教材。

root12-minimal-fix-visual-proof.json／兩份views保存scope和served SHA。C與候選完整check／公開UI仍待actual執行。

## 完整發布候選驗證

release12-candidate僅1–12章85小節＋首頁；本章77學生檔current／INDEX／candidate精確符合v5，六report SHA保護。C checker納入check.sh，Miri CI／script未提前納入，Cargo只已審members／候選獨立lock，nav無navigation.top。bash scripts/check.sh退出0：strict／fmt／all-target Clippy／all-target與doc tests、134指定negative、3正常C11與1指定C編譯失敗通過。381個test target結果，213個通過tests、零失敗。指定C checker只執行有效材料，無host intentional UB。候選使用前章已生成target快取節省空間，來源與lock獨立；不把快取代檢查。公開部署仍待完成。

## 公開部署與UI確認

已提交並push 564916103b28f40c503cf34294b7065744c198a0。Actions [38656033726](https://github.com/birdhackor/learn_rust/actions/runs/38656033726) completed success。Root實際首頁＋85小節共86頁200/H1精確，7 SVG逐SHA符合v5，未釋出的13.1為404。公開GAT另以1280×800／390×844實際圖頂180停2300ms、親看兩張，末框讀到3項均完整，top component0且無整頁overflow，故共享取消navigation.top已完成公開確認；不是用localpreview代發布。逐頁proof／原位UI在release12-live-http.json、release12-live-gat.json。
