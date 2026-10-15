# 第17.2節獨立技術初判

封存：2026-10-15T09:18:59.463Z。必要問題0；可選問題0。

全文、26snippets、完整normal library及caller／helper／tests／fixtures／兩份manifest與lock已靜態核對。29份SHA-256和逐target閉包見JSON；未讀作者驗證或其他角色紀錄，未以17.3補先備。

macro_export的crate根路徑、裸Reading的call-site來源、$crate固定定義crate及保留private限制，均對實際程式。mixed-site hygiene解釋與同名value觀察正確；模板每個輸入各插入一次，求值順序及一次求值的原因都有交代，未歸因於hygiene本身。

expr／tt、block expression、compile_error、規則依序匹配與不因展開錯誤回退，皆在首用前解釋。3 runs／1 test（2函式）及4個跨crate負例，實際只載完整Reading＋hygiene library；不載後節derive。4份Cargo負例的本地path dependency、獨立workspace及lock皆一致，另一本地順序負例是自足Rust檔。

完整答案先左再右取得reference，再反向交回(30,80)，保留[1,2]與owner。沒有本波Cargo/rustc執行；SVG僅查XML/source，未做visual驗證。

封存時全域Cargo.lock有並行metadata更新；本波ch14-macros條目與root manifest不變，記錄前後hash，不影響已核閉包。
