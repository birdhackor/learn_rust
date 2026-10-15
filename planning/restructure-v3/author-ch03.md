# 第 3 章作者交接：ownership 與資料借用

狀態：2026-10-15 第 3 章作者稿與獨立 source 已完成；供 root 核對，不代表獨立首讀或頁面驗收。第 4、5 章沒有開始寫作。

## 授權與實際先備

本輪親讀 `AGENTS.md`、clear-tutorial 的 `SKILL.md`／writing-prompt／rust-terminology，以及已落地 `docs/chapters/01-01-lesson.md`～`01-08-lesson.md`、`02-01-lesson.md`～`02-08-lesson.md`。讀的是實際正文與本章需要的真實 source，沒有把 outline 或研究筆記當成已教前綴。新 2.8 已由 root 交接，沒有新增本章所需語法。

| 本章首次需要 | 實際前綴與本章解釋位置 | 讀完要能判斷的工作 |
| --- | --- | --- |
| 3.1 struct、欄位、pub struct／pub field | 1.8 tuple／array；1.3 mut；2.6 mod／pub／use／target-local helper。3.1 先定義、建立、讀改，再公開型別及欄位；位置式 println 先教後用 | 相同單價數量改用命名欄位，外部 module 可否建立／讀改 |
| 3.2 ownership／move／具體 Copy | 1.7 實際記憶體正文；1.6 函式與回傳。先交責任，再以 Order／i32 對照 | 自訂數值 struct 仍會 move；傳入／回傳何時接手 |
| 3.3 shared reference／dereference | 3.2 owned 交接；1.7 pointer 角色。先教 & expression／& 型別，再教借讀與 * | 只借讀的函式完成後，owner 仍可讀原資料 |
| 3.4 mutable reference／binding mut | 3.3 shared borrow。先教 &mut 的獨占修改契約，再比對 reference_binding | 變更所借數字與替 binding 換一份 reference 是不同操作 |
| 3.5 lifetime／scope／last use／reborrow | 1.6 block；3.2～3.4。區分名稱可見範圍、原資料存在與借用使用區間；只以直線程式解釋 last use | 讀改是否重疊；原資料是否還有效；一般指派 move 與函式位置 reborrow 不混同 |
| 3.6 impl／method／self／Self／associated function | 三種 ownership 契約已在3.2～3.5成立。先逐一教 receiver，再整合完整 impl；同名欄位簡寫先解釋 | 按需要選借讀、借改、接手；new 是自訂函式名 |
| 3.7 &[i32]／&mut [i32] | 1.8 array／usize／索引；2.3 for／range；3.5 借用區間。具體教 for 得到 &i32／&mut i32，不提前教 Iterator API | 起點、長度、相對索引；同一函式處理中段／全段／空段／不同長度 array |
| 3.8 String／str／&str／new／push_str／as_str | 1.4 已略教 char；1.7 編碼／heap；3.6 methods；3.7 slice。逐步追加後才完整模型 | owned 資源、borrowed 文字視圖、move、追加與借讀衝突 |

## Source 邊界

- `examples/intro-workspace/ownership` 的 package 是 `intro-ownership`，沒有 library、dependencies、workspace lints 或隱藏 helper。`src/main.rs` 只使用已教的 println。
- 新成員先以有效 manifest 與 println-only main 建立，再由 root 加入 intro workspace members／更新 lock；作者收到 root 完成通知後才執行 Cargo。作者未改 root Cargo、lock、nav、checks、其他 package 或舊頁。
- 每一節的成功例及完整答案都是自己的完整 `.rs`，同頁以實際 snippets 引用；本章只有 public_model 額外載入自己的 `model/mod.rs`，兩份 source 都在3.1完整列出。
- 3.1～3.7 沒有 String 型別或 API；3.8 的 strings／strings_answer／text_order 完全不載入舊 ch03-data library。沒有 Option、Result、泛型 T、Vec、clone、derive Debug、Iterator API 或自訂 borrowed-output method。
- 2.7 已教的 assertion 只比較 i32；slice 的 usize 長度用 `assert!(... == 3)`。本章沒有加入 suppress lint 的 attribute 或新測試架構。
- Python／JavaScript 選讀位於3.8後，不作主線先備；比較來源是 Python／JS 自己的完整檔與 Rust 獨立 example。沒有以 Rust Vec、Cell／RefCell 來說明它們。
- 沿用既有 move-copy、reference-owner、borrow-scope、slice-part、string-view SVG。3.5 圖只畫 shared borrow→owner 修改，正文相鄰說明完整 source 中還插入一次已完成的 mutable borrow。

