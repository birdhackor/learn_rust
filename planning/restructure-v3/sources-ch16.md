# 第16章v3來源實讀

原文先讀再寫；所有區間為本輪快取文字1-based inclusive，HTML保留導覽/主文去除script/style/空行，raw Markdown保留原換行。SHA是raw原文bytes，抽取文字另有text_sha256。研究取得不等於實讀；原文URL/本地路徑/範圍/取捨詳見work/ch16-v3/research/read-manifest.json。

| 來源 | 實讀範圍 | raw SHA256 | 採用與校正 |
| --- | --- | --- | --- |
| [std-waker](https://doc.rust-lang.org/std/task/struct.Waker.html) | 40–95, 185–217 | `d714ee0e734c1034e09897857962a332d8365eb46c2f516d9fa0c5573549ad6c` | 採wake/wake_by_ref所有權、SendSync、From<Arc<W>>實際bounds、Unpin、coalescing與不保證公平；wake-to-poll happens-before由channel/Mutex建立，不額外手寫Atomic。 |
| [std-wake](https://doc.rust-lang.org/std/task/trait.Wake.html) | 1–92 | `9479b4c5b009433ceb8647b538dc3da49a4b167e21a7bc0f6982c339494c1cec` | 採安全Arc回呼與default wake_by_ref clone Arc再wake；不採park/unpark完整executor（官方亦提醒巢狀/中間unpark死鎖責任）。 |
| [std-pin-macro](https://doc.rust-lang.org/std/pin/macro.pin.html) | 1–130 | `724b0188b805446de397cecac56670f2327ab5ad0bad22e9df61956b0b30837d` | 採safe local pin不新配heap、跨await局部值可能在外層future儲存中、借用不能逃離局部來源；不採nightlycoroutine/no_std新課程。 |
| [async-en-future](https://rust-lang.github.io/async-book/02_execution/02_future.html) | 1–174 | `77313a2a41fd951e85bd9954d0ab8711f3ca2b6ade077fe7d4f0319e84734f34` | 採poll/Pending/Ready與通知驅動的順序；不用SimpleFuture替代真正trait，不搬Socket未知helper、Join/AndThen多工作新scope。 |
| [async-en-wakeups](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html) | 1–119 | `24baa9d0e937a8ca08a0823a0b73123029890cf605ab118e3ec4fbd49d2b159a` | 採最新Context的owned Waker及共享狀態；原timer省略worker join並持鎖wake，不沿用；用本書受控事件/鎖外wake。 |
| [async-en-executor](https://rust-lang.github.io/async-book/02_execution/04_executor.html) | 1–168 | `3972e6ae5807806958d74086e79af178ce55b3a8239fef1189071679d621bc20` | 採executor owned工作/通知後重poll；不採第三方ArcWake/BoxFuture、容量10000、多task、UnsafeCell替代Mutex暗示，不新增依賴或通用排程scope。 |
| [async-zh-future](https://huangjj27.github.io/async-book/02_execution/02_future.html) | 1–164 | `77e6d1b162f7bee4222e54d3539a99e8e85246d22261d7b70562de794417c224` | 採先介面角色再例子；socket阻塞是工作尚不能進展，不是poll應阻塞thread。真正trait放Pin已教之後。 |
| [async-zh-wakeups](https://huangjj27.github.io/async-book/02_execution/03_wakeups.html) | 1–128 | `fd9f842a9840371f6ab9e8a524ddc31f2dc3e29b7ce80f7454d4301867a57082` | 採每次Pending更新Waker；clone保活通知位置而非複製future。修正持鎖wake與未join timer。 |
| [async-zh-executor](https://huangjj27.github.io/async-book/02_execution/04_executor.html) | 1–165 | `e88d518b15cce4accdd20d215784d50199faf03ee6c31058743a10d451dc5462` | 採任務ownership與重新安排poll；不採bounded wake queue/第三方helper或把drop(spawner)說成drop task/queue空即完全收尾。 |
| [async-zh-pinning](https://huangjj27.github.io/async-book/04_pinning/01_chapter.html) | 1–482 | `5551375248f3b16222059394cad23e497b07246fa16127df62a694e58044f050` | 全文對照self-reference/stack/heap；拒絕失配raw解參考、!Unpin語法誤寫、栈固定總須手寫unsafe、Pin<'a,T>錯型別等舊句；現在std pin!可safe local pin。 |
| [course-zh-future](https://raw.githubusercontent.com/sunface/rust-course/main/src/advance/async/future-excuting.md) | 1–499 | `30c18f38039305e042fe1eec08846f1f9c88184c5339b05292b0ef87cfa24226` | 全文；採狀態/通知/執行器三分工；拒絕先wake才可首次poll、持鎖wake、複製Arc等於複製future、IO/多task擴課及生產executor必不用Mutex的絕對說法。 |
| [course-zh-pin](https://raw.githubusercontent.com/sunface/rust-course/main/src/advance/async/pin-unpin.md) | 1–478 | `3cc1edd5ebacc4a784f19ade32e072bbc91cdf48d9f4e131e2d5efdce27914fe` | 全文；借用async內部關係動機；不用String raw dereference或swap後成功輸出示範UB；!Unpin不是天生不可move、固定wrapper不讓內部F取得Unpin。 |
| [std-pin](https://doc.rust-lang.org/std/pin/index.html) | 1–575 | `b0e1087c9c950c1a317d432a354179c11a17d06b6a57b6e523052b75ad67045d` | 現行官方：mechanical/semantic move、地址敏感phase、library contract、pointee Unpin、位置有效到drop返回或panic、Drop/欄位修改責任；後段projection細節不列read。 |
| [std-pin-struct](https://doc.rust-lang.org/std/pin/struct.Pin.html) | 78–192, 212–255, 313–355, 416–462 | `7184a3bc19613a9c3b73902c94dcf36830a1bb9687de861244c0f71ed75702ad` | 精讀new/get_mut的Unpin bound、Box/local pin、as_ref/as_mut重借、get_ref與unsafe get_unchecked_mut契約；new_unchecked只研究不提供逃生範例。 |
| [std-unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html) | 1–100 | `1cbd3afdd820e31f0cfac823e1f94f422be137a537d06bd4d80290c674284ab0` | 採Unpin只取消pinning限制，未pin的!Unpin仍可移交；Arc/Box handle Unpin不依賴pointee Unpin。 |
| [std-future](https://doc.rust-lang.org/std/future/trait.Future.html) | 1–144 | `e9946081963d9566bc3b0b2fafaec6a7b2901d84f78bd1fc3ba5df3ae253627d` | 採最新Waker、poll應快返、wake後仍未必Ready、完成後不可再依賴結果且safe poll不得UB；spawn handle底層工作可獨立進展，避免所有Future內外都完全惰性的絕對句。 |
| [async-en-pinning-current](https://rust-lang.github.io/async-book/part-reference/pinning.html) | 1–206 | `b5552b4fbd7d80dcaf4add9d4c72d03980ba848b93d54d8dd3b9e68385e0837c` | 全文現行章：採place/value、pinning phases、wrapper/pointee、reborrow與Drop責任；!Unpin不等於每個值必有self-reference，move不保證機械搬動；不教projection crates/提案/static API。 |

中文舊Pin章及RustCourse提到stack固定必須手寫unsafe，與現行std pin!不符，不沿用；!Unpin不等於建立起不可move或每個值實際自引用。外文String raw解參考的swap例不搬進host；本書沿數字地址比較、不讀失配saved。原timer的持鎖wake/不join不沿用，維持本書鎖外wake/完整worker收尾。

英AsyncBook舊Pin網址/原master舊路徑404，追查rename後親讀現行part-reference/pinning.html206行全文；archived英文Pin與Book補充頁只有快取、不列實讀。未把普通Future回傳工廠一概說成完全惰性，也區分14.3 spawn後工作與await JoinHandle。

本機原五頁全文、examples/ch13-futures排除target build產物後全檔、7 SVG source均實讀。13.5/6正式稿及所有同頁source已全文讀；13.1–4及必要11/12前輪本人實讀且hash未變者才承接，精確範圍在local-read-manifest.json。WakeCounter保留Mutex，不引未讀Atomic helper。

root正式開14.1–3後，親讀三頁與freeze14.2/3全Rust/manifests29檔且hash全匹配，記actual-prefix-14.json。不拿未gate14.6/9作必要先備。Option::take將在16.3首次前解釋；crossref87筆為保全提案，不盲換章號。採W11-15-010刪自我驗收尾句，不刪單工作executor真正界線。

## 全章完成採用

正式16.3–5依同一實讀來源與隔離方案完成，沒有新增未研究外部依賴/新課程。Option::take在16.3當節先教，Option::is_none在16.4重新給具體契約；不依賴14.6/9。std Waker的coalescing/不保證公平、driver與Future分工、Drop guarantee、poll完成後不得UB及單工作環境界線都保全。原例的source/test/答案均保留，沒有用外部timer/executor helper代替自己完整source。
