# CONT12：交叉覆核與v5局部複查

本輪確認原CONT12的4項必要、3項可選，以及peer新增的1項必要、1項可選，都有相稱的v5修正。本人的靜態接口核對與32張修後實頁支持這些修改；未新增必要缺口。GAT遮字已在自己的本地候選配置消除，不能據此宣稱全域設定或發布站已更新。

## 時序、角色與保留的原判

本人未寫第12章，既有第1／8、11.4、14.4／5／7／8作者背景仍在。CONT original正文和initial均保持SHA `1cd8d6348fca32e65aabdf771ec2033a62409b6081249af28ec123d44f032045`，沒有回寫當時unknown或原分級。

先完整互讀TECH12（82行，SHA `18a4a920e1158beed7f355095036b386699dd9381fbf58b1ed755cd564ab0b7d`）和FIRST12（349行，SHA `acebae2a5843d0a64275b88b76e57487bdae506e0d8879b235018f14723e0a7c`），包含其主要PASS、必要／可選、unknown及執行／圖文限度；輸出截斷部分另開原段補讀。peer是不同非作者角色：TECH有較早12.3／4背景，FIRST遮答案／只讀當段BODY與獨立SVG，各限制不能互換。

互讀後先保存 `/workspace/work/ch12-continuity-cross-v5/pre-root-cross.md`（SHA `dfb0432e92e62211354de1149b9e1edd189d37115715a226e12f7eec8649521d`），才讀unique v5。source／實頁核完先存 `fixed-before-root.md`，最後才全文讀root disposition。root的本次派工已透露候選修正範圍與方向，故這是有提示的交叉／修後驗證，不稱新的盲讀或原判獨立重現。

原CONT的N1曾有14.5同類作者經驗，仍明記；新peer項不是原CONT發現。未讀第13章作者材料或第15章，沒有修改學生、全域、凍結或任何original檔案。

## 互讀後自己的判斷

| 原CONT項 | peer對應／原判維持與差異 |
| --- | --- |
| N1 arrayref→slice | TECH／FIRST未列。FIRST003-005、006-004操作可理解，不等於內建arraycoercion最低契約已教；維持必要的小型先備。沒有將未報算同行獨立重現。 |
| N2 sizeof時序 | TECH認C規則先教，FIRST005-002整單位讀後已懂sizeof。原block早於object形式／商的解釋仍存在；維持小型時序負擔，不說整節沒有sizeof。 |
| N3 array traits | peer未列。一般trait、Vec／tuple與bytearrayreference不能自動證owned i32array契約；維持小型必要能力說明。原unknown後查官方才裁定的時序不變。 |
| N4 raw Copy | TECH N1提出同一缺少關係，兩份original封存前各自提出，可算原判獨立重現。FIRST004-005理解來源／唯讀wrapper，未明辨field按值取出依賴pointer Copy；其PASS不解除契約缺口。 |
| O1 bold | peer未列；CONT實頁字面**成立，仍可選標記改善。 |
| O2 Greeting/greeting | peer也能追同源owner，支持可選而非主線阻礙；拼寫對齊有回找source收益。 |
| O3 topchip | TECH的1280×1200／390×1200與FIRST獨立SVG不覆蓋CONT的1280×800位置，無碰撞不能反證該遮擋。仍可選實頁改善。 |

**採納TECH N2，保留CONT漏查。** 提示後重讀v4最後owner test：`assert_eq!(second.as_c_str().to_str(), Ok("Rust"));`。actual5.1 `001-005`要求Result兩側T／E都PartialEq；12.6只教Utf8Error的Debug供expect，沒有其PartialEq。我已全讀source，卻未追最後assert的E能力，原CONT的整體配套PASS因此有漏查；不能回填original為早已發現。新理解是交叉所得，不是本人新的獨立命中。

**採納TECH O1，保留CONT漏查。** 提示後看v4 `005-005`與SVG，原文「兩個箭頭都表示本次走訪範圍」「掃描停在終止零之前」混合了計入長度的四項與仍需有效NUL來判停止。原本先要求有效可讀終止零，所以不升成安全缺口。CONT原圖PASS／FIRST的strlen走4解讀沒有報這個差別；小型圖說修正有具體收益，不改配置／數值模型。

### 主要PASS與unknown的覆核

