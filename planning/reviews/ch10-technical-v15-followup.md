# 第 10 章 TECH CROSS 與 v15 局部覆核

原 TECH 的兩項必要首次介紹及兩項可選改善，在指定 v15 的三處正文已妥善處理。本輪未確認新的局部必要問題。原初判的分級、發現時點與未驗範圍保持；這是封存後交叉與相稱覆核，不是第二次獨立發現、blind 首讀或完整發布驗收。

## 角色、實讀與版本

- 審閱者仍是 `chapter02_ownership`，不是第 10 章作者；先前 ownership／lifetime／error／9.2–9.4 作者與 CONT3／7／8 背景沿原稿申明。原 `ch10-technical-v12.md` 的 SHA-256 仍為 `61c9d095c761620d571af0d04412625aff7802793695de5e697e5a5a882215cd`，未回寫任何分級或原句。
- 本輪在原稿封存且 root 開放交叉後，全文讀 FIRST10 的 `ch10-first-reading-v1.md`（594 行）、CONT10 的 `ch10-continuity-v12.md`（184 行）及 `ch10-root-disposition.md`；沒有讀 peer11、作者10 ledger／source notes、live10 或其他新版10。
- 唯一修後來源為 `/workspace/work/tutorial-implementation/frozen/ch10-review-fixed-v15`。manifest SHA-256：`b13b772650ec2ee1523f665617f6c25209cc6518a16b4dbd7a7a392573bbbde0`。原 v12 manifest 仍為 `b22762f35195ce6fc287cca720b5fa6812b519aaa0f8ef4e9a76c9e2e291632e`。
- 修後實讀為三個 material BODY 002-005／003-006／004-007，及前後單位 002-004／006、003-005／007、004-006／008；另讀 003-004，核一份原初判已通過圖的正文對應。共十個單位，不把機器重組三整頁當作重讀全部單位。
- actual 前文局部回讀 2v5／006-005 的 mutable slice 走訪、5v7／004-002 的 prelude；3v6／006-002 的 Vec 方法與 8v9／002-003 的普通 drop 沿原 TECH 已全文實讀證據。指紋另存，不以大綱標題支持新句。

FIRST10 是模擬基本 Python 讀者的 AI 首讀，不是實際新手研究；它逐讀全部 68 BODY，七份答案保持關閉，未讀 archive source 或執行 Rust。CONT10 全文讀 68 BODY／121 sources，沒有執行 Rust，另有自己的 48 個原位畫面；原 TECH 全文讀同範圍、實看自己的 18 個原位畫面，只有原稿列明的有限 probe。三者的不同證據範圍保留，不互借執行或視覺宣稱。

## 自己比對的修改範圍

逐一比較原 v12 與 v15 的 manifest source key、archive bytes 與實際 SHA：**121 個來源、Cargo.toml、helper、tests、八份答案、compile／runtime 負例、expected 及九 SVG 全部相同**。來源集合亦無新增／刪除。所有 v15 source hashes 與實際檔案一致。

BODY 有 12 個 raw hash 差異。自己消除僅 snapshot asset 起點的差異後，material 差異只有 002-005、003-006、004-007；另外九處 001-004／002-003／003-004／004-004／005-004／006-004／007-003／008-005／008-007 僅圖 URL，九個圖的 bytes 均相同。這項自做比對與 root 的 `ch10-fix-scope-proof.json` 相符，沒有拿 root proof 代替本人的比較。

完整輸入 SHA、逐檔 v15 指紋、source comparison 與 material diff 在 `work/ch10-technical-v15-followup/input-diff-proof.json`、`material-diff.txt`。本輪不改正文／source／共享 Cargo／lock／nav。

## 同一需要的原發現、分級差異與處置收益

