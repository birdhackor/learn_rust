# Chapter 3 銜接與實際頁面：獨立初判（round 03）

角色為獨立銜接／頁面審閱者，不是作者、技術審閱者或首次閱讀者。依實際已讀1–2章接續3.1–3.8，讀取當下引用的完整source、helper、答案與失敗fixture。未讀第三章作者ledger、technical／firstreader結論、舊audit或root新問題；不宣稱盲讀或全書驗收。

初判保留 **1項必要的局部順序調整，無另外提出可選內容擴寫**。五張圖的必要對應在兩種尺寸均可辨認；完整程式沒有偷偷使用String::from、clone、derive、泛型helper或其他未教能力。

## 版本、來源與限制

初始56份來源hash保存於`round03-ch03-sourcehashes.json`。閱讀末發現3.2、3.3 Markdown及reference-owner.svg改動；初始版本都保存在`round03-ch03-frozen/`，完整hash已驗證。3.3初始兩檔由改後來源還原先前讀到的字句，完整hash精確相符，不以記憶近似版本代替。

版本分開如下：

- **初始文字／source閱讀：**3.2的Copy基本敘述在後段；前面的圖已明示i32複製、原count仍可用、修改另一份不影響它。3.3圖底有`*reference 找到這筆 Order`，正式dereference說明在:35。
- **3.3實際頁面視覺：**看的是變更後圖／alt：底部改成「借用同一筆Order」，不提前放dereference標記。實際伺服器image bytes的SHA-256與當時改後來源相同，兩種尺寸均核對；沒有聲稱看過原3.3圖的實際頁面。
- **3.2後續來源：**末次diff顯示把既有Copy基本敘述提前到圖前，並刪後段重複句。這是被動偵測來源更新後的版本記錄，不能回算為本角色初判發現。其他四張圖及Rust來源未變。

root曾通知有一張未指名SVG正在窄修，但未給具體問題；本角色照既定逐圖檢查路線，並保存來源／頁面時點。沒有把這個通知算獨立發現，也沒有據此補報原未列的問題。

`intro-ownership`完整manifest只有已教的name/version/edition；沒有library或build.rs，預設main只顯示文字。各例子、public_model的model/mod.rs、章末LabelledOrder都在當頁內嵌；失敗例提供完整可另放入獨立專案的程式。不跑Cargo、build或測試，未修改教材或共用設定。

## C3-01：自動reborrow的說明應在首次依賴它的呼叫片段前（必要；增加理解負擔；信心中等）

- 原位置：`03-05-lesson.md:63–71`。實際首次依賴點為`ownership/examples/reborrow.rs:18–20`，calls片段：

  ```rust
  add_one(&mut *parent);
  add_one(parent);
  println!("parent 最後讀到={}", *parent);
  ```

- 當時已教：3.2:11、19–25說明ownership交接與值參數；3.4教`&mut`／`*`；3.5:55–61教明寫`&mut *parent`的短借用，以及child最後使用後parent再使用。:63在片段前只說「函式參數也能收到這種較短的借用」。
- 片段之後:69才具體說：「第二個呼叫的參數型別已明定是`&mut i32`；在這種呼叫位置，Rust可以自動reborrow……之後parent仍可使用。」:71再說mutable reference不能像i32一樣Copy，一般`let next = parent;`會move。
- 已教部分：手動短借用、原數字仍由count持有、暫時借改後收回使用權的需要，都已充分交代；完整小節能說清楚兩種呼叫的結果與差別。不是缺少reborrow動機，也不要求提前完整教所有自動轉換或型別推導。
- 最小缺口與實際影響：讀者第一次看到`add_one(parent)`後還讀parent時，尚未取得「這個已知參數型別的呼叫位置可自動建立短借用」規則。只看程式可能把成功重用parent解釋成mutable reference也會Copy；緊接下文才把這個理由排除。本輪要求先解釋再讓程式依賴新能力，故保留為必要的局部順序項，而非主要概念阻礙。
- 最小修正：將現有:69的兩種呼叫解釋移到calls片段前，以「下面第一個／第二個呼叫」銜接。:71的move對照及失敗例仍可接在後面；不增加新概念或例子。
- 實際頁面證據：`desktop-auto-reborrow-calls.png`與`mobile-auto-reborrow-calls.png`，1280×800、390×844都確認先展示三行程式，再出現自動reborrow與非Copy解釋。這是可合理預見的理由誤接負擔，非真人首讀實測。原報告封存時未修。

## 實際依賴與正面證據

