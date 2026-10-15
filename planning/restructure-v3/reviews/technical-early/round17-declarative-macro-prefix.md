# 第17.1節獨立技術初判

封存：2026-10-15T09:17:41.852Z。必要問題0；可選問題0。

全文、20snippets、12份owned package檔、4個run與1個selected example test、3組完整compile-fail／expected皆靜態核對。23份SHA-256與逐target閉包見JSON。未讀root作者驗證或其他角色結論，未以17.2+補先備。

最小library只定義Reading和read_value；四個example都不載下一節ch14-macros。手寫與單名字macro使用的main片段exact bytes一致，借讀與各單位型別身分保留，沒有添加Copy或數值驗證。

macro_rules!、ident／metavariable、token、匹配與展開、+／*／?以及尾逗號皆在首用前說明。repetition與空清單答案對實際規則正確，包括單獨逗號也可匹配。三份負例確實隔離輸入不匹配、至少一次的空輸入，以及展開後E0308；原始碼完整且不需後節依賴。

1項測試借讀0與負數後仍使用原wrapper；輸出80／30／500和來源一致。SVG僅核對XML/source，編譯期展開與runtime借讀界線正確。未執行本波Cargo、rustc、展開工具或browser。