## 本輪驗證

命令先載入 `scripts/env.sh`，使用 pinned toolchain；沒有跑整個 intro workspace 的 all-targets，也沒有 git 操作。

| 項目 | 範圍及結果 |
| --- | --- |
| 執行／Clippy | 明確枚舉21個 `--example NAME`，各自 `cargo run ... --locked` 與 `cargo clippy ... --locked -- -D warnings`；另選 `--bin intro-ownership`，全部成功 |
| 預期編譯失敗 | rustc edition2024 單獨驗9份 fixture；E0382／E0594／E0506／E0499／E0597／E0507／E0502符合各自 `.expected` |
| 預期執行失敗 | slice_bounds 可編譯；執行時 index out of bounds，符合 `.expected` |
| 跨語言 | 完整 Python、Node.js 比較程式執行成功，輸出與選讀相符 |
| 格式 | `cargo fmt --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --check` 成功 |
| Snippets／輸出 | 8頁主文＋1頁選讀用真實 pymdownx.snippets、check_paths 展開成功；15組 text 輸出逐行匹配實際 run logs |
| 尚未驗證 | 未跑全站建置、未看實際網站桌面／手機版面；沒有聲稱獨立首次閱讀審查通過 |

選讀最初的單字元 push_str 觸發 `single_char_add_str`；改成多字內容追加，保持本節只教 push_str，不提前加入 push(char) 或 allow。原始失敗與修後 selected-target 紀錄分開保存。

詳細命令、逐 target stdout／stderr、初次失敗、修正後結果與 source SHA-256 位於 `/workspace/work/tutorial-implementation-20261015/ch03-authoring/` 的 `validation-initial.json`、`validation.json`、`checks/`；實際前綴指紋是 `actual-prefix-sha256.json`。外部來源讀取及取捨見同目錄的 `sources-ch03.md`。本節用語依 clear-tutorial；本輪驗證是作者自查。


## root 凍結前銜接複查

root 親讀3.5～3.8後指出兩項說明時序：3.7 的具體 for 借讀契約與3.6 的 Self／欄位同名簡寫。已將既有說明前移到首次 snippet 之前；3.3 欄位存取自動跟隨 reference 的既有說明也前移，並將 reference 的 strong 邊界只包英文。未改 source，不重跑 Rust；修後再做9頁 snippets／標記展開與15組輸出核對。


## root 手機實頁複查後的圖說修正（2026-10-15T06:53:59.648497+00:00）

原freeze與原作者判定保留不改。root親看新3.3手機實頁後指出：reference-owner圖底的`*reference`在正式dereference介紹之前已進入必要圖。依此新提示只把底字改為「借用同一筆 Order」，desc及本頁alt同步只描述借用；其餘圖形、正式*教學及全部Rust source均未改。修改前freeze／圖／頁與前後hash另存`ch03-authoring/revisions/round01-reference-caption/`。只做XML與9頁snippet重新展開；修後實頁待root確認，未重跑Rust。root另已親看其餘四張沿用圖可讀，本作者沒有把它改寫成自己的獨立視覺驗收。


## root 3.2 圖文複查後的Copy介紹順序（2026-10-15T06:58:10.045566+00:00）

root完整回看3.2及move-copy手機實頁後指出：圖下半先用Copy解釋兩份數值，原正文卻在稍後snippet之後才教基本契約。只將既有i32 Copy的通用基本解釋移到首次圖之前，後段保留another_count由3加成4、原count不變的實際解讀，移除重複句；沒有增加規則或提前推演細節。程式與所有圖均未變，未重跑Rust。先保存上一輪freeze及本頁／ledger修改前全文，再追加本輪前後hash與freeze-after；原初freeze及各輪原判保留。修後只重新展開9頁snippets，實頁由root後續核對。


## independent continuity C3-01 採納後的reborrow操作順序（2026-10-15T07:04:46.371871+00:00）

independent continuity的原始初判已由root封存；root採納C3-01必要窄時序：calls片段在再次使用parent前，需先知道第二次呼叫會自動reborrow。依提示只將既有完整操作說明移到calls snippet之前，保留數量5→6→7及前段3→4→5、不新增機制。只改3.5與此ledger，所有Rust與圖未變，未重跑Rust。原章3 freeze／validation及前輪原判保留，修前全文／hash與新hash另存`ch03-authoring/revisions/round03-reborrow-before-calls/`。修後版面及reader固定版本由root處理。
