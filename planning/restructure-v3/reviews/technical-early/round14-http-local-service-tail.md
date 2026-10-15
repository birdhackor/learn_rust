# 第14.7–14.9節獨立技術初判

必要問題：0。可選修正：0。

依14.7→14.9順序完整實讀及核對選定命令實際載入來源，初判前未讀作者驗證、peer review或歷史問題。只依基本Python及實讀1–13、14.1–14.6作先備。

- 14.7：HTTP回應與2xx政策／Request失敗分開；Body失敗保存已收到status。URL、redirect、proxy及TLS設定角色吻合。實查固定reqwest0.13.5：blocking send與text各自等待的期限敘述正確；UTF-8替代解碼且未設body大小上限亦吻合。四unit＋兩CLI測試確實離線，公開網站命令明列選用、沒有固定結果。
- 14.8：async send是普通future工廠；總timeout在準備請求時建立並傳入Response/body。Client／pool重用、final URL的owned副本與elapsed範圍正確。basic不載JSON，完成版normal library才載完整reply module；三core＋一reply及兩CLI計數正確，owned模型與完整答案吻合。
- 14.9：無lib的最小服務不會編譯後段Fixture。IntoFuture→spawn、動態port且持續持有listener、ready／entered／release不同證據、短Mutex guard、一次性Option::take、先關gate後graceful shutdown＋await，以及client另行回收的責任均吻合。
- 七個獨立integration targets依進度展示全檔後才執行；固定狀態／final URL／redirect hits5或6、兩gate完成順序、真網路前置後pause、absent-entered保留原Request錯誤、client取消與503關gate答案均與source相符。所有targets來源到齊後才all-targets。

37個owned files（35個編譯來源／設定、2個不載入的README）、48個snippet references、30條Cargo命令、26個test functions；53個hash含8個cached reqwest／axum API。完整清單及closure證據在同名JSON。

本波只做static source查核；未執行Cargo、Clippy、binary或測試，也未發HTTP、bind socket、操作timer或browser。沒有把作者成功驗證當成自己的執行證據。
