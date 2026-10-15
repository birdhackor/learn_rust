# 第6章前綴6.1–6.2：獨立銜接與實頁初判

封存時間：2026-10-15T07:50:51.457101+00:00。scope只限已freeze6.1→6.2，接續actual1–5。未讀6作者ledger、technical/firstreader結論、歷史audit/issues、rootdispositions或6.3+/7prose/Rust；previewnav看見後文標題不當先備。本角色不是作者、technical、盲首讀，也非全書通過聲明。

**結論：一項必要的窄借用銜接，無新可選finding。** 初始18份frozen/sourcehash均保存吻合，末次重算選定body、Rust、fixture、package設定未變；globalworkspace/lock新增futureerror-stages，另存end-config，只讀其package-onlymetadata。

## C6a-01（必要／中嚴重度／高確定性）：首次整筆Debug顯示前，交代這次println只借讀

首次依賴在6.2:13–14內嵌的`debug.rs`：12行 `println!("整筆資料={summary:?}");` 顯示整筆owned摘要，接著13/14行又讀`summary.amount/count`做assertion。實際桌面／手機都呈現同一順序。

前文7行說「Debug提供供開發者檢查的文字表示」及`:?`，9行教derive產生trait實作，11行教每欄位也要有Debug。這些已足夠辨認Debug、用途與語法，但沒有交代這次格式化對summary使用哪種權限。第3.2:21/27/35已教owned參數交接會move；3.6:78提醒自訂Order沒有Copy；3.8:11–17的show接收&str，5.5:9也是先taken.as_str()借讀，不能由這些「傳入已借好的視圖」推導「直接寫ownedsummary的println仍只借讀」。查actual1–5沒有找到這項非Copyownedformatting規則。6.2:41正確說明後面的比較借讀，但不能向前涵蓋第一個顯示call。

建議在第一份完整Debug程式前補一句「這裡的println!只借讀summary來顯示；顯示後仍可讀它的欄位。」讓讀者用已有move／borrow模型解釋後面的assertions，不把新增Debug能力誤解為順便Copy整筆struct。不需展示Formatter、fmt簽名、macro展開或任何深層內部；Debug的動機、角色、derive與欄位能力已完整，這裡只補操作權限這一點。

## 正向證據

### 6.1

位置：3/5/9/11;21/23/29;33/39;43/45/47;53–60;64/66/68;89/95;99/105/115;119/121。

Independent need is two data shapes with one threshold rule; material contract nonnegative small arithmetic is scoped. Model/public fields and helper module path known before snippet. Charge defines shared amount→i32 method signature and no-body semicolon before trait; no promise of automatic implementation. implChargeforOrder/Bundle matches contract with differing body, no fake shared representation. T:Charge and &T privileges explained before generic body; table computes360 true and160 false. Dot-method trait scope/import and example crate root explained before external caller. Bound failure is complete and explicitly about generic definition guarantee, not business result. Two tests compare numeric/bool and inclusive300/301 boundaries. Answer adds DiscountedBundle360−80=280 with same generic threshold and old models.

Full charges and charge_support module incl2tests, package-only manifest, missing_bound fixture and fullanswer on samepage. selectedcharges/charges_answer load exactly this helper; no lib/build.rs/dependency. No hidden later traits loaded.

### 6.2

位置：3/7/9/11/25/27;35/37/39/41/58;62/64/70;74/80;84/86/92/96/114;122/124/139。

Concrete need is complete summary view and application-specific field equality. Debug format/derive/field support and unstable external-format warning are explicit. MissingDebug fixture follows named failure. PartialEq supplies ==/!=; derives compare allfields, enumvariant thenpayload; multi-derive and assert_eq! Debug/PartialEq roles explained before whole comparisons. Comparison borrowing is correctly explicit before retained originals. Examples isolate samefields, countdifference, variantdifference and samevariantpayload difference. String/&str content equality before textcode; sharedreferences compare borrowedvalues, notaddresses; Option shape/content and conditional trait support before Option<usize>/Option<&usize> direct equality/debug. Allstandard snippets have fullsamepage source. Answer builds two independent Strings manually and changes only second amount, retaining labels and original280.

No new generic equality/display operation is used before its trait ability. No deriveCopy/Clone assumed; no Eq/float/deepPartialEq theory needed for current ints/strings. The one necessary gap is initial whole-value Debug formatting access contract C6a-01, not missing trait definition/motivation.

## 實際閉包與實頁

已讀charge_support全部型別／trait／兩impl／泛型fn／兩tests，charges及answer、missing_bound；6.2debug、not_debug、equality、standard_equality三段＋完整main、equality_answer，以及minimalmanifest/defaultmain與兩.expected（E0599/E0277）。trait-stages實際沒有lib、build.rs或dependencies。所選target僅自身完整檔或同頁完整helper，不載後文正在寫的能力。每個fullinclude與actualDOM一致，named局部區段同頁有完整來源。未Cargo/build/Rusttests，未改任何repo教材、程式或設定。

兩頁1280×800與390×844，14張焦點截圖全部已實看，確切名單在JSON。trait契約、bound＋兩型別同一規則、跨module引入、Debug第一程式、derivedcomparison、reference值相等、Option形態／內容比較均已查看。兩頁actualimg皆0，所需對應由prose、實作body與三欄table建立，未發現缺少必要圖；mobiletable三欄可完整比較。沒有raw**，長程式只有局部code水平捲動，inlinecode即使換行仍保留符號。此輪無新視覺finding。

來源版本、末次變動與hash見round06-ch06a-sourcehashes.json；actualURL／時點／HTMLhash／viewports／fullinline比對見round06-ch06a-visual/metadata.json。本角色只審已授權前綴及必要實頁關係，未聲稱每個長頁pixel或最終strictsite驗收。第5修後preview已收到通知，遵照先封本輪獨立判斷，再另做第5窄followup，不混入本初判。
