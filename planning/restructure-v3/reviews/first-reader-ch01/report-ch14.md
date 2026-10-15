# 第 14 章 staged 首讀封存報告

## 範圍與證據界線

沿唯一 reader_feed 依序讀完 units **232–311，共 80 單位**：14.1（232–238）、14.2（239–247）、14.3（248–255）、14.4（256–262）、14.5（263–271）、14.6（272–280）、14.7（281–288）、14.8（289–295）、14.9（296–311）。每包完整閱讀、保存自身四題／引文／預測／未知後，才取下一包。records/232.md–311.md 保留；311 後 feed 明確回覆 STOP，未讀第 15 章，僅見章末標題／連結。

背景是基本 Python 加實際讀過的 1–13 章。本輪是 AI 角色閱讀，不是真人驗證、實跑或技術稽核。全部當包 source、manifest、鎖檔、fixture、tests、fold 與同包答案均已讀；答案同包可見，不能稱隔離解題。未執行教學、安裝／編譯／測試、HTTP 請求或 OS signal；未讀 queue、repo 章頁／來源、作者 ledger、其他 review 或外部資料。未發現未釋出正文曝光。先前普通整頁先備閱讀沒有改稱逐概念盲讀。

## 自帶詞彙曝光限制

不能聲稱完全盲讀或零既有 Rust／async 詞彙。以下原句保留，更正只追加，沒有倒寫初判：

- 246 後進度訊息的「poll/waker」、273 開頭的「pin/poll」是自帶詞彙，當時正文未教。它們用來排除未知內部內容，沒有被用來補答機制，但說出術語本身已是曝光。273 於讀完 295 後追加更正，root 已知悉。
- 275 鎖檔的 `pin-project-lite` 是實際教材依賴名稱，不是 Pin 教學，不能倒算為 273 的依據；275 自己寫的「不提前展開 Pin」也有相同限制。
- 311 後自查發現 282 的「握手／憑證」、290 的「socket 等後文 API」也是自帶排除性詞彙；已分別追加更正。298 後來教 `SocketAddr` 不能倒算為 290 已教。

沒有因上述詞彙讀外部資料，也沒有用它們推演未教機制。這仍限制首讀證據的純度。先前 190／194 的誠實更正維持原樣。

## 首次理解與後釐清路線

下列只指出路線與來源；各概念的原始四題、引文、當時猜測仍以 records 為準。

| 單位 | 當時恢復的必要規則／後釐清 |
| --- | --- |
| 232–238 | 先保存 A 未完成狀態、做獨立 B，最後仍需兩者。236 圖把等待期間安排與額外資源真正同時工作分開。Future／執行工具在這時只是責任預告，尚不需要內部機制。 |
| 239–247 | 240 正式區分 future 值、Output、async 本體延後、參數立即求值／移動；241 才給 runtime、current-thread 入口。244 修正「回傳 future 的普通函式也可能先做準備」；245 await 不會 unwrap Result；246 的完整來源／測試明示未 await 的本體不會因 Drop 自行執行。 |
| 248–255 | 249 的 spawn 是提交而非同步進入本體；250 的 started／release 通道給事件證據。251–253 分清工作 Result 與 JoinError、abort 請求與 await 確認、Id 與輸入位置、完成順序與輸入順序。254 的固定次序來自逐一控制，不是 JoinSet 一般保證。 |
| 256–262 | 上限是已提交且未收集；已完成未收集仍占位。259 的 collect 後 refill，260 圖／表才逐步確認。259 我先預測收 input 1 的錯誤後追蹤 0/2、submitted 3、peak 2，260 再確認。零上限、空輸入及後續 sender 存活各有來源。 |
| 263–271 | timeout 持有並可能丟棄直接的 future，不回復已完成修改。265 先預測負值工作會是外 Ok／內 Err，266 才明示。268 的同步阻塞例限制了硬 deadline 解讀；269 分清 Tokio 虛擬時鐘與實際時間；270 的整體預算與每步預算不同。20/50 的自我預測在 271 答案前已保存。 |
| 272–280 | select 丟的是其他等待 future。273 尚需「丟 waiter 是否也移除任務」的具體依據，274 的 S1/T0/W1 圖與 API 契約才支持這個 JoinSet 情況；未推廣到所有 API。275 guard 範圍與 Send fixture、276–278 stop→abort→drain→Unstarted、279–280 listener／READY／清理後 exit code，逐步補齊必要事件順序。 |
| 281–288 | 282 才引入 URL／HTTP／HTTPS 的必要角色。285 區分 response 與請求失敗，blocking send/text 各自 timeout；286 先預測 bytes 已接受仍可能 flush 失敗，287 RefuseFlush 來源才確認。404 是可用 response／passed=false，不自動變 AppError。 |
| 289–295 | async send/text 的 future 與同一份完整請求預算；elapsed 觀測範圍；client 重用能力不等於已測得重用。294–295 的明確 JSON 解析產生 owned PageRecord，能在原 body/report 離開範圍後使用；HTTP／body／JSON 錯誤有既定順序。 |
| 296–311 | handler、router、同一個已綁 listener、serve、shutdown 各有責任；IntoFuture 在 299 首次正式介紹。301–305 Fixture 分開 ready、entered、release、stop、server join。302 的 loop hit 數尚未知，306 明示 5 次跟隨加首次共 6 才補上。308 是 headers 前 Request timeout；309 觀測通知缺席的 timeout 不能取代實際 redirect error。310 取消 client 不停止 server；311 關 gate 而保留 client，得到 HTTP 503，與取消／RequestError 不同。 |

