# 第14章 C14 契約窄複核

指定三組變更均已解決，沒有新增問題。這是root在本人第14章初判封存後提示的複核，不算本人獨立重現；原initial及hash不回寫，未讀其他peer finding。

- **14.5:9 Duration／Copy**：敘述正確且在首次範例前。標準Duration具備Copy；Tokio 1.53.3 sleep.rs:123、timeout.rs:86按值接收Duration，呼叫後原binding仍可重用。新句教的是跨度值可複製，不是多次共用同一deadline。核心函式庫Copy實作另以本機nightly-2026-10-01標準庫來源佐證，來源版本明記，沒有宣稱讀到已安裝1.99的原始碼。
- **14.3:23／:52及14.5:43 格式化能力**：Tokio 1.53.3 SendError<T>的Debug實作沒有T:Debug限制；JoinError明確實作Debug與Display；Elapsed derive Debug並實作Display。expect診斷及8.5 Display→to_string回指正確，補句均在相應首份需要的完整source之前。
- **14.9:145 Client Send／Sync**：Reqwest 0.13.5原始碼具Client的Send／Sync／Clone型別檢查，Client內部Arc及Clone契約一致。新位置在完整main snippet（:150）前；main.rs:13的async move確實取得owned client／address，再借client跨await。先前最小程式spawn server、直接await client，未存在更早的client-spawn缺口。後段保留clone共用pool說明，移動沒有改動該契約。

三份root保存before均精確匹配本人原初判的頁面hash。99份相關owned Rust／manifest與本人第14章初判一致，三頁snippet引用與Cargo命令也完全相同。current hashes：

- 14.3：`83f75035694bb35cb3312db869913d5df758e27d49ab5aa4d7b21b2ee33ede42`
- 14.5：`9beaab3af9f93e4c3b5f65aae7cde69026db426d5811110b1ec14cb39343309f`
- 14.9：`cb3206ccfd5d9d6034cd0dc94001a496378b518fe61fa507acea64217425b7c2`

本輪只做局部diff、API原文、先教位置及hash核對。未改正文或source，未跑Rust／Cargo／HTTP／瀏覽器。23份證據hash與精確差異在同名JSON。完成後STOP。
