# 14.4／14.5 作者先備與首次API紀錄

使用clear-tutorial與Rust術語指引，基本Python起點。先備核對、作者執行與獨立release QA各自記錄，不修改原review。

## 14.4 寫作前安排

| 必要內容 | 第一次需要處 | 真實先備 | 本節先行解釋 | 可觀察的結果 |
| --- | --- | --- | --- | --- |
| 公有API的輸入／輸出／行為契約 | BatchLimit／Options／搜尋入口 | 7.9契約與分層、8.9newtype | 開頭材料、表格 | 同一caller明白零結果、上限、空query與錯誤各表示什麼 |
| 私有newtype不變條件 | BatchLimit::new | 5.4驗證建構、8.9私有wrapper | 型別定義前 | 任一可取得的BatchLimit在1..=1000，getter只借讀 |
| non_exhaustive enum契約 | LimitError／SearchError | 3.1match、8.6Error | 第一個attribute前 | 外部known variant仍可建構，分類須保留未來分支 |
| Default:Sized／default()->Self | BatchLimit與Options預設 | 4.1trait、8.1Sized、2.5associated fn | Default source前 | 手動預設100有效，derive usize欄位產生0的負向示範 |
| consuming設定method | Options with_limit／with_ignore_case | 2.2move、2.5self | Options source前 | 接手設定、修改指定欄位、交回新版設定，不要求mutable caller |
| typed public Error與source | SearchError | actual8.6完整Error、7.9io分類 | Error source前 | EmptyQuery無cause、Read借保留io原因，不靠文字判斷 |
| 純搜尋／文件入口 | search／read_matches | actual7.9lines／enumerate／contains／ASCII／fs、7.5Path | 函式 source前 | owned結果保留原行序與文字，空query先拒絕 |
| pub use重導出／private module | library root | 1.9mod／pub/use | lib.rs首次展示前 | caller穩定走crate root，不能依賴內部module路徑 |
| feature公開能力／default | text報告與manifest | actual9.1cfg／feature合併 | 第一次manifest前回收 | 關text只關可選函式，搜尋答案與Options預設保留 |
| 來源相容／行為相容／SemVer | 雙版本與consumer | 7.1版本、以上API | 快照與版本表前 | 私有欄位增加仍接受同caller，預設改變仍可能破壞需求 |
| 隔離Cargo workspace／explicit bin path | 版本快照manifest／crosscrate fixture | 1.3workspace、1.9targets | manifest與--manifest-path前 | 同consumer.rs在兩個真正library版本建置，不以module模擬crate |
| package list／allow-dirty／verify | 發布前本機檢查 | 7.2doc/tests、9.1feature matrix | 新commands前 | 檔案清單及解包建置成功，各自不等於已上傳或相容性證明 |

14.5待14.4完成後另記實際先備、重新閱讀與選定proptest API；目前不先寫其source。

## 14.4 實際完成與驗證

上表依序落在`14-04-api-compatibility.md`。新增Default／derive Default、non_exhaustive、pub use、render_text功能角色、隔離workspace／literal package／[[bin]]與--manifest-path、package新旗標都先教契約再展示source。未使用尚未交付14.1–3的macro能力。

