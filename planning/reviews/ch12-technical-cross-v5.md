# 第 12 章 TECH12 交叉與局部覆核：v5

本輪採納的最低修法能補足原先備與呈現問題，未發現新增未解必要項。原71來源中68份維持相同 bytes，Rust只改一項FFI測試，兩張SVG只改文字；C、headers、ABI宣告、build.rs及所有unsafe實作保持不變。本人沒有重跑Rust／C或故意UB，只核來源、契約與修後實頁。

## 角色、原判保護與閱讀順序

本人延續非作者TECH12角色，曾撰寫2、4.4–4.5、5、9.2–9.4，並有既有審閱與12.3／12.4背景；不是新blind initial。原報告 `ch12-technical-v4.md` 的必要N1 raw Copy、必要N2 Utf8Error PartialEq、可選O1 strlen措辭維持原分類與 bytes，SHA **`18a4a920e1158beed7f355095036b386699dd9381fbf58b1ed755cd564ab0b7d`**。

授權後本輪完整讀 CONT12 184行、FIRST12 349行；原SHA各為 **`1cd8d6348fca32e65aabdf771ec2033a62409b6081249af28ec123d44f032045`**、**`acebae2a5843d0a64275b88b76e57487bdae506e0d8879b235018f14723e0a7c`**。先把自己的差異／分級、重要PASS／unknown保存為 `work/ch12-technical-cross-v5/pre-root-cross.md`，才核唯一fixed-v5；完成自己的material／契約／18實頁判斷並保存hash後，最後才完整讀 `planning/reviews/ch12-root-disposition-v5.md`。同行原判與本人原判都未回寫，不用root裁定代自己的交叉判斷。

FIRST原判只讀44閉合主文與七個孤立SVG，未讀完整archive或執行；CONT完整44／71及85實頁、正式選段，沒有Rust／C probe；本人原TECH完整44／71、16有限命令／7 selected library tests、22親看畫面。各自證據範圍不互借；本人交叉後同意的項目不稱 original獨立命中。未讀第13章作者或第15章新內容，沒有修改學生／根Cargo／lock／nav或其他原判。

## 原判相同、未命中、分級與unknown

| 問題 | 原角色的判斷與證據 | 本人交叉判斷／分級保留 |
| --- | --- | --- |
| raw pointer Copy | TECH N1、CONT N4各自行必要。FIRST的完整wrapper主線能理解，但未報borrowed self按值取pointer field所需Copy。 | 維持本人原必要，兩份獨立命中不相互合算另一證明。pointer不接ownership不能推Copy；`*mut T`亦不是mutable reference。初讀能懂同源／SendSync不使型別能力已教。 |
| Utf8Error PartialEq | TECH N2自行必要，完整lib最後test用整Result相等；CONT未報，FIRST僅見BODY六tests摘要、未開lib。 | 本人原必要保留。其他角色未列不等獨立重現或驗證該test；固定ASCII的expect後比str縮小能力需求，仍驗一份C配置先清理不影響另一份。 |
| array reference→slice | CONT N1自行必要；TECH original未列，FIRST未將直接arrayref轉換留未知。 | **同行提示後本輪採必要，非本人原獨立發現。** 回讀actual2.6 `006-003`只用 `&prices[..]`，與直接 `&mut values`不同；Vec Deref也不能代array的特定coercion。原TECH未充分核這個輸入轉換契約，合法source不等讀者已學。 |
| numeric array Debug／PartialEq | CONT N3先unknown，再actual／Std核成必要；TECH original未列，FIRST無完整source檢查。 | 提示後採必要，保留CONT自己的unknown→查證時序。actual6.6 `006-004`明說Vec條件能力，不能推出所有容器；i32有trait亦不能自動放行整份array assertion。本人原未命中，不因原有限tests通過回算已教。 |
| sizeof code先於具體說明 | CONT N2自行必要；TECH原未列，FIRST讀完同unit能解公式而未列時序缺口。 | 提示後採小型必要時序改善，不重判整節缺sizeof。原段已給全部意義，搬到code前就符合嚴格先教後用；讀完段落後理解不回填第一次code前已教。 |
| strlen計數／讀終止零 | TECH O1可選；CONT未報，FIRST的孤立圖稱「strlen走4」。原valid NUL契約與4／5／3／2數值皆正確。 | 本人維持原可選。明標上箭頭計入長度而判終點仍讀有效NUL，避免把回傳範圍當全部檢查範圍；未升成unsafe C錯誤。 |
| GAT粗體與FFI值名 | CONT O1／O2可選；TECH原未命中，FIRST沒有整頁或同名cleanup形式查核。 | 交叉提示後採可選。原需要與同配置圖能理解，修Markdown解析、`greeting: Greeting`／`drop(greeting)`只減少回找型別／值的負擔，不重判ownership機制。 |
| 浮動回頂chip | CONT O3可選：1280×800、GAT頂約180、停留2200ms遮末框。原TECH1280×1200未見；FIRST無實頁。 | 這是不同geometry的實證，與原TECH較高viewport不矛盾；不能用可再捲抹掉當時遮擋。本輪自己的local feature-disabled控制另證，維持可選，未宣global／發布配置已改。 |
| FIRST cc精確receiver／alias界線 | FIRST未列必要／可選，保留cc設定method精確receiver／返回型未列及完整alias模型合理後學。 | 不把unknown升必要，也不算所有API已驗。當前temporary鏈只設定至compile，沒有用未知型別推後步；TECH原正式cc selected另核具體契約，不回填FIRST當時已讀外部答案。 |

