# clear-tutorial：通用寫作與審閱 skill

使用入口為 [SKILL.md](SKILL.md)，可用 `$clear-tutorial` 明確指定。

本目錄複製來源的通用準則，保留寫作、概念依賴、方法理由、例子、公平比較、圖解與審閱方法。專案背景、技術事實與操作約定依所在專案另行指定；本次不匯入 YOLO 專案參考，也不新增 Rust 專案規則。

| 文件 | 用途 |
| --- | --- |
| [SKILL.md](SKILL.md) | 通用寫作與審閱準則 |
| [寫作短 prompt](references/writing-prompt.md) | 可直接使用的寫作指令 |
| [審閱流程](references/review-protocol.md) | 逐段閱讀、證據保存與交叉核對 |
| [跨領域校準例](references/calibration.md) | 通用判準的例子與充分性界線 |
| [UPSTREAM.json](UPSTREAM.json) | 固定來源版本、原始檔案雜湊與本地調整 |

依使用者指定的階段與範圍工作。準備或更新 skill 不代表教材已改寫或通過審閱，也不自行啟動實驗或獨立審閱。

## 來源與維護

來源：[birdhackor/learn_to_yolo 的固定版本](https://github.com/birdhackor/learn_to_yolo/tree/eec3edc59003ec2775578025a436613844a8d72a/.agents/skills/clear-tutorial)。

三份通用參考檔保留來源原文；主入口只將 YOLO 專案參考改為通用的專案資料分工。未匯入 `references/project-context.md`。同步時先比較來源差異，保留本地已授權的調整。

`UPSTREAM.json` 的雜湊描述上述固定版本的來源原始檔案；本地主入口與本說明已調整，不以來源雜湊宣稱逐位元組相同。