## 必要圖確實查看

只開當包 metadata 列出的三張 PNG：

| 單位／圖 | 實際讀出的對應與限制 |
| --- | --- |
| 236 `ch14-waiting-arrangements.png` | A 最終 3、B=200；三條路徑的箭頭是工作先後。第三條另假設兩份資源及實際同時工作。圖高不是時間，也不證明加速。 |
| 260 `ch14-bounded-refill.png` | 追蹤 0/1→收 1 的 WorkError→只剩 0→補 2 成 0/2；同一個 0 持續存在，收集在補位之前。 |
| 274 `ch14-borrowed-wait.png` | S1 擁有 T0，W1 只可變借用 S1；stop 丟 W1 後 T0 留存，再放行／等待得 Value(80)，最後清空。 |

## 未解、推論與可選內容的區分

**當時必要而待補的理解**沒有倒判為已通過：例如 240 尚不知誰推進 future（241 補）、273 丟借用 waiter 的後果（274 補）、302 redirect loop 計數（306 補）。所有後釐清保留時間線。狀態表和 API 契約讓我能追蹤範例，不表示理解執行器或協定內部；Q2 的用途不能代替機制。

**閱讀至末仍有限的理解**：runtime／executor 的內部實作未教，我只按公開契約理解。URL query 完整語法／解碼、TLS、OS 網路及 signal 內部也未教到完整程度；對這組明確 library 呼叫屬可延後內容。u16／u128 與既讀整數型別的部分關係是命名推論；as_millis→u128 的實際指定有後來完整來源作依據，不能冒稱已學全部底層性質。

**驗證上的未涵蓋**：308 讓 handler 在 headers 前卡住，只涵蓋 Request 階段；Body 階段 timeout 是正文契約，這組 fixture 未另造 headers 已到、body 未完的案例。client pool／clone 的能力不證明同一 TCP 連線真的重用。Ctrl-C、外部站點與所有測試均未實跑。CLI 三行是在結束時印出的回顧證據，不能當印出當下仍在等待的即時觀測。

這些不是對全章的 blanket pass／fail。整章我恢復的問題是把「建立、開始、等待、收集、超時／停止、清理完成」分成不同事件，再按各自擁有者與 Result 層追蹤；單位層級仍以原紀錄的當下理解與不足為準。

**封存狀態：80 單位完成，feed STOP；不修改原始 records，等待另行正式釋出的複讀。**
