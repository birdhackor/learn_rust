# A3 工作版

本階段承接 A2 的 collection／content／files／stream 完整成果，保留 Unix 20 項既有測試。metrics 回收 12.4 的 checked 計算；report 的 owned 模型、兩種 issue-row helper 與 JSON wrapper 已提供。

自己完成 src/report.rs 的 order_issues、build_report、render_text。固定 Overflow／空清單／空文字只標示待做，沒有計算結果的意義。原生 PathBuf 排好才轉成 Unicode 報告，diagnostic Debug 只供辨認。

保存完整 manifest 與 source 到自己的練習資料夾，保留自己的 A2 進度。先用 cargo +1.99.0 check 解析新依賴，再以 cargo +1.99.0 test --locked 確認舊成果。

checks/report-tests.rs 不是自動 test target；把整個含條件屬性的 tests module 加到 report.rs 末尾後，才真正驗收 A3。新增九項包含 native 排序、兩種 issues、完整性、JSON 往返、控制字元、checked 指標與 Unix 非 Unicode／hard links。起始版固定 Overflow 會使溢位檢查碰巧通過；不能拿它當成果。

本階段沒有完整參數、exit 0／1／2 或最終 CLI。工作量只算本階段新增的三個待做 body；前面完成的 collection／content 不需重新寫一次。
