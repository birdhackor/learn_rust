# 第9章中段獨立技術初判：9.3–9.4

8.3–8.5初判封存後，依實際1–7、8.1–8.5及9.1–9.2讀9.3→9.4，核對全部actual source、manifest、fixture／expected與iterator-next SVG原文。8.6、9.5之後、10／11未讀；封存前未看作者verification、continuity、reader、其他review或known findings。

**必要0項、選讀0項。** 本輪為靜態技術推理及hash／snippet／XML核對，沒有執行Cargo、rustc、Clippy、Rust測試或瀏覽器。

10個run目標全部是no-lib intro-iterators完整單檔，所有型別／impl／helper／main都在當頁實讀；無dependency、build script、hidden library或後段adapter。25個snippet引用與起訖存在，完整三欄manifest在本節提供。沒有test／all-targets／Clippy／fmt命令，既有assertions檢查本頁材料。

9.3先分APIT由caller提供、RPIT由函式實作選擇；匿名參數不能以turbofish指名、opaque仍有具體型別、同函式分支須同一型別，皆與實際source吻合。不同capturing closures為E0308，改成同一closure body內分支正確。evaluate只呼叫一次卻採Fn的對照目的有明說，並沒有錯稱Fn為必要最低能力。

2024 RPIT implicit capture（含省略輸入lifetime）與closure實際保存reference的capture，在首次借用回傳工作前定義並區分。字串來源仍須支持回傳工作的使用；有opaque介面不取消E0597。precise capture的use<'a>／use<>、與use statement的差異及+用途，也在首次source前說清。constant工作刻意保留未用輸入的對照能顯示型別契約overcapture；明確use<>後才可帶出內層。正文正確限定不能排除真實借用，具名型別參數現行仍需全列，不把只有lifetime例子推成任意省略T。FnMut工廠按值保存計數，1→2→3與另份10→11正確。

9.4先建立Iterator／Item／next mutable receiver、Option結束與非所有iterator永久None的契約，再列Countdown實作。lazy限於由next逐項交出、沒有預建整份清單，不說初始化零成本。3→2→1→0的state與Some3、2、1、None／再次None相符；zero／negative也直接None。

IntoIterator與IntoIter associated bound、按值self對owned或reference的不同作用、for先建工作再next、Iterator本身的blanket轉換都在實用source前。NumberList使用已解釋的std::vec::IntoIter，泛型消費端只需Item=i32；三種來源合計6，空來源0。StepDown先next5再for只得3、1，能檢查工作沒有重建。

三條Vec<String>路徑在table中先交代owned／shared／mutable與Item，才顯示source；for mut item與&mut元素的差別有解釋。讀原值、追加!、再owned移入新Vec而非clone，輸出與ownership相符。直接&Vec／&mutVec的IntoIterator、原值1／2改成11／12，以及E0499／E0382反例均正確。未偷用後面的collect／filter／map等API。

5份compile-fail與非空expected分別E0308、E0597、E0597、E0499、E0382；與實際失敗契約相符，未宣稱重新跑2024 compiler。10個run輸出及self-checks靜態一致。SVG四個時間框標示同一Countdown推進、狀態與Item分開，title／desc／alt及箭頭吻合；XML可解析，未做視覺驗證。

同名JSON保存33份SHA-256、25個snippet、10條命令、逐target完整閉包與四題依據。此結論限9.3–9.4，並非後章、整站或讀者實驗驗收。
