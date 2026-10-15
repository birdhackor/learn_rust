# 15.11 B 完整工具：作者來源與取捨

唯一新範圍15.11；A及B1–4的教材、notes、proof與frozen不改。沿clear-tutorial、writing-prompt與專案術語，以既有停止Batch接真正CLI與同模型報告，不增加爬蟲、重試或body處理。

## 開寫前的fresh英中來源

本輪重新完整讀英文Rust Book的[12.4測試驅動](https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html)正文13–257、[12.6 stdout／stderr](https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html)13–121；繁中對應[12.4](https://rust-lang.tw/book-tw/ch12-04-testing-the-librarys-functionality.html)107–578與[12.6](https://rust-lang.tw/book-tw/ch12-06-writing-to-stderr-instead-of-stdout.html)107–180。繁中頁的目錄較長，已另開位置讀完整正文；初次混合工具輸出截斷stderr時另單頁重讀，不把目錄擷取當完整章。

採用先寫明確結果、驗真失敗、局部完成再驗收，以及資料／診斷分出口的教法。這裡的舊排程已驗收，讀者主要新增映射報告、狀態優先序與輸出完成責任，不重寫整套B1–4。沒有照搬Book立即process::exit或把println成功當成已flush；typed原因沿本書8.6／14.4。測試結果只能證明所選契約，不沿用來源「通過所以全部正確」的廣泛語句。

## actual前文與新增能力

回讀ROOT actual `ch15-b4-prerequisite-v13`九個BODY，含signal完整demo；單筆single104、Input29與排程core277，另完整讀七項tests470。B4由new只準備、每輪先查stop、只record不補工、全部abort＋join、pending成Unstarted到listener正常收尾都保留。actual7章`ch07-review-fixed-v13`的7.6–9全部BODY、14.4 `ch14-api-prerequisite-v1`十一BODY、8.6 `ch08-review-fixed-v9`六BODY已實讀；不是從名詞清單推測API已教。

fresh fixed crates.io來源：clap／clap_derive4.6.7、serde1.0.229、serde_json1.0.154，selected raw source與本機registry相同，存work-only `ch15-final-b-author/selected-api.json`。兩個docs.rs固定頁工具回Internal Error，未說已讀成功；改用官方固定crate來源。Serde Serialize固定頁的所需primitive／Option實作另核，沒有把完整1576行頁面冒稱全部讀過。

- clap `default_value_t = 值`：為該typed欄位設定省略時的值，以ToString形成可解析文字；這不是對Settings derive Default，也不自動驗證正數。整數long參數要跟一個值；bool json仍是不用值的flag。新attribute在本文第一個config source前教。
- Serde對usize／u16／u128、String／bool、Vec與Option提供對應能力。Option在JSON中Some交內部值、None交null；沒有得到headers或沒有開始量測時，不能用0替代None。新Option／整數能力在第一個Report source前教。
- `serde_json::to_string`與已教pretty版本同樣借Serialize資料、回Result<String, Error>，但不加縮排；用於文字報告內原文及診斷的JSON引用，讓引號／換行不破壞一筆一行。完整JSON仍用to_string_pretty，兩種格式都由同一owned模型產生。

最終私有實作與root re-export沿14.4；具體RunError的Display只負責這一層、source借原io／reqwest／JSON原因，字串型signal診斷沒有捏造Error cause。fatal2優先於cancel130，再未通過1／全過0；只能在write_all和flush都成功後交回資料狀態。主程式正常返回，不提前丟owner替代收尾。

## 獨立測試材料與本輪API查證的界線

成品與起點各內置同一份Fixture，正常依賴沒有加入Axum。Fixture沿前文動態port／gate／ready／shutdown；移除本次未用的舊convenience methods及章節path import，ready確認改由局部ready_response接手Response後讀固定ASCII小body。這份test-only能力與產品single只等headers的範圍，在首次helper source前分清。原11.8及B1–4的Fixture／archives不改。

固定reqwest0.13.5的async_impl/response.rs另於實作後核130–183：text消耗Response，等待完整body；本配置沒有charset feature，固定ASCII ready不受字元集判別影響。這次是已知API的**實作後核對**，不是冒稱新讀的文件已在開寫前讀完。selected-api.json分別保留每份raw檔SHA、所讀選段與時點。cargo1.99的install --help亦唯讀核過path／locked契約，沒有因此宣稱已執行安裝或另外跑release建置。

## 採納內容如何成為真正練習

沿Book的測試導向順序，起點只留三個新動作未完成，已有工具流程可編譯；check不是看起來合理便算數。作者把整個起點複製到repo外、明選1.99.0、追加隨附完整checks後，實得3通過／6失敗；完成映射後5／4、完成狀態後6／3、完成編碼及write／flush後9／0。提示及完成參考預設收合，不把新helper藏在答案才首次出現。

沿stdout／stderr分工，產品資料只交stdout，fatal診斷交stderr；候選0／1／130只在編碼、write_all與flush全成功後交出。來源失敗／關閉依typed Err回fatal2；不能因停止發生就一概回130。純映射材料明標手工六份結果；真正HTTP混合測試重用同一份Batch驗映射。作者另外用實際binary與本機受控服務驗CLI出口，不把library tests當作已驗過真SIGINT與程序退出碼。

## 範圍與證據

本節只有最終報告／CLI邊界的新能力；Input及single去除snippet標記後與B4相同，十一個Scheduler／停止／controller函式逐函式相同，getter只移到test helper。沒有增加重試、body下載、認證或爬蟲。報告穩定性指同模型、原序、欄位及跳脫規則；不固定網路回應、毫秒或OS／dependency詳細診斷。

source-identity-audit.json保存B1–4全部舊handoff指紋不變、兩份Fixture相同、checks整module與成品相同、repo外完成的report／app與成品相同，以及整份portable final package相同的核對。不是聲稱reader缺省的排程tests與成品每個檔案都一致。兩份package專用lock各183個packages，直接版本與教材一致，normal／dev全由registry取得，沒有外部path依賴；在教材workspace內仍使用ROOT的根lock。

作者已親看1280×900與390×844各20個實頁位置共40張PNG，含圖及相鄰圖說、手工材料、必要API說明、三步指引、提示／答案收合與展開、真正入口及安裝段。手機長code使用區塊內水平捲動；本次不是逐一檢查所有捲動位置或所有裝置。strict preview建置成功；完整正式證據與實際失敗保存在work-only handoff。作者期ROOT先備precheck與本作者自檢都不等於第15章獨立三角色QA或已發布。
