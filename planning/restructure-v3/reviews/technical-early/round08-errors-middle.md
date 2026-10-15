# 第8章中段獨立技術初判：8.3–8.5

依實際1–7、8.1–8.2，順序讀8.3→8.5及全部target source、helper、test與manifest；已讀過的9.1–9.2不拿來補本章先備，8.6及其餘未freeze稿未讀。封存前未看作者驗證、continuity、reader、其他review或歷史問題。

**必要0項、選讀0項。** 未執行Cargo、rustc、Clippy、Rust測試或瀏覽器；採整頁技術重讀與source靜態推理。

10個run目標皆屬no-lib intro-error-stages。4個單檔、6個folder targets的完整closure逐一核對；實際18份helper讀取並與已完整讀過的同類base逐byte比較，沒有把快照相同當成假定。所有normal與cfg(test)來源同頁完整列出，含摺疊source；沒有提前載入?、propagation或std::error::Error，也沒有後章closure／iterator API。manifest仍3個已教欄位、無library／build script／dependencies。13條命令皆精確selected target，測試數為display3、conversions6、parsing10。

Display、fmt module／method、Formatter狀態與內外兩份借用、fmt::Result alias及Ok(())，均在實作前給角色。write!回傳寫入結果，format!建立owned String；CountError業務拒絕不是Formatter出口失敗，這個界線與實作相符。Display／Debug／保存String的三種用途有相同原值對照，沒有宣稱Display必能解析還原。訊息測試未模擬寫入失敗，正文也明記此限制。

From／Into來源與目的、i64全含i32、From帶出Into而非反推、TryFrom的associated Error／Result及prelude，先於實用例。Count公開型別而欄位私有，TryFrom只在checked_count成功後建值；公開getter不開放修改，From<Count>消耗來源。0／3／100往返、−1／101保留原因，維持公開API的有效性；內部新增操作仍須維持不變條件的限定正確。

FromStr／parse與ParseIntError、十進位格式／空白／溢位規則均在首個source前介紹。ParseCountError兩variant保存整數與件數錯誤本體；From僅封裝、不新增失敗。Count::from_str先解析再TryFrom，沒有偷用下一節?。Count不借輸入文字，內層String結束後仍使用Count的測試與型別相符；空字串／abc／3空白／2147483648歸Integer，−1／101歸Range。

AsRef及&&str的角色、receiver連結先說後用；String與&str傳入借讀後均保留。Label兩欄決定完整身分，AsRef只投影text，id不同而text相同的材料能說明為何不符合Borrow<str>同一Eq／Hash身分契約。ToString源於Display；兩個ToOwned實例確實產生獨立String／Vec，原error、文字、array保留。正文未把任意ToOwned當成一律移除內含借用，也未混同reference Clone與str→String。

完整答案、assertions與10個run輸出靜態相符。本範圍沒有引用新的compile/runtime-fail fixture；拒絕材料是正常Result分支，未擴讀後段package fixtures。41份SHA-256、38個snippet、13條命令及逐target closure／四題依據存於同名JSON。此結論不是本輪Rust實跑或版面／讀者實驗。

封存時workspace／lock metadata變動，已實讀新版本：較後cargo-* members與其registry lock entries已加入，但intro-error-stages的無dependency entry及全部本章source／package設定不變。原snapshot及封存新hash分開保存；沒有讀較後member的manifest／source，也不把它們當先備。
