# 第14.6作者依賴帳冊

作者chapter04_traits；本輪只寫14.6、自己的Loom/Miri package／fixtures／答案／SVG與筆記。14.1–3已凍結，保持bytes。Root維護workspace/lock/nav/sharedscripts/CI。

actual前文與fresh英中／官方API閱讀範圍見source-notes/ch14-tools-author04；沒有讀peer14或原審查答案，這是作者自查，不是獨立盲讀。

## 本節安排

原需要有兩種未由普通PASS涵蓋的責任：thread正常跑只看到一個交錯；safe façade正常結果沒逐項檢unsafe動態契約。主線是「選工具查哪種缺口、保留什麼證據與界線」，不同工具不互相代答。

| 必要概念／契約 | 第一次需要前的來源或本文教法 | 能由材料解釋的關係 |
| --- | --- | --- |
| atomic各步 vs整次增加、全部join | actual10.8全部，10.1data race、10.2spawn/join、10.6Arc | 0→兩份各加1應2；read/read/store/store可1，合法atomic不等於正確結果 |
| Loom model重跑與替代API | 本節先說狀態每次重新建立、同步由Loom追蹤，再介紹model closure及Arc/Atomic/thread | 輸出反例來自某個允許模型路徑，不是本機race頻率；不用std替代型別矇混 |
| dev-dep／精確版／integration target | actual7.1/7.2、9.1 | 正常套件只有測試用Loom，Miri只選miri_contracts、不解譯Loom target |
| read/store反例及fetch_add修法 | 順序圖／counter1先教，再看隔離fixture與modeltest | 更新合成一次RMW，不以改SeqCst誤稱修復，也不測速度 |
| Miri／MIR／nightly／component／setup | 本節在任何工具指令前教身分、固定工具鏈、選定flag與std準備 | 解譯所選執行查UB，不是普通host native執行或全部soundness證明 |
| 正常safe API的unsafe內部 | actual12.4全部helper及39source | 正常split與0／len／空Vec邊界來自契約，無新parser或外部C |
| Box保活／drop／raw pointer不保活 | actual8.1/8.2及12.3，本文在UB source前回收 | owner被清理後舊pointer不能讀；null檢查和unsafe block不恢復storage |
| 故意UB與isolated package | 當節source前明說只用Miri，manifest空workspace／ownlock沿14.2 | UB binary完全不成root正常target；不提供cargo run／test native UB命令 |
| 結果與工具限制 | model及Miri結果後各收束原問題 | Loom有限模型未找反例不涵蓋所有Rustmemory行為；Miri未報不證明任意safeclientsoundness；unsupported不算UB |

新真實manifest：examples/ch14-model-checking，dep已教ch12-unsafe公開helper，devdep Loom0.7.2。主package lib無新增安全抽象；模型都在integration tests。實作及檢查結果會另追加，不在此預填PASS。

## 完成稿逐API／helper／答案自查

正文依原問題先比較可能路徑，才介紹model的重新建狀態、Fn()/Send/Sync/static契約與Loom替代Arc/Atomic/thread/join；所有Loom資料都在model body內。normal／預期失敗模型都先join兩份，再判成果；只有兩個worker與單一counter，無spin迴圈、IO、隨機數。Rust syntax與std操作沿實讀10章；Loom本身的selectedmock契約在source前逐項教，不暗套所有std traits。dev-dep/精確版/--test沿7章，ownfixture manifest/emptyworkspace/--manifest-path沿14.2。

Miri／MIR、解譯對象與限制在指令前介紹；nightly日期、+選工具鏈、component/rust-src/minimal與setup分工在用前教。Miri測試只用12.4已完整教的公開split_i32_at_mut，不複製或修改既有helper。測試沒有使用尚未教的slice-vs-array跨型別比較，採len/索引與已教array比較。UB source前回顧Box／drop／raw pointer，説明cfg(miri)、not(miri)與compile_error如何拒絕普通建置；只有Miri命令解譯它。正常答案先讀Copy i32再清理Box，SAFETY有來源／有效性／alignment／權限依據，不謊稱故意UB有合法SAFETY理由。

