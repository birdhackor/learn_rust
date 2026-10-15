# 14.8 作者來源：用配置profile核對可移除的工作

使用clear-tutorial、writing-prompt與Rust術語指引；基本Python起點，來源閱讀不是release QA。

## actual gate

Fresh實讀root `ch14-benchmark-prerequisite-v6` 全七units與凍結package的完整counts／make_workload／tests／兩份benches／measurement：相同query／行數／輸出、黑箱／計時範圍、release、噪聲與資料分布限制已教。本節不改原source，不拿兩個shared-environment時間估計解作CPU熱點。

## 英文與中文實讀範圍

| URL | 實際閱讀 | 取捨 |
| --- | --- | --- |
| [EN Bencher Custom Harness](https://bencher.dev/learn/benchmarking/rust/custom-harness/)／[中文對應](https://bencher.dev/zh/learn/benchmarking/rust/custom-harness/) | 完整主文What is Benchmarking到Wrap Up；中文主文從什麼是基準測試到總結，包括global allocator、Profiler／HeapStats、六項指標、輸出範圍與完整runner | 題目先固定、再選要量的指標；可把stdout放外面，但就須承認未量stdout。中文頁標記AI翻譯，與英文逐項核對；英文現多Gungraun而中文舊列表不同，不搬工具流行／維護時效主張。不用inventory、自訂註冊框架、serde_json::Value或Bencher Cloud服務；也不採「black_box必定讓全部loop保留」的過強描述。 |
| [Performance Book Heap Allocations](https://nnethercote.github.io/perf-book/heap-allocations.html) | 全文：Profiling、Box／Rc／Arc／Vec／String、clone／to_owned／Cow、重用容器、讀行與避免回歸 | Clone成本依型別與工作而變；先用profile找發生位置，不把原書rustc數字外推固定比例。選已懂String；不新增SmallVec、Cow或替換allocator來改任務。 |
| [中文Rust精選：張漢東原文](https://rustmagazine.github.io/rust_magazine_2021/chapter_12/rust-perf.html) | 引子與總原則段落；其餘主文尚未作完整閱讀驗證，不算本次全部已讀 | 補可讀性與避免提早最佳化的背景；本次完整中文操作來源以上述Bencher主文為準，技術契約仍由固定官方API核。 |
| [Valgrind DHAT manual](https://valgrind.org/docs/manual/dh-manual.html) | 前輪已實讀10.1–10.3／10.5，這輪核對total／peak與PP stack角色 | 區分Valgrind的讀寫／copy追蹤與dhat-rs僅配置資料。沒有安裝／執行Valgrind，不把crate資料稱成Valgrind或CPU profile。 |

rustwiki.org兩個translated PerfBook URL回403，未繞過、未算已讀；RustCourse幾個TODO空章也未當完整中文閱讀。EN與ZH Bencher的原HTML與主文文字保存work供核對，下載全文不代替以上真正閱讀。

## 固定官方API

從crates.io取dhat0.3.3正式crate，實讀src/lib.rs setup／running／viewing的完整doc、Profiler／ProfilerBuilder、HeapStats以及寫檔／drop流程。取得來源與SHA在 `/workspace/work/ch14-profiling-research/download-manifest.json`。

- [dhat0.3.3官方](https://docs.rs/dhat/0.3.3/dhat/)：experimental工具、系統allocator wrapper、只記選定Profiler活躍範圍；工具自身追蹤活動忽略，沒有memory-access或memcpy／CPU追蹤。範圍外原材料配置被忽略，範圍內若realloc原物件則當新配置，故本例只借讀已建材料、不擴容它。
- [Rust static reference](https://doc.rust-lang.org/reference/items/static-items.html)：item／型別／initializer與Sync、整段程式可用、不在退出時drop。只用immutable static的dhat Alloc值，不用static mut或manual unsafe allocator。
- [std global allocator契約](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)：#[global_allocator]挑單一程式使用的配置介面；dhat::Alloc提供實作。本節只使用公開工具型別，不寫trait unsafe impl或逐pointer配置。
- `Profiler::builder()`建立預設heap設定；`file_name<P: AsRef<Path>>(self,P)->Self`保存路徑；`build`開始且已有Profiler時panic。Profiler drop先結束追蹤再寫JSON／stderr；寫檔失敗輸出診斷，不從drop提供Result，不能只以程式exit0證明檔案完整。
- `HeapStats::get()->HeapStats`需活躍heap profiler，否則panic。total_blocks／total_bytes是u64累計，curr_bytes與max_bytes是usize；max為同時活著的受追蹤bytes峰值，不是累計。snapshot自己擁有數字，可以先取後停再顯示。
- `CARGO_PROFILE_RELEASE_DEBUG=1`的command範圍profile覆寫角色按Cargo profiles／environment variable正式說明；保留release最佳化，只讓工具取得來源位置，不修改root manifest。

## 選定實驗

獨立examples/ch14-profiling，library只依賴已完成ch14-performance，dhat只用於配對profile examples與64行參考答案。各example是自己的可執行crate／唯一global allocator；各自一次Profiler、同256行／100次／Rustquery、總行數6400。預先建置及答案核對，profile內只做同一函式重複計數；snapshot後drop，再stdout。正常14.7 benchmark保留其原配置器與數字，這裡不量有工具介入的執行時間。

## 補充實讀與config核對

實際閱讀std GlobalAlloc主文「The #[global_allocator] attribute」：選定static作程式配置介面，編譯器可略去／合併配置，不能用allocation數作普通測試的安全或語意保證。本文將數字限定為固定工具鏈／這次實測，不設allocation threshold test。另實讀Cargo environment variables的 `CARGO_PROFILE_<name>_DEBUG` 項，連回profiles的debug層級；選release debug=1保留來源位置，沒有切換成dev profile。已保存原HTML及selected文字於work。

## 實際profile／viewer核對

三個release／debug=1 examples皆exit0，fresh JSON可解析，實際載入正式DHAT viewer的HTML／JS／CSS（public來源保存SHA，不改viewer）。cloned只有一份非空PP；已讀其Total／At t-gmax／At t-end與Allocated at全部12frames，確實從main18→repeat_counts12→count_cloned21→String Clone。borrowed只有空root，沒有配置PP；solution為同來源、64行，總量下降峰值仍15。viewer的raw文字與真實截圖另存work/viewer-evidence，學生正文選必要指標與source四層、不教其餘標準庫私有框架，也不拿viewer經過／平均存活時間解作CPU比例。

## 最終局部名稱核對

Root實讀14.8指出8.2以use匯入std::mem::drop，本例未寫use而直接drop。實際核對[edition2024 prelude](https://doc.rust-lang.org/std/prelude/rust_2024/index.html)重導出v1，以及[std prelude v1](https://doc.rust-lang.org/std/prelude/v1/index.html)的 `pub use crate::mem::drop`：在API表之前用一短句連回5.4已教prelude。這是正文名稱銜接補強，不修改Rust、profile原檔或原驗證。
