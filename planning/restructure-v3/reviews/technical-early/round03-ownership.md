# 3.1–3.8 技術初判與版本分離

凍結稿中未發現其他獨立必要技術缺陷、所選target閉包缺口或可選新建議。root在閱讀期間提示的三項前移和一項strong邊界修正另列，不算本角色獨立重現。

## 來源、獨立性與版本

完整讀取八頁、所有實際引用Rust／fixtures／manifests及五張SVG source。先備維持基本Python與實際已讀1–2章主文；未讀章3作者ledger、freeze／validation結論、首次閱讀、continuity／audit、其他審閱結論或歷史問題，也未用選讀或章4內容補主線。

初讀快照為 **2026-10-15T06:45:47.931410+00:00**（48檔），凍結稿快照為 **2026-10-15T06:49:05.165462+00:00**。兩者只有3.3、3.6、3.7正文不同，引用Rust／SVG／manifest當時皆相同。root提示到達時已讀完八頁及sources，但尚未封存報告，也尚未形成那幾項獨立發現；同名JSON保留初讀hash、凍結hash及改動頁的兩份全文，避免後見覆寫。

本輪是整頁技術重讀，不是盲讀或目標讀者實驗；沒有使用作者跑過commands的結果作自己的執行證據。

## root提示後修正：只作來源複查

| 位置 | 原版本與已教部分 | 凍結稿判定 |
| --- | --- | --- |
| 3.3欄位自動跟隨reference | 原25行在函式片段22行後才說order.count自動跟隨reference；reference角色、借讀目的已教。 | 現19行先於片段，並保留「不是Order變成Copy」。通過。 |
| 3.6 Self建立式與欄位簡寫 | Self=Order原7行已教；Self{...}與同名欄位簡寫原45行在constructor片段42行後。 | 現41行在片段44行前；尤其同名欄位簡寫的局部第一次使用已先說明。通過。 |
| 3.7 for產生&i32與*value | 原23行在sum片段20行後才說明；前文by-value array的for取得i32。 | 現17行在片段前區分這次項目是&i32，*value讀出Copy數字。通過。 |
| reference強調邊界 | 原粗體範圍包含括號；原定義／角色完整。 | 現只強調英文名稱，定義未變；markup來源通過，未做實際渲染驗證。 |

這四項明確受root提示，不計為第二份獨立發現。

## 技術與必要先備證據

