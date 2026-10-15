# 第 10.8 節獨立技術初判

封存：2026-10-15T08:58:06.208Z。必要問題0；可選問題0。

全文、8snippets、完整受測main／directory／cli sources、manifest與所有動態fixture材料已靜態核對。16個repo hashes和1個cached tempfile source hash見JSON。未先讀作者驗證或其他review，不用另讀11章補10的先備。

TempDir的owner、借path、close消耗／回Result和自動清理忽略錯誤均先教；PathBuf只存位置。fs::write、try_exists、Command設定／啟動、原生arg、child.cwd、Output及status、env!與Cargo本binary定位、借用UTF8 view和starts_with都在必要首用前說明。

精確target closure是無lib的本package main加兩份獨立integration tests。選directory不載cli.rs；選cli即全檔編譯五tests，再依filter只跑空檔一項。main有效程式對10.5快照相同，未附原unit tests，正文已明說。devdependency只有tempfile，無隱藏helper。

六項tests依source分清啟動I/O失敗、child完成但status失敗、成功空內容，並核對兩個出口。成功檔中文與空白名不經shell拆字；missing/FF正常child失敗1；缺argument2；不存在程式外層NotFound。所有TempDir owner都留到child結束／assertions後才close，沒有共享固定目錄或權限假設。

沒有新增Cargo/Rust或CLI執行，也沒有實際建立fixtures；結果為靜態核對。fixture-sequence.svg只讀XML/source，不稱browser或讀者驗證。
