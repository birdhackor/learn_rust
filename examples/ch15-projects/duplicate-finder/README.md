# duplicate-finder：完整參考版

一個可用的重複內容報告工具。指定一個root，預設穩定文字，可用--json取得同一份結果。先用大小與BLAKE3縮小候選，再用真正bytes確認；不跟symbolic links、不刪除檔案。

## 在教材repo使用

依第1章完成Rust環境後，從repo根目錄執行：

```bash
cargo build -p duplicate-finder --release --locked
cargo test -p duplicate-finder --locked
cargo run --quiet -p duplicate-finder --locked -- examples/ch15-projects/materials/duplicate-demo
cargo run --quiet -p duplicate-finder --locked -- examples/ch15-projects/materials/duplicate-demo --json
```

固定六檔得到complete=true、2群、重複內容10bytes、空問題清單與狀態0。把root換成自己的目錄即可。root依真正CWD解讀，--manifest-path不改CWD；以shell引號保留路徑中的空白。--help和--version顯示資訊並回0。

需要存成檔案時，依7.5在shell重導向stdout，並緊接查看狀態；請把報告存到掃描root之外，避免shell預先建立的報告自己也進入掃描材料。

## 獨立保存完整答案

完整manifest欄位皆為literal。把此目錄的Cargo.toml、src、tests與fixtures保存到repo外同一資料夾，不依賴其他stage或path dependencies。依15.1選固定toolchain並先解析依賴、建立自己的lock：

```bash
cargo +1.99.0 check
cargo +1.99.0 test --locked
cargo +1.99.0 build --release --locked
cargo +1.99.0 run --locked -- fixtures/duplicate-demo --json
```

最後一行使用此 package 隨附的六檔 fixture，同樣得到10bytes／兩群；連同fixtures保存便可重做。要用自己的資料，將root換成自己的目錄。Cargo.lock保存這一份解析結果，下一次使用--locked核對它。

## 狀態與資料

| 觀察 | 狀態與出口 |
| --- | --- |
| 完整報告成功write_all與flush | 0；stdout是報告、stderr空白 |
| 有項目問題，部分報告成功write_all與flush | 1；stdout仍是完整格式報告，issues保留、stderr空白 |
| 用法、root、指標、JSON、write或flush致命失敗 | 2；stderr嘗試寫診斷。輸出失敗時stdout可能已有部分bytes |
| help／version成功寫出 | 0；stdout是資訊、stderr空白 |

JSON的五個top-level欄位是complete、duplicate_bytes、groups、scan_issues、file_issues。group有len／paths／duplicate_bytes；診斷有path_debug／action／reason。群組路徑是Unicode文字；非Unicode項目只在診斷用Debug表示，不能拿這份文字當成可反向開檔的路徑。

完整性描述掃描，不保證出口已成功；程式明確flush後才回0或1。指標是不同路徑的邏輯重複bytes，hard-link路徑也算，不宣稱能回收同量實體空間。乘法與加法都checked，不能表示正確總量就回致命錯誤。

讀前／讀後的原生路徑與openedhandle觀察是盡力而為，不是檔案系統一致快照。一般檔案問題保留且繼續；exact offender移除後從剩餘桶有限重建。私有callback違約的panic是程式作者接錯，不是正常檔案失敗政策。

## 從library使用與查答案

公有入口從duplicate_finder的root取得：scan_report借Path回owned Report，項目問題仍可是Ok的partial結果；run借Settings與Write出口，真正輸出完成後回0／1。RunError保留typed分類，Error::source借出可用標準原因。實作module與callback不公有。

```bash
cargo doc -p duplicate-finder --no-deps --locked
```

產生target/doc/duplicate_finder/index.html。API文件的scan_report範例是真正doctest，不依賴固定CWD；tests/public_api.rs是外部caller。src/tests.rs可驗私有core，tests/cli.rs真正啟動binary。

Unix有8核心、7CLI、1公有API、1文件測試，共17；其中有多組材料。非Unix不編入3項UnixCLI fixtures，作者未在非Unix執行。fixture控制短讀、Interrupted與指定讀錯誤、強制摘要桶多真群、變動、完整ties、overflow、write和flush；真檔案另驗root、links、hardlinks與partial1。

這一份快照按全部手寫.rs物理行數計：核心995，測試與fixture662，合計1657，包含註解／空行／文件測試，排除lock／建置產物／A1～A4copies。讀者按前四階段自己完成，再用此版本對照；不用一次重寫全部參考tests。
