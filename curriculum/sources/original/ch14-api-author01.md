# 14.4／14.5 作者來源與取捨

作者：chapter01_foundations。讀者具備基本Python；寫作依clear-tutorial與本專案術語指引。本文件不是首次閱讀提示，也不代表獨立審閱通過。

## 14.4 實際先備

重新實讀`ch08-complete-prerequisite-v8`的8.6全部6units、8.9全部7units，及凍結errors.rs／newtypes.rs完整source；`ch09-tooling-v1`全部6units、manifest與library source；`ch07-output-fixed-v11`7.9的契約、模型／錯誤、純搜尋、報告／I/O及main邊界units1–6與library source，另實讀7.5 Path／PathBuf與Path::new段落。只採用真正已教的操作，不把14.1–14.3尚未交付標題當先備。

## 14.4 動筆前重新實讀網路

| 來源 | 本次確切閱讀範圍 | 借鑑與本課取捨 |
| --- | --- | --- |
| [英文Book14.2](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)／[簡體對應14.2](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html) | 文件註解與doctests、pub use重導出及公有路徑、發布metadata／版本段落；中文pub use指定正文位置亦另讀 | 公有入口與內部module分開，caller不需跟內部檔案重排。7.2已教文件測試，不照搬unimplemented!色彩範例或登入token流程。 |
| [Cargo SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) | 導言／change categories／0.x；public item路徑、private/all-public struct fields、enum新增variant、non_exhaustive追加、trait簽名／generic bound與feature移除條目 | 以同一caller比較source與行為，不把「編譯成功」等同契約保留。小型雙版本快照實際編譯相同consumer，版本表只推本例能支持的範圍。 |
| [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) | default與啟用、feature unification／additive／互斥段落 | 9.1基本契約回收為公開能力承諾；text只增加報告函式，不替純搜尋換規則。不加入新的optional dependency／resolver例外旁支。 |
| [RustCourse原始features](https://raw.githubusercontent.com/sunface/rust-course/main/src/cargo/reference/features/intro.md) | 本課採用的定義、default與多feature啟用段落 | 中文對照工具角色；不採「定義webp即肯定啟用」的錯誤說法，當前契約依官方Cargo文件。 |
| [API Guidelines future-proofing](https://rust-lang.github.io/api-guidelines/future-proofing.html) | private fields、newtype封裝、公有bound相關部分 | 私有欄位支持驗證建構／有限method，但不是禁止所有資料struct有公有欄位。未用sealed／未教thread bound，舊impl Trait限制不搬入正文。 |
| [官方Default](https://doc.rust-lang.org/std/default/trait.Default.html) | 完整Default:Sized簽名、default required function、Derivable、manual impl | 預設應符合自己的不變條件。usize預設0與bool預設false，只選本次必要標準型別；手動BatchLimit=100，不加入struct update／enum default新語法。 |
| [Reference non_exhaustive](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute) | 允許位置、same-crate／external-crate、enum建構與match限制 | 本次僅放enum整體，外部須wildcard但已知variant仍可建構；同crate的match仍可窮舉。跨crate負向fixture精確核對，不拿同module假裝外部效果。 |
| [cargo package](https://doc.rust-lang.org/cargo/commands/cargo-package.html) | 本機tarball／解開編譯步驟、list／allow-dirty／feature選擇 | 本機打包與實際registry上傳分開；教材publish=false，只驗證package內容與建置，不執行upload或帳號設定。 |

## 14.4 避免的超前內容

不使用尚未作本節必要先備核對的Send／Sync公開bound，不依賴未完成macro作者的自訂derive。source顯示每項Default／non_exhaustive／pub use契約在第一次使用前；typed Error沿8.6，Path／read_to_string／lines／ASCII／contains沿7章。所有負向source隔離，不讓一般tests紅燈。

## 14.5 動筆前實讀

實際前文：`ch04-publication-target-fixed-v8`的4.3全部8units與counts.rs，包含Eq／Hash／numeric鍵借用、get／get_mut／insert與缺鍵契約；`ch07-output-fixed-v11`7.8全部7units、7.9已讀的契約／純工作／外部驗收。14.4先前已完成，其新增API不當成本節產生器的必要前提。

| 來源 | 確切閱讀部分 | 教學取捨 |
| --- | --- | --- |
| [Comprehensive Rust EN unit tests](https://google.github.io/comprehensive-rust/testing/unit-tests.html)／[中文對應](https://google.github.io/comprehensive-rust/zh-CN/testing/unit-tests.html) | 全文三組first_word材料、test module與外部tests角色；中文部分仍混有英文，據實閱讀 | 既定規則要有不同材料，測試能執行不等於已涵蓋所有工作情況。接回已懂分組計數，不採未選的文字find／byte截取helper。 |
| [EN Other Tests](https://google.github.io/comprehensive-rust/testing/other.html) | 外部caller與doctests全文 | property仍可放integration tests，不替CLI出口與I/O fixture驗收。中文other兩個猜測URL本次無法存取，未算成已讀來源。 |
| [Proptest Getting started](https://proptest-rs.github.io/proptest/proptest/getting-started.html) | 固定日期範例漏掉UTF-8、產生資料找錯、縮小反例、以原輸出建往返性質與保存失敗部分 | 不採「先忽略regex」入口、ASCII rejection當一般文字處理或parser鏡像oracle；用已懂小整數／Vec與明確資料域。 |
| [Strategy basics](https://proptest-rs.github.io/proptest/proptest/tutorial/strategy-basics.html)／[Shrinking basics](https://proptest-rs.github.io/proptest/proptest/tutorial/shrinking-basics.html) | strategy生成與縮小角色；ValueTree current／simplify／complicate例子、縮小仍符合資料域 | 本節選macro使用契約，不直接寫低階Tree runner；以同一失敗判準決定縮短是否合法，不稱所有策略都得到唯一數學最小值。 |
| [proptest!](https://docs.rs/proptest/latest/proptest/macro.proptest.html)、[prop_assert](https://docs.rs/proptest/latest/proptest/macro.prop_assert.html)、[prop_assert_eq](https://docs.rs/proptest/latest/proptest/macro.prop_assert_eq.html) | 1.12.0頁面：in strategy、Config內部語法、回傳TestCaseError而非每步panic、可選格式訊息 | 所有macro使用形式先說明；explicit imports只引選定API，不等14.1 matcher定義。prop_assert_eq會評估並取得輸入expressions，source只交Copy數字，不假稱和assert_eq一樣借所有owned資料。 |
| [Just](https://docs.rs/proptest/latest/proptest/strategy/struct.Just.html)、[Config](https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html)、[RngSeed](https://docs.rs/proptest/latest/proptest/test_runner/enum.RngSeed.html) | 1.12.0的constant策略、with_cases、cases、failure_persistence、rng_seed與Fixed(u64) | 一般測試保留隨機seed與失敗保存；故意錯誤fixture明選seed、關保存以便重播而不寫教材記錄檔。各欄位意義先教。 |
| [v1.12.0 collection原始API](https://raw.githubusercontent.com/proptest-rs/proptest/v1.12.0/proptest/src/collection.rs)、[config source](https://raw.githubusercontent.com/proptest-rs/proptest/v1.12.0/proptest/src/test_runner/config.rs)、[assert macros source](https://raw.githubusercontent.com/proptest-rs/proptest/v1.12.0/proptest/src/sugar.rs) | 實際下載並讀vec／VecStrategy、Range到SizeRange、Config與RngSeed及兩assert macro相關完整區塊 | docs.rs latest vec頁此刻標1.11.0，固定1.12.0URL未可用，故對選版正式tag核對，不把不同頁的latest當同版本證據。原始區塊確認長度range不含上界與assert ownership。 |
| [官方crate v1.12.0 manifest](https://raw.githubusercontent.com/proptest-rs/proptest/v1.12.0/proptest/Cargo.toml) | package版本／最低Rust1.88、features與std／fork／timeout／bit-set關係 | 只選std，關預設features；不為這份有限純函式測試新增行程隔離／timeout能力。library本身不依賴proptest，僅dev-dependencies。 |
| [Failure persistence](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html)／[Limitations](https://proptest-rs.github.io/proptest/proptest/limitations.html) | 全文：儲存失敗seed、先重播、生成域改變影響、有限抽樣漏單一極端值 | 保存seed檔也保存普通固定反例。成功case數不代表域窮舉，不讓bounded策略掩蓋域外輸入；已知邊界仍用傳統tests。 |
| [HashMap values](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.values) | 1.99.0完整values簽名與描述：shared iterator、Item為&V、任意順序 | 新API在total_count前教清楚；只加總次數，不依迭代順序，不搬copied／sort／capacity旁支。 |

## 14.5 性質與材料安排

從4.3的依相等鍵合計工作開始，以小整數取代文字名稱，讓生成器／縮減材料直接可讀。生成域是i32 −3到3、Vec長0到31。第一項檢查positive groups與總次數守恆；第二項每個既定鍵用原輸入直接逐筆計數，與HashMap更新法不同。練習接清單合併的可加性，同一錯誤算法可能通過部分關係，所以不能只選一項弱性質。

隔離錯誤fixture每次insert 1覆寫重複值。使用Just(0)聚焦重複鍵、仍生成不同長度，故意失敗能縮成兩筆0；其property tests不放一般package tests。把反例回收普通regression test，SVG若使用，標明手工示意而非工具原始trace。
