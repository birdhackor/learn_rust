# 第17.3節獨立技術初判

封存：2026-10-15T09:21:07.617Z。必要問題0；可選問題0。

全文、27snippets、三個packages的完整實際library／helper／consumer、2個run與1個lib test（2函式）、5組跨crate負例已靜態核對。32份repo SHA-256及8份固定版本第三方來源hash、逐target閉包見JSON。未讀作者驗證或其他角色紀錄；未以17.4補先備。

proc-macro crate設定、host工具與consumer分工、derive保留原item後增加impl、兩種TokenStream及轉換皆準確。對照syn 2.0.119原始碼，derive／parsing／proc-macro已足夠支持實際AST、parse及Error API；不靠full／printing，quote使用固定1.0.47。root lock另有其他syn版本，未載本波。

AST欄位、Punctuated methods、borrowed match、parse失敗early return、Span診斷、quote插入及固定外部crate路徑，都先於必要使用交代。source檢查有限外形，欄位能否回傳i32由Rust檢查；alias、私有欄位、零與負數測試吻合。四種形狀拒絕與bool的E0308有完整獨立fixtures及正確path dependencies。

普通run會載consumer library和兩份完整path library；fixture則只載trait library及derive工具。全部inline source及解答完整，不載後節能力。SVG只讀XML/source。沒有本波Cargo/rustc執行或visual驗證。