## 原需要、重要通過項與限度

| 節 | 同原需要的機制與例子支持；本次交叉界線 |
| --- | --- |
| 12.1 | 普通Vec已有view可完成單源需要；GAT解不同視圖型別同trait契約、每call借用與不同guard責任。Self:'a不是延長來源，generic S::View不保索引；ordinary最後使用與Ref guard清理分開，2→push40→3與missing bound／source已結束各回答不同關係。修粗體不改需要，正文不靠選讀補必要規則。 |
| 12.2 | 替換需滿全部介面，不由literal結果證一般規則；外lifetime可縮、可寫T不變、Cell shared set仍能破原容器承諾，fn input／output各看caller可給／可依。另建Cell只複reference到另一容器，不改原Cell variance。三角色理由相符，不要求完整型理論或執行失效讀取。 |
| 12.3 | reference已能修改，raw是後續接口責任的材料；建立、檢查與存取分開，nonnull／地址不能代有效來源／alignment／初始化／權限／provenance。第二項120→140、one-past不讀只示有限用途；本輪補Copy與array介面，原安全理由不靠260輸出。 |
| 12.4 | caller unsafe fn前置與作者safe abstraction分工；mid檢查只負範圍，valid input、同allocation、不重疊及輸出borrow接手其他條件。ReadOnlyI32 private同源／全API唯讀／i32 Sync支持Send與Sync，兩join先等待；六cases不證所有safe clients soundness。新增前文能力讓field取副本／array caller可解釋，unsafe body無變。 |
| 12.5 | 先懂C對方的資料／長度／存取／清理，才接Rust；pointer+len、NUL與一般零元素、唯一malloc/free、prototype／body／object／link各回應實際需要。sizeof bytes÷bytes的商是項數，固定型別不讀值；240／260的獨立配置與原source不變支持真copy，未模擬malloc fail、完整輸出政策或signed-overflow輸入。 |
| 12.6 | safe本地同步wrapper分別借buffer、借Rust-owned CString、接C-owned工廠配置；uint32/u32、repr(C)、checked len不截、named CString保活、ASCII檢查view與配對Cfree各有契約。只有固定工廠五bytes／無保存／無callback，不以heap名稱套Box/CString allocator。新test仍保同兩份配置與原所有責任，UTF8 Debug／expect先備已足。 |

這些PASS有原需→機制→操作／資料與界線，不是因每節有例子或原tests綠燈。FIRST能解局部用途與概念，不能代替技術輪核完整source首次typeability；反之，額外小先備缺口也不抹去已成立的主要需要／safe契約理由。

## 本人固定稿、最小修法與正式契約

唯一fixed為 `/workspace/work/tutorial-implementation/frozen/ch12-review-fixed-v5`，六頁44 BODY／71 sources；manifest SHA **`018ddb72f2de3d26e4428d9ed79a2eeec6240332662ab963f28f141ecd5bdad2`**。本人實際old／fixed逐byte與manifest hash核對，68 sources相同、3改動：FFI lib最後test、c-string-buffer.svg、ffi-owner.svg。原C／headers／build.rs／Cargo／expected與全部unsafe本體無變。44 BODY有10個raw hash變化，正規化snapshot資產根路徑後只五個material；006-006本身只asset URL，但所載owner SVG已改，另核。

本人完整回讀17個受影響／相鄰BODY，並核三source完整diff、改test與相鄰已教expect用法、兩SVG文字／desc及原位置；沒有宣稱重新首讀全44或重跑全71 target。