| 關係／位置 | 原 TECH 自行發現 | 封存同行與 root 紀錄 | 交叉判斷；保持原分類 |
| --- | --- | --- | --- |
| literal 型別後綴／002-005 | N1 必要：明選 Any payload 的確切 u32 型別，需要先懂 `7_u32`。回查 actual1 與 canonical1–8，確認數字中間分組底線不等於尾端型別後綴；後續 `80_usize` 同一語法。 | CONT N1 也獨立列必要並回查前文；FIRST 只列可選 unknown，當時沒有完成全前文核對，能猜型別但未確定語法來源。root 紀錄它更早自行在 v14 準備發現；本人沒有見證其當時過程。 | 專家確認首次缺口與 FIRST 的當下未知是同一需要，證據與分級不同。採最小介紹適當；不事後把 FIRST 升為已確認必要，也不把 root 的先前發現算本人重現。 |
| `cargo test --lib`／002-005 | N2 必要：命令首次採新的 target selector，須說此次只選 library 測試，不能由 `mod tests` 猜 Cargo 選項。原有限 probe 後的查漏已誠實記錄。 | FIRST／CONT 原稿都未指出；root 後續查 canonical1–9 並採一句契約。 | 這是 TECH 原獨立發現、同行的局部漏查；root 後核是交叉證據，不是第二份原獨立重現。未因此否定同行的 payload 理解，也不以他們整體通過遮掉操作缺口。 |
| 省 use 的普通 drop 名稱／003-006 | O2 可選：8.2 已教函式與清理行為，合法 bare name 少交代 prelude 名稱來源。 | FIRST 列必要，具體負擔是需猜「為什麼不用 use」；CONT O1 可選。三者都已理解普通函式會消耗 captured String，沒有認為整個清理機制漏教。root 採句在 FIRST 封存後；兩專家仍各審原 v12。 | 必要／可選差異來自讀者能否自行接起名稱作用範圍，沒有技術行為矛盾。短句能解 FIRST 的實際猜測，也改善兩專家指出的一致性；原三份分級完整保留。 |
| as_mut_slice 的前文定位／004-007 | O1 可選：API 實教於3.6，2.6教的是 slice，原回指會讓人找錯頁。 | CONT O2 同為可選；FIRST 未列此項，但已能從具體練習推兩份不同資料各自更新、兩個結果先等待。 | 已教方法的錯誤定位，不是 API 超前。兩個正確回指讓需要回讀的學生找到方法來源與 mutable reference 走訪；不升成缺少整個借用機制。 |

以上是各 original 真正記錄的關係；本人沒有事後提示才新增的原發現，也沒有把全文讀同行後的理解冒稱重新獨立驗證。

## 三個修改單位與相鄰過渡的實際核對

**002-005：**測試之前新增「數字 literal 後可接型別名稱」與 `7_u32`／`7u32`、底線可省、與1.4變數 `: u32` 註記的區別。先教後用的位置正確，既有 u32／usize 已有來源；學生可把規則用於之後未改的 `80_usize`。句子沒有把數字後綴說成轉換。測試後、命令前的新 `--lib` 句明選 package library 測試 target，並說不一併執行 binary／example／integration test；沒有宣稱不編譯任何其他材料或改變測試內容。符合原 TECH 已實讀的 1.99 Cargo Target Selection 契約，無須新 probe。

前段仍先做兩種文字的 owned downcast 與未知型別 fallback，再用直接建立的三種 payload 測同一 helper。新說明沒有把測試宣稱為真的 spawn／panic，也未以一次 panic 的型別代表所有 panic。後段仍從 join 的 owned 權限進入 detach 與兩份工作先收尾的練習：通過／業務 Err／panic／等待權限四個問題沒有被新工具句混合。

**003-006：**新增普通函式是 `std::mem::drop`、已教 prelude 提供短名稱，並區分 `Drop` trait。actual5.4 確有「每個 module 自動帶入常用名稱」的 prelude 背景，actual8.2 已教這個普通函式；因此沒有新增清理機制或默認學生知道名稱來源。完整 code 仍在 closure body 消耗 captured owned String，這才只提供 FnOnce；沒有把 `move` 本身當只能呼叫一次的原因。