| 閱讀單位與首次需要 | 角色／機制／使用規則依據 | 需要／用途與例子關係 | 判定及可待教範圍 |
| --- | --- | --- | --- |
| 3.1欄位型別、實例、欄位讀寫、跨module資料模型 | :7在struct定義前教自訂型別與欄位；:13在實例前分定義與具體值；:21在讀欄位前教點號；:25在更新前教持有實例的mut；:59在pub模型前教型別與欄位各自公開；:69在public_model/main前教空`{}`按expression位置格式化 | :3從tuple位置約定導入命名，不改120×3；:19明說欄位列出順序換了仍按名對應；:31／55分已保存的360、新計算480與資料表示的用途。沒有宣稱tuple算不對。 | 足夠。helper完整顯示，沿用2.6mod.rs與crate路徑；答案assert只用2.7能力。 |
| 3.2owner、scope結束、move、Copy、參數接手與owned回傳 | :7–11先交代管理與交接規則，:19在完整Order參數前說接手；原圖下半部已給i32複製的最小角色與獨立值關係；:43–45在完整例／後續推理前正式分i32與自訂Order；:81在答案前教mutable參數 | :3先問交出後原名稱能否用。圖上first失去使用、second接責任；下半count3仍可用、another_count3→4不影響count，正好對照同樣指派在兩種型別下的差異。:11不把move當實體heap搬運。 | 足夠。初始圖已解釋Copy基本行為，不只出現名稱；不需要提前trait實作、機器bytes搬動或String清理。後續把文字敘述提前是版本更新，不算本初判補出的缺口。 |
| 3.3reference、borrowing、shared、&建立vs型別、dereference與自動欄位跟隨 | :7–11先定reference角色、&的兩種位置、共享讀取與原資料有效；:19在函式前教&Order與欄位自動跟隨；:25在main片段前教&order；:35在dereference程式前教*與Copyi32；:41–43分*與乘法、括號範圍 | :3明說計算相同，但呼叫者仍要訂單，借讀免先交出再收回；:31分reference用完與清理原資料。圖只是一份Order，reference不是第二筆訂單。 | 足夠。原圖的*reference只給找到資料的預告／最小角色，正式操作在需要i32讀出前說明；未將其列必要缺口。實頁看的是刪去提前標記的新版圖。reference有效區間深度可待3.5。 |
| 3.4mutable reference、原值mut與reference binding mut | :7在片段前定獨占借改與&mut建立／型別；:17在數字片段前說先*找到原值再改；:23分不改reference binding；:46在另一完整例前教let mut reference重新指派借用對象 | :3需要修改同一筆訂單又不交出；Order數量3→4單價不變，數字*reference3→4；另一例reference從first3換到second8但原值仍3/8。兩個mut對象以有意義對照呈現，:62收束。 | 足夠。shared寫入失敗可由既有借讀限制預測，無需interior mutability；本章限制明確落在普通欄位。 |
| 3.5lifetime、scope、last use、衝突、reborrow、dangling | :7–11分名稱可見、值結束與reference有效區間；:17–23用last use接續修改；:37、45解兩種衝突；:55–61教明寫短借用；:69–71教自動reborrow與mutable reference的move；:79–85解內層值與外層reference | :3需要先兩份shared讀再改；失敗例把讀移到改後，從同一值／同一借用區間解釋衝突；兩份mutable不是單純因數量2而錯；dangling與owned數字回傳對照直接回答要留下8的需要。 | 除C3-01的局部先後外足夠。無分支程式的last-use限定已明說，不將lifetime一概等於大括號或秒數；不需先教annotation。 |
| 3.6impl、Self、三種self參數、associated function與欄位簡寫 | :7–9在read片段前教impl／Self／method／&self；:19在write前教&mut self；:29在take前教owned self；:39–41在new前教無self與Self literal／同名欄位簡寫；:47在完整程式前教Order::new是自定名稱 | 將已知獨立函式的第一參數契約放到method，原來借讀／借改／接手工作不變。完整main由new→read→write→read→take，160／240及tuple結果對回同一訂單；:35不把方法名當ownership規則。 | 足夠。是操作與組織方式的教學，沒有宣稱獨立函式不能完成。move_from_borrow由非CopyOrder與借讀不接手規則預測，方法簽名才是修正位置。 |
| 3.7slice表示、range對應、slice相對索引、&[i32]迴圈與mutable slice | :3、7先說連續借用／位置與長度；:9在圖前給1..4；:13教selected索引與len；:17在sum前明說每項&i32及*；:31在完整程式後續重用selected前教slice reference Copy；:35–44給端點、合法空段與界線；:84–86在答案前教&mut slice及每項&mut i32 | :3需要同函式處理中三項、全五項、另一兩項；圖保留同一列值而換索引，selected0=prices1的120。完整程式給380／500／0／200，不必為每個array長度重寫。mutable答案只改原1/2兩項、空段不改，借改完成後才讀原array。 | 足夠。沒有要求先學Iterator trait或所有slice底層結構。首次selection片段在傳selected後不再用它；Copy的明說位於完整程式真正重用selected之前，未誤報成同一種時序缺口。 |
| 3.8str／UTF-8／&str、owned String、new/push_str/as_str、String move及帶文字模型 | :7–9先給文字編碼到bytes的角色與&str型別；:21–23在build前教String管理資源及三操作；:45在move前說非Copy；:93–95在LabelledOrder完整source前說String欄位、owned輸入、append與into_label | :3先給逐步組成文字與只借讀顯示的不同工作。Hello,Rust與追加!結果解讀、兩個指向同一文字的角色圖，支持不複製owned內容；LabelledOrder把String移入／借改／整體接手取出文字，外層saved_label可再顯示。 | 足夠。沒有String::from／clone／derive等未教助手。UTF-8在本節只做整段顯示與追加，不需要切中文字的bytes／char界線；已明確待5.3。選讀Python/JS不構成主文先備。 |

