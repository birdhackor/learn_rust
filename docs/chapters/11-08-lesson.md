# 11.8 用 Rc 共享 ownership，用 Weak 觀察

第 11.2 節每段鏈由前一份 Box 管理。現在要兩條鏈分別是80接120、40接120，而且尾端120是**同一份資料**；任何一條先結束，另一條仍能讀它。重建兩份120能算相同數字，卻沒有保留這項共享關係；只借同一尾段則需另外維持 owner。

## Rc 用計數維持共同 owner

**`Rc<T>`** 位於 std::rc，Rc 是 **Reference Counted**。它用 **reference counting** 記下有多少個共享 owner 持有同一份 heap 資料。保活 T 的 owner 稱 **strong handle**；handle 自己仍是普通值，受 move 與 scope 規則管理。

最後一個 strong handle 結束，才清理內部 T。數的是 Rc owner，不是每次 `&T` 借用；借用不會自行增加 owner。Rc 是單一執行緒內的共享型別，跨執行緒的管理留後章。

先介紹本步會使用的操作：

| 操作 | 本節的契約 |
| --- | --- |
| `Rc::new(value)` | 接手 T，建立第一個 strong handle，數量1。 |
| `Rc::clone(&handle)` | 新增指向同一配置的 handle，strong加1，不 clone T。 |
| `Rc::strong_count(&handle)` | 借讀目前 strong 數量，交回 usize。 |
| `Rc::ptr_eq(&a, &b)` | 檢查兩個 handle 是否指向同一份配置，交回 bool；不是比較 T 的內容。 |

Rc 的 Clone 只複製 handle，因此 T 不必有 Clone。Rc 也提供 `Deref<Target = T>`，可借讀內部資料，並沿第 11.4 節的 method／欄位查找使用它。

本節共用以下完整 manifest，沒有 library：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

完整程式讓沒有 Clone／Copy 的 Tracked 由兩個 handle 共同持有，以第 11.3 節的 Drop 追蹤清理：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/rc_handles.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example rc_handles --locked
```

```text
建立後strong=1
clone後strong=2
兩個handle指同一配置=true
第一個handle結束，仍讀到id=1
剩餘strong=1
清理原資料id=1
最後一個handle已結束
```

drop(first) 只移除一個 owner；second 仍讀同一份 id1。drop(second) 才讓 strong 降到0、清理 T。若寫 `let second = first`，則只 move 同一個 handle，first 不能再用，strong 也不會增加。

## 兩個開頭，接同一份尾段

SharedChain 保留 End／Link，只把 next 改成 `Rc<SharedChain>`，讓不同開頭共同持有尾段。total 這次接受 `&SharedChain`，不消耗它。

match 借讀 enum，value 是 `&i32`，next 是 `&Rc<SharedChain>`。累加 `*value` 後，`chain = next` 在需要 `&SharedChain` 的位置利用 Rc 的 Deref 轉換；沒有 move 或 clone 原鏈。

這一步的完整 helper 只有型別與加總，不含後面的 Weak 測試：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/shared_tail/shared_chain/mod.rs"
```

圖中的 tail、first.next、second.next 是三個 strong handle，指向同一份120；尾段自己的 next 再管理 End：

![tail、第一條鏈的 next 與第二條鏈的 next，三個 strong handle 共用120尾段](../assets/types-v3/shared-tail.svg)

完整 caller 使用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/shared_tail/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example shared_tail --locked
```

```text
共同尾段的strong=3
第一條總額=200
第二條總額=160
第一條結束後strong=2
原tail handle結束，第二條仍是=160
```

first 結束時，其 next 一起清理，尾段由3份 owner 變2份；再移除 tail，second.next 仍保活共同尾段。這回應了任一條都能先結束的需要，代價是 clone／清理需更新計數，配置也需管理資訊。

## 共享 owner 不提供任意修改或移出

Rc 有 Deref，沒有 DerefMut。宣告 mutable handle 只允許修改 handle binding，不會替共享 T 取得獨占修改借用。以下完整負例執行 `cargo check` 得到 **E0594**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/rc_mutate.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/rc_mutate.expected"
```

