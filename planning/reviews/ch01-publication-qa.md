# 第 1 章發布檢查

2026-10-14，root 協調者檢查；發布正文為 ch01-complete-source-v8。其餘章節是 repo 中的初稿，尚未加入公開導覽。

正文先讀紀錄保存於 prefix v1、tail v2；prefix 與 tail 的技術／銜接初判及修後複查分開保存。全部已報必要與可選項依兩份 disposition 裁定，v8 的技術與銜接 followup 沒有未解除必要項。首讀 v6 的非負範圍／locked 約定複查保留，v8 的其餘局部修正由非作者技術／銜接角色複查，不冒稱另一輪盲讀。

Root 實際看過各頁代表位置的桌面 1280×800 與手機 390×844 畫面：prefix、計算／型別表、分號失敗程式、條件、tuple／array、迴圈、module 圖與測試工具。SVG 原位兩種箭頭分別是 binary 呼叫 library、lib.rs 宣告 pricing module；不是同一種 runtime 動作。圖文相鄰，手機整圖字級與標記可辨；native integrated TOC 沒有浮動按鈕遮圖。

保留畫面於 `/workspace/work/tutorial-implementation/screenshots/`。頁面路徑檢查全為200，沒有頁面橫向overflow；code block 的較長行採區塊橫向捲動。最初個別剛捲至 H2 的截圖尚未完成 paint，並非缺程式；等待兩個 animation frames 後已實看正確 code。畫面紀錄只支持受查位置，不宣稱閱讀每一種裝置／每個捲動高度。

手機實際由首頁連結進1.1，開導航drawer後點1.10成功；最後一頁沒有連向尚未發布章節的下一頁。十節可從drawer按順序找到，已教H2也在當頁導航中。

發布設定只開第1章。`bash scripts/check.sh` 完整通過：strict文件建置、workspace fmt、Clippy全targets `-D warnings`、workspace tests與doc tests，以及18個故意失敗契約。第1章continue改skip2另實跑，結果與正文的4一致。第9.1的feature/release組合另有作者11命令紀錄，不當成本章讀者已學過。

GitHub Actions 部署與遠端URL仍待本次推送後另記結果；本地檢查成功不代替遠端發布成功。
