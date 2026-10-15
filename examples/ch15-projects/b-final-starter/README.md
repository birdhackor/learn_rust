# B最後一步：把既有Batch接成工具報告

本起點沿15.10完整排程、單筆與signal流程，只有三個未完動作：src/report.rs的build_report、Report.status，以及src/app.rs的write_report。render與寫出helper已提供；起點可編譯，回空items／狀態0不代表功能成立。

完整保存本目錄（包含Cargo.lock、src、checks、fixtures）到自己的repo外目錄，明選已安裝Rust1.99.0：

```bash
cargo +1.99.0 check --all-targets --locked
cargo +1.99.0 test --lib --locked
```

起點library零tests。將checks/report-tests.rs整份追加到src/report.rs，checks/app-tests.rs整份追加到src/app.rs，各自已有cfg與module。先得到3通過／6失敗；補映射後5／4、補狀態後6／3、補編碼／write／flush後9／0，重跑相同--lib檢查。imports與helper的角色在15.11先教，完整答案另在相鄰url-checker，這裡不隱藏新題目的完整實作。

batch／single的限定dead_code allow只為未讀結果欄位，完成映射後移除；不替stub實作功能。內置Fixture沿11.8，用已教axum dev依賴，沒有外部path；它供完整答案原有排程tests使用，純報告checks不先啟動它。ready_response的body只確認本機ready，普通single仍只等headers。

最終政策仍為fatal2 > cancel130 > itemfail1 > success0，只有報告成功write與flush後才可交0／1／130。來源失敗／關閉不等使用者Requested；普通完成也收妥signal controller。所有已提交tasks全await，原文／index／duplicates／空行gap及Cancelled／Unstarted都保留。

本目錄normal及dev依賴均可由registry解析；repo外使用隨附package lock，在教材repo內Cargo使用workspace根lock。完整可用版本及建置／安裝／真CLI使用說明在url-checker/README.md；各stage不累加成讀者重複作業。