也不能像 Box 一樣，直接以星號移出非 Copy 的 String；Deref 借用沒有給我們取走其他 owner 仍需資料的權限。以下得到 **E0507**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/rc_move_out.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/rc_move_out.expected"
```

本節只需共同讀取，這個介面已足夠；下一節才建立共享修改狀態所需的受控介面。

## Weak：想找到資料，但不要求它繼續存在

**`Weak<T>`** 也位於 std::rc，保存不保活 T 的觀察關係。它能在 T 結束後繼續存在，因此不能直接當成有效的 `&T` 讀取，必須先嘗試取得 strong handle：

| 操作 | 本節的契約 |
| --- | --- |
| `Rc::downgrade(&owner)` | 建立同配置的 Weak，不移除 owner、不增加 strong。 |
| `Weak::new()` | 建立空 Weak，不指向資料。 |
| `observer.upgrade()` | 借讀 Weak，回傳 `Option<Rc<T>>`；仍有 strong 時取得新的 strong，否則 None。 |

成功取得的 Some 裡面是新的 Rc owner，不是普通 reference，所以 strong 會加1；保留它也會保活 T。

**值與配置的清理要分開**：最後 strong 結束就清理 T；若還有 Weak，管理計數與位置所需的配置仍保留。這不表示 T 還能讀，upgrade 也不會復活已清理的值；Weak 結束後才不再因它保留配置。

下面的完整程式先試空 Weak，再把成功 upgrade 得到的 temporary 限在 match 分支。離開分支會移除這個暫時 owner，接著才能觀察最後 owner 結束的情況：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/weak_observer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example weak_observer --locked
```

```text
空Weak沒有資料
建立Weak後strong=1
upgrade讀到id=1
暫時取得strong=2
暫時handle結束後strong=1
清理原資料id=1
最後strong結束，Weak不能再取得資料
觀察handle也已結束
```

strong 的1→2→1對應建立、結束暫時 owner。drop(owner) 後立即清理 T，observer 還在卻只能取得 None。若把第一次升級的 Rc 留在外層，它本身就能繼續保活 T，不能只算最早那個 owner。

## strong 循環會阻止數量歸零

**reference cycle** 是沿連結又回到原對象的關係。下圖是用來推理的**假設資料圖**，不是本節已建立的可執行程式：A 裡的 Rc 保活 B，B 裡的 Rc 也保活 A。

![假設A與B各有strong連結保活對方，外部handle消失後，兩邊strong仍各為1](../assets/types-v3/strong-cycle.svg)

外部 handle 全結束，A、B 仍各有 strong1，兩邊都不能開始清理，內部 Rc 也無法跟著移除，形成 **memory leak**：沒有外部使用者，資料仍未釋放。Rust 的 borrowing 檢查不會自動清除這種 ownership 循環。

若 B→A 的責任只是觀察，就讓它使用 Weak，不替 A 增加 strong；A 結束後，B 再找它就應接受 None。要看實際哪些方向負責保活，才能安排 Rc 或 Weak。

## 練習與完整答案

建立保存120的 Rc，clone 成三個 owner，再建立一個 Weak。確認 strong3；drop 前兩份，第三份仍讀120、strong1。用短暫 match upgrade 後，讓新增 owner 離開分支；最後 drop 第三份，確認 Weak 得到 None。

下面是完整 SharedChain helper，此時才加入兩項測試，分別檢查共同尾段與已 upgrade 的 Rc 接替原 owner：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/rc_complete/shared_chain/mod.rs"
```

完整答案根檔案如下：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/rc_complete/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example rc_complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example rc_complete --locked
```

讀到120後，最後顯示 `所有strong結束後，Weak沒有資料`，兩項測試應通過。下一節保留 shared receiver 的需要，分開值交接與執行期借用兩種修改介面。

[下一節：Cell 與 RefCell](11-09-lesson.md)
