# 第14章作者交接

依 clear-tutorial／寫作短 prompt／Rust 術語指引寫作。作者自查不是獨立首讀、技術或 continuity 接受。

## 範圍與隔離

- owned：14.1–14.9、既有八個 async／HTTP 教材 package、`examples/ch11-async/stages/`、必要 SVG、本檔與 sources-ch14。
- root 整合八個新 stage members 與共享 lock；作者不改根 Cargo、lock、nav 或他人稿。
- 早期 target 用 no-lib 或真正只含當下能力的 basic library；完整 library 在同頁所有能力已教後才提供、執行。
- 全部 use 採單一路徑；不以測試 filter 隔離同檔後續 source。

## 14.1–14.3 第一波作者 freeze

逐節 manifest：`/workspace/work/tutorial-implementation-20261015/ch14-authoring/freeze-14-01.json`、`freeze-14-02.json`、`freeze-14-03.json`。各保存實際頁面與同頁素材／source 的 SHA256；不會以後續重寫覆蓋原 freeze。

14.1 固定 A=庫存3、B=80+120=200，三種安排材料不變；第三種明示另假設兩份資源同時執行。無 async Rust，完整手工練習與答案同頁。390px SVG 已實看、文字界線已記錄；此時尚未實看網站。

14.2 foundations-steps 先四份本地完整 example，再完成 foundations 全 library／async unit test／CLI test／四份完整 example。14.3 tasks-basic 只有 WorkError／controlled_work，教完才進完整 tasks／JoinSet／報表／測試／答案。

首次使用紀錄：

| 能力 | 本節首次必要解釋 | 實際前綴或本節新增 |
| --- | --- | --- |
| Future／Output／async fn／await | 14.2「呼叫後拿到答案，還是工作」 | associated type 6.4；本節教 Future 的最小工作契約，不教 poll／Pin |
| runtime／executor／Tokio current_thread | 14.2「讓 runtime 推進工作」 | dependencies/features 10.1；workspace 12.1；本節教 main attribute |
| async block／async move | 14.2捕捉段 | closure 9.1、ownership 與 borrowing；本節追 borrowed heading／owned note |
| 普通 future 工廠 | 14.2 explicit_future 前 | RPIT 9.3；本節區分普通函式準備與 block 本體 |
| await 得到整個 Result | 14.2 fallible_output 前 | 錯誤政策8.1；本節 Output 不自動 unwrap |
| tokio::test | 14.2完整 library 前 | attributes/tests 2.7、mutable borrowing；本節教每測試獨立 runtime |
| Command／Output／from_utf8／env! | 14.2 CLI test 前 | 實際10.8與12.2；不誤指10.5 |
| task／spawn／Send+'static | 14.3第一段 | 13.1、11.5；本節教提交與開始證據分離 |
| Tokio mpsc／started／release | 14.3 basic library 前 | std channel 13.5；本節教 async send/recv／容量／關閉 |
| JoinHandle／Id／JoinError／abort | 14.3兩個基本 example 前 | 本節教兩層 Result、abort後await、detach |
| JoinSet／AbortHandle／join_next_with_id | 14.3 collect_one 前 | 本節新增集合所有權及 Option/Result 對應 |
| HashMap.remove | 14.3 collect_one 前 | HashMap6.5–6.6；本節教移出輸入對照 |

實測檔：`checks-14-02.json` 11條、`checks-14-03.json` 9條，20條全exit0；stdout/stderr各另存。兩組 package rustfmt --check 通過。每shell source scripts/env.sh，compact Cargo環境，未跑根全套。

14.2 精確五行stdout由真正binary integration test核對；丟棄120不印本體。14.3收到started才控制release；取消後await，再send失敗證明接收端清理；四報告順序1／2／3／0與分類、正常binary退出及panic診斷都有實測。未使用sleep作started證據。

第一波交接時，14.4–14.9尚未作者完成；當時不可把研究方案當成已教。

## 第一波版面追加證據（不改原freeze）

在work下獨立Zensical config真build並以Chromium開啟14.1–3，390與1440寬。本人實看14.1原位圖兩寬、14.2正文兩寬、14.3桌面正文及手機三欄分類表；SVG與三欄表可讀，body無橫向溢出。site-layout-14-01-03.json與site-14-*.png保存證據。隔離站不等於root整合nav驗收；唯一build warning是當時未複製14.4的下一節link。

