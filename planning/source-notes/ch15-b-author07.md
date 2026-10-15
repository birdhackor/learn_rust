# 15.7 B1 作者來源：一份 URL 與一份 owned 結果

本輪只寫15.7。依clear-tutorial／writing-prompt／Rust術語指引；A全部artifacts不動，15.8等新actual與授權。初期B研究在ch15-b.md，由另一作者保存，本筆記不改其原研究時點。

## 實際前文與固定需求

- 重讀actual15.1 entry-v1六BODY：單步起始版／完成快照／完整參考版用途、不累加stages、repo外固定toolchain、真測試不把零項算能力。
- 重讀actual11.7 single-http-v8九BODY全部：sharedClient／ownedUrl、OutputResult、send ordinaryfuturefactory、headers與body界線、client與timer／量測分工、typederrors／imports／test範圍。不是拿自己的舊稿代替root最新gate。
- 重讀actual11.8 local-service-v13全12BODY及whole8packagefiles：listener持續owner、ready真HTTP、固定GET回應／redirect、hits／held notification與gate、pause/resume先ack後clock、clienttask與server收尾各自等待。lib/main/examples2/tests2/Cargo/README都實讀；不能只靠Fixture方法表當黑盒。
- Actual11.4 timing-v4全9BODY已在本session11.7寫前實讀並補讀被截斷的units，收據在author-ledgers/ch11-author07及source-notes/ch11-author07；root明許沿此receipt，不重做同版閱讀。timeout兩層Result、Duration/Instant、外層Elapsed、先inner機會、取消不撤銷、pausedclock與timerscope為本頁最低前文。
- v2 B固定政策：GET／最多5redirect／final2xx、只到finalheaders、預設每項10秒以Tokio timeout包完整headers工作；invalid是item結果。B1保存單筆原文，inputindex/dups與讀檔在B2，並行/取消/CLIJSON後節。client初始化失敗仍是準備失敗，不能假稱某筆URL已網路失敗。

## Fresh EN／ZH 對應教學

| 來源與當次範圍 | 採用的教學關係 | 避免直接搬入 |
| --- | --- | --- |
| English Book17.1完整body15–253及簡中17–267 | 先一頁HTTP工作與await Output，再組合；headers和text是不同完成界線，owned資料仍按規則流動 | 本題目的只有headers，停在第一步；不搬trpl便利介面、HTML選擇器、race/Either/select、Pin/poll，沒有以公共站title當固定答案。最初合併工具輸出截掉英語60–107與中文後段，另開80/110/220補實讀，不把截斷算全讀。 |
| Tokio UnitTesting的pausedclock完整相關段18–64；networkmock段亦讀供比較 | 等待條件由事件/受控timer提供，不用真毫秒推測 | 不搬tokio-test/AsyncRead/AsyncWrite/Unpin或未教mockbuilder。沿actual11.8先真handler ack，再pause/resume；stdInstant耗時仍可變。 |
| reqwest0.13.5 RequestBuilder／ClientBuilder正式selected段與精確cache | GET建builder→sendfuture→finalResponse；redirect政策與Deadline來源分開 | B1不直接用11.7帶reqwest.timeout的helper，不另加內層期限，也不讀text／bytes。原Clientbuilder/Url能力沿已教路徑，工作timeout用actual11.4。 |

URL：English https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html；簡中 https://kaisery.github.io/trpl-zh-cn/ch17-01-futures-and-syntax.html；Tokio https://tokio.rs/tokio/topics/testing；固定reqwest https://docs.rs/reqwest/0.13.5/reqwest/struct.RequestBuilder.html#method.send 與 https://docs.rs/reqwest/0.13.5/reqwest/struct.ClientBuilder.html#method.redirect。

## selected固定版查核

