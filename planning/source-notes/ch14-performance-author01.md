# 14.7／14.8 作者來源研究：公平量測與改善證據

使用clear-tutorial、writing-prompt與Rust術語指引，入口為基本Python。這份是作者實讀／先備紀錄，不是獨立release QA。

## 14.7實際先備

重新實讀 `ch09-complete-prerequisite-v5` 的9.1全六units：profile／release與overflow checks、features合併、cfg、target；`ch07-review-fixed-v13` 的7.9全八units及snippet中的真實search／報告／I/O／測試：同一行只算一次、區分大小寫、contains、owned判斷表示、純工作與CLI出口；`ch06-review-fixed-v10` 6.3全六units：FnMut重複callback、shared capture與function pointer。11.4為作者已完成、root凍結的 `ch11-timing-prerequisite-v4`，交付SHA與先前驗證相同；Duration／std Instant與受控時鐘不同已成立。沒有把並行章節未交付標題算先備。

## 英文與中文實讀

| URL | 實讀範圍 | 可學內容與本節取捨 |
| --- | --- | --- |
| [EN Rust Performance Book Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html) | 全文31行：相同工作比較、代表輸入、wall time變動與多workload摘要限制 | 先確認同一答案與工作量，再量固定純工作；用64／256行兩份可追蹤材料，不把合成輸入當真實使用分布，也不把某次較快外推所有程式。 |
| [Criterion官方Getting Started](https://criterion-rs.github.io/book/getting_started.html) | 全153行：dev-dependency、harness false、benches外部crate、公有函式、callback／iter、macro runner與結果 | 參考逐項介紹設定和runner；不採遞迴Fibonacci或inline attribute。書中版本0.5.1與例子圖表不可當作本次0.8.2契約，API另核固定版。舊bheisler起步頁0.3亦實讀，但不用其deprecated criterion black_box。 |
| [ZH RustCourse benchmark](https://course.rs/test/benchmark.html)／[原始章節](https://github.com/sunface/rust-course/blob/main/src/test/benchmark.md) | 全章240行及原始Markdown：nightly libtest、初始化在iter外、相同重複工作、結果未用可能被最佳化、Criterion入門 | 借鑑「先看不合理0ns再追計時工作是否存在」的問題；本節直接教std black_box最低契約，不改repo到nightly、不寫超過u64的Fibonacci100..200或照搬舊criterion0.3。 |
| [Criterion Timing Loops官方原始頁](https://github.com/criterion-rs/criterion.rs/blob/master/book/src/user_guide/timing_loops.md) | iter、drop、iter_with_large_drop／batched適用情況、固定輸入可重用、measurement overhead全文 | iter只對我們選定的callback計時；內部clone／drop仍包含，預先建立素材是排除準備，不等於測到整個CLI。只選iter，不先用batched／BatchSize等介面。 |
| [Criterion Command-Line Output](https://criterion-rs.github.io/book/user_guide/command_line_output.html) | Warmup／Collecting Samples／Time／Change段落：批次iterations與samples、每次估計、區間、同ID前次baseline與環境負載 | 在展示數字前解釋三欄是估計與confidence interval，不是min／median／max；不同ID不是自動前後改善比較。舊頁noise threshold敘述按本次固定source核，不搬入門檻主張。 |

course profiling/performance/benchmark只有todo標題，tools只有兩條外部連結；已實看但不把它們當完整教學來源。rustwiki與猜測course advance路徑無法開啟，未算已讀。實際中文來源為test/benchmark全章。

## 選定固定API來源

正式crate為Criterion0.8.2，最低Rust1.86。從crates.io下載官方 `.crate`，只讀必要manifest／lib／bencher／macros；錯猜0.8.2 GitHubtag得到404，實際tag是criterion-v0.8.2，保留取得紀錄，不把404算閱讀。下載只是取得素材，下面才是實讀內容。

| 正式來源與部分 | 本次契約 |
| --- | --- |
| [Criterion0.8.2](https://docs.rs/criterion/0.8.2/criterion/struct.Criterion.html)／官方crate src/lib.rs Default、sample_size、warm_up_time、measurement_time、without_plots／configure_from_args | Default建立runner；取樣數至少10、兩個Duration不可0；設定是新self交接；正常選30samples、500ms warmup與2s量測目標，可能自動延長。關圖表不關計時／文字報告。 |
| 官方crate src/bencher.rs的完整iter doc與實作 | FnMut()->O可重複，計整批後估每次；routine結果在迴圈內drop。選usize輸出，不忽略function內owned文字建立與收尾。 |
| 官方crate src/macros.rs兩macro完整定義與使用契約 | criterion_group設定form生成runner group，criterion_main提供main／CLI配置；先教每個name/config/targets與runner角色再source，不需要先寫自訂matcher。 |
| 官方crate Cargo.toml.orig features | 關default的rayon／plotters，僅選cargo_bench_support。core同步量測即可，不選async runtime或自訂measurement。 |
| [std1.99 black_box](https://doc.rust-lang.org/std/hint/fn.black_box.html) 全頁 | 泛型identity接手並交回同值，對最佳化提供best-effort提示；包輸入降低常數假設、讓輸出被觀察；不是關掉release最佳化或正確性證明。 |
| [Cargo bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html) 描述／harness／bench profile／--bench／--no-run | 新command與bench target先教，harness false需自己的runner main；只編譯不等於執行量測。bench profile採release型設定，實際root不另改。 |

取得URL與素材SHA保存在 `/workspace/work/ch14-performance-research/download-manifest.json`、download-extra.json；固定API使用官方0.8.2crate原始碼核對，不以latest混版本。

## 14.8預先研究，暫不寫程式

[EN Performance Book Profiling](https://nnethercote.github.io/perf-book/profiling.html) 全75行實讀：熱點、工具各自量什麼、Cachegrind／Callgrind指令計數／模擬資料、DHAT配置與memcpy、release debug info與stack資訊的限制。中文RustCourse performance/tools只有連結，仍需下一階段補真正中文profiling內容；目前不聲稱中英研究完成。當下command -v perf與valgrind均未找到；這是工具可用性觀察，不推論權限阻擋或要求使用者token。等14.7 actual凍結及root工具安排後，才選14.8 profiler／操作契約和source。


### 14.8後續取得與實讀範圍（仍不寫source）

[Valgrind官方DHAT manual](https://valgrind.org/docs/manual/dh-manual.html) 實讀overview、Running／output file、header／PP tree與allocation stack、total bytes/blocks vs global peak/end、copy profiling，涵蓋10.1–10.3與10.5。指令計數表示進度，不是正常機器的秒數；完整行程啟動／素材準備也在profile中，不能把全程Total當單次函式配置。下一節若選用，需先教工具模式、輸出欄位、byte/block與歸因範圍，並親看真實viewer／配對同工作profile。沒有實作custom allocator或任何unsafe碼。

Chinese RustCourse clone-copy只有標題，early-optimise原始章節僅短段並有函式呼叫必定消除的過強敘述，不能作全面效能結論；仍不把兩頁當完整中文profiling材料。本機apt-get3.0.3可執行，路徑/usr/local/bin/apt-get；uid1000、sudo未找到、cargo install list空。這些是可用性觀察，未安裝或執行profiler，也未推論安裝一定被拒。

## root查核後補交的actual先備

root在10.2 payload test前補齊數字literal型別後綴，交付 `ch10-thread-suffix-prerequisite-v13`。作者已實讀指定001-005：數字後接型別名稱、7_u32／7u32的可省底線，以及與變數型別annotation位置的區別。這使14.7的64usize有真正已教的chronological前文；原作者安排未定位此介紹位置，修後補足另外保存，不回寫成當初已知道。
