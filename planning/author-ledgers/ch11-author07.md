# 11.7 作者 ledger：單一 async HTTP

## 寫前安排

實讀actual11.1-v3全9、11.4-v4全9、11.6-v2全10、7.7-review-v13第7頁全7units；fresh EN/ZH與固定API查核記source-notes/ch11-author07.md。作者前置核讀不是獨立首讀。11.8/B另授權，不提前改其他作者source。

| 必要關係 | 首次需要位置 | actual／本頁解釋 | 能預測的效果 |
| --- | --- | --- | --- |
| 同一HTTP任務／新等待形式 | 入口 | 11.6完整角色，11.1future/result；本頁單一工作為後續組合準備 | 404仍HTTP報告，未得response仍Err；async本身不讓此次更快。 |
| Runtime／client／future分工 | firstmanifest與firstclientcode前 | 11.1入口／11.4time；新net支援網路driver，reqwest非blockingClient | runtime驅動future，client保存政策和可重用連線，不是runtime。 |
| requestbuilder→sendfuture→Response→textfuture | firstrequestsource前 | 前文builder與FutureOutput；本頁新selectedasync簽名與ownership | await後仍Result；status可先保存；text接手Response；None與空body不同。 |
| 當次reqwesttimeout範圍 | builder.timeout前 | 11.4scope；固定async0.13.5總deadline | send→optionaltext共10s，不给text新10s，不含build/parse/render。 |
| 量測與期限起點 | firstInstant／elapsed_ms模型 | 11.4stdInstant/u128；本頁量測request/optionalbody到ownedmodel | 數字依機器；不作固定斷言或性能優勢。 |
| typederror新等待不換原因 | firsterrorcode | 11.6AppError與HTTPstatus區別；同種reqwestErr按不同階段包 | 已有headers的bodyErr保status；await不改Err成panic。 |
| 讀取文字與JSON模型 | 最後練習model/answer前 | 7.7owned/denyunknown/fromstr；本頁HTTP/未讀body/Json三種拒絕 | 2xx不保證schema符合，人工Inspection只證明解码判準。 |
| 自動測試與真網路證據 | firsttests | 7.8CLI／11.1tokio::test；不建server | 離線input/report/JSON checks範圍清楚，11.8才建受控網路學生測試。 |

責任BODY docs/chapters/11-07-single-async-http.md、package examples/ch11-async/http-single(namech11-http-single)、localassets/ch11；root統一member/lock/nav。所有normal/tests/exerciseanswer僅actual或local BEFOREcode。

## 11.7 完成與作者自查