| 單位／依據 | 身分與機制 | 需要／用途 | source、例子與邊界判定 |
| --- | --- | --- | --- |
| 3.1:3、7–31 struct | 定型別／欄位，再建立實例；冒號、逗號、.欄位與mut更新分別說明。 | tuple的角色仍靠位置約定，改成具名欄位而保留相同計算。 | structs:120/3→360，只改count成4後480；before不自行重算。55行明說tuple仍能算對，沒有誇大能力差異。 |
| 3.1:59–81 公開模型 | pub型別和pub欄位各自公開；mod.rs、crate::、use已由2.6教；空{}按順序接expression在完整main前說明。 | 另一module要建立、修改和讀取模型。 | public_model完整main與model/mod.rs都嵌入；80/2改count3，小計240。沒有直接印Order而暗用Debug／derive；structs_answer完整模型與數值assertions都在同檔。 |
| 3.2:7–33 ownership／move | 一般局部變數仍持有的值離開scope時結束；Order非Copy，指派與owned參數交出ownership；不等於必須搬bytes或只有heap能move。 | 交出後判斷哪個名稱還能使用、誰負責結束。 | moves先first→second，再second→subtotal；後面只用返回數字360。Order無heap仍受move規則；use_after_move完整例的E0382一致。 |
| 3.2:37–45、79–91 Copy與交回值 | i32複製出獨立數字；自訂struct不因數字欄位就自動Copy，但單欄i32能Copy；mut參數先說明。 | 不把數字指派習慣誤推到整筆Order，並能交回修改後owned值。 | count3／another_count4獨立；ownership_answer接手80/2，回傳count3／小計240。沒有偷用Clone或derive。 |
| 3.3:7–43 reference／* | &在expression建立借用、在型別表示借用；shared只讀本章普通欄位；*沿reference找到值，i32可Copy。欄位跟隨前移另列。 | subtotal只讀，main仍需要原訂單，無需先交出再收回。 | shared_references算360後原count3仍可讀；copied_count是3、原count仍3。reference用完不清理原資料；(*order).count與自動跟隨區分清楚。shared_answer全helper／模型／main完整。 |
| 3.4:7–23 mutable reference | &mut提供獨占存取，活躍時不能衝突；owned order須mut提供借改。*reference+=1改資料，非重新指派reference binding。 | 修改呼叫者仍持有的同一筆訂單而不交接ownership。 | add_one後count4、price120；數字3→4。函式／Order／main完整，借改完才讀owner。 |
| 3.4:46–70 mut binding對照 | let mut reference:&i32可換到另一份shared reference；&mut才是對所指資料的借改權限。 | 區分定位值改變與原數字改變。 | reference_binding讀3再8，原first/second仍3/8；shared_write為E0594，不能只加binding mut解決。borrow_answer借讀／借改／再讀依序160→240。 |
| 3.5:7–45 lifetime／scope | lifetime是reference需有效的程式區間，不是秒數；在無分支例由最後使用追蹤，名稱scope不縮短。 | 解釋先讀後改何時可行，稍後仍讀為何阻止中間寫入。 | last_use依序3/3、4、5；borrow_overlap的E0506與double_mutable的E0499符合實際後續使用。沒有把所有lifetime等同lexical scope，或僅因兩份reference就判錯。 |
| 3.5:49–85 reborrow／有效資料 | &mut *parent建立短借用；明示參數型別的呼叫能自動reborrow；未註型let next=parent則move非Copy的&mut。reference不延長inside存活。 | 暫借child/helper後保留parent使用，並區分交出owned值和留下失效借用。 | reborrow3→child4→parent5→兩次呼叫6/7；helper全嵌。mutable_reference_move E0382、dangling_block E0597合理；lifetime_answer改順序並交出owned8。 |
| 3.6:7–47 impl／method | &self／&mut self／self分別借讀／借改／接手Self；接收者由點前提供；new非自動產生，Self和欄位簡寫前移另列。 | 把訂單相關操作放型別旁，讓receiver表明操作權限，保留既有borrow／move規則。 | methods先new80/2→subtotal160→add_count後240→into_parts(80,3)，原Order交出後不再用。方法名不決定ownership。methods_answer四項assertion及輸出一致。 |
| 3.6:72–78 借讀不能交出整筆值 | *self找到借來的非Copy Order，&self沒有交出它的owned權限。 | 不把dereference或into_order這個名稱誤當接手。 | move_from_borrow完整struct／impl／main，E0507合理；若本來要接手，改owned self是已教契約，不引入未教Clone。 |
| 3.7:7–44 slice | &[i32]帶定位與長度，借原資料連續一段，不另建array；相對索引從0起。for項目&i32已前移。 | 同一函式加總中段、整段、不同長度array，不再把3綁在參數型別。 | 原1..4→120/200/60、len3、selected[0]120、sum380；全500、空0、兩項200。Copy視圖不複製元素；start<=end<=len、5..5空段正確，不把for的空range規則誤套所有切片。 |
| 3.7:72–107 bounds／mutable slice | slice有自己的len；&mut[i32]提供區段修改，for取得&mut i32在片段前說明。 | 原array有後面元素不代表slice可越界；只改指定中段。 | 長3取索引3 panic；increase後80/130/210/60/40、和520，空段不改。答案含sum_values與increase全部來源，借改完才assert原array。 |
| 3.8:7–29 str／&str／String | str是UTF-8文字slice；&str是帶byte長度的借讀視圖，literal為&str在靜態資料；String owned，空值可尚未配置heap。new/push_str/as_str先說角色。 | 逐步組文字需owned可追加值；顯示函式只借讀。 | strings先追加Hello, /Rust並借讀，不複製成另一String；show借用完成才追加!、最後move。無突然出現的String::from、to_owned或format!。 |
| 3.8:41–112 move與文字模型 | push_str借讀來源但不接手ending；String非Copy；owned self可交出label欄位。 | 把1.7清理責任接到實際heap文字，訂單持有自己的標籤。 | strings輸出Hello, Rust與兩次Hello, Rust!；移後使用E0382，as_str view跨push_str為E0502。text_order的LabelledOrder／impl／main全含，subtotal240，最後抽出同一owned文字。 |
| 3.8:116–126 答案與選讀界線 | output:&mut String借改、label:&str借讀；權限均已教。 | 呼叫後仍由main持有輸出，依序追加兩段。 | strings_answer完整helper/main，只用new/push_str/as_str，顯示Rust 教學。Python／JavaScript對照明列章末選讀，不作主文先備。 |

