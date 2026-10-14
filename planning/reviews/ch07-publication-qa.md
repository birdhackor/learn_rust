# 第7章發布候選檢查

2026-10-14，root協調者核對。Original全章九頁／62閱讀單位的首次閱讀分prefix及tail封存；非作者技術與銜接角色各自完成初判後才互看同行。原稿為ch07-complete-prerequisite-v10；最終修後候選為ch07-review-fixed-v13。首次閱讀的可選分級、技術與銜接的獨立必要發現、root後續實跑的發現分開保存，沒有以後見同意改寫原初判。

處置涵蓋7.5與7.8的Stdout完成檢查、7.4 ToOwned名稱來源、7.9 Rust literal與JSON表示的橋接、generic writer與caller的flush責任，以及九份遺漏靜態材料。技術及銜接的交叉／局部報告已完成，root完整讀取並核對其受查範圍。修後material bytes／archive／manifest相符；original凍結包的缺漏與當時未驗證範圍保留。

Root親看四張SVG的桌面1280×800與手機390×844原位圖文：byte buffer的有效前綴、owned JSON模型、真child的stdin／stdout／stderr與exit，以及搜尋工具的資料／錯誤分流。最新7.9圖的write_all→main flush另作兩device修後局部實看，與正文和caller實作一致；圖中parser顯示請求另列，不誤作AppError或資料輸出。這是受查位置，沒有宣稱全頁或所有平台已驗證。

Git index的獨立候選release7-candidate只開第1–7章，workspace僅增加本章七個package；不帶後章作者活躍來源與依賴。固定候選Cargo.lock後，完整scripts/check.sh通過：strict文件建置、fmt、全target Clippy、workspace全targets tests、doctests與50個故意失敗契約。Log共有186個test target結果、97個通過tests／doctests、零失敗；含前章與原工程示例，不將合計當成本章測試數。候選新check.sh另明確跑all-targets，避免略過example內的tests。

證據保存於work/tutorial-implementation/release7-check.log及release7-check-counts.json。首讀者7.5修後局部已另封存；其餘五個受影響BODY單位與原位新版圖的額外局部複查也已完成，root完整讀取新紀錄，沒有新增必要或可選問題。讀者這次另有第8章背景，但其引用的所需前提仍在第1–7章實際已讀正文，沒有以第8章補原第7章的洞；範圍與提示時點另記，舊初判、分級與SHA不改。

所有已報必要與可選處置在對應修後範圍已關閉，固定候選的完整檢查通過，可以發布。遠端Actions與實際公開網址於完成後另行追加，尚未以本機檢查宣稱遠端成功。

提交`14ca5da0de4cbb21249be0b93bedbbf7e62672c7`的[Actions 38608408322](https://github.com/birdhackor/learn_rust/actions/runs/38608408322)已成功完成check／deploy。root實際請求首頁及第1–7章49節，50網址全200且H1相符；未發布8.1仍404，紀錄於work/tutorial-implementation/release7-live-http.json。本段部署證據於發布後追加，下次內容發布帶入，不為僅此紀錄再觸發一輪CI。
