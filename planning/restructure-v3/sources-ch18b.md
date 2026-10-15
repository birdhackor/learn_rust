# 第18章B實讀來源與寫作取捨

外部23份原文已本人親讀，沿用本次純準備證據，不無目的重新fetch。完整URL、取得時間、cache hash及實讀行範圍在work/tutorial-implementation-20261015/ch18b-preparation/external-sources/sources.json；external-reading-notes.md逐來源記採用與不搬用理由。這是作者來源紀錄，非首次讀者或獨立審閱結果。

- FastAPI async EN／zh-hant全文：採等待情境與完成後才領結果的區分；不搬Python threadpool規則或效能宣傳。
- English Async Book Why Async及async/.await全文、RustCourse對應原始Markdown全文：採lazy／await／owned資料的連續例子；不以drop借用等待等同取消spawned task，不引futures／Stream新依賴。course.rs三站URL404原紀錄保留，以sunface/rust-course原始repo成功頁實讀，不稱讀過失敗頁。
- Tokio Graceful Shutdown、RustCourse graceful shutdown全文：採決定停止、通知、等待三責任；不搬CancellationToken／TaskTracker／broadcast；mpsc關閉依真Option接口。
- Book CLI、TDD、stderr EN與繁中各全文：採小步可判結果與資料／診斷分流；不照抄args Unicode panic、process::exit或純測試無child清理的安排。
- Tokio1.53.3 JoinSet／JoinHandle／timeout／ctrl_c、Tokio testing、reqwest0.13.5 Client／Response／redirect Policy、serde_json1.0.154 to_string：實讀相關精確API段，保存取消安全、completed-uncollected、abort需等待、clock／body／輸出完成界線。不複製任意sleep當進度證據。

舊教材本人已讀15.7–15.11全文、B全74檔（50份Rust、37份不同內容）、HTTP閉包及五張SVG XML。完整tests suffix與重複module的byte證據在source-reading.json；兩final lock全TOML metadata解析另存lock-metadata.json。SVG先讀XML、正式視覺由root另驗；檔案存在不算視覺通過。

核准master每節範圍、inventory-late保全、75筆B crossref引用與audit包均已核對；按actual稿選引用，不全域換號。原B兩項採納：W11-15-007改用「觀察期限／測試timeout」，W11-15-008普通敘事改「起始檔／工作版」。原15.11拆成18.11引導與18.12全檔答案；不以單一完整檔連結代替答案，不為核心825行湊1000新增功能。

正式actual前提的46檔range/hash在ch18b-authoring/actual-prefix-read.json；不以outline／作者ledger代替正文。逐節限定Cargo與samepage全snippet檢查見author-ch18b.md。未做的全站、手機桌面實頁、獨立技術／連續性檢查明列未驗證。

正式稿已逐節freeze。外部實讀取捨實際落在18.7的工作／觀察期限、18.9完成待收slots與Id→完整Input、18.10停止後回收與S1/T0/W1、18.11三步checks及出口完成、18.12全檔可用工具。已知course.rs停止章404連結由root依本輪實讀Github原文窄換，保留舊freeze及來源差分。作者只自查契約、source閉包、selected執行與markup，不把這些算獨立首讀或手機桌面視覺驗證。
