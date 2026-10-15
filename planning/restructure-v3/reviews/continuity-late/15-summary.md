# 第15章獨立銜接初判

六節全文已讀，15.6分1–290與291–579行讀完；所有Rust／C／header／build.rs／manifest／完整helper／同檔測試／fixture expected／example／答案已讀。逐節四題、實源SHA256及rendered全檔嵌入比對見 independent-15-initial.json。未讀作者15章ledger或finding，非首讀盲測。

15章必要問題：無。可選問題：無。

GAT保留結果型別與當次borrow，variance從契約替換方向展開；raw pointer把safe建立與unsafe存取責任分開。safe split與readonly wrapper逐項建立範圍／來源／獨占及Send／Sync理由，並保留測試不能證明全部soundness的界線。C基礎從零教宣告／pointer／len／NUL／malloc free／編譯連結；FFI再以scalar、buffer、label三快照逐步接C契約，最後才載完整Greeting owner與六項測試。

七張原位圖皆實看1280×800及390×844，來源／borrow／範圍／ownership與編譯階段可對應；手機必要文字可讀。另開15.5手機C長答案，橫捲正常。所有完整來源在frozen preview均有嵌入。未重跑Rust／C。

讀15.3「第一次array→slice coercion」時自發回查13.2，新增CL-13-ARRAY-SLICE；這是13prefix後續查漏，另封存原始addendum，不回寫13prefix初判。root已採納，將另存修後窄複查。
