# 第16.1–16.2節獨立技術初判

必要問題：0。可選修正：0。

按兩頁全文與實際target-local closure獨立核對；未看作者驗證／peer／歷史問題，未讀16.3–16.5補本波先備。所需能力均在實讀1–15。

- AddressNote只保存數字與raw地址、用ptr::eq比較，沒有失配解參考。整值交換false／false、Box owner交換true／true、只交換i32欄位仍true的觀察與四測試相符；reference負例在保存借用後要求整值mutable交換，E0502原因正確。
- Pin wrapper／pointee、Unpin／PhantomPinned、safe constructor／getter bound、Box::pin與local pin!差別皆正確。!Unpin不是建立起永久不能move，local也不等於必定stack。
- PinnedNote先固定再記地址；private欄位及兩處unsafe只修改指定欄位，不移出／替換Self、不交出mutable reference。Drop原位責任與其仍接&mut self的事實均說清；owner移交／修改／清理輸出與三測試正確。
- no-lib stages沒有依賴，九條selected命令只載當時target與列出的本地module；後段poll／runtime例子不會被編入。三完整compile-fail與sidecars支持E0502／E0277。三SVG只核XML文字／箭頭、非視覺驗收。

24owned files、26snippets、7run＋2example-test命令、7test functions、3compile-fail fixtures、34hash。清單、每項closure與來源在同名JSON。

未執行Cargo／rustc／Clippy／tests／examples，沒有pointer實驗、browser或讀者實驗。