## 五張圖的實際頁面證據

使用8097最新預覽，八個Chapter3 URL皆以1280×800與390×844載入，保存首頁截圖及metadata；五張必要SVG都在**實際頁面原位**目視。沒有將尚未目視的其他首頁截圖算全長版面通過。預覽為partial nav，非final strict site。

1. **`docs/assets/ch02/move-copy.svg`（3.2）**：兩尺寸可讀first灰色虛線不可用、second綠色現owner及120/3；責任箭頭向下標轉交。下半count仍3、另一份another_count3→4，複製箭頭與「修改它不會改count」都可讀。沒有把一樣的數字當同一份值，正文也明說非實際地址。圖340×562，手機全圖可讀。
2. **`docs/assets/ch02/reference-owner.svg`（3.3改後版）**：reference:&Order在上，owner order與唯一Order在下；&order建立借用的箭頭與「沒有接手訂單」一起建立角色。圖340×342，不需要放大；正文後續函式參數也叫order，但&Order型別與:19文字已明說這裡是reference，main的owner並未改成Copy。
3. **`docs/assets/ch02/borrow-scope.svg`（3.5）**：四個編號依序建立兩份借用、最後讀取、直接修改count、main右大括號。藍線只到2，灰線到4，圖內兩項legend直接區分借用使用區間與first／second名稱scope。圖只省略中間mutable borrow，緊接正文:27明說這項簡化，未用圖聲稱完整程式只有三個操作或owner一定改成4。
4. **`docs/assets/ch02/slice-part.svg`（3.7）**：array索引0–4左欄、單一組值中欄、slice索引0–2右欄；藍框只圈120／200／60及相對索引。prices1和selected0在同一列，原0的80與原4的40在框外；下方1..4／長度3／上界4不含可讀。這不是兩套另複製的值；正文:7、13直接建立對應。
5. **`docs/assets/ch03/string-view.svg`（3.8）**：greeting:String由上方「管理」箭頭連到唯一heap文字，show的text:&str由下方向上以「借用」箭頭連到同一塊。兩種責任與同一Hello,Rust內容都可追蹤。原圖400px在手機縮至約343px，24px字成約20.6px，必要詞、箭頭頭端和框內文字均可讀，不必放大。

實際看過的圖片只有上列五份，兩尺寸各一次原位context截圖，檔名`{desktop,mobile}-{move-copy,reference-owner,borrow-scope,slice-part,string-view}-context.png`。metadata另保存實際HTTP回應image bytes的SHA-256，與視覺當時來源核對。3.3是改後hash；其餘四圖與初始hash相同。

另實看兩尺寸的auto-reborrow calls與slice端點表。後者手機會將`&prices[..]`等片段分行，但寫法／原區間／內容三欄仍同時可對照；0項、2項及5項材料含義未被截去。1、4、6節沒有圖片，相關操作可由同筆資料的欄位變化與完整程式追蹤，未發現必補新圖的空間／資料對應。

## 封存邊界

本報告只代表3.1–3.8的獨立銜接及上述版面範圍。沒有真人逐段首讀、Cargo執行、測試／Clippy結果、全書後文或最終站點通過結論。初始版本與被動看到的修後來源／實頁分開，沒有把root後續修正回算為本角色獨立重現。原判斷封存後才能交叉覆核其他角色，後續新增結論另存。
