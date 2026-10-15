# 2.6–2.8 獨立技術初判

發現一項必要的來源閉包／首次使用時序問題，集中在 2.7 首次 `--all-targets` 命令；未發現 Rust 計算、module 路徑、測試答案或測試數量的技術錯誤。這是靜態技術整頁重讀，不是執行、盲讀或版面驗證。

## 獨立性與方法

封存前未讀作者 ledger、first-reader、continuity／audit、root 技術結論或歷史問題。讀者可用基本 Python、已讀第1章和2.1–2.5。完整讀三頁、兩個package的全部相關自有source，以及本頁所有snippet與manifest；沒有把章3或後面才顯示的來源當成已教先備。

章3作者目前使用Cargo，因此本輪未執行Cargo/rustc/fmt/Clippy/tests或刻意修改門檻；以下結果均依source靜態核對。完整hash、片段起訖marker檢查與閉包清單保存於同名JSON。

## 必要問題 TC02T-01：2.7 的首次廣泛檢查早於 allow 解釋，也納入下一頁才展示的 source

- **位置／原文：**2.7:74–78 說 `--all-targets` 涵蓋library、binary、範例與測試，並說「此處選的是內容都已教過的 intro-control」，接著執行 `cargo clippy ... -p intro-control --all-targets --locked`。
- **實際載入：**`control/examples/lint_return.rs:2` 已含 `#[allow(clippy::needless_return)]`；這個allow的意義、scope、刻意保留寫法的理由到2.7:86–100才解釋，完整source在93行。這不是要求提早教整套lint機制，而是這個已被工具處理的具名allow仍在首次說明之前。
- **另一個閉包缺項：**同一命令也納入自動發現的 `control/examples/integration_answer.rs`。2.7的完整來源區塊只列manifest/lib/pricing/main，並於別處內嵌assertions_answer與lint_return；integration_answer要到2.8:95–105才完整出現。它沒有新的必要Rust演算法/API，這裡缺的是當前所選命令實際讀取的自有source，不能把尚未讀的下一頁回算成當前完整來源。
- **已教部分：**library、binary、use路徑、assertion、test/cfg屬性、同檔tests module都在相應正文先說明，保留這些通過範圍。
- **讀者影響：**按順序讀到首次廣泛檢查時，還不能從目前／已讀材料重建其完整source範圍，也不能說明其中函式為何關閉這個具名lint。命令執行成功不會消除「已教過」與實際載入範圍的落差。
- **分級：**增加理解負擔；依實際載入自有source與allow先教再用的要求屬必要待修，並非Rust語意錯誤或執行失敗。
- **最小修正方向：**把all-targets教學命令放在allow解釋及integration_answer完整來源之後（例如2.8答案之後），或在此選擇一組當前已完整展示的較小targets並說明選擇。若保留同段其他廣泛格式／檢查命令，也要讓其範圍與最終選定的階段契約一致。不要刪除有清楚理由的精確allow示範，也不需要新增未教API。

## 通過證據與概念四題核對

