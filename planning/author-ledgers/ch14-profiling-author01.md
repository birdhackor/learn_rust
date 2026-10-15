# 14.8 作者先備與first-use安排

actual14.7 v6全七units／source已實讀；術語依clear-tutorial指引。先來源／安排，再正文／source；root整合共享Cargo／lock／nav。

| 必要內容 | 首次需要處 | 實際前文 | 本節先教 | 可預測結果 |
| --- | --- | --- | --- | --- |
| profiling指標與目的 | clone候選工作 | 14.7同工作／正確性／timing | 開頭選allocation而非CPUtime | 配置資料只能支持配置來源，不支持CPU熱點 |
| repeat_counts／total_hits | profile材料 | 14.7兩count／black_box；1.8迴圈 | helper前有限100次與6400 | 兩方法完整巡訪同清單，相同結果，不少做工作 |
| allocator／global_allocator | 首個attribute | heap／String／8.2drop | source前角色／唯一程式配置器 | dhat包系統配置器，記錄不等於正常計時 |
| immutable static／unit struct值 | ALLOCATOR item | struct／型別annotation／'static／Sync | item行之前 | ALLOCATOR整程式可用，局部Profiler才控制追蹤 |
| Profiler builder／file_name／build／drop | main | 14.4builder形式、7.5Path、8.2drop | API表與scope圖之前 | scope外清單仍存在，get之前需active，drop後才寫報告 |
| HeapStats get與四欄位 | snapshot | struct公有欄位／數字型別 | 第一次source前API表 | cumulative、peak與currentlylive不同 |
| release debug資料／env覆寫 | commands | 9.1profile／release | 第一個命令前 | 仍最佳化，只加來源位置，不拿profile時間和14.7比較 |
| JSON／viewer／PP與backtrace | source歸因 | 7.7JSON／函式分層 | 第一個profile展示前 | 指向clone呼叫來源，不代表單一CPU耗時比例 |
| 改善與條件 | borrowed profile | 14.7count_borrowed契約 | 配對相同scope前 | 0新配置不等於0heap，更不能外推所有Clone |
| 完整練習 | 64行profile | 前段所有selected API | 題目前scope／材料／預期 | 16×100行數，累計／peak角色不同，不改規則 |

預計新package examples/ch14-profiling；dhat0.3.3為dev-dependency，normal helper不啟用global allocator。不要把後續root核對當獨立release QA。執行／view／SHA完成後追加。

## 最終作者驗證與交付

2026-10-14 完成14.8全文、獨立ch14-profiling package、兩幅SVG、配對profile與64行完整參考答案。新attribute、static／unit struct值、Profiler／HeapStats、debug環境覆寫、viewer欄位與backtrace／PP均在首次source或推理之前建立契約；Rust例子、普通tests、helper與答案只使用actual14.7／前文或當地已教內容。

- Root整合member／lock後，作者 scoped fmt check、alltargets Clippy `-D warnings`、dev／release test全部exit0；dev與release各有2項integration tests，涵蓋大小寫／每行只算一次、多次完成工作，以及0次與空資料，不設時間或allocation門檻。原commands／完整stdout／stderr／6份teaching source SHA保存 `/workspace/work/ch14-profiling-verification-v1.json`。
- 原模型、借讀模型與64行參考答案三個release／debug=1 examples各exit0。追蹤前材料／答案核對與追蹤後stdout均在範圍外；兩256行模型皆6400，clone為25,600 blocks／326,400 bytes、峰值15、snapshot0，借讀皆0；64行答案1600、6,400 blocks／81,600 bytes、峰值仍15。Fresh JSON解析成功並與HeapStats snapshot／stderr逐項吻合，所有原檔及環境、命令、source SHA保留package profiles，沒有用舊檔冒充本次成功。
- 三個JSON實際載入正式DHAT viewer，親看其真實截圖並讀來源文字；原模型只有一份非空PP，Allocated at連回main18→repeat_counts12→count_cloned21→String Clone，借讀只有空root，64行仍同一clone位置。此處只作allocation歸因，不把工具中的elapsed／存活時間當CPU耗時比例。Viewer證據及public工具SHA在 `/workspace/work/ch14-profiling-research/viewer-evidence/`。
- 使用repo同theme／extensions／實際snippets的隔離preview，最新正文strict build exit0。親看1280×800與390×844的34張final原位畫面，另親看prelude補強的兩張postfix原位畫面；涵蓋動機／工作／helper／tests／manifest／配置器／static／Profiler／欄位／完整main／命令／paired結果／viewer／兩圖／來源歸因／結論限制／練習與展開答案。畫面與viewport／SHA記在preview的visuals-final及visuals-postfix manifest，沒有覆寫原稿證據。桌面頁寬1265、手機375，小於各自viewport；code保留內部水平捲動。
- 兩幅SVG均有390px intrinsic寬與24px字，手機縮至約343px，追蹤框／準備與收尾／呼叫箭頭／main18與兩個source位置／15與0及累計數字均可直接讀。Source圖只節錄已知四層，正文先說省略標準庫配置框架、各層是同一來源，不能重複加總。
- Root本輪全文／source／API先備核對要求一項最小銜接：8.2的std::mem::drop也由標準prelude提供。作者核官方prelude重導出後，在first-use表之前補一短句；不改Rust、原profiles或SVG，不為這項純文字重跑已通過的Rust驗證。該段修後strict build與兩device頁面已親看。
- Root另自行讀source／JSON／指紋與selectedAPI，結果是root先備precheck；不把此回報改算作者獨立release QA。作者本輪驗證亦不宣稱全書或獨立章QA完成。
- 最終6份teaching source仍與verification v1／measurement v1 SHA相同，三份raw JSON亦同原measurement SHA；最終正文、兩圖、notes／ledger與所有profile記錄指紋另存 `/workspace/work/ch14-profiling-handoff-sha-v1.json`。未修改已凍結14.7／11.4或shared Cargo／lock／nav。
