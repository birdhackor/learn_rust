# B4：停止與完整輸入對應的起始版

對應18.10。沿B3提供Input／single、有限fill與設定檢查；record、stop_and_finish、run三個body尚未完成。new只準備資料，不先提交；固定Finished不是功能成立。普通target零tests。

將Cargo.toml、整個src及checks/stop-tests.rs保存到自己的url-b4-work。repo外先把dev-dependency ch11-local-service的path換成你電腦上教材examples/ch11-async/local-service的實際絕對路徑，再選固定toolchain：

```bash
cargo +1.99.0 check --all-targets
cargo +1.99.0 test --lib
```

正文先教typed理由／每筆分類、by_ref、取消與全join、控制helper，再把整個checks/stop-tests.rs追加到src/lib.rs；它自帶cfg與test module，不再包一層。起始版應1項通過、6項失敗，提供好的設定檢查不是停止已完成。

先完成record＋stop_and_finish，應2項通過、5項失敗；再完成run的try_recv／fill／biased select，應7項通過。第二步也加入正文已教的TryRecvError import，完成後移除起始版限定的dead_code allowances。

tests直接持有Scheduler／Fixture，用有限借用等待觀察；任何出口先收回所有已提交children、再shutdown server，才assert／panic。synthetic task故障是受控材料，不當正常網路失敗；其panic hook不是test失敗。

參考答案在b4-complete。正式CLI／JSON與退出優先序接續18.11；全部最終功能仍依原規劃，stage副本不累加成讀者作業。