- BatchLimit所有對外有效入口維持1到1000，手動Default=100；Options可以derive，因bool=false與BatchLimit自身default都符合約定。首次Clippy提醒手動Options Default可derive，改用已在正文先教的derive，不添加allow或其他未知API。
- 正常package fmt／alltargets allfeatures clippy -D warnings通過。一般與allfeatures有7個integration tests＋1 doctest，no-default有6個integration tests＋1 doctest通過；0項library單元測試不替代外部caller測試。
- consumer／typed_errors／完整答案與consumer的no-default、明選text輸出實跑。兩個真正library版本都編譯同一實體consumer.rs並執行相同assert與固定false輸出；consumer與兩份library SHA皆保存。兩個快照亦alltargets clippy -D warnings通過。
- 跨cratefixture自有空workspace，不進root members或一般alltargets：`private_limit`期望E0423、`non_exhaustive_match`期望E0004、`private_module`期望E0603，均實際cargo check精確失敗；各marker在examples/foo.expected。derived_default standalone runtime-fail先編譯成功，再因「預設必須符合非零規則」非零結束。
- 相應正常stdout／故意失敗stderr／commands與source SHA保存`/workspace/work/ch14-api-verification-v1.json`。root另負責把Cargo-based負例接既有checker，本作者未修改共享腳本。
- cargo doc與本機package解包建置通過。實際--list仍要求未提交變更處理，正文兩條package命令皆先教並加--allow-dirty；清單18files，排除三個nested packages但保留外層共用consumer.rs。metadata缺description／license警告保留，此publish=false教學package沒有上傳，也沒有替專案另選授權。
- 隔離preview沿repo的同一theme／extensions，strict build通過。親看1280×800與390×844的契約表、Default、Options、Error、重導出、跨crate負例、SemVer表與打包段落原位截圖；皆可讀，頁寬沒有溢出，手機code用內部水平捲動。沒有必需圖像關係因此未另畫SVG。
- 圖文證據在`/workspace/work/ch14-api-preview/visuals/manifest.json`及postfix-manifest.json。首次桌面Default截圖在scroll後尚未完成重繪，另等畫面完成重新捕捉親看，不把初始空白圖當通過；pub use前補render_text最低契約也另留修後兩device截圖。舊截圖保留，不改為修後證據。
- 正體字與angle型別inlinecode已自查，以上是作者檢查與受查畫面，不聲稱獨立release QA。14.4交root先備核對，14.5接著實讀必要前文與正式proptest契約後再開寫。

### 14.4 root先備核對後最小修正

root實讀後指出2.5尚未教mut self，所以在Options source前明教：self仍接手owned值，mut只讓函式內binding可修改，與mutable reference receiver不同。lib.rs未引用的//!換普通//，不另引入crate doc comment入口。Rust API／行為與已跑測試不變；原受查畫面保留，修後局部另建置供root凍結。

## 14.5 寫作前安排

| 必要內容 | 首次需要處 | 實際先備 | 本文預定先教 | 可預測結果 |
| --- | --- | --- | --- | --- |
| property／oracle與生成域 | 計數規則與性質 | actual4.3計數、7.9驗收、7.8fixture | 開頭固定材料與契約 | 不是照抄同algorithm，總數與逐鍵精確度不同 |
| 小整數鍵／get同型別borrow | frequencies／count_of | actual4.3Eq/Hash/get/get_mut、2章Copy | normal source前說改表示目的 | 原清單保留，owned map保存次數 |
| HashMap values與total_count | 守恆helper | 6章iterator／for、actual4.3shared map | helper source前簽名與Item | 只借數值，加總不受順序影響 |
| strategy／range／collection vec | 第一property參數 | 1.7range、3.6Vec | 第一次macro前操作表 | Vec數值−3..3、長0..31，策略不是實際資料 |
| devdep固定版／std feature | proptest manifest | 7.1dependency版本、9.1features | manifest前 | production library不依賴測試crate，選std即可 |
| proptest! in與test config | 首個test block | 1.10#[test]／macro用法、1.6函式 | 首段macro source前 | macro生成真實tests，參數來自strategy而非一般函式argument |
| Config with_cases／cases | 64case設定 | 2.5associated fn | helper/config使用前 | Cargo的2tests和每項64successful cases分開 |
| prop_assert／prop_assert_eq | property body | 1.10assert、5.3format | 首個assert前 | 交回此case失敗供runner縮小，選Copy數字避免隱藏move |
| Just／shrinking／Fixedseed／None保存 | 故意失敗fixture | 前段策略、3.1enum／Option、14.4隔離workspace | bug source與config前 | 重複0域，仍失敗才縮短，得到[0,0]，不進normal redtests |
| regression／failure persistence限制 | 固定反例與新抽樣 | 1.10test、前段seed角色 | fixture診斷後 | 保存具體反例與seed，策略改動不讓反例消失 |
| 合併性質練習 | composition完整答案 | 3.6Vec push／as_slice、上述counts | 題目前規則 | 每鍵合併次數等於兩份之和，不新增API |

