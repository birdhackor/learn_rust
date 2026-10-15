# 第 11.6–11.8 節獨立技術初判

封存：2026-10-15T08:40:27.029Z。所對新稿必要問題 0；可選待修問題 0。

逐頁核對 dyn／Error／Rc、Weak，以及全部 65 個 snippet、15 個 run target、3 個 selected test 命令（10 tests）、14 組完整負例／expected。73 檔 SHA-256、完整載入 closure 與22份helper片段bytes比較見JSON。封存前未讀作者驗證、peer、reader或歷史問題，未讀11.9以後補先備。

- dyn 的 DST／unsizing、borrowed與owned型別、簽名Box預設static和reference object bound、RPIT單一具體結果、closure共同呼叫、dyn compatibility及where Self:Sized例外均與實際source一致。vtable圖不冒稱精確ABI，三階段費用200／220／240與工廠結果正確。
- Error的Debug+Display、預設source、外層reference與內部static界線、borrowed context及Box From／?正確。報告器明定有限無環鏈，逐層借原因並交回owned String；abc／-1／101與訂單7答案的完整訊息、fixture原因及型別相符。
- Rc計strong owner而非一般reference；clone不Clone T、Deref無DerefMut、Weak upgrade取得新的strong、T與allocation清理分離均準確。共同尾段200／160及strong3→2→1、temporary upgrade的1→2→1、最後strong清理及Weak None均對source。cycle圖明定是假設關係，沒有偷用未教RefCell程式。
- no-lib stage只載各自完整target-local helper；complete階段才載全部功能和tests。沒有早期命令載入後文。14組負例的expected碼／字串與完整source靜態吻合。

版本時序：第一次全文讀取11.6:7仍是「非負小數」，但稍後08:37:35 UTC來源快照已含作者的「非負小整數」修正，hash為ec492344…dbb76f，與父任務後來通知的新hash一致。未量到舊頁hash。這一項原本只有字詞精確度影響，現在已修；明記作者/root歸因，不算自己的新增獨立finding，也不重寫既有封存初判。

沒有新增Cargo/Rust執行；輸出、tests及fixture診斷皆為靜態核對。4張SVG只檢查XML/source，不稱browser視覺或讀者實驗。
