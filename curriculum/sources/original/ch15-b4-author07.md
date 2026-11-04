# 15.10 B4 作者來源：觀察停止、保存結果、等待收尾

本輪只開15.10；15.11仍待ROOT actual gate。依clear-tutorial／writing-prompt／專案術語；A、B1–3與所有舊frozen／proof不改。

## 實作分段與實際前文

v2原表把取消、CLI與JSON一起列在B4。ROOT本輪明確確認：15.10先完成typed停止Batch、受控停止事件與真Ctrl+C有限demo；15.11才整合正式CLI／JSON及fatal2優先於cancel130、再一般項目失敗1／成功0。這是同一最終功能的分段，不刪掉最終要求。SignalFailed／StopSourceClosed本步已保有typed理由，後續不得誤算使用者取消。

重新完整實讀actual B1-v8七BODY／whole兩packages、B2-v9七BODY／whole兩packages、B3-v10八BODY／whole兩packages。各unique完整source讀過；B3 starter checks整份已逐byte證明是完整讀過的complete lib內相同module，兩份input／single相同內容不靠猜測。保留原行索引、原文／duplicates、空白／invalid／Empty、GET／redirect5／headers-only／每筆十秒、data-only pending／有限JoinSet／Id→Input、完成待收與收取後補工、owned Scheduler全children清理，全部沿實際前文。

目前11章actual為 `ch11-review-fixed-v16`，本輪完整讀005-001至005-013，whole cancellation lib／models／main／三examples／CLI tests／manifest／negative fixture source；lock只核來源指紋，不說逐行學過lock內容。它已教select同task、biased停止優先、recv／join取消安全、try_recv每輪補工前觀察、Requested與SignalFailed／Closed不同、abort_all後保留實際Output、pending Unstarted、ctrl_c首次實際執行註冊／Unix process政策、ready與controller收尾。這是作者前文gate，不是11章獨立QA或已發布主張。

## Fresh EN／ZH 對應教學

開寫前重新完整實讀 [Tokio Graceful Shutdown](https://tokio.rs/tokio/topics/shutdown) 正文23–148及 [RustCourse 優雅的關閉](https://course.rs/advance-practice/graceful-shutdown.html) 正文22–109。採「判斷停止→通知各部分→等待結束」三種責任；本題還要保存所有原輸入分類，所以不把停止訊息當報告完成。保持既有JoinSet owner，沒有改用CancellationToken、TaskTracker、broadcast／watch或unbounded channel；不照抄spawn十份或sleep猜停止時間。中文範例的recv關閉不是本題完整task結果分類，不能代替join收取。

Tokio Select與Async Book select頁也開過，但本輪工具輸出只顯示部分，不列完整章實讀；需要的select操作與取消契約以actual11.5及fixed官方source核對，不用部分擷取冒充全文閱讀。

## Fixed selected API

fresh下載官方Tokio tag1.53.3的signal/ctrl_c.rs、sync/mpsc/bounded.rs、macros/select.rs；全部bytes與當前registry cache相同，原檔／selected-api.json存work-only `ch15-b4-author`。

- ctrl_c.rs全檔：第一次實際poll才註冊；Output io::Result<()>；Unix註冊後handler維持整個process，drop不恢復預設。學生只沿11.5操作契約，不教raw平台實作。
- select.rs1–185：同task等待／別支future被取消、biased列序與公平責任、取消安全。B4只用普通binding接住所有Result／Option，不引新Pin／Stream／futures寫法。
- bounded.rs305–380（所需try_recv為324–366）：Empty與Disconnected不同、buffer仍有訊息就先交訊息。recv取消安全沿actual11.5；不把permits／poll_fn／recv_many內部示範搬入學生code。

第一輪Clippy指出pending的while-let可用for。actual6章／11章沒有教過 `Iterator::by_ref`，沒有照診斷直接偷偷換：fresh取得Rust官方1.99.0 iterator.rs，完整讀1875–1960的這個method說明／範例／簽名；本機未安裝rust-src，故只保存fixed官方tag來源，不冒稱與本機source bytes相同。本文在stop程式前補它的必要角色：mutable借pending iterator，交回同一iterator的mutable reference；for可取owned Input而不搬走Scheduler欄位。它不建立獨立副本，走完後原pending也耗盡。原始Clippy失敗／舊test PASS另存initial logs，採這份最小API後再核受影響的停止流程。

## 本步教學安排與受控驗收

從B3有限排程改兩個必要時點：new先備資料但不fill；收取只record，不立即fill。每輪先try_recv，觀察停止後不再fill；select停止與join，stop先選時仍由owner持有集合。abort_all後逐份join：Checked保留成果、is_cancelled歸Cancelled、其他JoinError仍TaskFailure；原pending取出成Unstarted。entries最後依原index排序，不能拿連續Vec位置替代帶空洞的原行索引。

test owner直接持Scheduler／Fixture。測試可在同一份task以select並行推進借用Scheduler的run與借用Fixture的有限控制工作；控制支路保存診斷、送停止後pending，不持有另一個server owner。整份觀察有期限，失敗後原owner仍能abort_all／join全children，再shutdown；不abort外層controller後把Drop JoinSet說成全部已等完。這個具體helper／借用與pending角色在第一次測試code前說明，不泛化任何Future皆能如此收尾。

驗收包含：提前停止沒有第一批、真held已有工作取消／pending未開始、正常成果已收後停止仍保留、已完成未收成果不被abort覆寫、來源失敗／關閉不等Requested、正常混合清單仍完成。全invalid只要已提交就正常Input；若停止前尚未提交則Unstarted，不偷偷先解析它。duplicates原字串相同仍各保留不同index。

## 實作後取捨

停止清理不能只是test helper代為補完：finish先記產品返回時JoinSet／Id表的數量，再後備收尾，最後斷言原快照。這讓忘記join／record的錯碼不能靠測試自己的cleanup通過。真正HTTP已收成果與合成完成待收task分開驗；後者只證明分類，不聲稱收到HTTP。

引導路線在repo外實跑為1／6→2／5→7／0，與本文兩步相同。真Ctrl+C demo用兩個held維持READY材料，與前面的held＋已完成HTTP材料分開說明；每份gate號對應哪個原index都在code前定位。host-only SIGINT proof不把Python服務或signal API搬入學生helper。初次host全形冒號比對錯誤與被迫終止的限制保留，只有後續READY後正確送訊號的run支持130與完整收尾結論。

最終SVG及頁面在兩裝置各18張實際視圖親看，沒有增加新API或功能。by_ref採本地先教再用；Std來源仍只承諾官方fixed tag，未安裝rust-src的限制不變。formal handoff含這份來源筆記、ledger、selected-api.json與全部raw proof；15.11尚未授權，正式工具功能不在本步假稱完成。
