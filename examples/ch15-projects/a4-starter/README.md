# A4 工作版

A3 六份 core 模組與29項Unix驗收已提供，保留自己的成果。新 app 保存 Settings／RunError／write_output；main 已接參數、出口與ExitCode。自己完成 app.rs 的 write_report／run，固定0或Overflow只是待做，沒有輸出成功的意義。

保存完整manifest/source到自己的資料夾後，以 cargo +1.99.0 check 解析 clap derive 依賴，再以 cargo +1.99.0 test --locked 確認舊成果。所有版本欄位皆為literal，沒有workspace繼承。

checks/app-tests.rs 的整個tests module需手動加到app.rs末尾；checks/cli-tests.rs 需整檔放到自己新建的 tests/cli.rs。checks 本身不是自動target。後者用本package binary的env!名稱 duplicate-a4-starter；改package名稱時也要依10.8改crate import／binary名稱。

ControlledWriter可控制write或flush失敗，真正child另驗出口和0／1／2。部分報告仍保留問題且完整寫出才回1；空掃描可完整回0；root/指標/JSON/寫入/flush的致命錯誤回2，沒有用expect替代正常政策。

本階段只接兩份函式，不需重寫A1～A3，也不把所有stage檔案累加成一次新作業。
