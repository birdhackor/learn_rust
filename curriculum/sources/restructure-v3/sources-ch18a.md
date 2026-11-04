# 第18章A外部來源實讀

實讀原文及hash／range／採用與不採理由：`/workspace/work/tutorial-implementation-20261015/ch18a-author/research/source-read-ledger.json`。共13筆，不只列書名；EN/TW新增原HTML與正文另保存，Cookbook沿用同一作者12章已親讀原文且再次核hash。

- Book [EN TDD](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html)全文1–261行、[繁中](https://rust-lang.tw/book-tw/ch12-04-testing-the-librarys-functionality.html)全文1–352行：採可編譯stub→確認預期失敗→小步實作→保留測試；不採一項通過即全工具正確的概括。
- Book [EN modularity](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)1–150行、[繁中](https://rust-lang.tw/book-tw/ch12-03-improving-error-handling-and-modularity.html)1–105行：採main／library責任與owned設定；本題仍真啟動child，不靠main很短就當行程介面已驗。
- Book [EN stderr](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html)全文1–116行、[繁中](https://rust-lang.tw/book-tw/ch12-06-writing-to-stderr-instead-of-stdout.html)全文1–43行：採資料／診斷分流；不照搬process::exit與未檢查flush，本題另定0／1／2與部分JSON契約。
- English Rust Cookbook Directory Traversal全文1–220行與Hashing全文1–95行：採walker逐項、分批有效bytes摘要；不採丟棄Err或只因digest相同即宣稱內容相同。
- 中文Rust Cookbook原始五頁：duplicate33行、find43行、hidden38行、sizes31行、SHA59行均全文實讀。採API角色與小材料；本題保留hidden、root-link政策、原生位置、失敗後有限重建及BLAKE3後完整bytes確認。其語言、URL與hash逐筆保存；不能把同名但實際英文的鏡像算中文實讀。

本章的大小→摘要→真bytes、hard links按路徑指標、callback確切失敗者、metadata僅盡力觀察、診斷完整排序、完整owned schema與明確flush，皆依實際A程式與既有12章契約保留，沒有聲稱外部短例已實作這整套政策。全部起點、helpers、fixtures、tests及答案仍需本頁可取得，延伸閱讀不是必要補洞。
