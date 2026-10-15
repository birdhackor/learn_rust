# 第 14.1–14.3 macro 作者依賴帳冊

作者 chapter04_traits；責任限這三頁、macro SVG、examples/ch14-macros 與自己的筆記。Cargo workspace/lock、nav、通用 checker 與 env 由 root 整合。此文件供協調者核對，不給首次閱讀者作提示。

## actual 前置及本輪閱讀界線

- 第 7 章 `ch07-review-fixed-v12`，manifest SHA `9ca4fa7843e75ed836df7b855a2aaa94d377435132ba3847232ca82660fbb187`：親讀 7.9 全 8 units（009-001..008）及 search 的全部 12 份 archived source（含 Cargo、兩 test、完整答案與素材）。不聲稱本輪新讀其餘 54 units。
- 第 8 章 `ch08-complete-prerequisite-v8`，manifest SHA `79036df9490cd5ad5874be7978062928ff185e1d25abc76adf610c2678eae9c1`：親讀 8.9 全 7 units、newtypes.rs 與其 20 份 examples/fixtures/expected。tuple struct、trait 實作、借用 `.0` 與型別邊界是真正來源。
- 第 6 章 `ch06-review-fixed-v10` 的 6.2：先前 review 已讀全稿與 source，本輪 derive 開稿前再回讀必要單位；目前 14.1 不使用 associated type，不把未 fresh 回讀的部分當新增 API 白名單。
- 第 9.2 `ch09-build-script-prerequisite-v2`：本輪已親讀全部 5 units；其 6 份 source 將於 14.3 package/host 對照前再核。本輪 14.1 未使用 build script、TokenStream 或 parser。
- 1–5 章 actual 背景沿用已完成的本角色寫作／技術與銜接閱讀；名稱來源、Debug/PartialEq、trait bound/use、i32、&self、泛型、test/assert 與 snippets 對應具體已教能力，不用未來課綱算先備。

fresh EN/ZH 網路閱讀及當下未讀段落見 `planning/source-notes/ch14-macros-author04.md`，原下載證據在 work/research-ch14-macros-author04，沒有冒稱取得全文就閱讀全文。

## 14.1 首次需求白名單

