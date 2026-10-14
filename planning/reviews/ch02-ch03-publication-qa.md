# 第 2–3 章發布檢查

2026-10-14，root 協調者覆核。第2章發布正文對應修後v5，第3章對應修後v6；原始首讀、技術、銜接初判及其hash都保留，提示後交叉／局部複查另存。所有已報必要與可選項見各章disposition，沒有未解除必要項。

第2章的短mutable參數提示由技術與原讀者局部覆核；空slice五項重複assertion維持可選並不採納，理由是本題函式只取得空段，依已教safe邊界和零次走訪不能修改原元素。此裁定不將相同總和當作一般逐項相等的證明。

第3章的String no-Copy首次使用時點、manifest單項Clippy設定、char顯示前置、Vec配置限定及泛型文字標記均有處置與修後局部覆核。技術／銜接角色沒有把提示後同意回算成原獨立發現；原讀者指定補讀manifest也不回填為初判讀過完整source。

Root親看兩章代表位置的1280×800桌面與390×844手機畫面：第2章四圖的owner移交、借用來源、衝突區間與slice索引；第3章String／reference兩種箭頭及UTF-8 byte／scalar分界。必要圖在原文需要處，相鄰文字可指認對象，native integrated TOC沒有遮圖。另於修後v6重新核3.6型別／操作表及owned輸出段；HTML DOM audit沒有未知泛型標籤。畫面證據保存在work下，不宣稱覆蓋所有裝置或全部捲動位置。

發布導覽只開第1–3章，後續初稿不進公開閱讀路線。發布候選從Git index建立獨立目錄，使用候選自己的workspace與lockfile做完整檢查，避免把活躍作者尚未提交的程式算成候選已驗。

獨立候選的 `bash scripts/check.sh` 已完整通過：strict文件建置、fmt、Clippy全targets `-D warnings`、workspace tests／doc tests及18個故意失敗契約。候選使用既有穩定workspace，沒有納入活躍作者新增的第4章以後packages。遠端部署結果待實際完成後追加，不以本機成功代替公開結果。

提交 `d13596a275ecdbe7129d94a977c12dd25f0ff63c` 的 [Actions 38575827782](https://github.com/birdhackor/learn_rust/actions/runs/38575827782) check／deploy均成功。Root實際請求公開首頁和第1–3章22節：全部200且H1相符；尚未發布4.1仍404。紀錄在work的release23-live-http.json。候選手機由首頁進1.1、展開第3章導覽再到3.6成功，最後頁沒有第4章連結或下一頁，整頁無橫向溢出；長code仍可在區塊內捲動。
