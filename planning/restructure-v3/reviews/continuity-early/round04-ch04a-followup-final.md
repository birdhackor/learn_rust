# 4.2 已採納兩項窄修：source＋實頁複查

檢查時間：2026-10-15T07:29:03.438206+00:00。C4a-01、C4a-02均關閉；保留原round04-ch04a獨立初判。

C4a-01：現4.2第9行先說「match 本身是 expression，結果來自被選中 arm 的 expression」，然後15–16行才第一次出現shipping函式。21行仍保留具體i32／函式尾端的推導。1280×800、390×844實頁都看見相同順序，DOM也確認結果規則先於第一code。原有文字前移，所有選定Rust未改。

C4a-02：第7行改為 `**pattern**（模式）描述…`。兩個實頁viewport均正確顯示strong英文pattern，中文解釋完整，article段落無可見原樣**。兩張實頁截圖都已實際查看：`round04-ch04a-visual/desktop-followup-match-result-and-pattern.png`、`mobile-followup-match-result-and-pattern.png`。

本次source差異僅4.2上述strong邊界與既有結果句前移。與初始4a相比的workspace/lock多出data-steps/data，已在原4a end-config記錄，並非此次修正；未讀第五章稿。初始4a其他已選source、manifest、default main均相同。來源hash、逐項差異見 `round04-ch04a-followup-source.json`，實頁URL／時點／HTML hash／viewport／DOM證據見 `round04-ch04a-visual/followup-metadata.json`；整合JSON為 `round04-ch04a-followup-final.json`。沒有執行Cargo/build/tests，沒有更動教材或原初判，preview檢查不等同最終strict site驗收。
