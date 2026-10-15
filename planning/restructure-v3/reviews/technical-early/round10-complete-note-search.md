# 第 10.9 節獨立技術初判

封存：2026-10-15T09:05:41.379Z。必要問題0；可選問題0。

以實際1–9、10.1–10.8為前綴，不借11/12章補洞。全文、18snippets、全部10份Rust（完整library/main、writer/cli tests與答案）、manifest及全部固定／動態材料已靜態核對。23檔SHA-256與逐command closure見JSON；頁hash與交接65e47b59…一致。未先讀作者驗證或其他角色結論。

搜尋政策與實作相符：片段匹配、一行一筆、原行號／原文字／順序保留、ASCII轉換只作兩邊比較，Ä與ä不混。contains／to_ascii_lowercase／enumerate、String::push(char)及新增Serde usize／Vec<Match>條件均在首用前說明。Report全owned，來源結束後仍可用。

完整library全部module先說明及inline後才首個--lib test。明選targets不提前編譯後面的integrationtests或答案example；每個所選target的normal library閉包都完整。先驗空query再讀檔，Read／Encode／Output原因分層，library write_all與main flush責任清楚，失敗狀態1/2與help/version0符合source。

13項unique tests的材料與預期一致：五search、兩render、一受控writer、五真正CLI。writer用非空結果確保呼叫write，CLI解析整份JSON stdout並核對完整模型，不依OS詳細錯誤或權限碰巧失敗。fold答案只數符合行，結果2。

本波沒有新增Cargo/Rust/child執行，也沒有建立fixtures；8個Cargo命令與結果是靜態核對。search-boundary.svg只讀XML/source，不稱browser或讀者實驗。
