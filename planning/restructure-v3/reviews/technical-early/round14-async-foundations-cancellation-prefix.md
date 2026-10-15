# 第14.1–14.6節獨立技術初判

必要問題：0。可選修正：0。

按14.1→14.6完整實讀正文及實際 selected-target closure，未看作者驗證或其他 review。讀者只承接已實讀1–13章；沒有用另已審的15／17章補本章先備。14.7–14.9雖後來獲放行，尚未閱讀或當成本波先備。

- 14.1：固定 A=庫存3、B=80+120=200 的手工追蹤正確；交錯與真正平行的資源條件分開，圖不宣稱量測。
- 14.2：async 呼叫的引數移入與延後本體、async block／move、await 的 Output／可能暫停、普通工廠的立即準備皆正確。早期 no-lib 與完成版 library 的命令閉包分明，lifecycle／CLI／收據答案與來源吻合。
- 14.3：started／release 提供確切進度；current_thread 不免除 spawn 的 Send＋static。工作錯誤、panic、取消與 detach 分開；Id 對照／四筆受控順序／ControlClosed 答案正確。
- 14.4：先回收再補位維持 tracked 上限；未回收的完成結果也占格，不誇稱記憶體全程只有兩筆。zero／empty、峰值2與提交5、一格答案均吻合。
- 14.5：實查 Tokio 1.53.3 timeout 原始碼先推進 inner future，支持立即／同步阻塞案例；沒有硬切同步工作。丟棄保留 visits=1，std 與 paused Tokio clock 不混用；整段／逐步期限及2／1／0答案正確。path dependency 的 foundations normal lib 已於14.2完整提供。
- 14.6：借用等待 W1 與集合／task 分開；JoinSet／recv cancellation safety、短 guard／負例、停止前查訊息、abort_all 後 drain、Unstarted 分類與 completed-but-uncollected 政策吻合。signal entered 證據與 current_thread 限定正確；controller／observer 皆回收，exit130／2及Unix SIGINT handler 不復原的敘述正確。

完整範圍為73個 owned files、84個 snippet references、48條 Cargo命令及35個 test functions；95個 hashes含8個 cached API檔。各 package 的 normal library、helper、cfg tests、獨立失敗 workspace 與答案皆核對，完整清單在同名JSON。三張SVG只讀XML及文字／箭頭原文。

未執行 Cargo、rustc、Clippy、async／timer程式或測試；未發送訊號、未做瀏覽器或視覺驗收。各輸出與失敗原因為 source 推導。
