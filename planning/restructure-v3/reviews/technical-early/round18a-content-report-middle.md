# 第18.3–18.4節獨立技術初判

必要問題：0。可選修正：1。

TC18M-O1（可選型別措辭）：18.4:29把action、reason、path_debug稱為「後三份String」。實際report.rs:25的path_debug是Option<String>，:152也直接比較Option，包含None／Some規則。可改為「action／reason使用文字Ord；path_debug使用上段Option<String>的Ord」。既有上段說明與完整source均正確，故不構成先備或編譯障礙，只是避免把可缺值欄位誤認成String。

A2的大小／digest只篩候選、bytes作最後判準、每組代表與原生路徑去重、精確失敗者移除後有限重建，均與完整source吻合。受控左a失敗確實保留B／B；短讀／Interrupted／9001bytes／空與binary／前後可觀察變動及舊issue保存的十二測試完整。

A3先排原生Path再轉String，diagnostic保存Option<PathBuf> key至排序後；checked指標溢位屬致命失敗，issues使complete=false，零群可完整。文字Debug處理換行引號，JSON同模型往返，九項新測試及29Unix／24其他平台計數吻合。兩個答案example先write_all再flush，cwd明確。所有normal library／test helper／起始checks／答案均在相應命令前全檔提供。

37個package-owned檔案（33個loaded或待手動加入的source/config，4個未載入README只hash未作證據）、14份byte-identical copy核對、50snippets、12literal commands、52hash，含兩SVG source及六材料。原文與來源清單詳同名JSON。

此為獨立初判，未讀作者／peer／歷史結論；沒有用18.5–18.6補洞。未執行Cargo／Clippy／tests／examples，未操作檔案或做視覺／讀者實驗。
