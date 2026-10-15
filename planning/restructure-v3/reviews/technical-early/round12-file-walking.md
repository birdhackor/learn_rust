# 第12.3節獨立技術初判

封存：2026-10-15T09:16:31.901Z。必要問題0；可選問題0。

全文、13snippets、全部9份owned檔案與3個selected Cargo命令已靜態核對。19份repo SHA-256、3份實際walkdir 2.5.0原始碼hash、完整載入範圍及fixture bytes見JSON。未讀作者驗證或其他角色結論；未以12.4或已讀13章補本節先備。

root驗證與逐項失敗分開，兩個link設定及其預設值吻合實際walkdir來源。metadata、Snapshot、cached file_type都沒有被寫成鎖定或內容一致保證；穩定目錄及外部同時替換界線已清楚交代。一般檔案、hidden、link及其他特殊項目的政策和source一致。

原生PathBuf保留、非Unicode項目保留原路徑後失敗、Option新methods與Hash/Borrow/Ord角色皆在首用前交代。完整library、main、tests、manifest、三份輸入與答案全部inline；selected test不載後節package。partial move與u64／usize的區別正確。

8項Unix測試（其他平台5項）含可控制的中間Err後繼續、root拒絕與內部link略過；不依登入者權限。fixture bytes和5／5／6輸出、答案Some(&5)吻合。

沒有本波Cargo/Rust執行，也未建立實體symlink或非Unicode測試材料。本節無圖。