相鄰 003-005／007 仍分開 capture 的來源 lifetime、固定 T 不可夾帶局部借用，以及每次輸入／輸出同 `'a` 的 reference 投影。新句只解決 bare name 的來源，沒有把 HRTB 的「每次合法輸入借用」偷換成重複呼叫能力、static capture 或來源延壽。

**004-007：**練習現在分寫「3.6已教的 as_mut_slice」與「2.6的 mutable slice 走訪」。actual3.6 的 Vec 操作表與例子支持前者；本輪回讀 actual2.6 的 `&mut [i32]` 走訪，確實說每個 value 是 mutable reference、可寫 `*value += amount`。因此兩項定位都精確，原 TECH O1 的最低修正之外也讓走訪有直接前文。

原材料／答案不變：兩 worker 各借不同 Vec、各加一，兩 handle 先 join 成 tuple 再 match；長度2／3、小計202／9、成功211；panic 分支只報可能部分修改，沒有許諾 rollback。前面的等待／unwind／自動收尾限定及後面的 channel 過渡仍一致。這裡仍是練習引導與完整答案，沒有把後文 channel 方法提前當解法。

## 同行通過理由與物件限制

全文交叉時也核同行「通過」的因果理由，以下沿原 v12 的完整實讀與 unchanged source 證據對照；不是本輪重讀全部 v15 或重跑各例。

| 同一原需要 | 同行理由與原 TECH 核對的支持；限制 |
| --- | --- |
| Send／Sync 能否跨 thread 交值／借值 | FIRST 能分 compile-only helper 與真工作；CONT／TECH 把值交接、`&T: Send`、closure 呼叫能力及 static 借用限制分開。Batch200／Cell4 是有限資料或編譯期核能力，不是實測速度，也不將 move 改成共享保護。 |
| 真工作結果何時可用 | 三原稿均分 worker200＋main6 的等待位置、業務 Err 的外 Ok、panic payload 的外 Err、丟 handle 不等也不取消。兩份先各 join 再判斷支持練習收尾；FIRST 未開答案，不能替它宣稱完整答案查過。 |
| HRTB 的輸入／capture／成果 | FIRST 能從原句分輸入全稱、外捕捉與輸出 T；CONT／TECH 的 full source／負例同樣支持局部來源不能逃出、只接 static 不夠通用、owned String 的一次消耗與借用通用性互不替代。新增 drop 句保持這一用途。 |
| scoped 借用與等待區段 | 三者分外 env 借用、scope 內建立來源仍可能太短、丟 handle 後 scope 仍追蹤；CONT／TECH 另核完整 bounds／答案。自動等 body 與手動 join 的 TLS 清理範圍依1.99契約，等待完成不等於業務成功或回復部分修改。 |
| channel 各種完成責任 | FIRST／CONT 與 TECH 都分 send accepted、receiver received、處理完成、worker joined；all-senders 消失後仍 drain，bounded／容量0的限定與錯誤保原值有明確角色。壞互等／保留 sender 不用執行才能依契約指出；公平／取消／固定時間不在保證內。 |
| Arc／Mutex 保活、guard 權限與狀態政策 | 三原稿均沒有把 Arc 保護計數說成保護全部 T，也分 Mutex guard 的整段 get/set 與 Arc<Cell>。80／0 poison 暴露業務 invariant；採 reset0／0 再 clear 是明選政策，未 poison 不代表業務正確。原 TECH 的有限 poison 執行只驗所選案例。 |
| RwLock reader／writer 與重查 | FIRST 理解最後使用不 drop、兩 reader 都解除才寫，以及重取 writer 需重查；CONT／TECH 另核所有 source、權限條件與 downgrade 官方契約。順序與優先規則沒有宣稱所有平台公平；一致 lock order 只排該循環，不證全部 deadlock 消失。 |
| atomics 的修改順序／發布／共同順序 | FIRST 的逐段理解與 CONT／TECH 的契約推理都維持原前提：fetch_add 加 join、Acquire 真讀自唯一 Release 且沒有後寫、四步皆 SeqCst 的共同順序矛盾。TECH 原 N4861 模型與 source 對應仍在；任何一次 host 結果、未見0／0或 yield 都不是不存在反例／保證收尾的證明。 |

