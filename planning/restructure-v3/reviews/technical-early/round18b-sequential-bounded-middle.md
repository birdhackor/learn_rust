# 18.8–18.9 獨立技術初判

必要修正0項；可選修正0項。完整正文依18.8→18.9順序實讀後封存；未看作者驗證、其他審閱或歷史問題。

## 版本與範圍

- 18.8：`88f5224655b0d0b037c4e51795f0bd71d81be264a33f5583d9d0a12c57a92d7c`。
- 18.9：`7c78f41be377d1b845d5274883b4f927370411e091b2f50761febee3965fed13`。
- root在首讀前僅通知這兩頁有label／prose版本更新；按current全文核對，不將先前修訂算自己的發現。
- 核對29個owned檔，其中25個實際載入的Rust／manifest／checks／example，4個README僅hash留存；47個snippet引用、10處命令、2張SVG原文，合計38份來源hash。未借18.10以後內容補先備。

## 判斷與證據

18.8先enumerate再判trim視圖，原索引空洞、重複行、無效但非空行與原始空格都正確保留。Input持有owned String；來源scope結束仍可使用。LF／CRLF與單獨CR的說明符合lines。prepare_inputs與runner分別拒絕零項；for內逐項await，Copy Duration為每筆建立個別期限。混合測試的索引1／3／4／5／6與9次HTTP命中、demo四份結果與3次命中均符合完整source。

18.9的jobs限制追蹤中的JoinSet項目，已完成尚未收取也計數；pending與結果記憶體的界線已說明。Id表保留原Input，成功Check與失敗JoinError都能回原行。next僅await一次join，之後同步remove／分類／fill／回傳，觀察期限取消不會在中途遺失已收取項。caller的唯一且遞增索引是契約，本文沒有誤稱執行期檢驗。task取得owned Client／String，沒有借用Fixture；最後sort_by_key只改報告順序。

六個依序測試與七個bounded測試的完整helper、cfg／mpsc feature與必要allow(dead_code)均在首次使用前交代。completed-uncollected實驗限定current-thread，signal後立即回傳而無await，足以建立所需狀態。panic／abort實驗保留實際JoinError與索引。失敗路徑先abort＋await／drain再shutdown；不透過next清理而意外補新工。起始版本六敗／一過六敗與只完成fill＋next後三過四敗的文字可由source支持。

四份single.rs精確等於本人已讀B1 core（不含cfg(test)）；兩份checks精確等於完整library測試尾段，B3 input兩份byte-identical。Fixture及http-single共七份normal依賴檔與18.7本人封存hash一致。各自normal lib、測試與demo閉包完整inline，不載入後續B4／CLI或依賴套件自身的測試。SVG原文的行身分與名額狀態對應程式；本次僅讀XML，未宣稱視覺驗收。

## 驗證界線

只做source／manifest／hash／anchor與靜態結果核對。沒有執行Cargo、Rust、測試、HTTP、範例或瀏覽器；未採作者PASS作自己的執行證據。逐檔hash、完整引用／命令及copy證明在同名JSON。
