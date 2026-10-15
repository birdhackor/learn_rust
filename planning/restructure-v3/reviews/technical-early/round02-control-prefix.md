# 2.1–2.5 獨立技術初判

本輪未發現必要技術缺陷、需要修正的首次使用時序／範例閉包缺口或另外提出的可選項。這是靜態技術整頁審閱，不是程式實測、盲讀、實際目標讀者實驗或版面驗證。

## 獨立性與範圍

讀者背景維持基本 Python 加實際已讀第 1 章；完整讀取新 2.1–2.5 正文與其所有引用來源。未讀 2.6–2.8 的正文或 Rust source，不把它們當先備；封存前未讀作者 ledger、first-reader／continuity／audit、root 結論或歷史問題。root 僅提供範圍、ready 狀態與 1.7 的 heading-only 變更。作者曾執行 targets 的訊息未當作本輪執行證據。

已查完整 source、manifest、lock、引用 section 的 start/end markers，以及每條公開命令對到的 package／target。完整 SHA-256 與靜態預測存於同名 JSON。依 root 要求本輪未使用 Cargo、rustc、Clippy、tests 或 fixtures。

## 逐節與主要概念核對

| 單位／原文依據 | 身分與機制／性質 | 需要／用途 | 操作、例子與技術結論 |
| --- | --- | --- | --- |
| 2.1:3、7–24 比較與 bool | 六個比較運算先給其成立條件，得到已教 bool；18 行分開 = 與 ==。 | 問的是小計是否達 300，而非另一個金額。 | `comparisons.rs:20–29` 對恰好300的三對輸出為 true/false、false/true、false/true；24 行用299檢查門檻差異，與程式和規則一致。 |
| 2.1:28–36 邏輯組合 | && 要兩邊true、|| 至少一邊true、! 反轉布林值；兩側bool與括號組合在使用前說明。 | 基本材料非負與是否有商品是不同檢查，不能把數量0誤當有商品。 | `comparisons.rs:33–38` 的 valid=true、empty=true、has_items=false 正確；Python鏈式比較不能直接照搬的說明正確。 |
| 2.1:40–65、102 短路 | 左側先執行；&& 的 false、|| 的 true 會跳過右側。positive_count 是已完整定義的有追蹤輸出 helper，->bool 的型別角色由48行接回第1章。 | 要辨認哪些工作真正發生，不能只看最後bool。 | 四個完整呼叫同時涵蓋兩種會跳過／不會跳過的路徑，只有B、D印「檢查數量」。規則來源、函式source和全頁輸出相符，未把 either_check 說成最終出貨政策。答案 can_ship 是三項&&、needs_preparation是!ready，真值預測正確。 |
| 2.2:3–19 if expression | 條件選一個block；沿用已教block尾端值，使if選出數字；各回傳結果的分支需相容數值型別。 | 根據小計是否至少300選運費0或20。 | `conditions.rs:8–11` 確為if expression；小計160運費20、300運費0，外層分號與分支不帶分號的角色正確。開頭明定非負小整數和不溢位範圍。 |
| 2.2:21–39 bool 條件與 return | if 條件不能以i32的非零值代替bool；return0立刻結束這次函式，不只離開if。沒有else時false路徑繼續後文。 | 空訂單答案已知為0，不必再計算小計／運費。 | `if_number.rs:1–6` 完整main的if count是i32而非bool，E0308及預測訊息一致；`conditions.rs:3–5` 對count0提早回傳，後面的乘法與運費不執行。診斷未實際執行。 |
| 2.2:43–49 索引保護 | if在讀取前先比較usize index與len，兩個分支選不同顯示工作。 | 回答第1章超界情況，選擇不讀無效元素。 | `show_selected_price` 完整定義於同檔26–34行。index3/len3使條件false，沒有讀prices[3]；index1則選80。沒有引入未教helper/API。 |
| 2.2:79–99 else if | 只有前條件false才檢查下一個；第一個成立後不再檢查後續分支。 | 三段運費要先處理最高門檻，避免500先被>=300接走。 | `conditions_answer.rs:2–10` 完整shipping函式先500再300；299/300/499/500輸出20/10/10/0，邊界的選擇確實對應含等號規則。 |
| 2.3:3、7–23 for 與累積值 | array依序提供本次i32值，不是索引；外層mutable total跨次更新，當次price限body使用。 | 不想每新增一項就再手寫一次索引與加法，保留逐項處理規則。 | `for_range.rs:13–18` 的三次更新是0→120→200→400；表格相同。重設total若放body會失去跨次累積，說明正確。改四項時明說固定array型別／資料需一起改，沒有暗示array可變長。 |
| 2.3:27–51 range | ..含起不含終，..=含兩端；for逐次取得整數，非先建立完整array。 | 材料本來就是連續整數，不需手寫一份array；是否含終點由要處理的數字決定。 | range_sum完整定義於同檔2–8行，0..3總和3、3..3零次為0，0..=3總和6；start>=end的半開整數range無項目正確。答案sum_three全定義，兩組結果400、0。 |
| 2.4:3、7–32 while | 每次body前檢查；body完再檢查，一開始false可零次。remaining/batches在外層保留，processed每次建立。 | 7項工作每批最多3，最後一批只處理剩餘，需要每批後按新剩餘量決定是否繼續。 | `while_work.rs:2–12` 正是7→4→1→0，批次1/2/3；最後false檢查不多算一批；work0不顯示批次且回0。table與完整stdout靜態相符。56行公平區分while與for的可見安排，不宣稱for做不到。 |
| 2.4:60–72 一般化批次上限 | capacity決定每次processed的上限；work>=0、capacity>0和小整數約定在答案前明示。 | capacity0會使remaining7減0而永遠成立，正值約定支撐停止。 | `while_work_answer.rs:1–14` 更新不超過remaining，於約定範圍內會前進且不減成負數；(7,3)/(6,3)/(0,3)分別3/2/0正確。 |
| 2.5:7–27 loop／break與交出值 | loop無入口條件，自body決定停止；break終止最近迴圈，break值讓loop expression交值；return則離開函式。 | 在body中間明確停止，以及在停止時把找到的數字交給外層。 | `loop_control.rs:3–21` remaining2→1→0後break；current依序1/2/3，found=3。loop結果的let分號、出口型別及for/while不支援break數值的說明正確。13行沒有把loop說成while無法完成的新能力。 |
| 2.5:31–39 continue | 跳過本次body餘下工作；for取下一項，while重查條件。 | 略過某個數字而保留後續項目，並辨認可能跳過更新而不前進的while路徑。 | `loop_control.rs:25–32` 跳2仍加3，總和4；改break只加0/1，總和1。39行的remaining1例子直接交代更新被跳過的實際影響，沒有要求未教控制API。基本答案先檢查正整數1再2，first_even2，1..=5跳3總和12。 |
| 2.5:63–101 巢狀與labels | 先定兩個range的列／格對應；無標籤作用於最近迴圈，'search:宣告名稱，break/continue後的標籤指定外層。 | 從內層結束整個搜尋，或只開始外層下一列，需把離開／繼續的層次說清楚。 | `loop_labels.rs:3–26` 搜尋在row2/column2停止，共3+2=5；換列例每列只有column1被計數，三列共3。正文與source一致，單引號／冒號的語法在片段前交代，沒有新的2D容器或未教型別。 |
| 2.5:115–125 labels答案 | 兩個獨立範例都改為三列四格；搜尋條件不變，換列改column3。 | 計數相等不代表走訪同一範圍，要求辨認控制去向。 | `loop_labels_answer.rs:2–23` 搜尋是4+2=6且在第二列結束；換列是每列2格×3列=6。125行明說兩者差異，未靠相同數字宣稱同一工作。 |