| 單位／原文依據 | 身分與機制／性質 | 需要／用途 | 實際操作、例子與閉包 |
| --- | --- | --- | --- |
| 2.6:7–16 兩種crate | src/lib.rs是library根、src/main.rs是binary根；Cargo依檔名辨認，程式內預設crate名把package連字號換底線。 | 另一個呼叫者需共用同一計價規則，不複製函式。 | modules-basic完整manifest無額外設定；binary載入library，再由pub mod pricing載入pricing.rs。沒有tests、cfg、allow或build.rs。 |
| 2.6:26–49 module與pub | mod宣告使檔案加入module樹；pub module不會自動公開所有函式；同module可用私有shipping，外部binary只能用公開函式。 | 公開完整計價能力，保留運費拆法為內部helper。 | lib只有pub mod pricing；subtotal、total_with_shipping、sum_three公開，shipping私有。相同module內呼叫與外部路徑都合法，沒有漏掉被呼叫helper。 |
| 2.6:55–65 路徑與use | ::表示名稱層級，crate是目前crate根；binary與library是不同crate，故binary使用intro_modules_basic起頭；use引入短名、不執行或下載。 | 讓呼叫者取得計算功能，把main保留給組合與顯示。 | 兩行use與實際main一致；80×2加20=180，三項和400。完整lib/main/pricing及manifest都在同頁。 |
| 2.6:95–141 目錄型example及mod.rs | examples/order_answer/main.rs是獨立example root；display.rs和display/mod.rs是替代來源；module宣告決定樹，子module路徑亦說明。 | 第二個呼叫者重用計價library，但用自己的display helper顯示。 | order_answer/main.rs與display/mod.rs全內嵌；mod display私有不妨礙上層main使用其pub show_order。library仍是同頁完整三檔；140×2=280，未達商品免運門檻，含運300。 |
| 2.7:7–17 assertion | assert要求bool，false panic；assert_eq比相等並列left/right；成功不顯示檢查結果。 | 保存由規則獨立算出的預期值，不能把現有輸出直接當規格。 | answer-checks片段280/300正確，宏的形式由println!銜接；已限定本章i32/bool，不需要先教traits或泛型實作。 |
| 2.7:21–37 屬性、test/cfg與同檔module | #[...]是屬性；#[test]標函式；執行器建立並呼叫測試；mod tests{...}是同檔形式；cfg(test)依test建置納入。 | 自動重新比對選定規則，並把檢查名稱與一般run建置分開。 | 這些全部先於unit-tests完整片段及首個cargo test。三行crate::pricing use從library根引入，沒有錯用binary根。測試函式無參數、unit回傳，四個測試互不依賴狀態或順序。 |
| 2.7:41–59 library測試與門檻變異 | --lib只選library測試；assertion失敗是規則不符，和型別錯誤不同。 | 恰好300的輸入應抓到>=變>的邊界錯誤。 | 四項library測試的現有assertions全符合pricing。暫改>後，299仍319、300變320；shipping_threshold_includes_300失敗left320/right300，其他三項不受影響。59行明說不證明所有i32輸入。 |
| 2.7:63–100 工具與allow局部說明 | fmt整理格式、--check只檢查；Clippy做lint；allow限定一個具名lint於那個函式，不合法化型別錯誤。 | 語法比較刻意保留return，需要留下此局部例外的理由。 | lint_return的相鄰註解與精確allow匹配，兩版本都160；保留allow的理由與用法本身足夠。首次廣泛命令時序／閉包仍見TC02T-01，不能由後段解釋回填。 |
| 2.7:140–154 外部example的測試 | assertions_answer是另一crate，其tests也用intro_control根，非crate::pricing。 | 確認免運依商品小計280判斷，而非含運後總額300。 | 全部source內嵌，main輸出280/300；cfg(test)的單一測試shipping_uses_goods_subtotal檢查兩值。--example測試只報該例一項，非library四項。 |
| 2.8:7–37 規格與重新接線 | 非負小訂單、數值不溢位；sum_three另允許負數且約定不溢位；計算放library，main只顯示，tests呼叫同功能。 | 重建一份共同規則、呼叫者與檢查的完整程式，分開改計算與改顯示的位置。 | total先處理空單，私有shipping用商品小計；sum逐項加入外層total。所有語法都已在前文教，四個主要檔案完整內嵌。 |
| 2.8:80–113 練習與完整答案 | my_report是要求讀者自行建立的target，不是宣稱repo現成有檔；integration_answer是另存的完整答案。 | main顯示一筆訂單與加總；tests依規格檢查門檻、空單和兩組array。 | integration_answer的main、兩個test、重複引入library的use都完整。run輸出280/300、400；example測試兩項，library的四項不混算。與所列命令一致，沒有hidden helper或跨檔漏項。 |

## 閉包與完整性補充

modules-basic的主要binary閉包是src/main.rs、src/lib.rs、src/pricing.rs；order_answer另加examples/order_answer/main.rs與display/mod.rs。無測試版本沒有不該提前出現的cfg/test/allow。

control的library測試閉包是src/lib.rs（含cfg(test) module）和pricing；normal run另加main而排除測試module；兩份帶測試example各載自己的root與normal library，故明選--example時不會順便執行四項library unit tests。lint_return也會連結同package library，即使本身不呼叫它，因此library/pricing仍屬其實際載入自有source；同頁完整來源已提供。沒有外部dependency或build script。

三頁所有snippet檔案與section起訖標記均存在。完整來源以Markdown折疊區塊呈現的部分已讀原文，但沒有確認實際展開位置、桌面／手機或可讀性。未做任何執行、Clippy、格式、測試變異或browser驗證；以上靜態預測不可當成該類實測。