- `001-001`：closing粗體後加空白，實頁真正strong突出「同trait／每次借self」原問題，沒有字面星號。
- `003-002`：兩種raw pointer Copy、不要求T Copy、只複pointer資訊／不保活／不增owner或free資格、mutable raw不放寬有效性與衝突限制，都在首次正常pointer碼前；後面ReadOnlyI32／Greeting field按值取用可沿此契約。
- `003-005`：arrayref→slice的預期型別／原長度／同元素borrow在第一array code前，區分Deref；i32 array Debug／同長逐位置PartialEq也在首整array assertion所在code前。不新增trait旁支、手算地址或影響source有效性。
- `005-002`：原sizeof段搬code前，size_t、整array／一項byte、商為項數、fixed不讀元素保留；沒有重寫C範圍。
- `005-005`：上箭頭計入長度四項、判停止仍須讀有效NUL，SVG字／desc同步；下arrow仍len3訪三項。原4／5／3／2與存取條件不改。
- FFI lasttest：`let text = second.as_c_str().to_str().expect("C 固定提供 ASCII Rust"); assert_eq!(text, "Rust");`。12.6既教固定ASCII與Utf8Error Debug、5.1 expect及字串比較已有前文；text仍借second，first先drop不影響second，無新error相等能力或owned複製。
- `006-006`新圖：首框 `greeting: Greeting`、清理 `drop(greeting)`與實際example同名；仍C配置→借CStr→另String→pairedfree→只用副本，不多造owner或改allocator責任。

本輪另實讀正式Std1.99 [array](https://doc.rust-lang.org/1.99.0/std/primitive.array.html)的條件Debug及同長array PartialEq impl、Reference1.99 [unsized coercions](https://doc.rust-lang.org/1.99.0/reference/type-coercions.html#unsized-coercions)的 `[T;n]→[T]`，支持最低契約；不把專家所讀const generic impl當學生新先備。raw Copy及C11 strlen/sizeof選段沿本人原已驗材料，不再下載或跑normal例。array初次matcher誤選VecDeque交叉impl，保留原擷取但排除該證據，後來真正取for `[T;N]`的same-length array impl親讀；採用 corrected proof，不拿下載全文算全API已讀。

## 本人的修後實頁與UI控制範圍

由exact-v5 44 BODY按原序重組六頁，只有去重H1／本機asset URL；七SVG使用exact固定source副本；本輪載入的GAT／Cstring／FFI三圖served bytes逐項核source，公開CSS沿原TECH12已授權的本人副本。明確extra_css，preview只在自己的config移除 **navigation.top**，未動global或學生配置；`preview-proof.json`明列config差異與SHA。Zensical build exit0／No issues found，不代正式strict發布驗收。

Chromium自己固定 `127.0.0.1:8792`：1280×800及390×844，六個新／搬動／標記paragraph各兩尺寸12份，兩改字SVG各兩尺寸4份，GAT同原圖feature-disabled停留控制兩尺寸2份，**共18張本人全部view_image親看**。新句可讀且在code之前；兩圖新字無缺字／碰撞，Cstrlen四項計數與有效NUL判定、FFI同名值／型別cleanup可對上相鄰BODY及source。

GAT桌面圖頂約180、停留2300ms後末框「讀到3項」完整可見；手機亦直接讀，同配置沒有浮chip。這是CONT O3所指geometry的本人local控制結果，**不能宣稱global working nav、根網站或第12章release UI已完成**；其餘scroll位置／裝置未驗。原TECH1280×1200沒見遮擋保持其原scope，原七圖／fold22份不當本輪新增驗證，未變四圖沿原source／物件與範圍保留。

首次capture只因raw Markdown有backticks、DOM沒有而guard誤拒sizeof，四partial PNG另存attempt1，不計成功或親看；只修本人的matcher後才完整18圖。沒有學生或網頁缺陷。未全站驗所有表、fold、code橫捲、links、dark mode／發布站；新局部畫面與原完整source閱讀各保不同證據。

## 最後對照處置、執行限度與封存

本人判斷及實頁已先保存後才讀root-v5處置；五項必要與四項可選（本人strlen、CONT三UI／命名／markup）的最低修法一致，保留原分級／未命中。root告知只重跑新owner那項FFI test，**1項0、其他5項filtered**，另fmt／lib/tests Clippy0；這是root的執行聲明，本人沒有讀其raw logs或重跑，不算本人PASS。本人本輪Rust／C／Miri／UB／全workspace／套件測試執行數為0。

原TECH16命令的版本、own lock、selected7tests／三輸出／四負例限制保持，只及原v4材料；不能拿它們冒稱新版完整套件。未模擬malloc fail、巨大buffer、cross-target、任意C library、alias模型或所有safe clients。本輪的source／契約支援這項測試與prose修正；shared發布UI及正式GitHub Pages驗收仍是root後續範圍，不因本輪local preview先結案為已發布。

報告、same-byte initial／SHA及work證據封存，包含peer original SHA、pre-root筆記、68原byte對映／三diff／五material／十raw／71fixed指紋、正式selected與修正擷取、config差異、18親看PNG。再次核TECH12 original18a4a920與peer originals均不變；TECH14 original／cross也未改。完成STOP，等待另行授權。
