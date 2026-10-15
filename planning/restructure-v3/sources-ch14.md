# 第14章來源與實讀範圍

作者研究在 `work/tutorial-implementation-20261015/ch14-preparation/`，教學執行證據在同層 `ch14-authoring/`。以下是實際讀取與取捨，不是獨立技術接受。

## 英中教學來源

- FastAPI [英文 concurrency/async](https://fastapi.tiangolo.com/async/)與[繁體中文對應頁](https://fastapi.tiangolo.com/zh-hant/async/)：prep親讀完整抓取正文（EN文字單位1–198；ZH1–202）與13張原必要插圖，原檔hash在prep來源manifest。正式寫作又親讀EN42–74／ZH46–78原文固定漢堡情境，不以摘要代替。借鑑固定物件、逐步進度、同一份材料比較；14.1另設具體A／B，不把餐廳角色硬配allocator或runtime，不把手工安排聲稱成效能實驗。
- Async Book [英文 primer](https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html)及[中文版對應章](https://huangjj27.github.io/async-book/01_getting_started/04_async_await_primer.html)，另讀兩種語言的 async/await 章。正式寫作重讀cached原文primer文字單位1–16、async/await1–21全範圍。採用建立工作與驅動到結果的順序；不採futures executor/join作另一套學生API，不採所有future工廠均完全lazy的概括。

## 固定 API 版本與原始契約

根 Cargo.lock 本輪核對：Tokio1.53.3、tokio-macros2.7.2、reqwest0.13.5、axum0.8.9、clap4.6.7、url2.5.8、serde1.0.229、serde_json1.0.154；root只加入八個本地stage packages，未升第三方版本。

14.2–14.3親讀官方cached原頁及安裝版本source：

- tokio-macros2.7.2 `src/lib.rs` 21–115、140–163、370–445：current_thread入口、預設main flavor、每async test獨立runtime。沒有把Builder／macro內部實作變成學生先備。
- Tokio1.53.3 `src/lib.rs` 312–345：feature分工；只用rt/macros，再加sync。
- 官方spawn頁文字單位1–26全；JoinSet頁1–77（後續poll API不納入教材）：spawn不要求先await handle才執行、Future/Output Send+'static、runtime結束不保證完成；JoinSet回收與Id，abort不是回收。
- Tokio1.53.3 `runtime/task/join.rs`175–265：abort請求、is_finished當下查詢、取消仍要等待；`sync/mpsc/bounded.rs`108–159、175–243、816–826：正容量、複製送端、send／recv與關閉契約。

所有cached原文與讀取hash保存在prep資料；正式前綴source/prose hashes保存於 `actual-prefix-read-start.json`、`reads-14-01-03.json`。其中13.1–13.8已親讀全文；必要13.1／13.2／13.5／13.6／13.8 helper與完整source按manifest逐檔保存，不冒稱讀過未使用的13.7所有source。12.1根設定與lesson workspace、12.2正文、10.8 Command契約與完整CLI tests另有實讀。

既有八個async／HTTP packages完整Rust source逐檔親讀後才拆快照；舊頁當保全材料，不以舊頁位置替代新版actual先備。

## 14.4–14.9正式寫作實讀與取捨

- bounded／timing／cancellation／blocking HTTP／async HTTP／local-service全部原始Rust檔、manifest及負例lock/expected親讀，才作staged隔離與test分檔；前綴另讀實際10.6 clap完整config/main、10.7模型與完整codec helper/tests、9.5正文、12.2正文，並核對10.4 Write／ErrorKind契約。compiled dependencies只指向已完成的14.2、14.3、14.8。
- Tokio timeout原始source全249行重讀；sleep.rs65–130、clock.rs65–190、ctrl_c.rs15–64及select cached原頁1–71重讀。採具體Future、合作式期限、paused時鐘與ready契約；timeout真正泛型簽名裡的IntoFuture留14.9，沒有提前給學生讀。
- reqwest blocking client374–401／1413–1538、blocking response274–310、async client1440–1458／2620–2718再次親讀；prep另已讀3080–3150及builder units81–91。這些分別支持各blocking等待跨度、async共享總deadline、send普通工廠準備與response ownership。library內部runtime／hyper／Pin細節不變成學生helper。
- axum serve119–171／218–236／274–368原始source重讀；安裝版axum-core IntoResponse具體實作與Redirect::temporary補核。採handler／Router／State／Path／listener連續持有／graceful shutdown；不先教完整Service內部bound。
- 另讀已安裝nightly-2026-10-01的core `future/into_future.rs`全文作標準介面補核（不是冒稱它是repo1.99.0的source）：associated Output／IntoFuture、接手轉換、await轉換與Future blanket能力，其中IntoFuture標示自1.64穩定，並以repo1.99.0實測本章使用方式；支持14.9首次教學，不提前用於14.5。clock resume契約另實讀146–190。

實際range與whole-file SHA256另存read-evidence-final.json；whole-file hash只識別版本，不把range外內容算成已讀。首波與各次freeze均保留，不用最後hash覆蓋較早觀察。14.7／14.8只有離線資料政策驗證；14.9真loopback網路測試按selected target通過，未對公開網站送請求。
