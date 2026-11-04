# 第 1 章 v3 作者來源實讀紀錄

- 日期：2026-10-15；這是作者的取材與自查，不是獨立首讀結論。
- 實作前讀過 `AGENTS.md`、`.agents/skills/clear-tutorial/SKILL.md`、`references/writing-prompt.md`、`references/rust-terminology.md`、核准的 `planning/restructure-v3/plan.md` 與本章 `sections.json`。
- 作者本輪負責 1.1–1.6、1.8 與入門 workspace。1.7、圖及 `memory.rs`、`memory_answer.rs` 已交給另一作者；本紀錄不宣稱驗收其完成稿。
- 下列行號均指保存的抽取文字行，不是網站行號。`sources/` 的實際位置是 `/workspace/work/ch01-v3/sources/`，先下載原 HTML，再擷取正文並實際閱讀；沒有把書目或舊作者筆記當成本輪已讀正文。
- 網路操作前讀 cloud runtime/networking skill 與 `/etc/codex/network-policy.json`；environment_status 顯示當前 policy enforced/unrestricted，使用繼承的 proxy，未讀取憑證內容。

| 本課 | 實讀來源與抽取行數 | 取材與沒有沿用的部分 |
| --- | --- | --- |
| 1.1 | [Book EN Installation](https://doc.rust-lang.org/book/ch01-01-installation.html) `en-ch01-01-installation.txt:1–144`；[繁中安裝](https://rust-lang.tw/book-tw/ch01-01-installation.html) `tw-ch01-01-installation.txt:1–83` | 先分 rustup/rustc/Cargo 角色，再做版本檢查；linker 有角色說明。不採 source 的先下載後章依賴指令，因為尚未教依賴。安裝指令按官方 EN TLS 1.2；不沿用繁中頁較舊工具敘述。 |
| 1.2 | [Book EN Hello World](https://doc.rust-lang.org/book/ch01-02-hello-world.html) `en-ch01-02-hello-world.txt:1–228`；[繁中 Hello World](https://rust-lang.tw/book-tw/ch01-02-hello-world.html) `tw-ch01-02-hello-world.txt:1–169` | `main` 外框、literal、macro `!`、分號分別解釋；使用 Cargo 統一操作，不先多教 OS 不同的直接執行路徑。 |
| 1.2–1.3 | [Book EN Hello Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) `en-ch01-03-hello-cargo.txt:1–243`；[繁中 Hello Cargo](https://rust-lang.tw/book-tw/ch01-03-hello-cargo.html) `tw-ch01-03-hello-cargo.txt:1–183` | 沿用 check/build/run 可觀察差別，最小 manifest 明寫 name/version/edition。繁中仍示範 edition 2021，本課以固定工具鏈及 edition 2024 實測；不搬 release/profile/gitclone。 |
| 1.3 | [Book EN Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) `en-ch07-01-packages-and-crates.txt:1–114`；[繁中套件與 Crates](https://rust-lang.tw/book-tw/ch07-01-packages-and-crates.html) `tw-ch07-01-packages-and-crates.txt:1–73` | 區分 package 設定管理單位與 crate 編譯單位。library 僅解釋角色，未提供未教 lib.rs；範例各自有 main，不自動引入後章 source。 |
| 1.3 技術補核 | [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) `cargo-workspaces.txt:1–180`；[Resolver versions](https://doc.rust-lang.org/cargo/reference/resolver.html) `cargo-resolver.txt:606–642` | 核對共享 lock/target、members 相對路徑、virtual workspace 明寫 resolver；本課只教最小必要角色，不提前教 feature unification/inheritance。補核在初稿之後完成，教材實際 manifest 的 Cargo 檢查亦已通過。 |
| 1.4–1.5 | [Book EN Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) `en-ch03-01-variables-and-mutability.txt:1–292`；[繁中變數](https://rust-lang.tw/book-tw/ch03-01-variables-and-mutability.html) `tw-ch03-01-variables-and-mutability.txt:1–210` | 採 immutable 拒絕、mut 更新、shadowing 新 binding 的可比較狀態。改用整數→f64，不提前借用字串 len、reference 或 parse。常數另屬後續章，不在本節塞完整清單。 |
| 1.4、1.8 | [Book EN Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) `en-ch03-02-data-types.txt:1–556`；[繁中資料型別](https://rust-lang.tw/book-tw/ch03-02-data-types.html) `tw-ch03-02-data-types.txt:1–432` | 保留型別推導、整數除法、scalar、tuple、array。移除 parse/expect/stdin/String/借用等先備；索引失敗改為已教數值函式傳入3，不借用黑箱 helper。不把 array 型別說成一定在 stack，不把 debug/release 溢位行為當本章可依賴契約。 |
| 1.6 | [Book EN Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) `en-ch03-03-how-functions-work.txt:1–408`；[繁中函式](https://rust-lang.tw/book-tw/ch03-03-how-functions-work.html) `tw-ch03-03-how-functions-work.txt:1–346` | 參數/回傳/expression/block/scope 用同一 subtotal 連貫呈現；unit 清楚是型別與值。保留尾端多分號錯誤，不沿用會引出條件語法訊息的 let-inside-expression 失敗例。 |

## 計算機基礎前置閱讀與交接

另實讀 `/workspace/work/tutorial-reassessment-20261015/research-foundations/report.md`，以及同目錄 evidence 中：

- [Brown 4.1](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)：`brown-en-ownership.txt:60–109`。借用同一數值函式呼叫前／中／後的觀察點、frame 與實作限定；不搬 assembly、Box、假想 free、未教 String。此作者只讀 cached 正文，沒有操作 Brown 互動圖。
- [Book EN 4.1](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)：`book-en-ownership.txt:1–78`；[Book 繁中 4.1](https://rust-lang.tw/book-tw/ch04-01-what-is-ownership.html)：`book-tw-ownership.txt:1–125`。核對 stack/heap、配置器與清理的動機，但不沿用 locals 一律 stack、固定大小不能 heap、由 placement 推導 ownership 的泛化。
- `memory.rs` 初始版本採 `subtotal(120, 3)` 與 `80, 2`，A/B/C 註解對應同一筆計算；實際 1.7 作者接手後負責圖文與對應 source。初稿中的 let-and-return Clippy 警告已回報，不宣稱該稿通過完整 workspace Clippy。

## 本輪取得的原始證據雜湊

以下是取材檔案 hash；不與最終教材凍結 hash 混用。

```text
207754a706267c23298b8c7efedd154533c1aa7f8182a80393f4007927e8186c  /workspace/work/ch01-v3/sources/cargo-resolver.txt
900a415ee0680c10e4a048c01441c1648b9f27bc9bed2e5fec2fb23831eb17f8  /workspace/work/ch01-v3/sources/cargo-workspaces.txt
e03d2adad74e05ff89d7cf1f2c9b5f2eaf058117daf7964429096f38a1f80c67  /workspace/work/ch01-v3/sources/en-ch01-01-installation.txt
49ad27e4411c70f7fa3efc10ed48a15a9ae28653bbadf598f2b06a1afdb69983  /workspace/work/ch01-v3/sources/en-ch01-02-hello-world.txt
7559f4d3667d425071952b1b72ecf9932f85b4c344860e82756232962c87cf44  /workspace/work/ch01-v3/sources/en-ch01-03-hello-cargo.txt
b926556c4ebd70305c7bdbd3212f30fe58225ddf03e9c67205b50c37764ff238  /workspace/work/ch01-v3/sources/en-ch03-01-variables-and-mutability.txt
eca970ff1b330d9a87bf6c5efe7234455e20e187e6719dcbca57f777ede01e56  /workspace/work/ch01-v3/sources/en-ch03-02-data-types.txt
ed0d55619ea9c2fa134682b1f04d23e4f8fd79b84ca0ecdf429e943ea6f29d03  /workspace/work/ch01-v3/sources/en-ch03-03-how-functions-work.txt
8431cad60ba21a97f9f89df41caf1bc5c934b3d7b7dd5d569cdab08ddf53e0e6  /workspace/work/ch01-v3/sources/en-ch07-01-packages-and-crates.txt
bbedd7acd575af40581b4955a4479530a42588be771df52c4b1788aefed6f0e2  /workspace/work/ch01-v3/sources/tw-ch01-01-installation.txt
4855c37592aa2648d8c7279cd4801867f2ba4d141be580a64a902818928ba725  /workspace/work/ch01-v3/sources/tw-ch01-02-hello-world.txt
f587247876fb5012a1d9b7f59b0180f9d578298ec4b36289dfa3a51963537b54  /workspace/work/ch01-v3/sources/tw-ch01-03-hello-cargo.txt
90c511ca8fd83b11f2c1473181301f5465855b067a90c4c825c187a9d16f7e46  /workspace/work/ch01-v3/sources/tw-ch03-01-variables-and-mutability.txt
35888721159c29a73fb977beeedd284765a47d63bd871f6c425f77b367b23b63  /workspace/work/ch01-v3/sources/tw-ch03-02-data-types.txt
d2abb7f44e15904878c1abfea2116a1fa78214e0a169a3665483586af1852a7a  /workspace/work/ch01-v3/sources/tw-ch03-03-how-functions-work.txt
e4559c90c3ee0f2474083a5faef385804eebd0dc738fdf1b1ad3116627021803  /workspace/work/ch01-v3/sources/tw-ch07-01-packages-and-crates.txt
58a1555bd1375e088a0a78ba761a8161130a86cdf688409869539ce50ef406a6  /workspace/work/tutorial-reassessment-20261015/research-foundations/evidence/brown-en-ownership.txt
05ffe689884418ca46392e2a6f981651ff14056a7319264f3fc76e7366677235  /workspace/work/tutorial-reassessment-20261015/research-foundations/evidence/book-en-ownership.txt
2cd1a511dae65ff912c64d7db22ced33d21a29fc770e0dd09da4ff172c6d0de7  /workspace/work/tutorial-reassessment-20261015/research-foundations/evidence/book-tw-ownership.txt
```