三角色皆有來源支持的主線：GAT保不同view責任／每call借用，不說普通Vec原fn做不到；variance維持使用端全部契約、寫回可破static；raw三動作與one-past分開；safe split以輸入／mid／range分前提；wrapper依全部唯讀介面分Send／Sync；C pointer+len、NUL、malloc-free、prototype/body分工；FFI三種ownership、同target ABI／layout與checked length。這些理由沒有從新產物倒推原需求，也不以正常結果宣告soundness。

FIRST的入口GAT／variance／unsafe／ABI未知在相應後續BODY釐清，維持合理待教。其cc builder精確receiver／return未列，當下只操作temporary設定鏈，沒有把它猜成按值ownership轉移，故保留該有限PASS。完整aliasing模型、Miri／Loom、callback沒有參與本章必要操作，不為它們新增缺口或用外部資料替學生補理解。

TECH的16命令、22實頁是TECH證據；FIRST零執行／獨立SVG、CONT原85實頁／零Rust-C執行各自保留。互讀不把peer結果變成本人自跑，不提升為完整安全證明。FIRST未讀archive，因此最後隱藏test的E能力不是其BODY角色已覆蓋的項目；不能拿三角色不同可見範圍作同質漏查排名。

## v5來源與修正核對

唯一修稿：`/workspace/work/tutorial-implementation/frozen/ch12-review-fixed-v5`，六頁44 BODY／71來源。自己保存初始124項SHA，對v4逐byte比較，不只照root摘要。

71來源中68exact相同；只變：

- `examples/ch12-ffi/src/lib.rs`最後一個owner test，先expect得str，再assert。
- `docs/assets/ch12/c-string-buffer.svg`的desc與可見文字。
- `docs/assets/ch12/ffi-owner.svg`的value/type標示。

44 BODY原始byte有10項變動；僅規範化v4／v5凍結資產路徑後，實際教材文字只變5項：001-001、003-002、003-005、005-002、005-005，其餘39同內容。沒有拿路徑變動誤報新概念。已實讀這五項全文、006-006圖相鄰正文、完整修後FFIlib與兩SVG；68個同byte來源的完整閱讀沿用v4，不聲稱全部又重讀一輪。

| 項目 | 本人修後證據／是否新增負擔 | 裁定 |
| --- | --- | --- |
| CONT N1 | 003-005首code前交代預期slice位置可由arrayref轉借同元素的slice、長度取array、不複製或延源，並分Deref規則；shared／mutable兩種都說。後caller／tests／答案可沿同規則，不改多份正常code。 | 必要契約已補。 |
| CONT N2 | 005-002把sizeof角色、size_t、wholebytes／itembytes→項數、固定型別不讀值移到第一Cblock前；未擴寫另一課程。 | 時序已補。 |
| CONT N3 | 003-005首wholearray assert前明教本例i32array Debug／PartialEq、等長逐位置比較，沒有推所有容器能力。 | 必要能力已補。 |
| CONT N4／TECH N1 | 003-002正常pointer例之前說兩rawpointer Copy、無T:Copy要求，不是T複製／延源／ownership／free權限，也不能套mutable reference Copy。後sharedself取field可沿已教契約。 | 必要ownership銜接已補。 |
| TECH N2 | lib最後test先expect固定ASCII（006-006已教Utf8Error Debug與ASCII理由）得到&str，只比較已教文字，不再比較Result<E>。兩份配置獨立、drop第一份再讀第二份的目的未改。 | 依賴已縮小，沒有再偷教新errortrait。 |
| TECH O1 | 005-005和SVG上箭頭是計入長度四項，判停止仍讀有效NUL／長度不含它；下箭頭仍按len3、零是資料。原合法終止零與4／5／3／2保留。 | 精確性改善有效，仍可選。 |
| CONT O1 | closing delimiter後空白，入口兩尺寸均正常粗體，無字面**。 | 改善有效。 |
| CONT O2 | 初框 `greeting: Greeting`、清理 `drop(greeting)`，同C配置／借view／獨立String仍是原流程。新label兩尺寸可讀且不出框。 | 改善有效。 |
| CONT O3 | 只在本人localpreview移除navigation.top，GAT末框「讀到3項」兩尺寸完整、沒有浮動chip。 | local候選配置有效，全域／發布待root核。 |