| 必要能力／規則 | 第一次需要 | 實際先備與本文解釋 | 能解釋／材料界線 |
| --- | --- | --- | --- |
| Reading trait、read_value bound | 開頭 library code | 4.1 的 trait/bound/use、2.5 &self；本文先固定借讀 i32、不相加不同單位的需求 | 共用取值不取消 Meters/Seconds 身分 |
| tuple struct、public i32、`.0` | 手寫兩組 struct/impl | 8.9 actual009-003/005＋newtypes.rs；先說 Copy 欄位但整體不選 Copy | 借讀後 owner 仍在，不由欄位 i32 推出 wrapper Copy |
| Debug/PartialEq/derive/assert | 同一段與 main | actual4.2，本文指回其觀察用途 | 只檢查原值與取值，不驗證所有單位/範圍 |
| macro expansion、token、macro_rules | 首份規則 code 前 | 本文說明編譯時生成語法／結果仍由 Rust 檢查，公平保留普通函式已能做資料工作 | 理由是宣告模板重用，不是假想不定筆數限制 |
| ident／metavariable／matcher→template | single-rule 前 | 本文逐項表＋definition/call 區別；Reference fresh 部分 | 同一名字在 struct/impl 兩位置，不是 runtime 值 |
| 本地名稱前後順序 | 首個呼叫前 | 本文明說先定義再用；未提前要求 macro_export/$crate | 先用本地例，下一節才教跨 crate |
| repetition＋分隔符＋可選尾逗號 | repeat-rule 前 | 本文說 +/*/? 在此處意義、兩段輸入與模板保持同層次；Reference fresh repetition | 三組宣告而非 runtime Vec；*解答的空輸入沒有半份 impl |
| 輸入匹配失敗／展開後 Rust 檢查 | bad-name/wrong-unit code 前 | 本文先區分 ident 不接 literal、+不接空；8.9 E0308 型別身分 | 拒絕無關輸入不借找不到外部 crate 的錯誤代答 |

API／helper／答案自查：14.1 normal package 無依賴，lib 只有已解釋 Reading/read_value。manual_readings、reading_single、reading_many、reading_empty_solution 各自完整 source；解答只將兩個 + 改成已教 *。reading_many 的 cfg(test)/mod/use/test/assert 均來自 actual前文，零、負數與原值再使用驗借讀/不驗證數值的契約。三個 standalone negative 只用當下 macro 語法、tuple struct 與既有型別錯誤；沒有未教 library helper。

不在 freeze 後把 14.2 export/hygiene 或 14.3 parser API 加入這四份 example/答案。Reading/read_value snippet 保持不變；若後節增加 module，另外保存當節責任與 gate。

## 14.1 驗證與尚待完成

- 已執行 cargo fmt、Clippy all-targets/locked 且 -D warnings、cargo test all-targets/locked，1 項 example test 通過；沒有拿其餘零項當成測試覆蓋。
- 四個 normal examples 已實跑；manual/single同樣80/30及原newtype，many新增500，空清單解答維持80/30。三個 negative 逐個由 rustc edition2024 驗預定訊息/代碼，紀錄 `/workspace/work/tutorial-implementation/author-ch14/14-1/negative-checks.json`。
- macro-expansion.svg intrinsic400×720/24文字；使用同份正文、snippets、assets 與專案 theme/extensions 的獨立 Zensical 預覽，已親看 Chromium 桌面1280×1000與手機390×844的實頁截圖。圖分別顯示400×720與約343×617；所有文字／箭頭可讀，框內未裁切，document width未超過viewport。證據在 work/author-ch14/14-1 的 visual-page-checks.json與兩份actual頁PNG；不是 public站點或整書版面驗收。展示 compiler輸入／模板／生成宣告，再回到 runtime借讀，不把三次產生假裝同時存在的runtime物件。最初preview使用absolute docs_dir/site_dir造成Zensical path invariant panic，僅改work配置為relative後建置通過；未改共享配置。
- source/body已寫，仍待 root 實際核對與 freeze；此時不開14.2。14.2/14.3 帳冊將在取得實際 gate與新API閱讀後追加。

root實讀後指出兩個作者漏項，均採最小修正：圖URL由網站root絕對路徑改../assets，避免Pages的/learn_rust子路徑404；reading_many test的super改成已教crate起點。沒有為helper補教新語法。affectedexample的fmt/Clippy -D warnings/test再次通過，獨立頁重建通過；HTML img轉成../../assets路徑，HTTP200，解析到GitHub仍保留/learn_rust/assets。SVG bytes未變，沿用已親看桌面/手機的圖。修訂來源、範圍與指紋保存在work/author-ch14/14-1/root-fix-local-checks.json，不把root發現算作者先前自查命中。

## 14.2 actual gate 與首次需求

Root已凍結14.1為ch14-macro-rules-prerequisite-v3，作者親讀全部6單位，manifest SHA a0bf97a3ce622125803a978dbb0d0c600dc8eb7da703feead2b54eccc7a2e441。Reading/read_value snippet與四份14.1normalexample/答案保持，僅lib追加pub mod hygiene；14.2另存hygiene.rs，不回填前節模板。

| 新能力或轉換 | 第一次需要前的本文說明與真正先備 | 理由與材料界線 |
| --- | --- | --- |
| 普通read_pair_values與private read_internal | 開頭先固定兩份shared來源→i32 tuple，T/U各自bound；private/public、crate use沿1.9/4.1，module之後才顯示snippet | 同任務先可由fn完成；小helper分工為後續crate可見性對照，不假設macro才可處理兩個型別 |
| textual scope／path scope／macro_export | 宏順序負例及export code前先說本地定義位置、public root路徑與caller use | 找macro與找生成程式內的Reading分開；E0405在能找到macro之後才來 |
| $crate與private限制 | exported template前介紹定義crate來源、非caller crate；private-rule前說它不授予權限 | dummy同名Reading仍能用library read_value；E0603隔離private helper，不借依賴缺失失敗代答 |
| expr與two braces block | pair-rule前明說兩個表達式必須交reference；外層template／內層block分工 | 所有示例借既有owner，不需temporary lifetime extension |
| mixed-site hygiene與局部來源 | 同名value需求先有，pair-rule與圖前才解釋template local與插入caller片段上下文 | rootnamespace不是所有名字自動hygienic；caller value仍Seconds、template value是Meters reference；item Meters依舊會重複 |
| expression求值次數／次序 | pair-rule前let／tuple L→R規則；側效觀察前說每block計一次 | 每個metavariable只插一次才保證本模板once；hygiene不保證once。兩tests分別查來源及[1,2]順序 |
| compile_error、tt、fallback／第一匹配 | checked-rule前說標準編譯診斷macro、token-tree形狀、*與先後規則；nested macro展開在code前說明 | numeric input自訂錯誤；先matched ident後E0428不回退。不是runtime panic或matching自身失敗 |
| isolated Cargo負例與locked依賴 | 具體命令前回顧7.1的path相對這份Cargo.toml，新增空workspace隔離與--manifest-path目標選擇；--example、package／crate沿前文 | manifest只有localdep，無未教third-party框架。Cargo.lock已固定；checker由root維護 |

全部14.2helper/tests/答案先備：普通新fn已在開頭給契約，std tuple/Vec的Debug/PartialEq來自6.6/6.7、crate起點/#[cfg(test)]/test/assert已教；沒有super。three examples是macro_export_consumer、macro_hygiene與macro_hygiene_solution，名稱避免泛用consumer碰撞。答案只保存兩個reference再反序回傳，Vec/push觀察仍[1,2]；沒有未教closure/static/parser。

14.2實際驗證：fmt、Clippy all-targets/locked -D warnings、全package tests（本節新增2項、前節1項）通過；三個新normalexamples實跑，結果80/30、caller來源不混、calls2、逆序結果30/80且取得仍[1,2]。四個crosscrate負例逐個locked/offline check命中E0405/E0603/自訂訊息/E0428，另standalone順序負例命中cannot find macro；work/author-ch14/14-2/negative-checks.json保存各命令與stderr。沒有重跑整repo fixtures。SVG macro-hygiene的兩條獨立來源線已實作；實頁視覺正在核對，親看後另追加結果。14.3仍待root凍結14.2actualgate。

Root14.2實讀起初指出path dependency與--manifest-path前置，作者依提示補最小manifest片段、相對基準、空workspace隔離及命令角色。其後Root更正自己的窄範圍搜尋：7.1已明教path dependency及相對成員manifest的基準，並非本頁原稿漏掉的必要新知。本頁真正新增的必要契約是--manifest-path與空[workspace]隔離；現有path片段保留作本fixture定位回顧，不另開Cargo章。此更正屬Root後見查核，不能把已教path算作者初判的缺口；v4正文/source不改、不重凍。

14.2實頁圖已親看：同份正文/專案theme與extensions的Zensical獨立預覽，Chromium桌面1280×1000及手機390×844。圖400×710／約343×609，caller綠框沿右側線到讀右30，template藍框沿中間線到讀左80；全字與箭頭可辨識、無裁切、整頁無水平溢出。HTTP服務在環境重連後重啟，首次拒絕連線未產圖不算驗證；有效PNG與visual-page-checks.json在work/author-ch14/14-2。root補Cargo段在圖後，SVG與已看圖位未變；重新建置核對片段與相對URL，沒有把預覽等同publicPages或整書QA。

## 14.3 開稿依賴與安排

Root14.2 complete precheck的actual gate為ch14-hygiene-prerequisite-v4。作者已重新讀7units；6.2全部8units及associated.rs fresh回讀，9.2 source圖亦已讀。fresh英中網路閱讀與fixed API範圍在專屬source-notes，不用未來outline當API白名單。

| 必要新能力 | 第一次需要／本文先教的位置 | 實際先備與可觀察關係 |
| --- | --- | --- |
| procedural macro／derive新增item | 開頭同樣Reading任務後，先分清已寫struct與產生impl | 4.2derive使用、14.1宣告模板；新能力在編譯期讀宣告，不消耗runtime值 |
| proc-macro crate／entry與host | 新Cargo及source前介紹3個package、[lib]proc-macro=true、另一crate使用、pub root fn簽名 | 1.9 crates、14.2path依賴、9.2 host先編譯；不把macro當consumer runtime函式 |
| fixedfeatures與parser提供能力 | 新manifest前說syn/quote角色及derive/parsing/proc-macro開關／quote預設bridge | 7章第三方固定版與Cargo.lock；features設定本節明說，不默藏full |
| compiler TokenStream／AST／Span | pipeline圖與parse source前說語法片段、AST為依形狀可讀struct/enum、Span指原碼位置 | 3.1/3.2/3.3 shapes、14.1tokens；沒有先要求讀完整AST |
| parse_macro_input(input as DeriveInput) | 解析code前教專用matcher語法、成功AST、失敗返compile_error stream | 14.2compile_error／4.1trait；不誤說as是數字cast，不使用unwrap |
| ident／generics.params／data／fields | 主entry之前逐項有限資料表、借用match與Punctuated is_empty/len | 5章borrowed match、3.3泛型、3.6容器計數；params檢查是選定拒絕規則 |
| syn Error診斷helper | reject首次呼叫前展示其已講輸入與回傳 | Error::new(span,msg)→into_compile_error→.into，5章From/Into先備；不是runtime Result或panic |
| quote!／#name／ToTokens／兩TS型別 | 生成snippet之前先說interpolation與語法產物／Into轉compiler | 4.1trait契約、6.2associatedtype作API閱讀背景；不偷渡quote repetition |
| 絕對外部crate路徑／有限名稱契約 | quote模板首次前說::ch14_macros起點、caller依賴名稱固定 | 14.2名稱來源／$crate僅macro_rules；不暗加rename補救crate |
| shape拒絕 vs payload型別拒絕 | fixture與練習code前列表，回收每個已教階段 | 泛型／namedstruct／enum在前文已教；bool形狀通過後E0308，不假裝parser懂Rust完整型別 |

程式規劃限examples/ch14-macros/derive與derive-consumer兩個新package，root統一Cargo/lock。原14.1/2library/snippets/答案不改。計畫以consumer測試真實展開後借讀、alias欄位仍可、private field保持／owner仍可用，isolated Cargo固定依賴並逐項核shape診斷及bool型別錯誤；不在compiler TokenStream context之外假裝可執行macro entry。SVG只畫Tokens→已選AST→impl→consumer，未視覺驗證前不算完成。

## 14.3 作者自查與實際驗證

本文先教procedure/derive分工與3個package、[lib]proc-macro=true、selected features，再教最低AST欄位／Punctuated／借用match；parse與Span/Error/兩份TokenStream/From轉換都在reject程式前，quote與ToTokens/#name/絕對crate名稱契約都在生成snippet前。流程圖移到已教全部表示之後，避免圖先要求讀者解釋#name或proc_macro2；完整entry再接起每個已有操作。多欄tuple的逗號形式在負例之前作最小介紹，不擴成更多生成支援。

helper、test、答案／config範圍：derive只有7個use、reject與pub root entry；沒有Error::new_spanned、printing、full、AST Debug、custom Parse impl、quote repetition或未教rename工具。consumer2個model、2tests及2normalexamples（包含完整練習答案），private欄位在自己的module使用，alias來自8.9/5.2，From/Into明確引用5.4。5個isolated負例含泛型、enum、named fields、兩欄tuple與bool；前4項逐一對應本例自己的compile_error訊息，bool是Rust E0308，沒有借依賴缺失失敗代答。

- fmt --check、Clippy兩新package all-targets/locked -D warnings、all-targets tests已執行通過。真正測試是consumer新增2項；proc-macro及example的零項不算額外測試覆蓋。兩份normalexamples實跑，輸出80/30及(80,-5)，原wrapper仍可比較。
- 5個isolated Cargo check逐一locked/offline預期失敗並命中expected原因；Error/parser工具本身都正常編譯。cargo tree features在隔離workspace只出現syn derive/parsing/proc-macro，沒有printing/full，排除根workspace其他dependency的feature聯集幫例子偷補能力。實際命令/stderr/hash與feature材料在work/author-ch14/14-3/negative-checks.json及isolated-features.txt；不是全repo重新驗證。
- 用專案相同theme/extensions與完整當頁snippet的獨立Zensical預覽，桌面1280×1000、手機390×844截圖已產生。流程圖400×900／343×771.75，HTTP200、相對URL ../../assets保留Pages子路徑，整頁沒有水平溢出。最後重排後的實頁截圖親看結果將另追加；未把build或尺寸資料代稱實際視覺。root nav與publicPages未改。
- frozen14.2 v4全部29 source及BODY fingerprints逐一與live核對一致；14.3另起nestedpackages，不回填前節source或答案。現已完成BODY/source，待root實讀與後續獨立QA，不能稱全章或全書驗收。

最終重排後的桌面／手機實頁PNG已親看：全部五個框、兩份TokenStream名稱、#name對應Meters、.into箭頭及最後runtime借讀清晰，框線／標籤無裁切，無需點擊放大。圖位在quote與最低API完整介紹後，鄰文解釋顏色／向下箭頭／Rust檢查界線；HTML抽查snippet全展開，reject泛型簽名、quote絕對路徑與manifest精確版完整可見。完整當下20份BODY/SVG/newpackages/source/答案/fixtures/lock指紋保存在work/author-ch14/14-3/final-source-body-fingerprints.json；BODY SHA cff1f88e5ca1b1996cbb2ae74c72c7153ff24c530910469668b8c5fa0d465ec6，SVG SHA ad2dbe412df2915c13f2b4473f2c709705b98c7d9a3237833f2034e560a8fb9d。此為作者自查及局部預覽，不代替root或獨立審閱。
