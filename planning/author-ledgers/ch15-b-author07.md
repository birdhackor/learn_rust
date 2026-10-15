# 15.7 B1 作者 ledger

## 開寫前安排

實際重讀15.1-v1六、11.7-v8九、11.8-v13十二BODY及whole8files；11.4-v4全九的舊readreceipt沿本session11.7作者記錄，root已確認可用。fresh EN／ZH單筆HTTP與固定API記sources；原A與他人的ch15-b研究不改。

| 必要關係 | 第一次需要 | actual／本文安排 | 能預測的結果 |
| --- | --- | --- | --- |
| 同HTTP能力／不同完成工作 | 入口／clienthelper之前 | v2、11.7已教status/header/body；本題只取finalheaders | 保存Http2xx/非2xx，dropResponse不主動讀完body。 |
| 原文與網路位置分開 | firstmodel | 15.1 B身份、11.7Urlparse/redirect；本節original不改 | original一直對回caller，finalURL可因redirect不同；不在B1做index/dups。 |
| 每筆總有owned結果 | enum／firstcheckstub | 11.7typedResult、3.1nestedpatterns；InputIssue保存Url／scheme | 無效URL是Check，不送網路；HTTP未通過不是Errtransport。 |
| client準備與item期限 | firsthelper | 11.7builder、11.4timeout；本節client不設reqwesttimeout | Clientbuild Err屬準備；Tokio wrapper只包一次headers工作，預設10秒。 |
| 兩層Result與捕捉 | firstcoretaskhint/code | 11.4nestedResult、11.1asyncblock；本文固定三主分支 | wrapper Err逾時、innerErr請求原因、innerOk才有status。 |
| 量測邊界 | firstelapsedfield／source | 11.4stdInstant/as_millisu128；本文input錯誤None/已開始Some | 不用None暗示0ms，真毫秒不固定；parse/build不在request量測內。 |
| 受控觀察與有限失敗 | beforetesthelper | 11.8ready/gate/pause/shutdown與abortwait；本文5秒guard只等通知 | placeholder不送request也會在有界時間fail並收尾；不是猜sleep。 |
| 借用handle等待、漏期限仍有限 | beforetimeoutcheck | fixedTokio1.53.3官方tagselectedcontract；本文先教`&mut JoinHandle<Check>`的Future Output與owner保留 | 20秒guard Err不停止task，resume後仍另abort/await；Ok所得結果不再取一次。 |
| 引導步驟與答案 | firststarter/source | 15.1與既有A里程碑手法；完整artifact先保存，snippet是定位 | 先編譯起點→手動checks真失敗→輸入分類→網路分類→期限→完整snapshot。 |

只寫BODY docs/chapters/15-07-check-one-url.md、b1-starter/b1-complete、checks/README/最小demo及需要的SVG；共享Cargo/nav/lock=root。scope約千行為最終負擔目標，stagecopies不累加，不湊features。

## 15.7 B1 完稿與實際驗收

正文 `15-07-check-one-url.md` 與兩份 literal manifest／README／library、外加 starter checks、完成版 example 已完成。六項 checks 的完整 module 與完成版 `#[cfg(test)] mod tests` 逐 bytes 相同；目前完成版 library 也與已跑過的 repo 外工作副本逐 bytes 相同。完成版 library 290 行、demo 31 行，含 checks；此為單一步快照，沒有把它與 A／後續 B 階段累加成練習量。

保存證據目錄：`/workspace/work/tutorial-implementation/ch15-b1-author`。

