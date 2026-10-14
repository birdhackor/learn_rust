# 第 6 章問題處置

原首讀稿為ch06-complete-source-v6，全50單位已先封存。非作者技術與銜接初判仍獨立核同一原稿，未取得本檔或首讀問題提示；root先保存已授權的修稿，原稿／初判保留。

| 問題／來源 | 理解收益、最小修改與新增負擔 | 最終裁定與局部範圍 |
| --- | --- | --- |
| FnOnce／FnMut／Fn名稱scope；首讀必要局部 | 在首次FnOnce契約、第一份bound程式前說std::ops與prelude提供這三項，連到已教use規則，不列完整prelude清單 | 必要採納。只補名稱來源，Rust bytes不改；不將「可照抄」當成已能解釋何時引入trait。修後獨立局部覆核待初判封存。 |
| evaluate只call一次卻選Fn；首讀可選局部 | 用兩句說這組對照刻意保留同一種Fn能力以聚焦型別選擇方向，並承認單獨的一次需求FnOnce已足夠。保留參數與回傳比較只增加一個主要變化 | 可選採納。避免為此換bound與重做source，也不把Fn說成所有一次callback的最小要求；沒有新語法／例子或測試。 |
| collect失敗fixture先用Vec Debug；TECH12必要N1、CONT04必要N2 | root核對original006-004及actualfixture，確有println整份Vec在能力段落前。把原Debug／PartialEq段落移到fixture前，不增加新API或篇幅 | 必要採納。source不改；不能因fixture本來就編譯失敗而豁免首次使用。首讀未原始抓到，CONT04已獨立重現；原始發現狀態保留，交叉覆核另記。 |
| adaptor「不是複製原iterator」泛稱；TECH12可選I1 | root核對map的self接收與前文Copy／move，按值接收可能Copy，應與主動clone區分。沿用已有ownership規則即可釐清 | 可選採納。改成按值接收、不主動clone、仍依Copy／move；不加入Copy iterator probe或強制新範例。 |
| Vec FromIterator先說一般T；CONT04可選I1 | root核對原collect句、actual3.6泛型Vec與後續6.7的char／borrowed str／tuple收集；泛型容器與Item已可組合理解，原分級可選成立 | 可選採納。把原句改成Vec<T>接T再給i32例子，減少誤認只有數值可collect的疑問；只改一句，無新語法或程式。 |

以上五項只改必要位置的文字，不重跑未改程式。技術／銜接角色的原判、後續提示後交叉與公開發布QA另記，root的先備修正不代替獨立驗收。三角色初判均已封存；CONT04獨立重現Fn名稱與Vec Debug時序，TECH12原判沒有自行重現Fn名稱，首讀原判沒有自行抓Vec Debug；未改寫原始發現狀態。
