# 14.6 Loom／Miri 作者實讀來源

作者chapter04_traits，本檔是14.6新scope，保留14.1–3所有原始BODY/source及其筆記。未讀任何peer14審查或首讀答案。

## 實際前文

- ch12-prefix-three-prerequisite-v1：全部21 BODY units，建立GAT／variance／raw pointer、storage有效性／provenance／unsafe block／UB的角色。current ch12 archive的17份prefix Rust／Cargo來源與舊版相同；額外gat-borrows、cell-variance、raw-array三SVG source已讀，未宣稱本輪實頁視覺。
- ch12-contracts-prerequisite-v2：全部6 BODY units及39完整archived sources（library全部helper/tests、全部13examples/完整答案、fixtures/expected/Cargo/safe-split SVG）。manifest SHA 20c9c7881080e12b3940e3d31b4051d3d2ff7ef11aeb52f5008536bca8c0de3c；39 source bytes逐hash與live一致。split_i32_at_mut及其unsafe前提已完整教，不能拿safe caller正常輸出代替soundness。使用該公開helper，不另複製實作。
- ch10-complete-root-name-fixed-v14：10.8全部9units、10.1全部8units，必要10.2的002-002/003（spawn／join/payload）及10.6的006-002/003（Arc與存取權限）。其中10.8根據單一counter的修改順序與全部join解釋Relaxed最終值；分開load/store的lost update不是data race。manifest SHA f025148311374864df6bbcfe065a48a1d4922261c2f20c9528aa87ff0894bebc。未將全章其餘正文假裝新讀。
- ch09-complete-prerequisite-v5：9.1全部6units及engineering相關7份source；features/profile/cfg與integration選target來源已核。manifest SHA c90b88a838d69a6ae1107656d49207bc98108c8a994b12ad63ba862a59f6b161。
- ch07-review-fixed-v13：7.1全部6units、7.2全部5units，tooling4份source/Cargo；dev-dependency、精確版、lock、tests目標及--test已教。manifest SHA 87fec0581beb0f2398a81061060c0e47f5d83dfe6cf54b193e9461b38169cfef。
- 額外回讀ch08-complete-prerequisite-v8的Box／Drop必要單位：Box保有heap allocation，raw pointer不保活；std::mem::drop接手非Copy owner，返回前已清理，之後不能依舊pointer讀取。章12的&raw const與dereference規則才是fixture的語法來源。

## fresh 英文、中文教材

資料原文/hash/HTTP結果在work/research-ch14-tools-author04，不以取得檔案等於已讀。

- [Loom 0.7.2官方README](https://github.com/tokio-rs/loom/tree/v0.7.2)：全文實讀。借鑑兩份atomic增加工作、分開read/store的反例與替代同步型別；不搬#[should_panic]吞掉失敗、不引入collect/自訂cfg/RUSTFLAGS切換，改成固定Loom integration target及隔離預期失敗package。其Unsupported features明說不是完整C11模型；不採泛稱所有硬體／所有輸入都證明。
- [Loom 0.7.2 crate guide](https://docs.rs/loom/0.7.2/loom/)全文243行已讀（小批補讀被工具輸出截斷的部分），涵蓋少見交錯、重複探索、instrumented types、determinism、model限制、debug及Relaxed不完整。模型只選2workers、單一counter、無等待迴圈，避大狀態空間／fairness假設；code不混std thread/atomic與Loom追蹤。
- [Learning Rust With Entirely Too Many Linked Lists：Miri](https://rust-unofficial.github.io/too-many-lists/fifth-miri.html)全文162行；[Rust語言聖經中譯同節原始檔](https://github.com/sunface/rust-course/blob/main/src/too-many-lists/unsafe-queue/miri.md)全文實讀。網站course.rs回403，直接從其公開官方repo取同教材原文（不是代理拒絕繞路）。教法借鑑普通tests已過仍要查unsafe動態執行；不沿用2022 nightly、xargo流程、複雜list Option::take／borrow stack，更不把Miri說成「生成MIR就能在編譯期證明安全」。選明確的storage已釋放而仍讀取，無須先教完整aliasing模型。
- [中譯production deque testing](https://github.com/sunface/rust-course/blob/main/src/too-many-lists/production-unsafe-deque/testing.md)實讀246–317的normal tests與Miri對照，不算前245行所有測試已讀；舊-Zmiri-tag-raw-pointers不沿用。
- [Rust語言聖經：Atomic原子型別與內存順序原始檔](https://github.com/sunface/rust-course/blob/main/src/advance/concurrency-with-threads/sync2.md)實讀1–95（計數、join、interior mutability與ordering入口），未把後半算已讀。保留同一count與工作完成需求；不採「atomic暫停其他CPU」「一定更快」「100%並行安全」等過強推論。此中文材料沒有Loom API，本節模型探索的技術契約以Loom官方固定版核，不假裝讀到中文Loom章。

## selected 固定工具／API

- Loom0.7.2：model完整9行／spawn完整11行；AtomicUsize的new/load/store/fetch_add selected介面與mock表示；Arc的new、Clone及Deref/Drop；JoinHandle::join selected方法。另讀Builder的限制與check介面（156行）作作者查核，但學生只用model的預設有限探索，不暗塞builder欄位。thread source selectedspawn_internal及join已讀，知道它不是普通OS測試、panic本身會令模型失敗；不概括成完全相同的std panic捕捉協定。
- features頁實讀：預設功能為空，本文只使用sync/thread/model，無checkpoint/futures新增features。原文固定docs.rs0.7.2，URL/raw SHA見fetch.json。
- [Miri official README](https://github.com/rust-lang/miri)實讀1–155的身分、UB型別、soundness限制、單條執行與CLI；246–251 explicit setup、287–290 setup/driver區分、533–553 tool環境契約。重新取已安裝Rust21b707e3f內src/tools/miri/README.md，HTTP200/hash2a49a1cfdceaaa53d3ee7a51249796f95fe1dc5dddb755962ed0845814d2376c，與此次master取得原文相同；不拿其他nightly的文字暗代目前tool。
- 實際安裝nightly-2026-10-01（rustc1.101.0-nightly，21b707e3f，2026-09-30）、Miri0.1.0同commit及rust-src。main rust-toolchain仍1.99，安裝器自行設globaldefault後已rustup default none恢復此前狀態；所有student Miri命令明選+nightly日期。setup成功後另以MIRI_SYSROOT把本作者驗證sysroot存work，未改共享env／CI。實際log在work/author-ch14/14-6；install/setup不等於測試PASS。

Miri的unsupported operation與UB報告須分開，只有後者可支持本例storage違約。Miri不分析external C實作，也不保證FFI的釋放／ownership契約。正常case只選已教pureRust split helper與當節test；預期UB只在獨立miri-fail binary、僅以cargo +固定nightly miri run解譯，不host執行、不加入root alltargets。
