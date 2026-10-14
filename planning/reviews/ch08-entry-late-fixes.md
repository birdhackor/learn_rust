# 進入第8章前的晚期正文補讀

這是既有第6章背景的指定修後補讀，不算第6章primary盲讀QA，不改原entry／初判。此前已實讀6v6全50及6v8的003-002／004-002，此輪不重讀它們。

先完成並封存第7章005-006有提示followup，才只讀frozen/ch06-review-fixed-v10的006-002，再006-004 BODY。使用ch06fix2順序gate／closed遮蔽與save_visible_record.py，每單位先存Q1／Q2a機制／Q2b用途／Q3／Q4、引文／未知／hash，再下一段；不看其他正文、peer／author／source／答案。

兩段保存封存後，才開始尚未讀過的第8章正文；前文最新版本只有此實際局部補讀，不宣稱6v10全章實讀。未實跑／看整頁。

## 原始追加紀錄

### 006-002 — adaptor按值交接仍依Copy／move

Q1：adaptor新工作按值接原iterator、保存它與callback；map Item→FnMut→新Item／lazy推進規則沿本人6v6已讀。
Q2a：新限定「不主動呼叫 clone；交接仍依來源型別的 Copy／move 規則」，不是說任何iterator都絕不複製值或必移走。Q2b：原需辨認逐筆工作交接→按值接收與Copy／move分開→不把未主動clone誤讀為一律source失效。
Q3：此例slice iter的&i32經*Copy後乘二，new Item i32；visits mutable capture、最後工作後讀與交錯輸出既已教，不新增執行動作。
Q4：按值／Copy限定在adaptor首次說明加入，原code及預期仍同；新理解只依這段正文，不讀peer或答案。
背景追加：前次v6摘要未特指source Copy，此後採本較窄契約，不改舊初判。無必要未知、無圖／closed fold，未實跑。

原始SHA-256：a4f30717a33e9cc2177d96f3cca703521b7e1b2d8a073eeeed57c815cf6a8613；visible SHA-256：a4f30717a33e9cc2177d96f3cca703521b7e1b2d8a073eeeed57c815cf6a8613。

### 006-004 — Vec<T>收集能力與Debug／PartialEq前置

Q1：collect依目標FromIterator建結果，方向與IntoIterator分開；本次明教標準Vec<T>從T元素依序建立，i32只是具體例，不代表Vec只能收數字。
Q2a：「標準 Vec<T> 能從 T 元素依取得順序建立」要求Item與T一致；Vec的Debug／PartialEq仍依元素能力，說明移到fixture格式使用前。Q2b：原需讀每步Item及目標→先給一般建立／顯示條件→可辨&i32不能自動變i32，並讀新元素目標而不靠答案。
Q3：原filter&&i32→map&i32/i32→Vec<i32>與NumberList helper／turbofish皆前次實讀；Vec若收reference仍只保存reference，可由本一般契約＋6.7 Vec<&str>實讀最短推出，不取被借資料owner。
Q4：相鄰表與「collect…不是先建立…完整中間清單」維持原pipeline關系；本人先前已看v6同原位圖，本輪僅BODY／表核對，未開v10 SVG或聲稱其bytes／整頁驗證。
背景追加：兩段新限定都足用，沒有新必要未知；這不是第6章primary QA，closed答案／source／執行未取。

原始SHA-256：be52997350e070801727535bc54be1f36afd7644d13228d715f20745a98fd526；visible SHA-256：be52997350e070801727535bc54be1f36afd7644d13228d715f20745a98fd526。

## 指定背景封存

第7章005-006局部followup先封存後，才依序實讀第6章v10兩段；現在封存新增Copy／move交接限定與Vec<T>建立／元素能力的正文背景，才進入第8章。沒有重讀6v8兩units或其他章內容，不回填舊初判。新增必要未知0；未實跑、取source／答案或重看圖。
