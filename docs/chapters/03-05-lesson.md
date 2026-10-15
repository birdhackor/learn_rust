# 3.5 borrowing、lifetime 與 scope：判斷何時能再修改

前兩節已能借讀與借改。現在把兩種操作放回同一個 main：先用兩份 reference 讀數量，再修改原數量。能否通過，取決於這些存取是否在使用上重疊。

## binding 的 scope 與 reference 的使用區間

**scope** 是程式中一個名稱可見的範圍，例如 main 的 `{ ... }`。在本節的一般區域變數中，還由它持有的值會在離開自己的 scope 時結束。

**lifetime** 在這裡指 reference 必須保持有效的程式區間，不是幾秒鐘。被借的資料必須在所有使用 reference 的地方仍然有效。編譯器也會檢查這段區間中有沒有衝突存取。

名稱還看得見，不代表借用一定持續到大括號結束。在下面這類沒有分支的程式裡，可以從 reference 的最後一次使用判斷：後面不再用它，就不必為它繼續保留借用。先看兩份 shared reference：

```rust
--8<-- "examples/intro-workspace/ownership/examples/last_use.rs:shared-finish"
```

first、second 都只讀同一個數字，所以可以同時使用。println 是兩者的最後一次使用。後面便能借改，再回到 owner 直接修改：

```rust
--8<-- "examples/intro-workspace/ownership/examples/last_use.rs:modify-after"
```

writable 的最後一次使用是第二個 println。之後 `count += 1` 可以執行，結果依序是 3、4、5。這不是把原變數的 scope 縮短，而是各次借用都已完成工作。

![owner 保留在 main 裡，shared reference 在最後一次讀取後完成借用，後面才修改原資料](../assets/ch02/borrow-scope.svg)

圖先只畫 shared 借讀與 owner 修改的關係；完整程式還在兩者之間安排了一次 mutable 借用。它也在自己的最後一次讀取後結束，才輪到 owner 修改。

## 先保留借讀，會擋住修改

下面這份完整失敗例把 println 移到修改之後：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/borrow_overlap.rs"
```

reference 稍後還要讀，shared 借用便必須涵蓋 `count += 1` 所在位置。直接修改與這份借讀衝突，編譯器回報 **E0506**。放進第一章的獨立練習專案後執行 `cargo check`，即可看到診斷。

兩份 mutable reference 同樣不能以這種方式重疊：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/double_mutable.rs"
```

first 在建立 second 後還要使用，因此建立 second 時，第一份獨占借用還沒完成。這份失敗例會得到 **E0499**。判斷的是借用同一資料時的使用衝突；分別借用兩個獨立數字，不會只因 reference 有兩份就失敗。

## reborrow：暫時從借用再借一次

有時呼叫者已持有 `&mut i32`，卻要讓另一段程式暫時修改同一數字，再收回使用權。先備好這個函式：

```rust
--8<-- "examples/intro-workspace/ownership/examples/reborrow.rs:helper"
```

**reborrow** 是從現有 reference 建立一份較短的借用。`*parent` 指到原數字，再寫 `&mut *parent`，便從那個數字借出 child：

```rust
--8<-- "examples/intro-workspace/ownership/examples/reborrow.rs:short-borrow"
```

child 使用期間，parent 不能同時做衝突存取。child 在 println 最後一次讀取後便不再使用，parent 接著把 4 改成 5。原數字的 owner 仍是 count。

函式參數也能收到這種較短的借用。第一個呼叫把 reborrow 寫完整，數量由 5 變 6。第二個呼叫的參數型別已明定是 `&mut i32`；在這種呼叫位置，Rust 可以自動 reborrow，數量再變 7，之後 parent 仍可使用：

```rust
--8<-- "examples/intro-workspace/ownership/examples/reborrow.rs:calls"
```

這不表示 mutable reference 能像 i32 一樣 Copy。一般的 `let next = parent;` 會 move 這份 mutable reference，以下完整失敗例會得到 **E0382**：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/mutable_reference_move.rs"
```

## reference 不會延長原資料的存在

最後看另一種無效借用。下面的 inside 屬於內層區塊；區塊結束時，它的值就結束了，但外層還想借讀它：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/dangling_block.rs"
```

編譯器以 **E0597** 拒絕這份程式，避免外層 reference 指向已失效的區域變數。把 reference 存在外面，不會讓 inside 延長存在。若需要在外層留下數字，便應讓區塊交出 owned 的 i32 值；這個值可 Copy 到外層，內層名稱結束後仍能使用。

## 兩份完整程式

兩份成功例使用同一份 package 設定：

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

先看依序完成各次借用：

```rust
--8<-- "examples/intro-workspace/ownership/examples/last_use.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example last_use --locked
```

```text
兩次 shared 讀取=3、3
mutable 借用讀到=4
借用結束後，owner 改成=5
```

再看完整的 reborrow 程式：

```rust
--8<-- "examples/intro-workspace/ownership/examples/reborrow.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example reborrow --locked
```

```text
短借用讀到=4
parent 最後讀到=7
```

## 練習與完整答案

修正兩個工作：先借讀 count 的 3，再把原數字加成 4；另外從內層區塊把數字 8 保存到外層。第一項可調整最後使用的順序；第二項需要交出值，不能留下指向內層變數的 reference。

```rust
--8<-- "examples/intro-workspace/ownership/examples/lifetime_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example lifetime_answer --locked
```

```text
先讀借用值=3
再修改原值=4
保存到外層的是 owned 數值=8
```

借用規則現在能回答兩個問題：存取是否衝突、reference 使用時原資料是否仍有效。下一節把接手、借讀、借改這三種契約放進 Order 自己的方法。

[下一節：impl 與 method](03-06-lesson.md)