- reqwest0.13.5 client.rs2532–2547確認get直接選MethodGET；request.rs497–527確認send保存ResultResponse，且包含redirectloop/limit失敗。redirect.rs127–147排除初始URL，5次後第6次轉向拒絕；11.8的loop總hit6也支援此階段驗收。
- ClientBuilder正式timeout/readtimeout/connecttimeout預設None已重讀，不用「刪去11.7 timeout呼叫」卻暗留第二份預算。no_proxy selected body清除自動代理；只在已控制loopback測試明選，一般client保留原設定。
- Tokio1.53.3 timeout固定source／11.4 actual支持work Output為ResultResponse時套Result<ResultResponse,Elapsed>；B1將各路轉單筆Outcome，不讓問號把item錯誤提早交給整批caller。
- 真headers-only需清楚source只有get/send/status/url，保存owned資料後dropResponse，不呼text/bytes。11.8既有fixture body都是立即小字串，只能驗它已教的headers/gates／分類；另做有限作者host held-body probe，不把未知body-stream API放學生tests或假稱Fixture已能控制body。
- Starter只有固定尚未完成結果，checks另手動加入；錯碼可能沒有entered通知，5秒test觀察guard有界等待失敗後abort＋await/shutdown，與被驗的10秒工作期限各有角色。這些收尾API來actual11.8，不是B工具取消功能。

## 有限錯碼驗收的補充契約

固定docs.rs JoinHandle頁本次不可取，沒有記為已讀。改fresh讀官方Tokio1.53.3 tag的`tokio/src/runtime/task/join.rs`，selected lines20–34（cache22–36亦親讀）：借用`&mut JoinHandle<T>`等待保留owner及未取得的task Output；丟掉handle則detach，不停止task。來源：https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.53.3/tokio/src/runtime/task/join.rs 。本文只教具體borrowed handle作Future、Output相同與逾時後另abort／await，不教Future blanket／Unpin／Pin。Root明確採納此最小新增契約。

只守entered通知的5秒guard還不夠：合法HTTP已能送出、但讀者漏Tokio工作期限時，held永遠不放行，結果測試會一直等。因此在真handler ack後pause，用20秒虛擬觀察guard等待`&mut request`；guard Err先resume，再abort／await及server shutdown。guard成功已取得Output就不從同handle再取一次。它不是正式client第二份期限，stdInstant真耗時也不判虛擬10秒。已實際驗無期限版本5pass/1fail，診斷清楚且有限收尾。

Tokio UnitTesting頁補開45起後段，18–64相關段全部已實讀；不把第一次工具截斷的可能段落當已讀。

## 作者期補查與採納

ROOT 先讀完整 B1 稿時指出 SVG 雖寫解析失敗，箭頭卻只有經 timer 的主線；採納為圖解修正。補橘色 `Input／None → Check` 早退支線、藍色原文保留線與 HTTP 專用的保存／drop 說明；`Request／Timeout` 沒有「最後」回應，不誤指 redirect 過程完全沒有任何 HTTP 回應。正文圖說與箭頭相鄰。沒有新增 API、功能或改 Rust；此為作者期 precheck 發現，沒有冒稱獨立 QA。

額外作者 headers-only probe 的原始程式、建置 log 與 JSON 在 `/workspace/work/tutorial-implementation/ch15-b1-author/headers-probe.py`、`headers-probe-build.log`、`headers-probe.json`。Python 真 loopback server 發 `Content-Length: 5` 與 headers 後守 body gate；Rust 呼叫實際 `url_b1::check`。binary 的三秒 host 上限內返回 HTTP200，body gate 尚未開；handler 的 body 等待最多八秒，finally 才放行並關閉 listener。另送 `/five/0` 至 `/five/5`，五次 redirect 得到200；紀錄共七份 GET。這是額外有限 host 證據，不是學生 `Fixture` 能控制 body 的能力，也不把 gate 尚未開推成「網路完全沒有 body bytes」的一般主張。

ROOT 補查初版 probe 時指出：`body_release` 沒有 set 本身不能排除八秒等待自行到期後開始寫 body。採納為證據強化，舊 script／JSON／build log 以 `.before-body-state-fix` 保留。只在 work-only host probe 加 `body_started`（真正寫之前）與 `body_sent`（寫完 flush 後）Event；新的有限 probe EXIT0，check 完成時 `headers_sent=true`、`body_started=false`、`body_sent=false`、gate 尚未開。不靠時間不等式推論 body 狀態；沒有新增學生 API 或重跑六 tests／引導路線。
