# 第14章獨立銜接與實頁初判（sealed）

初讀snapshot：2026-10-15T10:16:18.997863+00:00；封存：2026-10-15T10:35:13.192740+00:00。結論：**必要1、可選2**，請root逐項裁定。非作者、technical或firstreader；未讀他角14結論、author validation、root dispositions/known audit。已實讀1–12前綴，另按13.1–13.8順序實讀全部正文與140個實際來源檔案，只為本章先備，沒有挪用13 reviewer結論。第16章由另一位reviewer負責，本章未用未讀後文補缺。

範圍：14.1→14.9全部正文、108個唯一actual include檔（Rust/helpers/tests/fixtures/answers/manifests/獨立lock/expected）、24個named region與完整載入閉包。共123個來源hash初讀到封存皆相同；兩份未載入README不作教學/validation。108次完整include各在1280×800、390×844 actual DOM比對全相符。30筆四題證據在JSON；Q2機制與獨立需求分開，引文逐行取自frozen原文，其他文字為本reviewer的判斷。

## Findings

### C14-01｜必要｜Duration首次by-value重用前，補明這個標準型別的Copy契約

位置：docs/chapters/14-05-lesson.md:9；在:52完整timing_outcomes之前。首次必要操作：14.5:52載入的timing_outcomes.rs:9把limit按值交給timeout，:15再次交同一limit；同節two_steps的delay也交兩次sleep。

原文：

> `docs/chapters/14-05-lesson.md:9` 標準庫 **`std::time::Duration`** 表示時間長度；`Duration::from_secs(1)` 建立1秒，`Duration::from_millis(20)` 建立20毫秒，1秒等於1000毫秒。`duration.as_millis()` 回傳總毫秒數，供本例顯示，會捨去不足1毫秒的部分。

> `docs/chapters/14-05-lesson.md:43` **`tokio::time::timeout(limit, work)`** 接手本例的一份 future，建立帶期限的等待；呼叫時就記下 deadline，緊接著 await 取得 **`Result<T, Elapsed>`**。T 是原工作 Output，Elapsed 是 Tokio 的逾時錯誤，也能用 Display 顯示。

> `docs/chapters/14-05-lesson.md:52` --8<-- "examples/ch11-async/stages/timing-steps/examples/timing_outcomes.rs"

> `examples/ch11-async/stages/timing-steps/examples/timing_outcomes.rs:7`     let limit = Duration::from_millis(20);

