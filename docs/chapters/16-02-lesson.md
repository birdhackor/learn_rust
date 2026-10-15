# 16.2 Pin 與 Unpin：把地址穩定放進介面約定

上一節交換整份80／120資料，地址記錄便對不上；只交換 Box owner，內部資料則留在原位置。但普通 Box 還允許移出內容。現在要讓其他程式處理地址敏感的資料，同時維持約定：**原值需要保留位置時，不能任意移出；回收或重用空間前，也要先在原位收尾。**

不同 async 工作可能有不同內部借用。共同的介面能讓使用端遵守同一約定，不必猜每份 future 裡有哪些 pointer。本節仍先用數字與地址觀察，下一節才呼叫 poll。

## Pin 包住 pointer，約定作用在 pointee

**`std::pin::Pin`** 是標準函式庫的 pointer wrapper。**pointee** 是 pointer 找到的那份資料。`Pin<Ptr>` 包住 Ptr，保留位置的對象卻是 Ptr 的 pointee：

- **`Pin<Box<T>>`** 包住擁有 T 的 Box，wrapper 繼續管理 ownership。
- **`Pin<&mut T>`** 包住 mutable 借用，沒有取得 T 的 ownership，也沒有因此配置新 T。
- **`Pin<&T>`** 提供同一份 T 的 shared 存取。

因此，移交 `Pin<Box<T>>` 可以只交接 owner，沒有把 heap 中的 T 移出。要分開 wrapper 與它找到的資料。

Pin 是 **library contract（函式庫契約）**：safe API 限制取得可能移出資料的能力；unsafe API 的作者則要維持約定。它沒有改寫 move 語意，也不會修正內部 pointer。錯誤 unsafe 可以破壞約定，讓後來的 safe 使用受影響，不能以編譯成功代替理由。

## Unpin：這個型別不依賴 pinning 保證

**`Unpin`** 是 `std::marker` 中的 auto trait，沿第13.1節，由型別與欄位性質決定是否自動具備。它表示型別不依賴 pinning 保證。當 **`T: Unpin`**，Pin 可以交出普通 mutable 存取，不再額外禁止移出 T；ownership／borrowing 規則仍要遵守。

i32 有 Unpin，可以使用相同介面而不失去原有交換能力。先介紹兩個操作：

**`Pin::new(pointer)`** 接手一份 pointer，回傳包住它的 Pin；safe 版本要求 pointee 有 Unpin。這次接收 `&mut i32`，得到 `Pin<&mut i32>`。

**`pinned.get_mut()`** 接手這份 `Pin<&mut T>` wrapper，回傳裡面的 `&mut T`，同樣要求 T 有 Unpin。它沒有取得 T 的 ownership；不符合 bound 時是編譯拒絕，沒有執行期 Err 分支。

本節仍使用無 library 的 `ch16-future-stages`。完整 manifest 與預設入口如下，沿12.1繼承根共同欄位：

```toml
--8<-- "examples/ch13-futures/stages/Cargo.toml"
```

```rust
--8<-- "examples/ch13-futures/stages/src/main.rs"
```

下面是完整的 i32 程式。取得 mutable reference 後，使用上一節的 swap：

```rust
--8<-- "examples/ch13-futures/stages/examples/unpin_access.rs"
```

```bash
cargo run -p ch16-future-stages --example unpin_access --locked
```

輸出 `i32 是 Unpin，取得普通 mutable 借用後可交換：120、80`。所以「進了 Pin 一律不能 move」不正確；要看 **pointee T** 的 Unpin，不是外層 Box handle 的 Unpin。

反過來，解說中的 **`!Unpin`** 表示「沒有實作 Unpin」，不是所有值從建立起就不能 move。尚未進入 pinning 約定時，仍能建立、回傳、交進 Box；需要保護的是進入約定後的位置。

## 讓數字模型不自動具備 Unpin

上一節 AddressNote 的 i32 與 raw pointer 欄位都有 Unpin。因此只把它交給 `Box::pin`，仍不能依賴 Pin 禁止移出。現在另建 **PinnedNote**，保留數字與地址記錄，增加一個 marker 欄位。

**`std::marker::PhantomPinned`** 本身沒有 Unpin。放進 struct，會使外層不自動具備 Unpin；本例不另手寫 Unpin 實作。它是沒有欄位的 unit struct，值直接寫 `PhantomPinned`。`_pin` 只是本例的欄位名稱，底線表明我們不需要讀取這個標記。

先用它檢查 safe `Pin::new` 的 bound：

```rust
--8<-- "examples/ch13-futures/stages/fixtures/compile-fail/pin_new_requires_unpin.rs"
```