## 完整來源與命令閉包

所有12個正文成功命令都選 `--manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example <本頁target> --locked`，target名稱對應實際檔案。package manifest只有已教name/version/edition，workspace/lock已於第1章介紹。`control-steps`沒有src/lib.rs或build.rs，也沒有外部依賴；所選sources沒有mod/include、attributes或Clippy allow。每份example含自己的main與所有被呼叫的自有函式，兩份labels程式也各自完整。

compile-fail fixture放在fixtures而非自動發現的examples目錄；它不會成為成功target的隱含程式碼。唯一.expected標記是E0308，與i32作if條件的靜態錯誤一致。全頁短片段引用的檔案與section起訖markers均存在；沒有只給答案卻漏設定、helper或被呼叫函式的情況。

## 1.7 heading-only 追加

依root明示的標題變更，窄範圍核對H1改為「1.7 計算機基礎：程式、記憶體、stack 與 heap」。把當前檔案第一行換回原標題後，恰好重現自己round01-tail原封存hash，證實其餘正文未變；原有steps sources與五張SVG hash亦未變。已保留原初判並在round01-tail.md/json追加新hash，沒有重查整頁或重跑程式。此項由root提示，不算獨立新發現。

## 未驗證與邊界

本輪無任何Cargo/rustc/Clippy/test/fixture或browser執行；上面的stdout與診斷為依source的靜態預測，不借作者已跑命令改稱本輪實測。沒有評估後三節、全workspace檢查或未開放章節的先備履行情況。這五頁沒有必要圖檔；目前列／格範例已有明確枚舉、條件與計數對應，靜態技術檢查未發現必須新增圖才可理解本次控制去向的缺口，實際閱讀／版面負擔仍交由相應角色驗證。