所有C、header、normal unsafe實作、ABI、allocator配對、其他Rust／manifest皆samebyte；沒有為文字／SVG／已驗byte重跑Rust／C／Miri／benchmark。本輪只靜態核修後test；root的一個filtered FFI test及其他五個filtered不是我的執行。

## 自己的修後實頁與預覽差異

工作目錄 `/workspace/work/ch12-continuity-cross-v5/preview` 只由v5的有序44 BODY重組六頁：移除合成重複H1、改相鄰同byteSVG URL，不重排正文／code／折疊。沿自己原preview config，只取消本地 `navigation.top`；沒有修改sharedconfig。build.log實際記No issues found／Build finished。

本輪自查發現v4preview有extra_css設定，卻未複製 `tutorial.css`。舊85張確為本人實看，但不能聲稱全部公開stylesheet已載入；這項限制在cross新增保存，不回改original或圖文判斷。v5另copy通用CSS，唯一規則是details的 `font-size: inherit`。同byte SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`，actualHTTP200且details與article均15px。這是預覽補足，未改教材CSS。

Chromium實際viewport1280×800／390×844，32張全親看：

- GAT入口粗體2。
- raw Copy／arraycoercion／arraytraits／sizeof／strlen說明，共10。
- GAT／Cstring／FFIowner三圖，兩尺寸context＋element，共12。
- C／FFI兩份答案，兩尺寸closed＋open，共8。

三SVGserved bytes逐byte等v5，CSS亦驗HTTP200／SHA；沒有以bbox或檔案存在代實看。GAT同source2→push40→3、a／b不同borrow和順序箭頭保留；Cstring上箭頭四項計入長度、橘NUL另判停止，下buffer三項含零對nonzero2；FFI同C配置→借view→另String→free原配置→只String，greeting／Greeting與Dropvalue對齊。新契約段都可直接讀，在code需要前；泛型／type參數沒有被HTML吞。

C／FFIclosed狀態已給工作／步驟／cleanup／預期值；展開的文字、source及code可讀，無需答案替主文補API。長C答案不是單張全部顯示；其完整source在v4已讀且v5samebyte。手機長code用區塊內水平捲動，wholepage沒有觀察橫向overflow。這次沒有重驗所有未變圖表，不把32張稱第二輪全84張或已發布網站。

## 最後比對root處置

上述互讀判斷與v5實頁核對已保存後，才完整讀 `planning/reviews/ch12-root-disposition-v5.md`。其逐項採用原CONT N1／2／3／4、O1／2／3，TECH N1／2與O1，與我的pre-rootcross／fixed-before-root裁定一致，沒有因root清單改寫原判必要性或計成新獨立發現。

root選expect再比str、最小契約補在code前、只改兩SVG文字，和本輪source差異一致。root對navigation.top明記發布候選方向／共享UI尚待，本人也只確認local候選配置，未用本輪推全域已完成。root聲明定向fmt／一test／Clippy exit0；本人未讀其rawlogs，作協調者報告，不列本人驗證結果。

合併原發現是5項必要、4項可選（rawCopy由TECH／CONT重疊一次）：CONT原報4必要／3可選，TECH原報2必要／1可選，FIRST原未確認缺口。此數量僅交代原發現覆蓋，不作審閱品質唯一排名。CONT新採納的Utf8Error PartialEq及strlen精確性是2項交叉後漏查，不補算original；自己的原預覽CSS未載入是本輪追加限度，也不藏成v4完整stylesheet驗收。

## 未驗範圍、原判保存與封存

本輪沒有Rust／C／全workspace／Miri／benchmark執行，沒有故意UB、invalidpointer、doublefree或ABImismatch hostprobe；未模擬mallocfail、巨大buffer、其他target／compiler／crosscompile。TECH的原有限執行、root本輪filteredtest、FIRST未執行各自分記；正確stdout／tests仍不證任意safe client完整soundness。

沒有新盲讀／基本Python真人測試，也沒有讀FIRST新的local修後判斷；保留所有FIRST原unknown／PASS和可見範圍。自己actual32張是exact v5 localpreview，不是sharedsite／GitHub Pages部署證明。navigation.top的發布設定與root actualsharedUI仍由root另驗。

封存時再核v5初始124項未變、manifest的44BODY／71source符合SHA、三角色original SHA不變；同byte cross initial與SHA sidecar列完整source／preview／32實看PNG／CSS／各階段紀錄。後續處置另存追加，不回填任何original。本輪到此停止。