比對診斷的完整 sidecar：

```text
--8<-- "examples/ch13-futures/stages/fixtures/compile-fail/pin_new_requires_unpin.expected"
```

得到 `E0277`，因為 PhantomPinned 沒有 Unpin。這不表示它不能被安置在固定位置，而是 **safe new 不提供這條路徑**。

## 先放好資料，再記地址

**`Box::pin(value)`** 接手 owned T，將它放在 owned heap 空間，交回 **`Pin<Box<T>>`**。這條 safe 路徑不要求 T 有 Unpin；對沒有 Unpin 的 T，後續 safe 介面保留 pinning 限制。

owner 上的 **`as_ref()`** shared 借用 wrapper，交回 `Pin<&T>`；**`as_mut()`** mutable 借用 wrapper，交回 `Pin<&mut T>`。它們都借同一份 T，沒有新配或複製一份 T，也沒有直接交出普通 `&mut T`。

接著有兩種 getter。**`get_ref()`** 接手 `Pin<&T>`，交回 `&T`，可 shared 讀取；不能因此移出非 Copy 的 T。**`get_unchecked_mut()`** 則 unsafe 地接手 `Pin<&mut T>`，交回 `&mut T`：作者必須保證不透過它移出被固定的資料，並持續維持 pinning 契約。它給更大的能力，沒有取消責任。

### method 接收哪一份 wrapper？

沿第3.6節 receiver 的用途，這裡寫 **`self: Pin<&Self>`** 或 **`self: Pin<&mut Self>`**，直接指定 method 接收的 wrapper。Self 仍是目前的 PinnedNote；第一種作 shared 觀察，第二種作受控修改，都沒有接手整份 PinnedNote。

本模型的介面固定如下：

- `PinnedNote::new(value)` 先以 null 建普通值，Box::pin 放好，再記當下 value 欄位地址，交回 owner wrapper。
- `view.value()` 接收 shared Pin，讀正常 i32 欄位、交回副本。
- `view.address_matches()` 接收 shared Pin，只比較地址、不修正或解參考 saved。
- `view.set_value(value)` 接收 mutable Pin，只改 i32 內容，交回 unit；欄位位置與整份 PinnedNote 留在原處。

constructor 只寫 saved；set_value 只寫數字。兩次 unsafe getter 都不移出或覆蓋整份 T，不讓 `&mut PinnedNote` 離開 method，也不公開欄位的 Pin。本模型沒有另對欄位建立固定約定，修改 i32 內容不會移走整份資料。

### Drop 也必須維持原位關係

pinning 不只管這次 method。對依賴它的原值，儲存空間在 **Drop 返回或 panic 前，不能先被回收或重用**；回收／重用前先在原位收尾，稱為 **Drop guarantee**。第11.3節的正常清理由 owner 安排；這裡多了位置要求，不是保證任何值最終都一定被清理。

**Drop 的 receiver 仍是 `&mut self`，不會自動換成 Pin。** 作者不能利用這份普通 mutable 能力移出地址敏感原值。本模型 Drop 只讀正常數字、比較地址，沒有移出或替換 Self；之後 Box 依正常流程處理空間。

下面列完整模型、所有 methods、Drop 及逐處 SAFETY 理由；它始終不透過 saved 讀資料：

```rust
--8<-- "examples/ch13-futures/stages/examples/pinned_owner_swap/pinning/mod.rs"
```

## 交換 owner，修改數字，最後收尾

交換的是兩份 **`Pin<Box<PinnedNote>>`** owner；接著把 left 數字改140，再依11.3的 `mem::drop` 收尾。完整入口：

```rust
--8<-- "examples/ch13-futures/stages/examples/pinned_owner_swap/main.rs"
```

```bash
cargo run -p ch16-future-stages --example pinned_owner_swap --locked
```

```text
交換前：left=80，right=120
交換 Pin<Box> owner 後：left=120，right=80，兩份地址仍一致
只改 left 的 value：140，地址仍一致
清理 PinnedNote：value=140，地址仍一致=true
left 的 owner 已清理，right 仍是=80
清理 PinnedNote：value=80，地址仍一致=true
兩份資料已正常收尾
```

數字可以變，記錄仍對上欄位位置。Drop 輸出是本次原位收尾的觀察；完整安全理由仍來自受控介面與實作責任。

圖中 first 灰框是交接前的名稱；灰箭頭表示 ownership 交給 next。藍箭頭表示 wrapper 找到同一份 heap 資料，V 是 value 欄位位置，紫箭頭只表示 saved 記錄。heap 資料只畫一次，沒有因 owner 移交另建一份：

