# 第12章外部來源與實讀證據

作者本輪親讀21份實際body，逐份SHA256、inclusive ranges與取捨在/workspace/work/tutorial-implementation-20261015/ch12-author/research/source-read-ledger.json；HTML／原始Markdown及失敗fetch也保存在相鄰目錄。下載、空索引與其他作者的摘要不作實讀。中文名稱repo內有些頁實為英文，已按實際語言標記；真正中文目錄／hash章另讀zzy/rust-cookbook-zh-cn原文。

| 來源與實讀範圍 | 採用與限制 |
| --- | --- |
| [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)全文201行；[繁體Book](https://rust-lang.tw/book-tw/ch14-03-cargo-workspaces.html)全文153行 | 成員共同管理與dependency不同；共同欄位由成員明選，profile只認根manifest。用完整小workspace實作，不把共享lock曲解成所有相容版本必合一。 |
| [EN Profiles](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)全文45行；[繁體](https://rust-lang.tw/book-tw/ch14-01-release-profiles.html)全文22行 | dev／release與opt-level角色。只驗配置與數字，不把最佳化設定當速度測量。 |
| [Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html)全文338行 | package features、default／命令列、dependency default-features與加成能力。先教實際小開關，不引入optional／no_std等旁支。 |
| [Cargo Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)全文394行；[RustCourse中文原章](https://raw.githubusercontent.com/sunface/rust-course/main/src/cargo/reference/build-script/intro.md)全文140行 | host／target、stdout逐行協定、rerun、OUT_DIR及分開的build-dependencies。中文舊cargo:形式用固定工具鏈可用的cargo::核對；C／ABI實作留後章。 |
| [EN Cookbook Directory](https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html)全文220行；[中文重名](https://raw.githubusercontent.com/zzy/rust-cookbook-zh-cn/master/src/file/dir/duplicate-name.md)33行、[條件查找](https://raw.githubusercontent.com/zzy/rust-cookbook-zh-cn/master/src/file/dir/find-file.md)43行、[hidden](https://raw.githubusercontent.com/zzy/rust-cookbook-zh-cn/master/src/file/dir/skip-dot.md)38行、[sizes](https://raw.githubusercontent.com/zzy/rust-cookbook-zh-cn/master/src/file/dir/sizes.md)31行，均全文 | walker管理進度、資料觀察與收集。教材保留逐項錯誤、hidden與原生身分，不沿用filter_map丟錯、!is_dir當一般檔案、lossy文字當key或跟隨link的不同政策。 |
| [EN Hashing](https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html)全文95行；[中文SHA摘要](https://raw.githubusercontent.com/zzy/rust-cookbook-zh-cn/master/src/cryptography/hashing/sha-digest.md)全文59行 | 分批送有效buffer前段；不把不同演算法/API照搬成BLAKE3，不以摘要等同內容確認，也不帶入HMAC／PBKDF2。 |
| [繁體Book讀檔](https://rust-lang.tw/book-tw/ch12-02-reading-a-file.html)全文48行 | 真實材料先行、讀出後觀察。第12章保留bytes／原生路徑與具體失敗，不回退以args索引和所有錯誤expect處理一般檔案。 |

舊稿與actual loaded source／manifest／test／fixture的hash/range見ch12-author/local-read-ledger.json；新前文實讀見prefix-read-ledger.json。研究取捨不能代替新章已教與程式驗證；正式freeze、成功／失敗原始證據另列author-ch12.md。

12.2正式寫作另核對[env!](https://doc.rust-lang.org/std/macro.env.html)全文26行、[option_env!](https://doc.rust-lang.org/std/macro.option_env.html)全文23行、[VarError](https://doc.rust-lang.org/std/env/enum.VarError.html)1–86及[Cargo Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)1–96／150–190；range與HTML/body hash另存research/api-read-ledger.json。採用的是編譯時文字／缺值／非Unicode契約、VarError資料形狀及Debug、根profile與runtime overflow行為；沒有把nightly與長頁未讀部分帶入正文。12.2的host／target與模式錯誤另有本次真實執行證據，來源閱讀不能代替它。

12.3／12.4的API核對另存research/files-api-read-ledger.json，包含[WalkDir 2.5](https://docs.rs/walkdir/2.5.0/walkdir/struct.WalkDir.html)1–228、[DirEntry](https://docs.rs/walkdir/2.5.0/walkdir/struct.DirEntry.html)1–105、[walkdir Error](https://docs.rs/walkdir/2.5.0/walkdir/struct.Error.html)1–120、[Metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html)1–175、[symlink_metadata](https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html)全文26行、[Path](https://doc.rust-lang.org/std/path/struct.Path.html)52–94／670–710／750–788及[BLAKE3 Hasher](https://docs.rs/blake3/1.8.7/blake3/struct.Hasher.html)1–148。選用兩個link開關／Item與owned path、metadata的當下觀察、borrowed key身分及依序update；不引入API頁其餘parallel／mmap／nightly旁支，也不搬性能概括。正式source、輸入、受控錯誤與程式結果另依作者驗證證據，API閱讀不能代替執行。