## 實際驗證與界線

- 新member尚待root整合時，cargo fmt -p曾因「package不是member」拒絕；不算PASS。先以work獨立manifest引用同份4個test source，Clippy及6tests通過，保留其是別份manifest的界線；root後來整合真正member/lock後，已對canonicalpackage重新跑fmt --check、Clippy all-targets --locked -D warnings及cargo test all-targets --locked，全部PASS。6項為2個Loom modeltests、3個正常契約tests、1個修正owner順序答案；空library的0項不算新增覆蓋。主workspace/lock與nav由root修改。
- Loom負例用isolatedCargo，命令確實exit101並在已join後的assertion得到left1/right2；沒有使用should_panic吞掉失敗。expected記left1，實際stdout/stderr/命令在work/author-ch14/14-6/loom-negative.json。Loom預設模型的排列數、時間、preemption與checkpoint控制在本環境未設定（loom-bound-environment.json），不以模型通過保證所有C11/硬體/輸入。沒有重跑全repo測試替代局部判斷。
- nightly-2026-10-01、Miri/rust-src實際安裝與setup成功，工具版本與repo仍stable1.99紀錄在selected-tool-versions.json及安裝/setup logs。本作者之後明選work/author-ch14/14-6/miri-sysroot，未改env／main rust-toolchain或CI。最初工具自己的defaultcache/setup不算測試PASS；重新以workspace/work sysroot完成selected實測。
- canonical Miri --test miri_contracts實際3PASS，--test miri_owner_solution實際1PASS，logs是miri-positive.log/miri-solution.log。selectedtargets都只呼叫pureRust；即使Cargo編譯其他devdeps，也沒有聲稱解譯了Loom或外部C實作。
- isolated Miri-only binary實際exit1，報Undefined Behavior及「has been freed, so this pointer is dangling」，指*pointer並指出Box配置與drop釋放。同一command/stderr存在miri-negative.json，expected改為具體freed原因，排除unsupported operation或工具缺失。只做一次cargo check驗普通建置guard，exit101命中compile_error，紀錄native-build-guard.json；**沒有native執行故意UB**。此fixture有emptyworkspace/ownlock，沒有root member或normal test target。
- 專案相同theme/extensions／當頁真實snippets的獨立Zensical預覽建置成功。首次8814 HTTP服務已停、Playwright連線拒絕，未產圖不算視覺；另開8816後完成desktop1280×1000/mobile390×844，兩圖400×620／400×600及手機約343×532／343×515，HTTP200、相對URL保持Pages子路徑、無水平溢出。不是publicPages或整書nav驗收。
- 最終4張實頁PNG已親看，Loom甲乙兩次起始狀態／四步／結果2與1清楚，Miri同一allocation、owner保活、舊pointer不保活與drop界線清楚。初次畫面發現合併SVG subpaths的marker只在右邊顯示，已拆為獨立paths、重建與重新親看兩端箭頭；pointer標籤改不保活資料，避免把已教provenance縮成「只有地址」。均為作者自查後修正，不算獨立review。
- HTML抽查全部snippet已展開，model／cfg(not(miri))及兩個錯誤理由可見，基本理解不需要點擊放大。當下18份BODY/2SVG/全部source/fixtures/expected/兩isolated lock指紋在final-source-body-fingerprints.json；BODY SHA c7ac4cd1a476b0c4501519e2fe08e9f6da4d837d3e122b74baf3c5176a9db2af。14.2全部29 frozen source/BODY與14.3交稿20指紋逐核未變，也涵蓋保持14.1原source不回填；12.4 archive39source仍與actual gate一致。

本稿完成作者selectedchecks與實頁自查；待root實讀、freeze及獨立QA，不宣稱第14章或全書驗收。完成交付後停止，不擴peer14或共享設定。
