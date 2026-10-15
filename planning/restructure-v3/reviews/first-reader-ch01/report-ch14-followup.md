# 第 14 章修訂後複讀

只讀正式釋出的 `reader-ch14-followup.md` 六段短更新，使用實際既讀前文與例子；未開其他報告、repo、queue、原 freeze JSON 或外部資料，未執行程式。本輪有明確提示且發生在 232–311 原首讀封存之後，不能算新的盲讀，不能反證原初判當時已懂。原報告及 records 本輪均未修改。

## 同一份 limit 交給兩次 timeout

文字現在明說 Duration 具備 Copy，按值傳給 sleep／timeout 後原 binding 仍可用；timeout 在每次呼叫時記下 deadline。我的理解是：複製的是「時間跨度」，不是共用同一個已開始倒數的期限。因此兩個不同工作各呼叫一次 `timeout(limit, work)` 時，同一個 limit binding 能合法再用，各自依呼叫時點建立期限。

用途是理解既讀「整體一次預算」與「每步一次預算」來源為何不同，而非只記結果。以既讀兩步各 30 ms、limit 50 ms 為小變化預測：每一步各包一次 timeout 可以完成兩步；整體兩步只包一次 50 ms 則在完成第一步後逾時。第二次可用 limit 並不代表兩份工作可以重複移入同一個已消耗的 future，也不表示兩步總共只用一份 50 ms 預算。

這裡 Copy／每次建立 deadline 的必要依據已補明，沒有仍阻礙該例閱讀的缺口。實際排程耗時不由這段保證。

## expect／to_string 與 Tokio 錯誤

新文字分別明說 SendError 有 Debug，JoinError 與 Elapsed 有 Debug、Display。接上既讀 Result／expect 與第 8.5 節：expect 的成功路徑取出 Ok 值；失敗路徑可使用錯誤的 Debug 產生診斷並 panic。Display 則支持文字顯示及 `to_string()` 保存為 owned String。這說明既讀 source 中這些呼叫為何具備所需格式能力，不必憑「錯誤應該都能印」猜測。

兩層 Result 的區分仍保留：`handle.await` 外層 Err 是 JoinError；若外層 Ok 而工作回 Err，那是工作自己的 WorkError。`timeout` 外層 Err 是 Elapsed，inner Output 若也是 Result，不能把其業務錯誤叫作逾時。channel 的接收端關閉導致 send 回 SendError，也不等於 recv 的 None。

我的小預測：把已持有的 JoinError 呼叫 `to_string()`，得到的是錯誤文字 String，不會因此改變任務結果或取消其他工作；判斷取消仍用既讀分類方法。錯誤文字的精確字樣未提供，我不推測，也不需要靠它理解控制流程。對目前 expect／文字保存的必要理解沒有剩餘缺口。

## owned Client 放入 spawn

新文字明說 Client 支援 Send／Sync，且明確接回 14.3 的 spawn Send 要求。結合既讀 `async move` 與 14.9 Client clone：先取得 owned Client（也可 clone 出 owned handle），移入 spawn 的 async move 工作；工作內呼叫 inspect 時借用的是工作自己持有的 client，不是把外部短期借用留給可能繼續存活的 task。clone 所共有的 pool 能力不等於複製網路連線，也不證明此例實際重用了某條連線。

我現在能把「Client 可跨相關 Send／Sync 邊界使用」與「這個 task 實際擁有什麼」分開說。這不是任意 async 工作只要含 Client 就自動符合全部條件；既讀 spawn 還有 future／Output 的 Send 與生命期要求，其他跨 await 保留的值也仍重要，275 的 guard 反例沒有被取消。

小預測：若把某個 owned client 直接移入 async move，再在外面使用同一個已移走的 binding，會遇到已移走值的使用問題；先 clone 一份再移入，可留下外面的 owned client。主例中 client 結果與 server 結果必須分別收妥；Client 具備這些 trait 不會自動替 Fixture shutdown。新段落重申的「失敗先收 client／shutdown server 再 panic；成功先存 server 結果再 assert」讓這項責任仍可追蹤。

對目前 Client 進 spawn 的來源與擁有權路線，沒有仍阻礙理解的必要缺口；這不是編譯驗證。

## 複讀結論與停止

這六段把本次提示的三組操作所需 trait／期限建立依據補成明說。我能依新文字與既讀來源解釋其操作，沒有發現這三組仍缺必要前提。內部實作、精確錯誤字樣與實際網路／時間表現仍未驗證；這些不妨礙目前例子的閱讀，也不因此宣稱全章全部概念通過。原首讀的未知、詞彙曝光及未實跑限制繼續成立。

完成本次修訂後複讀，STOP。
