# 第10章前段獨立技術初判：10.1–10.2

依實際已讀1–8、9.1–9.4，順序核對兩頁正文、全部owned source／manifest、path dependency正常library與lock。未讀作者validation、其他review／continuity／reader或歷史問題；未用10.3或11補先備。作者行內表格解釋窄調整通知在審閱期間到達；我首份實讀已見36行緊貼manifest，記為作者修訂，不算自己的發現，未讀作者freeze結論。

**必要0項、選讀0項。** 唯一Cargo執行是唯讀cargo update --help，實見Usage支援位置式[SPEC]；未執行build／check／run／doc／test／tree／add／update，也沒修改manifest／lock。沒有瀏覽器或目標讀者實驗。

cargo-setup的main只呼叫modules-basic pricing兩次，80+20與120+20合計240。已追正常pathdep閉包lib.rs＋pricing.rs，binary／other examples未載入，沒有後章source。clap確為一般registry依賴，tempfile是dev dependency；前者雖編譯並未由本節main呼叫API，後者不是一般binary依賴。兩者作用、feature／預設功能、path起點及TOML inline table都在完整manifest前交代。另讀本機快取的原始clap4.6.7 manifest，derive及default features與正文相符；外部library內部Rust不是本書owned source先備。

版本要求與精確版本、SemVer≥1範圍／0.x限定、shared lock、tree direct/dev edges與重複標記、add／update的practice-copy mutation，均清楚分開。當前lock實際clap4.6.7／tempfile3.27.0與設定吻合。位置式cargo update clap經help確認合法；沒有誤認為run的-p選自家package，也未把lock當跨OS完整保證。

cargo-docs與答案皆只有library，沒有隱藏CLI binary；唯一依賴仍是modules-basic，全部設定／normal source同頁完整列出。///／rustdoc／Markdown fence／doc test wrapper、產文件與執行doc test的差異在首用前說明。公開API明定兩筆各自算運費及非負小數字／不溢位，實作與240／340／440相符。

Integration test本身獨立crate、crate::不同起點、公開路徑、#[test]不用再包cfg module及--test target，在對應使用前都有解釋。實際1 unit＋2 integration＋1 doc符合指令；答案新增第二fence，--doc為2項，不包含其unit test。150／150的340可抓到錯誤合併訂單後300，符合測試目的。doc生成／例子assertion失敗的關係可由source判定，未宣稱本輪改稿實驗或actual頁面驗證。

同名JSON保存21份repo SHA-256、2份外部registry manifest hash、16個snippet、11條實際fenced命令、完整pathdep／test閉包與四題證據。沒有引用新fixtures；此結論限10.1–10.2技術／先備。
