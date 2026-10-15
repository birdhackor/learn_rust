# url-checker

檢查UTF-8清單中每個非空白原行的HTTP headers，依原行索引報告；重複網址仍是不同輸入。接受http／https，GET最多追蹤五次redirect，最終2xx通過，不主動讀完整body。

本目錄是獨立package，normal／dev依賴都來自registry，內置本機測試Fixture。可以複製整個目錄到repo外使用。Rust1.99.0；在此目錄執行，檔名以此工作目錄解讀：

```bash
cargo +1.99.0 build --release --locked
cargo +1.99.0 run --locked -- fixtures/invalid-urls.txt --json
```

隨附材料不送HTTP，應得到原index1／3／4的三筆input結果、相同ftp原文保留兩筆，退出1。換成自己的清單即可檢查真網址：

```bash
cargo +1.99.0 run --locked -- 自己的清單.txt --jobs 2 --timeout-secs 5 --json
```

省略設定時jobs8、每筆期限10秒；數值必須大於零。整數long參數要跟值，--json是不用值的bool開關。省略--json得到同一owned Report的逐行文字；文字內原文與detail用JSON引用保留引號與換行。普通client保留環境proxy設定，測試的本機client直接連loopback。網路結果及毫秒依實際環境而定。

stdout只放報告，stderr只放fatal診斷。0為全過；1為有項目未通過；2為設定／讀取／client／訊號／收尾／編碼／write／flush的fatal；130為已觀察使用者Ctrl+C、全收尾且報告成功。優先序fatal2 > cancel130 > itemfail1 > success0。Ctrl+C後不再補新工作，保已完成、Cancelled與Unstarted，等待所有client及listener；普通完成也結束listener。強制終止不等於這份收尾。

清單依lines分行，空白行略過但索引從0計全部原行。非空invalid仍為項目Input問題，空有效清單為fatal。elapsed_ms沿單筆從發送前到finalheaders或失敗結束；Input／未提交等無量測時為null。HTTP4xx／5xx仍保HTTP結果，不混成transport failure；取消／未開始也不假造HTTP或時間。JSON的items依原index，詳細診斷不是可解析的固定錯誤字串。輸出失敗可能已交出部分bytes。

Clippy命令的`--`將後面的選項交給Clippy；`-D warnings`把回報的警告提升為錯誤，令檢查失敗。沒有警告仍不代表功能已正確。

```bash
cargo +1.99.0 test --locked
cargo +1.99.0 clippy --all-targets --locked -- -D warnings
```

tests包含有限本機Fixture的真HTTP、上限／補工／取消／完成待收、純報告映射與JSON往返、正數設定及受控write／flush失敗；不靠公共網站或sleep猜進度。Fixture的ready確認只在測試讀小body，產品single仍headers-only。

安裝到Cargo的binary目錄：`install --path .`選這個本機package，不上傳registry；--locked使用隨附lock。

```bash
cargo +1.99.0 install --path . --locked
```

shell能找到Cargo binary目錄後，使用`url-checker 清單.txt --json`。repo內Cargo使用workspace根lock；目錄複製到repo外則使用此處的package專用Cargo.lock。

公開library入口在src/lib.rs；實作module保持private。check_text借caller的client，要求正數jobs／timeout秒數、caller保持停止送端有效；以Requested要求中止並await到結果。丟棄整份future不能替代等待children。Report／Row是可交換的被動資料，Error提供typed原因與source。CLI完整錯誤政策由run與write_report共同完成。

這份工具讀完整小清單並保存所有結果；jobs只限制已提交tasks，不限制整份輸入／報告的記憶體。沒有重試、認證、爬蟲、HEAD fallback或背景服務。引導練習見教材15.11；stage副本不累計成反覆作業，實際core／tests／helper行數另記於該節作者ledger。
