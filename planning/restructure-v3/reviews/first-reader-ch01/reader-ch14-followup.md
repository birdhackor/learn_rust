# 第14章完成首輪後的修訂複讀

這份材料只在000–311初讀完成並封存後交讀者。它是修訂後複讀，不是新的盲讀；原初讀不回寫。以下三頁只有短說明更新，例子、命令和完整來源沒有變動。請按你已讀的前文解釋現在的操作，區分文字明說與你的推論，報告是否還有會阻礙理解的缺口。

## 14.03

來源：docs/chapters/14-03-lesson.md:23

**`mpsc::channel::<T>(capacity)`** 建立有界緩衝區，交回 `Sender<T>` 與 `Receiver<T>`；capacity 必須大於0。Sender 可以 clone，共用同一條 channel；Receiver 由一個接收者持有。`sender.send(value).await` 等待空位，成功交回 Ok(())，收端已關閉則交回帶原值的 SendError；它具備 Debug，可供 expect 在失敗時產生診斷。`receiver.recv().await` mutable 借用收端：有訊息得到 Some(value)，暫時沒有訊息但仍有送端時繼續等，所有送端結束且緩衝區已空才得到 None。

來源：docs/chapters/14-03-lesson.md:52

**`handle.await`** 取得 `Result<T, JoinError>`。JoinError 具備 Debug 與 Display，因此可用診斷格式顯示，也能沿第8.5節以 to_string 保存文字。本例 T 已經是 `Result<i32, WorkError>`，所以有兩層：

## 14.05

來源：docs/chapters/14-05-lesson.md:9

標準庫 **`std::time::Duration`** 表示時間長度；`Duration::from_secs(1)` 建立1秒，`Duration::from_millis(20)` 建立20毫秒，1秒等於1000毫秒。`duration.as_millis()` 回傳總毫秒數，供本例顯示，會捨去不足1毫秒的部分。Duration 具備 **Copy**；把同一份時間跨度按值交給 sleep 或 timeout 後，原 binding 仍可再用。

來源：docs/chapters/14-05-lesson.md:43

**`tokio::time::timeout(limit, work)`** 接手本例的一份 future，建立帶期限的等待；呼叫時就記下 deadline，緊接著 await 取得 **`Result<T, Elapsed>`**。T 是原工作 Output，Elapsed 是 Tokio 的逾時錯誤，具備 Debug 與 Display，可供 expect 的診斷與文字顯示。

## 14.09

來源：docs/chapters/14-09-lesson.md:145

Client 支援 Send／Sync，owned client 能移進 spawn 工作，對應第14.3節的 Send 要求。

來源：docs/chapters/14-09-lesson.md:147

完整 **`src/main.rs`** 先spawn client，等entered，再放行並取得結果；每個失敗分支都先收client、shutdown server，才panic。成功路徑也先保存server結果，再assert，避免assert提前離開使資源責任不明：

請特別追一次：同一份limit交給兩次timeout、expect或to_string讀取Tokio錯誤，以及owned Client放進spawn工作。請用現在文字和既讀source說明，並保留本輪只是提示後複讀的限制。
