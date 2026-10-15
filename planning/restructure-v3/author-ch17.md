# 第17章作者交接

作者/root。當前僅17.1完成，後頁待各自actual gate，不作整章驗收。

## 17.1

必要前綴已實讀：11.10 tuple struct／單位型別／local impl責任，12.1 metadata繼承／root與selected-target角色；精確hash與必要newtype source在work/ch17-root-authoring/actual-prefix-for-17-1.json。型別取值trait沿6.1，Debug/PartialEq沿6.2，macro的編譯時生成角色沿已教derive使用。

同一工作先手寫Meter/Seconds並借讀80/30，再ident與單條模板，再repetition三種單位，最後空清單練習。每份例子、答案、library、manifest及三份fixture/expected同頁完整。新增macro-basics的正常library只有Reading/read_value，永久不載17.2 hygiene；保留原ch14-macros全package供後節。三種literal輸出及空清單輸出逐字實跑、reading_many1測試、strictClippy/fmt、3份expected fail均過，共10selected checks；新正文最後只補已教rustc重現方式，不改Rust。work/ch17-root-authoring/validation-17-1.json、source-closure-17-1.json、freeze-17-1.json保存實際證據。

首次位置：開頭給省重複struct/impl需要；「一條規則」先解expansion/token/metavariable/ident後列規則；「一份清單」先定義+/*/?與分隔符後列repetition；診斷段先分matcher拒絕與生成Rust型別不符。圖箭頭只表示編譯生成與之後runtime借讀，不畫為move；actual site視覺與獨立技術/銜接待審。


## 17.2–17.3 author update
17.2 complete normal library/hygiene/callers/diagnostics and 17.3 complete proc-macro/consumer/tests/diagnostics inline. Actual EN/ZH originals and fixed syn/quote API reading precede drafting (ch17-root-preparation). Existing Rust unchanged; early command placement now waits actual full library teaching. Selected verification and immutable freezes: ch17-root-authoring/validation-17-2.json, freeze-17-2.json, validation-17-3.json, freeze-17-3.json. Independent technical/continuity/site reviews pending, not accepted.


## 17.4–17.8 author completion
已完成全八節作者稿與每節source freeze。17.4 complete public API/library/tests/compatibility/fixtures/答案同頁，20項current selected checks；17.5完整生成/負例縮減/回歸/合成答案同頁，8項current selected checks；17.6完整Loom與Miri normal/helper/UB-only fixture及答案同頁，current普通/Loom/固定nightly Miri均符合預期。17.7/8完整manifest/lib/example/test/bench/答案同頁，current selected correctness/compile/Clippy/fmt；保留2026-10-14歷史bench/profile，12份owned source/manifest hash無drift，不將新root lock指紋寫回歷史記錄。current viewer親看三歷史JSON，cloned326400bytes/25600blocks、borrowed零root、solution81600bytes/6400blocks、峰值15/end0及clone來源鏈吻合。viewer的file URL初次被環境禁止，改用本機HTTP提供相同公開viewer靜態檔後正常，不冒稱量測重跑。
root草稿17.6編號替換曾誤碰0.7.2/17.2，凍結前已改成有數字/小數點邊界的替換並親核版本/URL；沒有改Rust、沒有重跑成功Miri。每節freeze/validation及historical-source-check/current-view-evidence在ch17-root-authoring。獨立technical/continuity與最終網站原位驗收仍另行完成，不以作者check當accepted。
