# B4：停止、逐份收尾與typed Batch

對應18.10，只加停止控制；Input／single沿B3。new先準備，run每輪先查停止再fill。觀察Requested後不提交新task，abort_all後逐份join，保留真正Check／task failure；已取消為Cancelled，pending原Input為Unstarted。最後按原行索引排序，duplicates與空白造成的索引空洞保留。

StopEvent／EndReason保存Requested與SignalFailed／StopSourceClosed的差別。來源失敗不是使用者中止。零jobs／空有效清單先拒絕；全invalid非空清單仍有逐筆Input結果。每筆GET／redirect最多5／headers-only／Tokio十秒預設不改。

教材repo根目錄：

```bash
cargo check -p url-b4 --all-targets --locked
cargo test -p url-b4 --lib --locked
cargo run --quiet -p url-b4 --example b4_signal_demo --locked
```

七項tests驗設定先拒絕、提前停止零提交、已進入真held工作取消／pending分類、收取後補工再停止保留成果、完成尚未收取Check／task panic不覆寫、來源失敗／關閉、正常混合清單。測試自己後備drain前保存集合／登記快照，避免把產品沒有收尾的錯碼補成綠燈；全部資源收尾後才斷言。

最後一條是有限真Ctrl+C demo。看到flushed READY後立即按Ctrl+C：應有原index1／3兩份Cancelled、index4／5兩份Unstarted，server工作GET=2，client／listener／server皆已收尾，退出130。動態port不固定；ready與final報告均明確flush，輸出／基礎失敗回2。若不按，單筆十秒期限會讓demo自行完成；其中一定有/notes輸入問題，退出1。它沒有正式產品CLI參數或JSON。

一般caller使用run_batch並await到結果，以停止channel要求中止。不要drop整個run_batch future當作已等完children。需要像測試一樣保留owner時，自己持有Scheduler，借用run；失敗時另stop_and_finish並await，最後into_batch接手owner。run一次；Input由prepare_inputs取得或保有同等唯一遞增原索引。

repo外複製整個manifest／src／examples，先改Fixture絕對path，再沿18.1用cargo +1.99.0。普通sync／signal features沿14.6；直接版本要求／lock沿10.1。正式CLI／JSON及完整fatal2 > cancel130 > item failure1 > success0在18.11整合，最終功能不縮。