![first把Pin Box owner交給next，next仍找到同一份heap中的PinnedNote；value位置V與saved記錄V保留](../assets/ch13/pin-owner-transfer.svg)

## 固定也能使用局部儲存

Pin 本身不等於 heap 配置。標準 **`std::pin::pin!`** macro 接手一份值，把它安置在目前的局部儲存，產生 `Pin<&mut T>`；這個操作不為 T 新建 heap 配置，對沒有 Unpin 的 T 也有 safe 路徑。

下面的 LocalNote 只保存數字與 marker，沒有自引用；用它分開「不具 Unpin」「是否實際有自引用」及「存在哪一處」。value method 只需 shared Pin，沒有 unsafe：

```rust
--8<-- "examples/ch13-futures/stages/examples/local_pin.rs"
```

```bash
cargo run -p ch16-future-stages --example local_pin --locked
```

輸出 `局部固定的數字=80；Pin 借用可以交接`。first 交出的是 wrapper，next 仍借同一處 LocalNote；這次沒有 owned Box 可以回傳。局部來源會在目前 block 正常收尾，這份借用不能逃出來源的有效範圍。

普通同步函式中常稱這種方式為 stack pinning；但 async 中跨 await 保存的局部資料，使用的是外層 future 的儲存，可能在 stack，也可能在 heap。**地址約定與儲存來源是兩個問題**，不能從 Pin 或「local」一詞推定一律在哪裡。

## safe caller 不能取得 mutable reference 交換整份值

若對兩份 `Pin<&mut PinnedNote>` 呼叫 get_mut，再 swap 呢？完整負例把同一份私有欄位模型放在 pinning module，main 只用 public 介面；沒有連到後節的 library：

```rust
--8<-- "examples/ch13-futures/stages/fixtures/compile-fail/pinned_get_mut.rs"
```

```text
--8<-- "examples/ch13-futures/stages/fixtures/compile-fail/pinned_get_mut.expected"
```

得到 `E0277`，指出 PhantomPinned 使 PinnedNote 不符合 Unpin。沒有先交換，再檢查地址。把 get_mut 改成 unsafe getter，也不會讓這次整值交換合法；那只是把維持契約的責任交給 unsafe 作者。

## 練習：交 owner，改值，再 drop

建立80的 PinnedNote，把 first 交給 next。用 next 的 mutable Pin 把數字改100，檢查數字與地址，再顯式 drop。預測哪個名稱仍可用、是否需要重記地址，以及 Drop 顯示什麼。

??? example "完整可執行答案"

    完整本地模型：

    ```rust
    --8<-- "examples/ch13-futures/stages/examples/pinning_solution/pinning/mod.rs"
    ```

    完整入口：

    ```rust
    --8<-- "examples/ch13-futures/stages/examples/pinning_solution/main.rs"
    ```

    ```bash
    cargo run -p ch16-future-stages --example pinning_solution --locked
    ```

    first 交出 owner 後不再可用，next 保留同一份資料位置，不需重記地址。修改與 Drop 都顯示100，地址比較仍是 true。

## 完整測試與重借用

as_ref／as_mut 重新借用 owner，之後仍能再借。method 若接手 `Pin<&mut T>`，那份 mutable wrapper 本身不是 Copy；需要重複呼叫時，在 wrapper 上再次 as_mut，交出較短借用。下面第二個 test 以同一份 borrowed wrapper 反覆設負數和零，然後交接 owner。

完整 module 含三項 tests，另核對交換 owner 和 Unpin 數字的普通交換；完整 root 一併列出：

```rust
--8<-- "examples/ch13-futures/stages/examples/pinning_complete/pinning/mod.rs"
```

```rust
--8<-- "examples/ch13-futures/stages/examples/pinning_complete/main.rs"
```

```bash
cargo test -p ch16-future-stages --example pinning_complete --locked
```

現在 caller 能交入 `Pin<&mut T>`，讓依賴地址穩定的實作按約定工作；Unpin 型別也能使用相同介面。下一節用這個 receiver 讀真正的 Future::poll，再分清尚未完成、通知與再次執行。

延伸閱讀：[std Pin 契約與 Drop guarantee](https://doc.rust-lang.org/std/pin/index.html)、[Pin methods](https://doc.rust-lang.org/std/pin/struct.Pin.html)、[Unpin](https://doc.rust-lang.org/std/marker/trait.Unpin.html)、[safe local pin!](https://doc.rust-lang.org/std/pin/macro.pin.html)。其他 projection API 與第三方工具不是下節必要材料。

[下一節：Poll、Context 與 Waker](16-03-lesson.md)