- BODY docs/chapters/11-07-single-async-http.md；source examples/ch11-async/http-single全manifest/lib/config/error/reply/main/tests/cli.rs/examples/page_reply_solution.rs/README；圖docs/assets/ch11/single-http-budget.svg。Root已加member／更新lock，自己的canonicalalltargets check亦EXIT0，log由root保存。本作者不動共享Cargo／lock／nav或任何frozen前文／A stages。
- 入口保留11.6相同HTTP工作與0／1／2意思，說明單一request沒有量測並行收益；runtime/client/future有不同角色。Newselectedasyncsend ordinarymethod→implFutureResultResponse、text async接手Response→ResultString，以及ordinarystatus/url在首次requestcode前教；同一send含redirect→headers→optionalbody共用10秒，與blocking各自等待分清。模型elapsed_ms=u128依actual11.4，量測不含parse/build/render。
- 模組imports於firsthelper前展示短名來源，newClient來reqwest root。Error source與11.6實際src/error.rs逐bytes相同；同種reqwestErr在client/request/body當步明包，bodyErr保已有status；不加新的From壓平。普通Write明確flush，asyncmain OutputExitCode的macro交回契約在firstmaincode前補教並查固定source，不依賴讀者自行猜。
- JSON練習先固定PageRecord／ReplyError與所需判準：HTTP失敗先拒絕、未要求body另分類、owned模型再from_str，範例與tests均人工Inspection，沒有把它說成真正取過網路。顯式String.as_str及已教Deserialize能力，不偷用reqwest JSONshortcut／DeserializeOwned、rawliteral、bytesstream或socket server。
- 初次sharedfmt/check在root加member之前回101/notmember，沒有當source檢查通過；繼續獨立可做工作，不自行改rootmanifest。首次localrustfmt發現CLItest declaration誤寫fn_invalid，已加真正fn；後續shell的docbuild當時PASS不代替rustfmt，修後localfmt實際PASS。初次outsidepackage四欄literal-normalization誤把version子字串也改進rust-version成0.1.0，被Cargo拒絕；改成已固定1.99，失敗保留commands.json，並非學生manifest缺陷。
- Canonicalfmt/check/alltargetsClippy-Dwarnings/6tests＋例子PASS；例子精確stdout筆記：revision=3。學生4library為invalidinputawait、完整givenreport/Someempty、controlledflushError、ownedJSON/HTTP/None/型別失敗；2CLI為help/version與fourinvalidinputs。0main/example/doctests不能當成果count。沒有公開站點固定內容assert，也不以不同實際毫秒猜timeout結果。
- Repo外解開四workspace package欄位到真literal，保留同dependency設定及其餘檔案；independent alltargetcheck、fmt、Clippy-Dwarnings、同6tests、example及build PASS。最後只補imports snippetmarkers，不變code tokens，再拷canonical各source到outside逐bytes核同（Cargo四package欄位差異明示），不因純comment重跑整套Rusttests。證據 /workspace/work/ch11-single-verification/{canonical.json,commands.json,source-bytes.json}。
- 有限Python host loopback server在確實bind/listen後開probe，沒有猜某sleep後應ready；Rust學生package不含這些API。11受控記錄涵蓋headers-only200、完整hello、空body、404有報告status1、5redirect成功／6redirectErr、UTF8replacement、transport無response、未有headers期限、已有200但body期限、Linux/dev/full輸出1。兩deadline保持未完成條件到client真結束再releaseEvent，不以「某sleep猜誰先」作學生驗收；觀察約10秒是紀錄，不作精度斷言或總CLI硬deadline承諾。
- host servershutdown/close並join；直接連線只用loopback。另一次正常環境proxy外網https://www.rust-lang.org/觀察最後https://rust-lang.org/、200、elapsed489ms／status0，不把它或未來公共網站回應變成固定測試預期。12記錄在network-probes.json；/dev/full不單獨證明lateflush，controlledwriter單獨控制flushErr。
- strict scratchdocs build /workspace/work/ch11-single-render PASS，所有snippet實引用normal/test/answer。期限SVG intrinsic340×570，desktop1280×800／mobile390×844原位實測同尺寸，全部文字bbox在viewBox內。親看兩平台圖與相鄰圖說，藍等待／綠已得資料／橘同期限、Client準備不包在框內、量測起終與body可選關係可辨；不冒稱source-only等於看圖。
- 實際親看手機入口/roles、API與timer工廠、借用request／writer，桌面API表與main責任段；兩平台期限圖、公有短名imports、mainOutput與await、commands、JSON模型/提示/answer已親看。手機table343、頁scrollWidth375<390，長code在pre內水平閱讀；手機圖无需放大。兩details各真的展開，desktop/mobile都看提示和完整answer；未說所有自動capture或正式nav/darkmode／獨立首讀／releaseQA全驗。圖和43captures/bounds/追加actualOutput等在 /workspace/work/ch11-single-visuals/{manifest.json,followup.json,final-scope.json}；最後HTTP停止再handoff。
- 只完成11.7作者範圍。等root實讀本節actual，11.8還要11.5等前文，B實戰另等pernode授權；不自行先寫下一節或當已發布。

11.7 收尾追加：main Output與整份send含redirect的最新兩段均strictbuild後親看desktop/mobile；latestfinal-scope與followup保留。預覽HTTP8043已停止，有限hostserver也早已shutdown/close/join；交rootactual核讀。最後只有正文釐清／snippetmarkers與維護紀錄變動，Rustsource tokens／6tests沒有改，不重跑已通過套件。
