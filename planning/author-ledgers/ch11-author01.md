# 11.4 作者先備與API首次介紹

實讀來源見 `planning/source-notes/ch11-author01.md`。actual gate是11.1 v3全9 units與5.6 v7；本作者不預借並行11.2／11.3的task／channel介面。使用clear-tutorial、writing-prompt與本專案術語指引。

| 必要概念或API | 首次需要處 | 實際已教位置 | 本節先行解釋 | 可預測結果 |
| --- | --- | --- | --- | --- |
| Duration與Instant的不同角色 | 等待跨度與量測example | 1章整數、2.5associated method、11.1await | 第一個Rust source前 | 20ms是等待設定，elapsed是所包工作當次經過跨度，不是日期 |
| from_secs／from_millis／as_millis與u64／u128 | measured example | 1.4數值表示 | 單位／API表 | 總毫秒與不足一毫秒截去的意義可解讀 |
| time feature／sleep／Sleep Output | 首個timer | 11.1rt/macros／current-thread runtime、7.1依賴 | manifest與timer source前 | 建立future不阻塞thread，await等timer，不保證精準醒來 |
| timeout／Elapsed／owned future | timeout_outcomes | 11.1Future Output／owned await、3.2Result、8.2drop | 首次timeout呼叫前 | Ok保存原Output、Err為期限原因；wrapper接手工作，取消不是rollback |
| pending::<T> | 可確定未完成材料 | 3.3泛型型別引數、11.1Future | outcomes source前 | Output宣告為T但不交結果，timeout可停止等待 |
| outer／inner Result | nested example | actual11.1checked_total、5.6問號一次處理一層 | type與分支表先於source | Ok(Err)已完成但資料失敗，Err(Elapsed)尚未完成 |
| borrowed async future在取消後的狀態 | cancellation example | 11.1capture／mutable borrow、8.2drop | source前 | 開始標記保留，pending後步驟未執行，借用在wrapper結束後可再讀 |
| std thread sleep阻塞與timeout非hard preemption | blocking對照 | 10章thread、11.1同步本體不自動搬走 | source前本地教sleep | 不交還控制權時timeout不能硬切斷，完成仍可能Ok |
| test-util／start_paused／Tokio Instant | controlled tests | 11.1tokio::test／current-thread、已在本節教timer | test manifest與attribute／API前 | 用Tokio測試時鐘決定timer先後，真實std時鐘不同 |
| auto-advance與受控前提 | Result三路與clock test | 前段paused clock角色 | 第一個paused test前 | 只有本節timer／future，無其他可執行工作時推進到下一timer |
| total_after與問號原錯誤 | 有等待的Result工作 | actual11.1checked_total、5.6相同錯誤From | library source前 | 負小計不等timer，正小值完成等待才回總額 |
| whole scope／per-step新期限 | two_steps與scope tests | async block／nested await、Duration Copy | helper與比較source前 | 相同兩段30ms，整段50ms完成1步；各段50ms完成2步，既有步數不回復 |
| 完整練習 | 三組scope材料 | 上列所有已教介面 | 題目前固定範圍與預期 | 預測0／1／2步與Ok／Elapsed分支，不加新API |

預定隔離package `examples/ch11-async/timing`／name `ch11-timing`；共享Cargo／lock／nav由root整合。需使用foundations已交付library時，path dependency的角色沿7.1，在manifest前交代；不用自己重印一份foundation程式當先備。

## 實際完成與作者驗證

11.4完整正文在 `docs/chapters/11-04-time-and-timeout.md`；獨立package為 `examples/ch11-async/timing`，包含library、五份正常examples、六項契約tests及一項完整練習test。`docs/assets/ch11/timeout-scope.svg`把整段共用預算與每步重新給預算的不同範圍放在正文原位。固定版Tokio1.53.3的selected source、實讀範圍及取捨保存於本作者source notes，不把下載全文等同實讀。

- 已有 `/workspace/work/ch11-timing-verification-v1.json` 記錄fmt、alltargets Clippy `-D warnings`、七項tests全部exit0，以及五份正常examples的stdout契約通過。本次恢復工作直接核對這份原紀錄，不重跑已通過的tests，也不把root工作程序失去session後的推測當作新的退出碼證據。
- 真實量測例當次讀到20ms設定／21ms elapsed，阻塞對照當次讀到30ms elapsed；正文只保留固定行與可變讀值的意義，不把這兩個單次觀察當成精準延遲或效能改善證明。取消例留下visits=1；嵌套Result保留內外層的不同錯誤角色。
- 受控tests使用先在正文教過的test-util、current-thread、start_paused與Tokio Instant。所有等待材料只有本節timer／future，靠自動推進固定先後，不以真實std時鐘等於指定毫秒作斷言，不借未交付11.2／11.3的task／channel介面。整段50ms／兩段30ms留下1步；每步各給50ms得到2步；練習另檢查0／1／2步。
- 恢復工作時親看已有preview的26張1280×800／390×844原位畫面：Duration／Instant表與量測、timeout、nested Result、取消、阻塞限制、測試時鐘介紹／source／三路結果、scope介紹／SVG／source及練習。正文與表格可讀；手機code維持內部水平捲動，頁寬沒有溢出。SVG上下預算框、剩20ms／重新給50ms箭頭、完成數1／2以及手工示意標記都能直接辨認。圖與相鄰正文各自說明範圍，不把圖的30／50／60ms當真實排程截圖。
- 受查畫面、SHA與viewport在 `/workspace/work/ch11-timing-preview/visuals/manifest.json`。本次只讀既有原位圖片與證據，未覆蓋畫面。以上是作者視覺範圍，不等同獨立release QA；root另回報已通讀BODY/source並親看scope圖，紀錄其先備核對用途而不改寫成作者的獨立審閱。
- 交付前逐項重新計算verification v1所列11個檔案的SHA，manifest、library、五份examples、兩份tests、正文與SVG全部和原驗證完全相同。對照保存 `/workspace/work/ch11-timing-handoff-sha-v1.json`；此次只追加本ledger，不修改已受查教學source或BODY，交root凍結後才開下一節效能工作。