> `examples/ch11-async/stages/timing-steps/examples/timing_outcomes.rs:9`     match timeout(limit, ready).await {

> `examples/ch11-async/stages/timing-steps/examples/timing_outcomes.rs:14`     let unavailable = pending::<i32>();

> `examples/ch11-async/stages/timing-steps/examples/timing_outcomes.rs:15`     match timeout(limit, unavailable).await {

前置位置：

> `docs/chapters/06-03-lesson.md:39` 再回到只有兩個 i32 欄位的摘要。第 3.2 節說過：整數已有 Copy，自訂 struct 不會因欄位都是整數就自動 Copy。

> `docs/chapters/06-03-lesson.md:41` **`Copy`** 允許在原本會 move 的動作中，隱含複製值本身的資訊。對這個數值摘要，`let second = first;` 之後兩份摘要都能使用，不必另外呼叫 clone。這項複製不能由實作者插入另一段任意工作。

> `docs/chapters/06-03-lesson.md:57` second 的數量加一，不會修改 first 的數量。這次保留的是兩份可獨立修改的數字欄位；Copy 的判準是型別的複製契約，不能從實際資料放在 stack 還是 heap 推出。

判斷：目前Duration段只教時間長度與建構/顯示；先前Copy規則明說應看型別契約，不能從數字欄位/stack推出。讀者若按普通非Copy值追ownership，第一次timeout後會判limit已移走，與下一次使用直接衝突。這是本章當下例子所需，18.8才教不能回填。

最小建議：在14.5 Duration首次段加一句它具備Copy，因此把同一Duration按值交給sleep/timeout後仍可再用原binding；不需任何內部表示/記憶體布局。

具體收益：讀者能用已教的ownership規則正確追兩次呼叫，不必由可編譯source倒猜型別能力。

版本/曝光：此候選在10:22:51.314128UTC自己的progress已記錄；final seal前收到root關於18.8改正回指的同題提示。本判斷的first-use/舊規則證據先已自讀，曝光限制保留。

### C14-02｜可選｜外部錯誤型別的Debug／Display能力可用短語補齊

位置：14.3:23/52首次介紹Tokio錯誤型別、14.5:43首次介紹Elapsed。首次必要操作：14.3:39的started.send().await.expect使用Tokio SendError Debug；:63的{other:?}使用JoinError Debug，:109 collect_one中的error.to_string使用JoinError Display；14.5:136 clocktests對Elapsed expect要Debug。

原文：

> `docs/chapters/14-03-lesson.md:23` **`mpsc::channel::<T>(capacity)`** 建立有界緩衝區，交回 `Sender<T>` 與 `Receiver<T>`；capacity 必須大於0。Sender 可以 clone，共用同一條 channel；Receiver 由一個接收者持有。`sender.send(value).await` 等待空位，成功交回 Ok(())，收端已關閉則交回帶原值的 SendError。`receiver.recv().await` mutable 借用收端：有訊息得到 Some(value)，暫時沒有訊息但仍有送端時繼續等，所有送端結束且緩衝區已空才得到 None。

> `docs/chapters/14-03-lesson.md:39` --8<-- "examples/ch11-async/stages/tasks-basic/src/lib.rs"

> `docs/chapters/14-03-lesson.md:52` **`handle.await`** 取得 `Result<T, JoinError>`。本例 T 已經是 `Result<i32, WorkError>`，所以有兩層：

> `docs/chapters/14-03-lesson.md:63` --8<-- "examples/ch11-async/stages/tasks-basic/examples/join_handle.rs"

> `docs/chapters/14-03-lesson.md:109` --8<-- "examples/ch11-async/tasks/src/lib.rs:collect-one"

> `examples/ch11-async/stages/tasks-basic/examples/join_handle.rs:16`         other => panic!("本例應正常回傳金額：{other:?}"),

> `docs/chapters/14-05-lesson.md:43` **`tokio::time::timeout(limit, work)`** 接手本例的一份 future，建立帶期限的等待；呼叫時就記下 deadline，緊接著 await 取得 **`Result<T, Elapsed>`**。T 是原工作 Output，Elapsed 是 Tokio 的逾時錯誤，也能用 Display 顯示。

> `docs/chapters/14-05-lesson.md:136` --8<-- "examples/ch11-async/stages/timing-clock/tests/clock_and_results.rs"

前置位置：

> `docs/chapters/08-01-lesson.md:69` | Result 的 `expect(message)` | 交出 Ok 中的值 | panic，包含 message 與 Err 的 Debug 內容 |

> `docs/chapters/08-05-lesson.md:121` **`ToString`** 的 `to_string(&self)` 借讀值，交回owned String。標準函式庫替有Display的型別提供它，不必自己實作ToString。這與 `format!("{error}")` 一樣保存其Display文字，單一值不必再寫格式字串。

> `docs/chapters/13-05-lesson.md:85` 下面會用8.1的 expect 檢查已知條件。expect 要求錯誤 E 符合 Debug，以便失敗時產生診斷；標準函式庫的 `SendError<T>` 與 RecvError 都有 Debug，因此這裡可以使用它。

判斷：錯誤的角色/分類與expect的保證已說清，所有source完整；這些只是在已教format/expect上使用外部型別，沒有新的控制流，故不升必要。不過13.5已明說的是stdmpsc錯誤，不能當Tokio同名型別契約。14.7對url/reqwest錯誤已有同款Debug/Display短說，前面也補可省讀者查API或自行猜。

最小建議：Tokio SendError說明補Debug；JoinError首次說明補Debug/Display，Elapsed現有Display改Debug/Display；不用教trait實作內部。

具體收益：能就地核對expect／格式化／to_string為何可用，保持章內外部型別介紹一致。

### C14-03｜可選｜將Client的Send／Sync短說提早到第一次spawn它之前

位置：14.9:148完整main前；既有能力句目前在:189。首次必要操作：14.9:148 main的tokio::spawn(async move {...inspect(&client...).await})第一次把owned reqwestClient保存在spawn工作；既有ClientSendSync說明在後面的gates tests導讀:189。

原文：

> `docs/chapters/14-09-lesson.md:144` 

> `docs/chapters/14-09-lesson.md:146` 

> `docs/chapters/14-09-lesson.md:148` --8<-- "examples/ch11-async/local-service/src/main.rs"

> `docs/chapters/14-09-lesson.md:189` 這次提交/held/0與/held/1。**`client.clone()`** 交出另一個Client值，共用內部連線池與設定，不是重新建另一份獨立pool；Client支援Send／Sync，owned client能移進spawn工作。

> `examples/ch11-async/local-service/src/main.rs:12`     let address = fixture.url("/held/0");

> `examples/ch11-async/local-service/src/main.rs:13`     let request = tokio::spawn(async move {

> `examples/ch11-async/local-service/src/main.rs:14`         let url = parse_http_url(address.as_str())?;

> `examples/ch11-async/local-service/src/main.rs:15`         inspect(&client, url, true).await

> `examples/ch11-async/local-service/src/main.rs:16`     });

前置位置：

> `docs/chapters/14-03-lesson.md:11` spawn 要在 Tokio runtime 內呼叫。本章仍用 current_thread，所以這些 task 共用該 thread，不能以建立多份 task 就宣稱 parallelism。這個 spawn API 仍要求 future 與 Output 符合 **Send + `'static`**：第13.1節的 Send 允許跨 thread 交付；`'static` bound 表示不攜帶只在短範圍有效的借用，不要求值必須活到程式結束。owned String 可以符合。即使這次選單一 thread，使用的 API 契約也沒有因此免除這兩項限制。

判斷：14.3已說future/Output Sendstatic，main目前可從已完整介紹的API直接讀出控制流，不需探索reqwest/連線池內部，所以分可選。若要讓每項外部能力就地可核，現成短句提早可回答ownedClient如何符合跨等待spawn契約；clone共享pool的額外內容仍留gates處。

最小建議：把既有「Client支援Send／Sync，owned client能移進spawn工作」短句移到完整main前，不增加新功能/新抽象。

具體收益：讀第一次spawn client時即可對上14.3的契約，不必讀到後面才找到明示。

## 正向證據與負載控制

14.1固定A庫存3、B80+120=200，比等待安排時不偷換任務，也沒把B當完整報告。14.2先分asyncfn本體與呼叫引數、普通future工廠，再介紹入口/blocks/Result/測試；早期no-lib套件不編譯未教library。14.3 started/release與nestedResult、Id對照/取消回收全部先說再展示；14.4上限是尚未回收tracked數，不是CPU平行數或全部記憶體限制，0/empty與單槽答案完整。

14.5合作式timeout、已改不回滾、std阻塞與Tokio虛擬鐘分清，兩種預算使用同兩必要步驟。14.6圖把W1借用等待、S1集合與T0工作拆開；short guard→完整獨立負例→batch受控完整基本版→真訊號，順序不靠完整最終source偷帶未教signal；先查queuedstop、abort後drain、unstarted逐筆drop，completed結果保留及current_thread證明界線皆清楚。listener entered、工作started與READY不混用，兩份controller/observer都有收回。

14.7 HTTP收到回應與2xx通過、未讀body與空body、Request與帶status的Body失敗、blocking分階段10秒與完整body/UTF8政策均在實作前。14.8先保同政策改async，普通send工廠時點、totaldeadline、stdelapsed邊界、pool可重用但不保證效能皆明說；JSON再驗HTTP/body/schema順序且PageRecordowned。14.9先最小實際200與有限停止，再Fixture/State/Path/gate/Optiontake/控制與收尾；每個測試及main的失敗分支全讀，先收client/server後assert，pause只在真網路前置後，沒有拿entered當headers或join證據。未要求16章poll、Pin、服務內部Future表示；目前只使用已提供Future/IntoFuture契約，深層實作非此刻必要。

## 實際頁面

8102 preview九份canonical copy各與snapshot相符。初次capture10:27:41–10:28:23 UTC，補拍時間在extra-metadata。桌面1280×800與手機390×844共80張原圖，全部組pair以view_image親看，精確檔名在JSON/visual metadata。必要三張圖均看實頁文字與箭頭，不只SVG檔存在：14.1三安排圖含bottom另拍，14.4回收補位圖、14.6借用等待圖，各框/數字/先後指向清楚。手機圖343px，文字約19.35px；正文不被圖撐寬。

10張表九張手機fit343px；14.3三欄四筆分類需最多25px小量橫移，欄位與主要語意仍同屏，未升新增修改建議。九頁details皆0，完整程式與答案普通inline；長行pre可獨立橫移，未將預期code橫移當正文裁失。

初次抓圖有五個近似搜尋fallback或同名後段，檔名歷史保留，不假稱其標籤精確；另外以全檔DOM相等定位補拍14.3完整tests、14.5首個Duration重用與隔離clocktests，已親看。TOML marker normalizer最初少去`# --8<--`造成8個manifest假不匹配，補正後重訪九頁actualDOM，108完整include各尺寸均吻合；教材沒有因此改稿。

## 限制與封存

不宣稱blind首讀/整書pass；未執行Cargo、教材命令、build或strictsite。parent事前告知14.7/8作者自驗句窄修，本初判使用已修current，不倒算舊版。Duration候選早在自己10:22:51 progress記錄；final seal前root告知18.8修正回指並提到14.5未教Copy，這是同題曝光，已即時回報，不隱瞞也不把18.8當先備。其他14 peer finding/validation未讀；尚未讀18B正文。所有123 source hash封存時穩定。後續採納窄核另存，不改此初判。