不使用prop_assume/filter、regex、Arbitrary derive或低階ValueTree methods；沒有選它們不能算教過。SVG若畫，先固定判準與手工示意角色。

## 14.5 實際完成與驗證

正文完成於 `docs/chapters/14-05-property-testing.md`，真實程式在 `examples/ch14-property/`。從4.3分組計數接到小整數材料，先定義正數群組／總數守恆與逐鍵次數的不同角色，再教生成域、strategy、選定macro／Config與shrinking。逐鍵oracle直接掃原輸入，不再建立另一份相同HashMap更新法；合併性質的完整答案也明說單獨的可加性會漏掉永遠回0的錯誤。

- proptest實際解析為1.12.0，測試dependency關閉default features，只要求std。正常package的fmt、alltargets clippy `-D warnings`、6項tests與counts預期stdout通過；兩項基礎property與一項練習property各設定64份成功新case目標，不把case數當作不同輸入數或完整域證明。
- 故意覆寫次數的fixture依root約定放 `fixtures/cargo-test-fail/`，自有空workspace／Cargo.lock／`.expected`。其fmt、alltargets clippy與指定test的 `--no-run` 通過；真正執行 `--test overwritten_count --locked` 後exit101，診斷為左1右2，`minimal failing input`精確為兩筆0。source與正常tests隔離，未把刻意失敗加進一般alltargets。
- 首次作者檢查的commands、stdout／stderr、source SHA保存 `/workspace/work/ch14-property-verification-v1.json`。root另回報其共用fixture checker實跑包含本例的127項負例通過；此項是root執行回報，不改寫成作者的獨立release QA。
- root實讀指出array reference直接交slice參數仍依賴未明教的coercion。counts與非空regressions最小改成 `&values[..]`，空regression改已有 `Vec::<i32>::new()`／`as_slice()`；沒有另加coercion旁支。修後fmt、alltargets clippy、全部6項tests及countsstdout再通過，另存 `/workspace/work/ch14-property-verification-v2.json`，原負例紀錄保留而未無故重跑。
- 隔離preview使用repo同一theme／extensions與真實snippets，strict build通過。親看1280×800與390×844的材料判準、strategy契約、兩項property、覆寫錯誤、seed設定、失敗診斷、shrinking、regression與練習原位畫面；手機code內部水平捲動，頁寬沒有溢出。SVG有明確390×620 intrinsic尺寸與24px字，手機縮至343px仍可直接讀；每個失敗／通過框與「保留兩筆」結果能辨認，箭頭角色與手工示意在正文及圖內先說明。
- 22張原位畫面與SHA／viewport記錄在 `/workspace/work/ch14-property-preview/visuals/manifest.json`，其中包含14.4首次 `mut self` 修後局部兩device畫面。array slicing修後再建置、捕捉並親看counts與regression的四張局部畫面，另存 `postfix-manifest.json`，沒有覆蓋原稿畫面。以上是作者受查範圍，未宣稱全書或獨立release QA通過。
- 正體字與angle型別inlinecode已自查。新API、Config欄位、manifest與負例規則在首次snippet前建立最低契約，完整練習未使用未教的append／extend／iterator chain等介面。

## Root後續先備補強：numeric literal suffix

2026-10-14 實讀root凍結 `ch14-property-prerequisite-v3/001-003.md` 的新增段落：在strategy表之前當地教 `3i32`／`0usize` 與可選底線形式 `3_i32`。原作者稿的source、tests、SVG與v2相同，root後續發現原jump依賴4／7未先解釋literal suffix才補此段；不倒寫成原作者早已教過、不回改原首讀或驗證紀錄，也不為純正文補強重跑Rust。
