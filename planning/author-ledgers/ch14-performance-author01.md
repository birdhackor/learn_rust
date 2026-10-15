# 14.7／14.8 作者先備與first-use紀錄

clear-tutorial／writing-prompt／Rust術語指引；基本Python入口。實讀紀錄見專屬source notes。14.7 actual gate為9.1 v5六units、7.9 v13八units、6.3 v10六units與11.4 v4；14.8尚待14.7實稿凍結。

## 14.7寫作前安排

| 必要內容 | 首次需要處 | 真實前文 | 本節先教 | 可預測結果 |
| --- | --- | --- | --- | --- |
| benchmark／microbenchmark／公平工作量 | 起點搜尋行數 | 7.9資料規則 | 任務與同輸入／答案表 | Rust Rust仍一行；空query維持拒絕，不把少做工作當快 |
| 借讀／owned複本與clone/drop成本 | 兩個count版本 | 3.4文字／7.9owned表示／8.2收尾 | source前說何種工作相同與哪種額外工作不同 | 相同行數，owned每行建立／釋放文字仍在計時內 |
| 合成workload／分布 | make_workload | 1.7range／1.8迴圈／3.6Vec | 固定64／256行，四行一個符合 | 可手算16／64，兩種輸入同時交兩個方法 |
| 正確性先驗 | integration tests | 1.10assert／7.9需求驗收 | 固定相等答案與邊界先定 | 綠色測試不等於任何效能門檻 |
| black_box | bench callback／練習 | 3.3泛型／6.3callback | 最佳化可能消失的工作與identity契約 | 同值交回、降低常數假設，不關最佳化 |
| benchmark target／harness false／bench profile | manifest／command | 1.9targets／9.1profiles | 欄位與命令前教 | benches是外部caller，用穩定Rust＋Criterion runner |
| Criterion／Bencher／iter | first bench | 6.3FnMut／11.4Duration | source前API表與兩層closure | 實際計時只包內層每次工作，samples不是呼叫次數 |
| group/main外部macro config | bench runner | 已有macro使用形式 | name／config／targets與CLI角色 | 不預借14.1自訂macro matcher，selected外部使用契約足夠 |
| warmup／sample／confidence interval | 結果之前 | 11.4量測不同於等待設定 | 用批次重複工作與區間解讀 | 三欄不是min／median／max，不保證單次延遲 |
| baseline／不同ID／環境限制 | actual結果 | 上述工作範圍 | sameID前次資料；實測環境固定、噪聲／代表性限制 | 不把不同ID change欄當二方法比較，不稱所有輸入必然改善 |
| 完整練習：把素材建置移入iter | scope比較 | 上述所有API | 新需求與預期范围先定 | 測到含準備的新工作，不能和純count數字混談 |

隔離package `examples/ch14-performance`，root整合Cargo／lock／nav。14.8只預研，不新增source。實際執行／視覺／sha於完成後追加。

## 14.7 完成與作者驗證

正文 `docs/chapters/14-07-fair-benchmarks.md`、獨立package與 `docs/assets/ch14/benchmark-boundary.svg` 已完成。測試snippet放在make_workload正式介紹之後，避免helper只憑前面提過名稱便進入測試。所有正常source、兩份bench及完整練習只使用實際前文或本頁先教的介面；Criterion group/main是選定的外部macro使用契約，不預借自訂matcher章節。

- scoped fmt、alltargets Clippy `-D warnings`、dev與release各3項integration tests、正常benchmark_inputs stdout、line_counts bench `--no-run`全部通過。兩份bench的測試模式亦成功執行各項routine，這只證明可運作，不算效能測量。原指令／exit／stdout／stderr／source SHA保存 `/workspace/work/ch14-performance-verification-v1.json`。
- 同一份source連續實跑兩輪line_counts，各含borrowed／cloned、64／256行四項；每項30samples、500ms暖身、2s目標。每份輸入的符合比例1/4，816／3264 bytes，答案16／64；iter外建立材料及核對答案，iter內包含完整count呼叫、cloned的String複本與收尾，以及呼叫／迴圈的量測成本。
- 整份結果保存package `measurements/reproduction.json`、run-1.txt、run-2.txt及exercise.txt，workspace work另保留同份measurement-v1。環境Linux x86_64／Xeon8573C、Rust／Cargo1.99.0／Criterion0.8.2、opt-level3／overflow checks；來源SHA、配額、觀察到的負載、工作次序也保存。共享環境與暖身重用的合成短行資料限制在正文說明，沒有固定倍率、通用勝負或把量測值當延遲保證。
- 同source第二輪出現regressed訊息，保留原報告，正文先教same-id baseline與不同id比較的差別；不把工具的樣本差異診斷說成程式修改造成退步。時間三欄／95%confidence interval／samples與iterations／µs單位都在實測表之前教，沒有把三欄誤作min／median／max。
- 完整練習另實跑兩種行數；計時改成包含清單準備與結束，不把這一輪與純count的差值解作精確配置成本分解。14.8尚未有profile，沒有由benchmark總時間虛構CPU熱點或配置占比。
- 隔離preview以repo同theme／extensions與真實snippet strict build通過。親看1280×800與390×844的26張原位畫面：材料／workload／tests、Criterion契約／計時SVG／bench、暖身設定／runner、結果意義／兩輪表／baseline及練習折疊與展開。手機數值表可直接讀，code有內部水平捲動，頁寬不溢出；SVG有明確390×645 intrinsic尺寸、24px字，手機仍可辨iter外準備與兩個各自計時框。各方法框不是並行、箭頭與框線的角色先在正文說明。
- 原位畫面SHA／viewport記錄 `/workspace/work/ch14-performance-preview/visuals/manifest.json`。實頁發現三處粗體收尾與CJK緊鄰使Markdown顯示literal星號，最小補空白後重建置、捕捉並親看macros／confidence兩段的桌面與手機四張局部，另保存postfix-manifest.json；沒有覆蓋原稿畫面，也沒有改Rust source而重跑測試。以上為作者受查範圍，不宣稱獨立release QA。
- 正體字與Rust angle型別inlinecode自查；14.8仍停在來源／工具可用性研究，等root正式凍結本節後才寫正文／配置／程式。

### root銜接查核後補足numeric suffix先備

root指出此前1–9正文未正式介紹數字literal後綴，作者原安排也未定位此語法的實際介紹位置。root在10.2的payload test前當地補 `7_u32`／`7u32`、型別名稱後綴與可省底線的契約，保存actual `ch10-thread-suffix-prerequisite-v13`；作者已實讀指定001-005新增段落。14.7的 `64usize` 現有完整、時間順序在前的必要介紹，六個已測source檔案保持原SHA，不在本節另重印一次或重跑量測；不把這項修後先備回寫成原稿本來就已核對。