## 五張圖的技術原碼核對

五份SVG均成功XML解析；只作source檢查，不代表圖片視覺驗收：

- move-copy：first交給second的120/3與count3→another_count3/4分開；desc明說非實體bytes搬動。
- reference-owner：reference:&Order借向仍由order持有的120/3，無第二筆Order；最新圖尾只標「借用同一筆Order」，沒有在formal dereference段落前加上新操作文字。
- borrow-scope：藍線止於first/second最後讀取，灰線到main大括號；3.5:27明說圖只畫shared→owner，完整程式另有中間mutable借用，未把簡化圖冒充逐行完整時間軸。
- slice-part：原索引1/2/3對slice0/1/2和值120/200/60，終點4不含；框選原資料同一批元素。
- string-view：greeting:String管理heap的Hello, Rust，show.text:&str只借同份內容；管理與借讀箭頭分開，不畫第二份文字。

## 命令閉包、完整性與限制

正文20個成功targets都明選intro-ownership及本頁--example／--locked。package manifest僅name/version/edition，無src/lib.rs、build.rs、外部dependency或repo Cargo config；一般例子各是單一.rs且含模型、helpers、main，public_model另載同頁完整model/mod.rs。沒有test/cfg/derive/allow或未教Clone／Vec API混入。九份compile-fail與一份runtime-fail完整，.expected錯誤碼／核心訊息均與靜態原因一致。全部snippet檔案及起訖markers存在。

封存前共享workspace另新增patterns成員及intro-patterns鎖定metadata。只讀其manifest確認仍是既有name/version/edition，未讀章4 Rust source；不因此擴大本章窄選target閉包。初讀、凍結、末次metadata版本分開存JSON。

本輪未執行任何Cargo/rustc/Clippy/fmt/test/runtime fixture、修改變異或browser，沒有宣稱實際字體／桌面手機可讀性通過。所有輸出／診斷是source靜態核對；不外推為任意i32輸入皆不溢位、所有Rust變體已教或實際讀者順暢。原獨立版本與root提示後複核的來源保持分離。

## 封存前偵測的另一次來源更新

於 **2026-10-15T06:55:52.398022+00:00** 的hash guard偵測3.3 alt與reference-owner.svg和前次freeze快照不同，已實讀兩份差異。alt刪除dereference句，SVG的desc與圖尾把「*reference找到Order」改成只說借用同一筆Order；其餘借用箭頭、owner、120/3及後文完整dereference教學保留。這個窄更新沒有改Rust或借用語意，來源複查通過。它是本角色偵測的版本變更，不是本角色提出的新缺陷，也不冒充先前已被獨立發現的問題。三階段文本／hash均保留在JSON；最終seal hashes對應這個更新後版本。
