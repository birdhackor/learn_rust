# 第 10.3–10.4 節獨立技術初判

封存：2026-10-15T08:37:04.678Z。必要問題 0；可選問題 0。

按實讀 1–9、10.1–10.2 接續，核對兩節全文、26 個 snippet、9 個 run 與 5 個 selected test 命令（9 tests）、兩個 manifest、完整 owned source 與實際 fixture bytes。27 檔 SHA-256 見 JSON。封存前沒有讀作者驗證、peer、reader、歷史問題；沒有用另讀11章填補本波先備。

- 10.3 的 io::Result 別名、ErrorKind 分類、non-exhaustive wildcard、File／Read／Write 與 write! 出口的角色均在首用前交代。read_to_string 的完整UTF8要求、追加與新讀入量、InvalidData時原String不變，以及其他I/O失敗可能部分進展的界線準確。File::create 清空既有檔案的行為已明說。
- 10.4 的 u8／buffer、有效[..n]、短讀、EOF、Interrupted 與 UTF8跨批次完整性均準確。ShortReader 真正形成3／2／0及舊C，ErrorReader 真正形成中斷→R→權限失敗；ShortWriter 的短寫、write_all零進展與flush契約一致。完整答案只計bytes，FF仍得到1。
- 逐一確認實際材料：note為9bytes的 Rust\\n中\\n，empty為0bytes，not-utf8為單一FF且沒有換行，missing實際不存在。沒有以畫出的文字255替代binary。
- io-text 是無lib package。io-bytes 的每個 binary／example 皆會編譯正常 src/lib.rs，即使 short_writer／byte_count_solution 沒有直接呼叫它；完整 library 已在本節先列出，只有已教 read_all。所有完整tests/helper與答案皆已inline，沒有未教後續library。
- 兩節各自明定先從repo根切到package目錄，後續相對檔名、manifest-path與Cargo test的package cwd一致；測試不依賴先跑write_note。

本波沒有新增Cargo/Rust執行，也沒有寫note-output.txt。結果為靜態source核對。buffer-prefix.svg只讀XML/source，不稱browser視覺或讀者驗證。
