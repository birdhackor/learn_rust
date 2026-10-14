# 2.2 值由誰管理：ownership 與 move

上一節用 `order` 管理一筆訂單。現在要把這筆資料交給另一段工作，例如讓函式計算小計。我們先分辨兩件事：交出去的是完整訂單，還是另外一份可獨立使用的數字？這會決定呼叫者接下來能做什麼。

## 先建立當下需要的記憶體模型

程式執行時，數值與其他資料需要存放在記憶體。這裡用 **stack** 與 **heap** 兩個角色建立概念模型。

stack 依函式呼叫安排工作區域。`main` 呼叫另一個函式時，被呼叫的函式取得自己的區域，保存參數與區域變數；它回傳後，這個呼叫的區域就結束。這種「後加入的呼叫先結束」的順序，和 1.5 的函式呼叫一致。已知固定大小的數值或資料欄位，可以用這個模型追蹤。

heap 則提供獨立請求的記憶體區塊，不把每一塊資料的存放期限直接綁在建立它的那次函式呼叫上。程式向**記憶體配置器**請求空間，它找出可用區塊；使用完畢後，這個空間需要歸還。要找到那份資料，可保存它的位置資訊，這種表示資料位置的值稱為 **pointer（指標）**。pointer 的概念是「在哪裡」，和被找到的資料本身不同。

這一節的 `Order` 只有兩個數值欄位，不會為它們另行配置 heap 資料。先認識 heap，是為了理解後面文字與可增長清單為什麼還需要管理另一份資料；本節不使用那些型別。stack／heap 是理解資料關係的模型，編譯器也可能將數值放在處理器內部的暫存空間，而非照圖逐項存放。**值是否會 move，要看型別與操作規則，不能只由「在 stack」推斷。**

## ownership 管理的是使用與清理責任

**ownership** 指一個值的管理責任：誰能把這個值交出去，以及這個值最終由誰負責結束。對眼前的局部變數來說，建立 `let first = Order { ... };` 後，`first` 就是這筆訂單的 owner。

owner 的名稱有上一章教過的 scope。當該值仍由這個變數持有，而變數離開 scope，Rust 就結束這個值；若值管理了需要清理的資源，會依其型別的規則清理。本節的兩個數字沒有額外的 heap 資源。這項管理規則由編譯器追蹤，不需要在 `Order` 裡另存一個「owner 名稱」欄位。

**move** 是把值的 ownership 轉交出去。這裡的 `Order` 沒有自動複製整筆值的能力，所以 `let second = first;` 會把完整訂單交給 `second`；之後不能再用 `first` 讀取這筆訂單。這個限制避免同一份被轉交的值還從原位置繼續被使用。函式參數與回傳值也能成為轉交位置。

## 為什麼數字指派仍能用原名稱

把同樣的指派寫法換成已學的 `i32`：`let another_count = count;`，就不會讓 `count` 失效。因為 `i32` 已經具備 **`Copy`** 能力：使用值指派或傳參數時，複製出另一個獨立值，原值仍可使用。眼前的整數、`bool`、浮點數都有這項能力。

自訂 struct 不會僅因所有欄位都是數字，就自動具備 `Copy`。我們目前的 `Order` 沒有這項能力，因此完整 `Order` 的指派會 move；單獨讀出一個 `i32` 欄位則可以複製那個數字。後面的 trait 章才會學如何為型別宣告能力，現在只需能分辨這兩種實際行為。

下圖只表示指派後哪些名稱能用，以及是否形成獨立值。move 的箭頭表示 ownership 轉交，不表示一定在記憶體中搬動幾個 bytes，更不表示 move 只發生在 heap。

![Order 指派後 first 失效而 second 接手；i32 指派後 count 與 another_count 分別保有獨立數值](../assets/ch02/move-copy.svg)

## 同一個程式中的兩種交接

`subtotal(order: Order)` 接收完整訂單，所以呼叫時會取得這個值的 ownership。它算完後只回傳小計數字；訂單本身仍在函式裡，隨這次呼叫結束。

```rust
--8<-- "examples/ch02-ownership/examples/moves.rs"
```

```bash
cargo run -p ch02-ownership --example moves --locked
```

```text
second 的數量=3
接手訂單後算出的值=360
count=3，another_count=4
```

依序追蹤訂單：先由 `first` 持有，再 move 到 `second`，最後 move 到 `subtotal` 的參數。呼叫完成後，留下的是回傳的 `i32` 小計。兩個 count 則各自是獨立的數值，修改 `another_count` 不會修改 `count`。

### 讓編譯器指出原名稱失效的位置

下面是故意無法編譯的完整 fixture。它在 move 之後又讀 `first.count`：

```rust
--8<-- "examples/ch02-ownership/fixtures/compile-fail/use_after_move.rs"
```

把這段放進自己的練習 `main.rs` 後跑 `cargo check`，錯誤重點是 `use of moved value`，錯誤碼為 `E0382`。編譯器會同時標出值交出去的那行與後來又使用的那行。若訊息提到未具備 `Copy`，就對照剛才的能力區別；不用為了讓舊名稱能用，就先把診斷中的其他建議全加上去。

## 練習：接手修改，再把完整值交回

寫 `add_one`，接收一筆完整 `Order`，把數量加一，再回傳這筆 `Order`。參數需要 `mut`，因為函式會修改接手後的值。呼叫者將回傳值保存為 `updated`，不再使用已交出的原變數。

用單價 80、數量 2 驗證：`updated.count` 應是 3，小計應是 240。這裡是「輸入值 move 到參數，回傳值再交到外層」，不是讓已結束的函式區域永久存在。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch02-ownership/examples/ownership_solution.rs)，或執行 `cargo run -p ch02-ownership --example ownership_solution --locked`。

有時函式只需要讀出兩個欄位，呼叫者之後仍要保留整筆訂單。下一節會讓函式借用資料，而不用每次先接手再交回。
