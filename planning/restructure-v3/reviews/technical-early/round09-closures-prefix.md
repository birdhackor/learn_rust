# 第9章前段獨立技術初判：9.1–9.2

第7章、8.1–8.2各自獨立封存後，再按9.1→9.2讀正文及全部actual source、manifest、fixtures／expected與closure-transfer SVG原文。先備只用實讀1–7及8.1–8.2；8後段、9.3之後與11未讀。封存前未讀作者verification、continuity、reader、其他review／歷史問題。

**必要0項、選讀0項。** 未執行Cargo、rustc、Clippy、Rust測試或瀏覽器；所有結果採實際source靜態推演，沒有把作者已跑當成自己的證據。

13個run目標都是no-lib intro-iterators的完整單檔，各自helper與main同檔，無dependencies／build script。完整三欄manifest在首次run前給出；default main也完整展示並說明只提示選example。後續同package的其他source不因存在而載入。沒有test／all-targets／Clippy／fmt命令，既有assertions只檢查各自材料。

9.1在首份source前定義closure／capture／環境與|參數|語法，區分呼叫時提供amount與建立時capture free_limit。280收60、320免運；建立工作尚未計算訂單。型別推導不等於泛型、單式body、無參數||與bool運算，也都有首次說明。

Shared capture、mutable capture與兩個mut角色在code前說明；tick的最後使用後才讀count，NLL規則與反例E0502相符。move按值capture不等於只可呼叫一次：owned String只讀可重複、i32／&str只Copy自身、借來view仍依賴原String；移出String的take才消耗這份closure並使第二次call為E0382。source／文字長度11與圖中兩個交接時點一致。

9.2先交代callback與FnOnce、std::ops／prelude來源、專用FnOnce()->T語法及associated Output，再呈現泛型helper。FnOnce是最低按值呼叫承諾，不誤寫成每個實作者只能一次。FnMut的mutable receiver及callee mut task、Fn的shared receiver及&F，都依實際需要選擇；FnMut足以重複呼叫，Fn選擇理由是shared access。能力包含關係、move capture與body的差別，以及Fn不保證固定結果／間接資料全不變，都正確。

Function pointer的fn型別與Fn trait、普通函式及不捕捉closure轉換，在code前交代；4、6、4結果與capture不能轉pointer的E0308相符。兩題答案分別保留運費與累計2次、逐項轉換10／20／30合計60且visits3；空slice不進while、不呼叫callback，與題意一致。無需未讀8後段或9.3以後API。

5個compile-fail及非空expected為E0308、E0502、E0382、E0525、E0308，與各實際失敗方式相符；未重新執行。SVG的text→take→saved三個時間框與建立／呼叫兩箭頭、title／desc／alt及正文相符，沒有clone或取得借用pointeeownership的誤示。XML可解析，但未視覺驗證。

同名JSON保存37份SHA-256、21個snippet、13條命令、完整target閉包與四題依據。本結論限9.1–9.2技術及首次必要先備。