FIRST 的 closed answers 與無執行、CONT 的無執行、TECH 的部分執行各自限制仍在。本人未因三者大多理解相同，就補稱其他41份 normal／答案、20份 compile-fail、其他平台、完整 Clippy／all-targets、輪詢協定、死鎖壞例或正式發布均通過。一般可推理的有限主線與沒有實測的性質也不自動列缺陷。

## 指定 CSS 的本輪實頁證據

只從授權的 ch09 frozen archive 複製 `sources/docs/stylesheets/tutorial.css`，沒有開第9章 BODY。其 SHA-256 為 `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`；內容僅讓 `.md-typeset details` 繼承正文字級。自己用原 TECH 相同 theme／font false 設定重組固定10.2／10.3／10.4預覽，明寫 `extra_css = ["stylesheets/tutorial.css"]`，HTTP 載入的 CSS bytes 實核與授權來源相同。未修改共享 CSS。

Chromium 在1280×1200與390×1200，分別擷取 literal 後綴、library target 句、drop 名稱來源、HRTB 圖、練習指引（答案閉合）、答案展開開頭、答案後的解說，共 **14 個畫面全部 view_image 親看**。記錄 URL／bbox／字級／開合狀態／asset SHA，保存在本輪 `visual-metadata.json` 與相應 screenshot。

- 新三處正文均在使用／命令之前可見，手機段落正常換行，沒有遮住 suffix／prelude／兩個回指。`--lib` 的相鄰命令仍可辨識；本輪沒有操作所有長 code 的水平捲動。
- 答案展開後，article／details／解說 prose 的 computed font-size 都是15px，code 是12.8px。已親看答案開頭與底部解說在兩種 viewport，字級一致、文字未出框；手機長 code 仍需水平捲動，沒有宣稱其每行都在同一畫面可見。這是有限頁面中的 CSS 效果，非全站 CSS／所有折疊區的驗收。
- 至少一份原本未報問題的圖具自己的舊／新實頁依據：hrtb-local 的原 TECH 18圖紀錄已通過，本輪再親看桌面400×690、手機約343×592。圖 bytes 與原／v15／HTTP 來源相同，SHA-256 `2a79b19cf773ad6245c6045b2966931f4f0dec24f58e82d430929ba91915aa08`。F任合法借用能力、body內80、藍色本次借用、綠色200成果、T=i32及相鄰「不延壽」說明清楚；沒有把框高度當 lifetime，也沒有帶局部 reference 出框。

隔離預覽建置 exit0／No issues found 只確認這三頁重組可建。本輪沒有驗全站 nav／外部連結／全部答案／發布；preview 配置、選定頁重組與截圖是自己的工作材料，不借同行或 root 的實頁觀察代答。

## 封存與停止

四項最小處置各有對應收益：必要 N1／N2 在第一次用法前補齊；可選 O1／O2 校正定位與短名稱來源。原三份獨立報告的分級、發現時點、closed-answer 與 probe／平台限制保留，不回寫原 initial。

原 Rust／Cargo／tests／fixtures／答案 byte 全同，本輪沒有新 Rust probe 或重跑未變 suite；新增文字只說既有語法／普通函式／Cargo 選取契約，不改 behavior。只做自己 hash／diff、actual 相鄰文本與前文、同 theme 的局部實頁覆核。

本文件、SHA sidecars、本輪證據副本封存後再次核原 TECH SHA、原 v12 與 v15 全檔指紋不變。完成 CROSS10 任務後停止，不開 peer11／其他未授權角色，也不改學生或共享檔案。