## 14.4–14.5作者freeze

freeze-14-04.json／freeze-14-05.json逐檔保存。14.4六條、14.5十條selected命令全exit0，完整stdout/stderr保存。早期timing-steps不載timing library或test-util；timing-clock只有本地total_after與四項當下測試；完成版才有two_steps／scope／答案。14.4首次fmt --check與14.5新stage格式化完成。手機原位補位圖及兩層Result表已實看；有隔離網站桌面截圖，完整桌面查核仍待本人口視。

first-use：14.4在各region前教Job／owned IntoIter／submitted與tracked／new-fill-next，完整outcomes module與三測試在指令前；14.5依Duration→std Instant→Tokio sleep/time feature→timeout/Elapsed/pending→兩層Result→drop不rollback→std thread::sleep（本節新教，不誤指13已教）→test-util／paused Tokio Instant→兩步預算順序。IntoFuture留14.9，不展示timeout的泛型完整簽名。

## 14.6作者freeze與中段版面追加

freeze-14-06.json保存24個page/source/fixture素材path。checks-14-06.json共12條按預期通過：負例check為101且expected診斷匹配，受控停止binary為130，其餘0；沒有把預期非零算失敗。另讀到READY才SIGINT的真binary檢查signal-ready.json通過，0／1取消、2／3未提交、返回130；沒有sleep猜ready。

first-use：select/biased與分支future丟棄先教；S1=JoinSet、T0=input0task、W1=mutable借用S1的join等待明確分開；guard Send負例附完整manifest/lock/source/expected；try_recv、Unstarted、停止原因、Runner、abort_all/drain、enumerate按先教後全檔；current_thread returning證據只限送完後無await直接return安排；signal最後才加入，listener entered不拿job started代替，env::args在main前明教。

14.4圖、14.5桌面正文及14.6圖兩寬已本人實看，補足先前桌面待看部分；隔離站不等於root整合nav驗收。

## 全九節作者完成與STOP

第三波freeze-14-07.json／freeze-14-08.json／freeze-14-09.json分別保存8／15／15個同頁page/source path。14.7六條、14.8八條、14.9十二條selected命令全exit0，含真loopback固定回應、redirect、兩gate控制、headers前timeout、absent-entered、client取消後另收server、未放行shutdown回503及有限CLI。公開URL的四條命令是選用觀察，未執行，不冒稱真網站測試。

全章74次selected Cargo命令按預期完成，另有一次READY後SIGINT的真binary驗證；16個owned packages最後fmt --check通過。78條正文Cargo指令中73條精確執行、4條選用公開網路未跑、1條signal模式由已build的同binary在READY後發SIGINT驗證（未重跑Cargo wrapper）。全部stdout/stderr、退出狀態、預期非零與命令映射保存在commands.json。沒有未處理的Cargo初錯，沒有根全套測試。

source-closure.json逐條記錄132個snippet、78條命令的實際本地source閉包及首次完整提供位置，0缺少全檔、0指令前閉包缺口。一般target不載fixtures；故意失敗fixture有獨立manifest/lock/source/expected。HTTP舊tests/http.rs依核准方案拆成五個獨立test targets，原檔及hash保存在work；absent_entered與cli另維持獨立。

first-use.json保存全章必要概念／方法／約定的位置、前綴與本節解釋；IntoFuture僅14.9首次教。所有Rust use採單一路徑，沒有把Pin/poll當必要先備。sources-ch14及read-evidence-final.json保存實際外部與前綴讀取範圍/hash。

九頁均以work下獨立Zensical站點build並在Chromium390／1440寬開啟；必要三張SVG已實看原位兩寬，HTTP表格與正文已實看。隔離站最後唯一warning是沒有複製第15章；真repo的15-01-lesson.md存在。這不是root整合nav或獨立continuity驗收。

各波freeze維持原紀錄。freeze-final-hashes.json保存全部owned交付檔最後hash；不包含共享rootCargo/lock/nav，不將他人變動算進本作者成果。後續root prose-only修正應另記差分，不覆寫本輪歷史。全章作者交稿完成，STOP；獨立首讀、技術及continuity仍由root安排。

最後彙總：first-use.json共59項，read-evidence-final.json共23份range/hash紀錄。章節頁面與各波素材hash最後再比對無drift。根要求不重跑已過checks、不追加畫面，作者在此交接STOP。
