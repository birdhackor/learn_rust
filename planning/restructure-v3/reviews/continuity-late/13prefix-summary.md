# 第13章前半獨立銜接初判

範圍：13.1–13.4 全文、每個 snippet 的完整原檔、target-local modules、所有故意失敗 fixture／expected、同頁練習答案與完整 Cargo.toml。來源 frozen SHA256、四題及必要前文實讀索引見 independent-13prefix-initial.json。這是非作者銜接審閱，不稱首讀盲測；未讀作者驗證、ledger 或其他 finding。

必要問題：無。可選問題：無。

Send／Sync、Fn 呼叫能力與 static 限制分開介紹；spawn／join 的工作與成果兩端 bound、nested Result 與 panic payload 均能依已讀前文銜接。HRTB 從 caller 提供借用轉為 helper 建立局部來源，清楚交代為何需 for，且分開輸入 lifetime 與 capture／輸出來源。scope 接著用此 callback 契約安排借用工作、手動 join 與自動 body 等待；外部 view、短命局部值、不能逃出 handle 及兩獨立 mutable 來源都有來源與反例支持。

以自有 frozen preview 實看 1280×800／390×844 四節原位圖文；手機四張圖的必要標示清楚可讀。另開13.3手機長答案、13.4桌面長答案，程式可橫捲、折疊正常。同頁完整 source 均已比對 rendered code，沒有只剩 GitHub 連結的來源。未重跑 Rust，未宣稱全書前文完整首讀。
