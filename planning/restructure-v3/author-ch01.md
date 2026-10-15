# 第 1 章 v3 作者交接 ledger

狀態：1.1–1.6 與 1.8 完成作者自查，可交獨立首讀與技術審閱；尚未宣稱全章驗收。1.7、SVG、memory 兩檔由另一作者接手。作者沒有修改根 Cargo/nav/check.sh/共用設定、舊教材或其他章 source。

## 真正的首次需要與實際已教位置

行號指目前 Markdown 原始檔。inline 說明與 snippet 的先後一併核對，名稱清單不作已教證據。

| 概念／操作 | 首次需要與解釋位置 | 先備與可預測結果 |
| --- | --- | --- |
| rustup/rustc/Cargo、編譯與執行 | 1.1「三個工具各做什麼」「版本檢查」 | 基本 Python；能分辨管理工具版本、讀原始碼與安排建置。 |
| package、cargo new/cd、manifest、最小 name/version/edition | 1.2 開頭與「完整的最小設定」 | 1.1 工具角色；自建專案可 check/build/run，沒有外部依賴。 |
| main/fn/()/{}/println!/literal/分號 | 1.2「先讀懂三行程式」，在完整 hello source 前 | 只用固定文字；兩行順序可預測；macro 僅教當前可用形式。 |
| package/crate/workspace/members/resolver | 1.3「一個設定檔與一個編譯單位」「用 workspace 一起管理」 | 已能跑單 package；完整 workspace/成員 manifests 皆 real snippets。 |
| --manifest-path/-p/--example/--locked/Cargo.lock | 1.3 每個選擇動作前的相鄰說明 | `-p` 用 package name，不是資料夾名；兩個同名 two_lines 有不同輸出。 |
| let/binding/型別/i32/推導/*/格式佔位 | 1.4「名稱讓材料與結果對得起來」 | hello 與 Cargo；120*3 得360，格式佔位在短片段前介紹。 |
| /、%、浮點數、優先序、u8/u32/bool/char | 1.4 逐段短片段與解釋 | 7/2=3、7%2=1、7.0/2.0=3.5；不是 Python / 契約。 |
| immutable/mutable/重新指派/+=/-= | 1.5「預設不能重新指派」「用 mut」與練習前 | 1.4 let/型別；immutable E0384、不能變型別 E0308。 |
| shadowing | 1.5「用 let 表達新的一個 binding」 | 1.4 let/型別；新的 let 可選新型別，未聲稱修改舊 binding。 |
| 函式參數與回傳、expression/statement/block、unit | 1.6「先定義要收什麼」「呼叫把資料交給參數」 | 先定義 subtotal/show_total，再展示 main 的呼叫；120/3→360、80/2→160。 |
| scope、內層 shadowing | 1.6「內部 block 也能交出結果」 | 1.5 shadowing；內部名稱不能外用，外層count仍3；不把block當新函式frame。 |
| tuple/.0/解構 | 1.8「tuple：每個位置各有型別」 | 1.4 三種 scalar；建立與解構前交代位置約定。 |
| array/[T;N]/索引/len/usize/[0;3]/mut 元素 | 1.8「array」「固定項數」 | 小片段先教建立與固定索引，再教方法呼叫/usize後展示 selection。 |
| panic 最小意義 | 1.8「索引超出範圍會怎樣」 | 1.6 完整數值函式；selected_price 函式在此定義，傳3預期執行失敗。 |

## Source 載入邊界與階段快照

- `hello`、`greeting`、`steps` 都明寫 name/version/edition，沒有 inheritance、feature、lints、dependencies、build.rs 或 lib.rs。
- 根 manifest 只有 workspace/members/resolver；root 整合者可以擴充 members，1.3 不以成員數作教學契約。之後成員路徑只作預告，不代表其 source 已教。
- 1.2 的完整程式／manifest 是 hello 的同一份 real source；已複製到 `/workspace/work/ch01-v3/standalone`，在所有 workspace 外 check/build/run 成功，確認不是依賴上層繼承才可執行。
- `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-hello` 只需要 hello/src/main.rs；加 `--example two_lines` 則選 hello/examples/two_lines.rs。greeting 同理。
- steps 的每個 example 都是獨立 main 的完整檔，沒有 mod/use/export/helper 間接載入。選 types 不會編譯 functions/tuple_array/memory；Cargo 會讀取 workspace 成員 metadata，但不會為這次選取編譯未選 source。
- docs 的短片段與完整程式都用 pymdownx.snippets 引用唯一真實檔。片段標記位於 Rust 註解，不新增語言能力。
- compile-fail 與 runtime-fail 放 fixtures，不是普通成功 examples，也不在 lib/test 中自動執行。正文完整顯示；runtime-fail 可放回1.2的自建 src/main.rs 重現。
- 1.1 無 Rust 程式需要：完整練習答案是已教的版本檢查命令；1.2–1.6、1.8 都有同頁完整 source、練習與完整答案。

## 已做驗證與正常／失敗代表

每個新 shell 先 source scripts/env.sh，並設 CARGO_INCREMENTAL=0、CARGO_PROFILE_DEV_DEBUG=0、CARGO_PROFILE_TEST_DEBUG=0、CARGO_BUILD_JOBS=2。沒有跑根 workspace 或根 check.sh。

- nested `cargo generate-lockfile`、`cargo check -p intro-steps --examples --locked` 通過初稿。
- `cargo fmt --manifest-path examples/intro-workspace/Cargo.toml --all --check` 通過；這裡 --all 只對入門 nested workspace，不是根 workspace。
- `cargo clippy --manifest-path examples/intro-workspace/Cargo.toml -p intro-hello -p intro-greeting --all-targets --locked -- -D warnings` 通過。
- steps Clippy 明選 types/types_answer/mutability/mutability_answer/functions/blocks/functions_answer/tuple_array/tuple_array_answer，全部通過。沒有把 memory 接手稿算通過。
- `/workspace/work/ch01-v3/verify.py` 實跑22個命令，13項正常輸出逐字比對成功；4項 compile-fail 各檢查 E0384/E0308/E0308/E0425；array_index 編譯成功、執行非零且出現指定邊界診斷、stdout空；standalone check/build/run 成功。
- 使用專案 uv 鎖定環境，對1.1–1.6與1.8實際執行 Markdown + pymdownx.snippets（check_paths=True）轉換，全數解析成功；短片段與完整原始碼標記均已展開。HTML留於 `/workspace/work/ch01-v3/01-NN.html`；這不是整站或實際頁面視覺驗證。
- 詳細 stdout/stderr/命令/returncode 位於 `/workspace/work/ch01-v3/verification.json`。正常代表：functions→360/160；blocks→380/4/3；型別練習→180/4/1；索引2→200。

## 尚未教與仍待整合的邊界

沒有 if/for/range/reference/String/Option/Result/assert/derive、自訂 module/use/lib、trait/泛型等可執行語法。1.8的 method 呼叫只教目前 array.len()，沒有提前要求理解 receiver/reference 或自訂impl。1.7建立計算機模型但未由本作者驗收；全章8頁尚須由root整合並獨立審閱。尚未做嚴格整站建置、桌面／手機原位視覺檢查與外部部署，不能以局部cargo通過代替它們。

## 本作者凍結檔案與 SHA-256

這是交接時的快照；不把後續 root 擴充 workspace 或1.7作者修改算成這個版本已驗證。獨立首讀可從1.1順序讀，不需要先取得本ledger或sources筆記。

```text
f67a72cfbbbd2557c856aa9efc5284a42b8eb8feb899ec2994949c12d03c6275  docs/chapters/01-01-lesson.md
d9e129971cb9ec8234de1046a936c6f2be99b09396059c2c1a2ff81a38651c22  docs/chapters/01-02-lesson.md
03ae122568dbb6c36812815148adb19bd337481c37421f3c82f5163a97bdc5c1  docs/chapters/01-03-lesson.md
cd61ce46e34365921fe5bca0f86918ab567f29638fe57a9db203c3ab653f04cc  docs/chapters/01-04-lesson.md
88627f3d2734717abff22fc4929ec6c8a24cfe33c7975a7d5b4a1cd60ec671c1  docs/chapters/01-05-lesson.md
53f814748fb6aa665fd39796da94a51becda827dbbfd083b66c8d40492babb1b  docs/chapters/01-06-lesson.md
4d1f23cea4daf930e04075fe08f544226142f8e774b20c7405d4003afa7c03cd  docs/chapters/01-08-lesson.md
8ac8f9953ef778c7aeaac60a2028ca05d8b2ce63ad9f1a516bab2899df96c6f6  examples/intro-workspace/Cargo.lock
46da4b3b01c5ecf499262e9b4effd21501fc18d879037a1511e6d1ff3c8753af  examples/intro-workspace/Cargo.toml
83300e56501d71d423e9a1aebbfd2cd1c6c4599238ed4884fcad8e93847615c5  examples/intro-workspace/greeting/Cargo.toml
164651a1bd3bdc96ce4bd04e44910830a1b009a7b9f61f91812169a2d1291cb8  examples/intro-workspace/greeting/examples/two_lines.rs
9a39e5130d1b88c38da1405f8b8822858bf6aae678883377761ee1bd7ccc446b  examples/intro-workspace/greeting/src/main.rs
946345ebb5bcb3465019d4d501ec888da32b198c9a87d4b2d91edb2fec085e9d  examples/intro-workspace/hello/Cargo.toml
d5bdb012bc0065481377bb52583546bd0db2bc780f29be1e8f8287e8dc99797d  examples/intro-workspace/hello/examples/two_lines.rs
0417da49d6287cfc46a7fbd206ce9b7d5a57496222247ed67e43e11f8e720deb  examples/intro-workspace/hello/src/main.rs
ba433fca9aa7ac14673d3b3727bf391f285a6013ec75ffa2a6ba60942e245dfb  examples/intro-workspace/steps/Cargo.toml
553033a4b67ceebba84ed19b98fe39300a9c86d147c09ce20303ca55c438bb6e  examples/intro-workspace/steps/examples/blocks.rs
8f0caa2a677c7c3725b22537796e8b0f99060bff2ac3a309b58509e19ad20f47  examples/intro-workspace/steps/examples/functions.rs
c1c964d47748ac981f02569af3f29db82bb4d4b71b2f5b1ea5b3baf69b911be1  examples/intro-workspace/steps/examples/functions_answer.rs
c48d64a718af19cc58192b8194f5e92b827d2a65226bc4d843fb49f2cab536ac  examples/intro-workspace/steps/examples/mutability.rs
fad62a34e971695ac415b7fba96d6f19d4eff729e433b776ffdca286d473f1fc  examples/intro-workspace/steps/examples/mutability_answer.rs
394066df88ff1a0a49da45db0bfb34fb27528eef4a19f17b120b8bfaac61d96c  examples/intro-workspace/steps/examples/tuple_array.rs
254acbc9c4d6b8c1e6d324e75b127b3a8e3045baf6258f875f2c3b77bc1e3a6d  examples/intro-workspace/steps/examples/tuple_array_answer.rs
de551dd2760674626aff0595c332cf23093e0dd05293ae94bec6568477906526  examples/intro-workspace/steps/examples/types.rs
1f2e106fa8d9d896a552f978af6ae8d0ce7a6a0f8620573c7eb1e79c2c5fb891  examples/intro-workspace/steps/examples/types_answer.rs
3480d3cbc086adf259cbd928d4db42f5e14706c9f58dfc02e034ad7019fbfdf9  examples/intro-workspace/steps/fixtures/compile-fail/immutable.rs
048b719e16026c84a919edda1a7a1ebca77daf02ee229cfd174555e39888fb05  examples/intro-workspace/steps/fixtures/compile-fail/return_semicolon.rs
c24c09f780beed9232e15db57bfa50fda770a523422f7d6afc1fe541dcb03d81  examples/intro-workspace/steps/fixtures/compile-fail/scope.rs
a702ccfb7e36043df1520bef92a4b3b385e23cce8076fdeefedeb9860e4b8811  examples/intro-workspace/steps/fixtures/compile-fail/type_change.rs
78efa165166ae79f032c42a45c459f54499d7735103aaed02d20bb8d52cb0b62  examples/intro-workspace/steps/fixtures/runtime-fail/array_index.rs
9d77c241170f05328e057efed81ad48b15cb9ce56ca87821ccf28a1c05365fc0  examples/intro-workspace/steps/src/main.rs
```


## 2026-10-15 補記：五份失敗案例納入維護 harness

原凍結稿、原作者自查與其通過範圍均保留。root 掃描 `scripts/check-fixtures.py` 發現本作者新增的五份失敗案例缺少同名非空 `.expected`；原先22條局部命令沒有執行全書 harness，故沒有發現這項整合缺口。

本輪先讀實際 harness 的判斷方式，再從 `/workspace/work/ch01-v3/verification.json` 逐一核對當時實際執行的 stderr 與非零 returncode；runtime 案例也核對當時先編譯成功。五個 Rust 原始檔的 SHA-256 都仍符合上面的原凍結 hash，所以原 stderr 對應同一份 source。據此補入以下五份 sidecar，並用 harness 的「非空且為 stderr 子字串」條件逐項核對成功，沒有猜測錯誤碼。

| 新增 sidecar | 實際 stderr 內的期待字串 | 既有失敗執行 returncode |
| --- | --- | --- |
| `examples/intro-workspace/steps/fixtures/compile-fail/immutable.expected` | `E0384` | 1 |
| `examples/intro-workspace/steps/fixtures/compile-fail/return_semicolon.expected` | `E0308` | 1 |
| `examples/intro-workspace/steps/fixtures/compile-fail/type_change.expected` | `E0308` | 1 |
| `examples/intro-workspace/steps/fixtures/compile-fail/scope.expected` | `E0425` | 1 |
| `examples/intro-workspace/steps/fixtures/runtime-fail/array_index.expected` | `index out of bounds` | 101 |

沒有改 Rust、正文或共用設定，也沒有重跑正常範例。本輪只補 sidecar 並核對既有執行紀錄，不宣稱重新執行全書 harness。新增檔案的凍結 SHA-256：

```text
b2422730b38bdbbee63fc0ce08dbd099e69094e77794a601843fa5cd88c79509  examples/intro-workspace/steps/fixtures/compile-fail/immutable.expected
358b95f8eb03a7a2729d09306a533bbcf4696ca590fc594ce6a9d655f2a85d27  examples/intro-workspace/steps/fixtures/compile-fail/return_semicolon.expected
358b95f8eb03a7a2729d09306a533bbcf4696ca590fc594ce6a9d655f2a85d27  examples/intro-workspace/steps/fixtures/compile-fail/type_change.expected
199eb18cb5837b9b01d8aaea0b5c2eef0d395288e0b7a1dedd6b511b2fa2efe1  examples/intro-workspace/steps/fixtures/compile-fail/scope.expected
9bc98e93f7bbaa33ec6e4a45dde7a4f391fb4fa34bf8231cb7c05d099692527b  examples/intro-workspace/steps/fixtures/runtime-fail/array_index.expected
```
