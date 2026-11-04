# 第17章來源實讀與取捨

本輪作者/root在寫17.1之前已親讀英文Rust Book Macros全文（433行）與簡體對應全文（199行），Reference macros-by-example選段及TLBORM英/中文hygiene全文。確切原始URL、cache SHA256與inclusive ranges保存在work/ch17-root-preparation/reading.json，不把前任作者notes當親讀原文；所有原文example未執行，不聲稱外站視覺驗證。

- EN https://doc.rust-lang.org/book/ch20-05-macros.html；ZH https://kaisery.github.io/trpl-zh-cn/ch20-05-macros.html：借鑑由普通Rust展開到模板；不採普通函式不能處理不同數量同型資料的籠統對比。此課目的為重複產生宣告，保留手寫同任務對照。
- EN https://lukaswirth.dev/tlborm/decl-macros/minutiae/hygiene.html；ZH https://zjp-cn.github.io/tlborm/decl-macros/minutiae/hygiene.html：用名字來源分清caller/context；mixed-site、求值一次與item唯一性須分開，17.2才使用。
- Reference https://doc.rust-lang.org/reference/macros-by-example.html：核對fragment/repetition、先成功匹配規則、textual/path-based作用域；不使用新Reference的非1.99特性。

後續17.3已讀固定syn2.0.119/quote1.0.47/compiler/proc2 API，以及EN/ZH Book procedural段；實作等12.2 host-build actual gate。17.4–8另已研究EN/ZH Book/Comprehensive、proptest、Perf Book、RustCourse/RustMagazine、Criterion/dhat/Miri/Loom選段，各範圍見同一reading.json及notes逐階段紀錄；研究完成不等於各節已寫或先備成立。


## 17.2–17.3 author update
17.2 complete normal library/hygiene/callers/diagnostics and 17.3 complete proc-macro/consumer/tests/diagnostics inline. Actual EN/ZH originals and fixed syn/quote API reading precede drafting (ch17-root-preparation). Existing Rust unchanged; early command placement now waits actual full library teaching. Selected verification and immutable freezes: ch17-root-authoring/validation-17-2.json, freeze-17-2.json, validation-17-3.json, freeze-17-3.json. Independent technical/continuity/site reviews pending, not accepted.


## 17.4–17.8 author completion
已完成全八節作者稿與每節source freeze。17.4 complete public API/library/tests/compatibility/fixtures/答案同頁，20項current selected checks；17.5完整生成/負例縮減/回歸/合成答案同頁，8項current selected checks；17.6完整Loom與Miri normal/helper/UB-only fixture及答案同頁，current普通/Loom/固定nightly Miri均符合預期。17.7/8完整manifest/lib/example/test/bench/答案同頁，current selected correctness/compile/Clippy/fmt；保留2026-10-14歷史bench/profile，12份owned source/manifest hash無drift，不將新root lock指紋寫回歷史記錄。current viewer親看三歷史JSON，cloned326400bytes/25600blocks、borrowed零root、solution81600bytes/6400blocks、峰值15/end0及clone來源鏈吻合。viewer的file URL初次被環境禁止，改用本機HTTP提供相同公開viewer靜態檔後正常，不冒稱量測重跑。
root草稿17.6編號替換曾誤碰0.7.2/17.2，凍結前已改成有數字/小數點邊界的替換並親核版本/URL；沒有改Rust、沒有重跑成功Miri。每節freeze/validation及historical-source-check/current-view-evidence在ch17-root-authoring。獨立technical/continuity與最終網站原位驗收仍另行完成，不以作者check當accepted。
