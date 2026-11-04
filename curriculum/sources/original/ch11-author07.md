# 11.7 作者來源：把單一 HTTP 工作接到 async

依clear-tutorial／writing-prompt／Rust術語指引；本筆記供作者／技術審閱，不作首次閱讀提示。範圍只有11.7，11.8服務與B實戰須另取得actual與授權。

## 已實讀 actual 前文

- ch11-foundations-prerequisite-v3全9units：async fn的future與Output、ordinaryfactory即時工作、await與Result分開、capture/lifetime、current-thread main/test。不借11.2／11.3的task或並行。
- ch11-timing-prerequisite-v4全9units：Duration/Instant、timer／timeout、兩層Result、取消非回復、阻塞不可硬切、pausedclock與整段／每步期限。不用Tokio pausedclock假裝實際網路完成。
- ch11-http-prerequisite-v2全10units：完整URL/HTTP/redirect/body/status語意、同步reqwest及features、parseHTTPURL、typedRequest/Bodyerror、writer/CLI、offline邊界與練習。
- ch07-review-fixed-v13依manifest第7頁全7units：ownedSerde、Deserialize借入文字來源、from_str與typedJsonErr、unknownfields與實際模型判準。
- numeric suffix已在前次任務補讀ch10-thread-suffix-prerequisite-v13/001-005，本節不用新suffix。全局工具首次web搜尋誤用search_query而回isError，改tool正式system2_search_query；未將失敗搜尋當已讀。

## Fresh 英中教材實際閱讀

| 來源與範圍 | 可採關係 | 本節取捨 |
| --- | --- | --- |
| English Book17.1完整body15–253／簡中完整body17–267 | request與讀body是兩個await；lazy async wrapper／runtime／普通入口負責驅動；等待保留ownership規則 | 不搬trpl包裝的無Result介面、HTML/CSSparser、環境argv索引、race/select/Either或Pin/poll。前文已教Tokio main，用真正typedreqwest，不把await必暫停當規則。兩份是同章翻譯，不作兩個成效證據。 |
| Tokio Tutorial overview23–61全body／HelloTokio23–175完整教學body | 正常流程可逐步await；runtime提供I/O與timer；async函式呼叫尚未執行本體 | overview明說只送單一webrequest不比blocking有實用優勢。本節是建立可組合介面及下一節受控驗證先備，不宣稱本次更快。MiniRedis/server/fullfeatures/block_on範例不搬入。 |
| English Cookbook MakingRequests全body15–94 | 同步／async仍取得同一HTTP資料，send/body分步 | 不用anyhow/未知headersformatter/auth/parse_with_params/JSONshortcut。取async段責任，不抄便利get每次建client。中文RustyBook requests.html與clients.html工具不可達，不算已讀；中文真正讀Book17.1及Course下列章。 |
| 中文RustCourse getting-started完整body22–339 | 未await子future不執行；runtime與語言功能分工；順序等待仍有先後 | 不搬通用效能數字、框架比較、async零成本泛化、join/spawnblocking或Future底層；其中「Rust不允許trait async fn」是舊敘述，不作本書事實。本節單一I/O只教await和結果角色。 |

URL：English https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html；簡中 https://kaisery.github.io/trpl-zh-cn/ch17-01-futures-and-syntax.html；Tokio https://tokio.rs/tokio/tutorial 與 https://tokio.rs/tokio/tutorial/hello-tokio；Cookbook https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html；Course https://course.rs/advance/async/getting-started.html。

## 固定版本 selected API 契約

- reqwest0.13.5 Client／ClientBuilder／RequestBuilder／Response docs.rs實開並實讀selected段：Client sharedborrow get接受ownedUrl；Client持連線pool且建議reuse，不需要外加Arc。Builderbuild同步回Result；send是ordinaryfn交implFuture<Output=Result<Response,Error>>；text是asyncfn接手Response，Output為ResultString。沒有將.method.text()當成已讀body。
- ClientBuilder.timeout：從request連線開始到body完成共用總期限，預設無timeout；不把11.6blocking的send/text各自預算抄過來。cache精確async_impl/client.rs2645–2700及3083–3158已再核：send建立timer、Response取得同一total_timeout，body接續而不重建。需runtime/timer且立即await，不保存future很久再當新完整預算。底層Pin/poll只作者查核，不向學生展示或算已教。
- ClientBuilder.redirect／Policy::limited(5)沿既有HTTP契約；no_proxy沿前文，只有讀者明選direct時用，作者外網probe不繞過環境proxy。ClientBuilder預設redirect10，本節明選5。builder/setup失敗、Request未取得回應、Body已有status但讀取失敗依角色明包，不靠同一From模糊來源。
- text的charsetfeature關閉只嘗試UTF8，不合法序列替代為�；精確response.rs125–178核非charset分支。不擴BOM行為、不當嚴格binary保真、沒有body大小上限。本節讀已知小頁，預設不讀body。
- reqwest0.13.5Cargo.toml的rustls、blocking、charset/json features及本機Tokio dependencies net/time已核。本節直接只要求rustls，Tokio1.53.3rt/macros/net/time：net為網路I/Odriver，不因此教TcpListener；time供請求timer，current-thread入口啟用所選能力。其他依賴feature合併沿9.1，不以default-features=false聲稱整workspace絕無其他能力。
- Response.status/url與StatusCodeCopy/as_u16/is_success、Error.is_timeout/Debug/Display在11.6 actual已教；今次asyncResponse選同契約。固定docs.rs.ClientBuilder.0.13.5 timeout與exactcache核讀，URL API沿url2.5.8前文不改。
- JSON練習只用已教ownedString.as_str＋serde_json::from_str<PageRecord>；不選Response.json的DeserializeOwned或reqwest jsonfeature，無HRTB／server/stream helper。

官方selectedURLs：https://docs.rs/reqwest/0.13.5/reqwest/struct.Client.html、struct.ClientBuilder.html#method.timeout、struct.RequestBuilder.html#method.send、struct.Response.html#method.text。同版cache /workspace/.learn-rust-tools/cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.13.5。

## 接續實作與補核的能力

- Tokio main future的Output本次改ExitCode。固定tokio-macros2.7.2/src/entry.rs441及493–572實讀：去掉signature的asyncness、保存原回傳型別、runtime enable_all並回block_on的成果；正文在firstmaincode前用使用契約說明Output交給普通入口。學生不讀Pin、syn或macro底層。
- redirect.rs126–153精確limited判定已核，第一個URL不計redirect；5個redirect可到最後結果，需要第6個回Error。client的一個totaltimer包住整份send所經FollowRedirect，不對每次redirect重給時間；正文期限段先說一次send的意思。
- 已實作ownpackage／BODY／期限SVG與ownedJSON練習。6個學生離線tests不偷放server API；作者另外用有限host本機probe取得headers/body/timeout/redirect分類證據。host scratch不供學生複製，未把它當11.8已教。書頁真正圖文／提示／答案與執行範圍見ledger；本作者不聲稱releaseQA。