| 實際操作 | 結果與證據範圍 |
| --- | --- |
| ROOT 整合兩 members 與 lock | `b1-members-lib-check.{log,json}` 僅 library check，EXIT0；沒有將它誤報 all targets 或六 tests。 |
| repo 外 literal 起始版，僅改 helper 的實際 path | `starter-check.log` all-targets check EXIT0；`starter-zero-tests.log` 零項，未作功能成立證據。 |
| 把完整 checks 追加到起始版 | `checks-stub.log` 0 pass／6 fail；兩項 entered guard 都有限失敗並收尾。 |
| 僅補輸入分類 | `checks-input.log` 1 pass／5 fail，正常輸入仍未完成。 |
| 補 HTTP，但故意不包工作 deadline | `checks-http-without-deadline.log` 5 pass／1 fail；結果 guard 在 paused clock 到期，`JoinError::Cancelled` 與 `server=Ok(1)` 確認有限收尾。保留此失敗，不冒稱 timer 已完成。 |
| 補完整實作 | `checks-full.log` 6 pass／0 fail；`outside-clippy.log` all-targets Clippy、warnings denied EXIT0。指令／cwd／expected exit 保存於 `guided-route.json`，操作程式為 `guided-route.py`。 |
| repo artifact 起始測試與完成版 demo | `canonical-starter-zero.log` 零項；`complete-demo.log` 真 redirect→200、two hits 並 shutdown；port／耗時可變，不拿固定毫秒驗收。 |
| 額外 held-body／redirect 邊界 | `headers-probe.py`、`headers-probe-build.log`、`headers-probe.json`：真 loopback GET，body gate 未開便取得200；五次 redirect 至最後200。host evidence 不放進學生 helper，不冒稱其 Fixture 已教 body gate。 |
| 前文 Fixture 依據 | `predecessor-bytes.json` 原八檔與 actual11.8 v13 全 bytes 相同。 |

斷線恢復後依 ROOT 指示，不重跑未改動 Rust 檢查。此次只補圖與圖說：橘線從解析失敗直達 Check，藍線保留 original，主線僅成功進 timer；HTTP 才保存欄位與 drop，失敗沒有最後 Response。ROOT 發現與採納留在來源筆記，非獨立 QA。

## 作者實際頁面檢查

scratch preview 使用正文／SVG 的實際 bytes 與嚴格 snippets；ROOT 的共享 nav 不改。`docs-build-final.log`：strict build EXIT0、No issues found。實際桌面1280×900與手機390×844，親看入口、模型說明、圖與鄰接圖說、fixture／guard理由、三份期限表、實作三步、三份展開提示與完整答案頭尾，共20張原位截圖。範圍是這些指定位置，不冒稱每張都包含整頁全部正文；手機長程式在 block 內水平捲動，文字／表格正常換行。全部圖片在 `preview/screenshots`，初稿截圖保留於 `screenshots-before-root-branch-fix`。

修後 SVG 原生340×680、字18px，兩裝置沒有 stretch；藍／橘平行走道可追至同一 Check，橘路不進綠框。圖說在相鄰正文解釋各路；HTTP 專用文字與「無最後回應」可讀。`render.json` 的 DOM 寬度為桌面1265／手機375，不超 viewport；SVG text bounds 均落在框的橫向範圍52–324。render 捕捉和 PNG 親看分別記錄，未用 source-only 取代實看。scratch HTTP server／browser 都由 finally 收尾。

ROOT source precheck 與作者操作／頁面證據不是獨立首讀、全第15章 QA 或發布完成。正式 handoff 的學生檔、notes／ledger 與 evidence fingerprints 保存到該證據目錄 `handoff.json`；等待 ROOT B1 actual gate，15.8 正文／程式尚未開寫。

ROOT 最後另指出舊 probe 的 gate Event 無法單獨代表 body 尚未開始寫，因 host 八秒等待可自行結束。保留三份舊證據 `.before-body-state-fix`，僅強化此 work-only probe 的寫入狀態：新 JSON 直接記 `headers_sent=true`、`body_started=false`、`body_sent=false`、gate 關閉，binary EXIT0；初始 GET 加五次 redirect 的七份 GET 次序相同。只重跑此有限 probe（新動態 URL 的小型 author binary 增量 build 1.12s），沒有重跑學生 Rust／checks／練習步驟或增加功能。
