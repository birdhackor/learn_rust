# 第14章原判裁定與最小修正

原稿為 `ch14-complete-prerequisite-v8`；修後採 `ch14-review-fixed-v10`。本文件是協調者讀完原判後的處置，不是另一份獨立發現。TECH、CONT、FIRST prefix與FIRST tail的原文、分級及SHA保持不變。

| 發現與原分級 | 來源核對、讀者收益及處置 |
| --- | --- |
| TECH必要／CONT必要：`expr`在私人macro第一次使用時尚未說明 | 採用。14.2在第一份code前接已教expression，指出`&distance`也屬expression。只補當下模式用途，不搬後面的fragment分類表。FIRST prefix當時未知、後文釐清的紀錄仍保留，不改算獨立必要發現。 |
| CONT必要：`assert!(條件, "訊息")`第一次使用前缺第二參數約定 | 採用。14.4的故意失敗fixture前說明false時的panic訊息，沿已教條件檢查；不新增格式化或測試框架。 |
| TECH必要／CONT必要：兩個獨立相容版本缺凍結的Cargo.lock | 採用。既有工作檔兩lock加入修後archive及發布allowlist。獨立版本讀者需要`--locked`可重播，不能借根workspace的lock補足。原審閱者各自的失敗與自己產生lock後成功範圍保留。 |
| FIRST prefix可選：`Just`、`RngSeed`短名稱的完整來源 | 採用。14.5第一次使用前列完整import，讓Python入口讀者能建立檔案；型別作用原本已教，沒有升級為必要概念缺口。 |
| TECH可選：`test --example`與執行`main`的責任區別 | 採用。14.1第一次命令前各一句，將既有測試執行器與example target接起；不重講Cargo全章。 |

FIRST tail為另一位實際讀完1–12及14.1–14.5背景的讀者，完成14.6–14.8共23 BODY，原判沒有確認必要或可選缺口。信賴區間的具體公式、超出有限實驗的極大repetitions上界保留為其未知；當前讀取欄位與100次實驗不依賴它們，沒有把未知改算通過所有API。入口41 BODY只作這位讀者的背景，不冒稱第二份prefix盲讀驗收。

修正僅四段正文，所有原134個source與SVG字節維持；另增兩個已存在的獨立lock。材料變動BODY為`001-004`、`002-004`、`004-003`、`005-005`，其他差異僅凍結資產絕對路徑。沒有因文字修正重跑已通過的Rust或量測。

第一次修後凍結v9漏了10份profiles資料，ROOT的來源集合檢查當場失敗；v9保留，未交給審閱者。v10明列profiles目錄，實際核得原134字節全部相同加兩lock，共136。具體proof在協調工作目錄`ch14-minimal-fix-scope.json`。

目前獨立交叉覆核、受影響單位LOCAL閱讀與修後實頁尚待完成，本文件不宣稱整章發布通過。AI審閱不等同真人新手測試。
