# 第 15 章 B 專案：來源與驗收研究

- 作者：`chapter11_async`，由 root 明確追加指派。
- 日期：2026-10-14。
- 狀態：只有來源／契約／驗收研究，尚未寫正文或程式；尚未取得全部已驗收前文。
- 已讀 `curriculum-plan-v2.md` 的 15.1、15.7–15.11、B 固定契約及前置 API 表。B 保持約千行含測試與本機 fixture；實作後再實際計數，現在不預報已達規模。

## 動筆前實讀來源與教學取捨

| 來源 | 值得學習 | 此專案的取捨 |
| --- | --- | --- |
| [英文 Rust Book：接受 CLI 參數](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html) | 先給使用者將輸入什麼、看到什麼；用可觀察操作驗證接線。 | B 先展示固定輸入與單筆結果，CLI 在 B4 接上已教 clap／PathBuf，避免讀者一開始同時承擔參數、檔案、網路、排程。此來源先用了 collect 再教 Iterator，本專案不能照搬順序。 |
| [英文 Rust Book：TDD 與核心 library](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html) | 一個可呼叫、可對照輸入／答案的核心函式；先看會失敗的驗收，再逐步完成，CLI 可與核心分開測。 | 把單筆分類、清單身分、排程、報告分成里程碑；每階段骨架只留當階段核心工作。引導不把完整答案放在第一個 code block，也不隱藏核心演算法在 helper。 |
| [簡體 Rust Book：TDD 與核心 library](https://kaisery.github.io/trpl-zh-cn/ch12-04-testing-the-librarys-functionality.html) | 中文說明直接連起測試的需求、預期失敗原因與下一個最小改動。 | 借鏡「預期失敗 → 實作 → 解讀」；來源中的 Box<dyn Error> 在其原教材順序存在先備問題，本書完成8.6後才可使用，且完整答案保留主線的具體錯誤／結果身分。 |
| [Tokio：Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown) | 拆開「何時停止」「通知停止」「等待收尾」三件事，避免只發通知就宣稱完成。 | B4 沿用 11.5 controlled mpsc stop 與 Ctrl+C controller，停止排新工作、abort 與 join；不加入 tokio-util。此處要回收正常完成時 controller 的生命週期，測試用可控制 channel 而非猜 OS 訊號時序。 |
| [第11章中英文 async 教材與官方 API 紀錄](ch11.md) | 使用已查核的 task生命週期、Id、timeout、Client pool、headers／body、axum fixture；不另造一套語意。 | 中英文對應章已實讀並記於 ch11；動筆前仍要讀真正凍結的11章 prose/code，不把來源研究紀錄當讀者前文。 |

## 固定契約如何落到里程碑

| 里程碑 | 讀者此刻要完成的主要問題 | 固定結果與最小驗收 |
| --- | --- | --- |
| B1：單一網址 | 判斷一次請求取得了什麼結果，何時工作結束 | GET、最多5次 redirect、最終2xx通過；final headers 到達就完成，不下載完整 body。先區分 URL 輸入問題、HTTP status、transport failure、timeout。用本機正常／非2xx／redirect／受控等待 endpoint 驗證，耗時只要求符合測量範圍，不把精準毫秒當固定答案。 |
| B2：依序批次 | 每行輸入與各自結果如何保持對應 | 跳過空白行；保留原行與輸入 index，重複 URL 是不同項目；有效 http／https 才執行。無效 URL 是項目失敗，空清單是設定錯誤。先處理成功與失敗混合，再增加並行。 |
| B3：有限並行 | 在等待工作時容納其他項目，又不無限制建立 task | 正整數 jobs，預設8；JoinSet 至多 jobs 個，完成收結果後補下一個。每項保留index及original input，task Id→index 在 panic／abort 仍有對應；報告依输入顺序，与完成顺序分开。受控事件讓前 jobs 項停在服務端，確認尚未派更多；放行一項後再觀察補一项。 |
| B4：停止與報告 | 停止後讀者如何知道哪些已完成、被取消、未開始 | stop 後不補新 task，abort 後 join 到 None。已完成保留，取消／未開始不算網路失敗。文字與 JSON 共享同一結果模型；區分 task failure。exit0全部成功、1有項目未通過、2致命設定／讀入／輸出錯誤、130中止且成功報告，優先致命錯誤→中止→一般未通過。 |
| 完整參考版 | 讓讀者需要時可查看與引導同範圍的可用工具 | 獨立 package，可建置／執行／安裝，完整 CLI 與 JSON／文字報告，測試同契約。不要換成尚未教的高手寫法，不加入重試、HEAD fallback、crawler、認證、DB／dashboard。 |

## 前置 API 的必要調整與 root 決定

| 必要能力 | 真正前置位置 | 已採納／待實作核對 |
| --- | --- | --- |
| task panic／abort 後仍對回輸入項目 | 11.2 的 task::Id、AbortHandle.id、JoinError.id；11.3／11.5回收 | root 已採納。只讓成功 Output 帶index不夠。 |
| 真正 CLI Ctrl+C 中止來源 | 11.5：controlled mpsc stop之後教 tokio::signal::ctrl_c；signal feature、io::Result、監聽失敗與controller收尾 | root 已採納。正常批次完成後，還在等Ctrl+C的controller要abort並await，不留detach。 |
| 不受環境代理干擾的本機 fixture client | 11.8：ClientBuilder.no_proxy | root 已採納；一般工具 client尊重環境proxy，不把fixture限制直接套到所有外部請求。 |
| axum 的動態port、shared state、response、controlled gate、shutdown及等待 | 11.8 | 尚待正文完成；fixture中的每個型別／方法也受已教前置規則約束。 |
| 持有同步 guard 的範圍、atomic計數與ordering | 10.6–10.8與11.5 | 尚待凍結稿；不能在受控等待時持有 std Mutex guard，也不能把可見性理由交給測試猜。 |

## 驗收設計：關注契約而不是模仿實作

1. 輸入：空行／空清單、有效 http／https、非法 URL／不支援scheme、重複URL身分、CRLF、混合成功與失敗；報告項目數與順序對應非空輸入項目。
2. HTTP：受控服務提供固定2xx、4xx／5xx、redirect正常到達與超過5次；一般HTTP失敗保留status，與沒有最終response的請求錯誤分開。確認請求policy是GET；若fixture需要捕捉method，所用extractor／Method API先在11.8教。
3. 期限：用已註冊的受控等待handler而非公共網路。timeout是整段取得最终headers工作的期限；不以完成body當驗收終點，也不主張能強制停止CPU同步迴圈。
4. 排程：以handler開始事件與可放行gate觀察數量，證明初始至多jobs、完成一個才補下一個、不是spawn所有項目後另擋請求。倒序放行亦保留輸入排序。
5. 中止：先控制一項完成，另幾項仍等待且尚有未開始輸入，再送stop；驗證已完成結果保留、運行中取消、其餘未開始、無新增工作，排程函式回傳時追蹤集合已收齊。
6. task failure：用本章已教的可控制task失敗路線驗證Id→index映射；不在真實URL業務加入隱藏特殊網址或「觸發panic」功能作測試鉤子。
7. stop channel關閉：明示None的契約與分支處置，不能讓已關閉recv分支在loop中立即重複就緒，造成busy loop或誤把正常關閉當中止。若採 `biased;` 讓stop優先，先教其分支順序責任；這是實作時需核對的具體API，不是已驗收的方案。
8. fixture收尾：不只cancel client task；server中已開始的controlled gate也要放行或明確取消，shutdown後await服務handle。避免graceful shutdown永遠等卡住的handler。所有test失败路線也要讓fixture有明示可完成的收尾。
9. 輸出／exit：同一模型的文字與JSON、一致summary、fatal比cancel優先；CLI整合測試檢查實際stdout／stderr／exit status。JSON中的耗時值可變，驗收看型別與意義，不斷言固定值。
10. 完整參考版需要一個簡短、本機可重做的使用例與真實外部URL使用命令。外部網站狀態不當固定CI答案。行數只計最終手寫Rust與tests／fixture，不累計里程碑快照或docs／lockfile。

## 圖解位置的研究建議

- B2用SVG畫「輸入原行 → index → 單筆結果」；重複URL用不同index，不用URL字串當唯一key。
- B3用SVG畫待處理佇列、固定jobs個slot、完成／補新流向；箭頭分清工作開始與結果回收，時間圖不宣稱實測效能。
- B4用SVG畫完成／運行中／未開始三區，在stop前後追蹤同一index；有task failure时Id還能連回输入项。必要圖與原位文字相鄰，後續需真實桌面／手機視覺驗證。

研究完成；沒有正文、code、共享manifest／lockfile／nav變更。
