# C6a-01 窄source＋actualpage複查

2026-10-15T07:54:48.439510+00:00：C6a-01關閉；原round06-ch06a初判不動。

6.2首次Debug段現在先說「這次格式化只借讀summary，沒有取得它的ownership；顯示後仍能讀取它的欄位」，再出現第一份完整debug.rs。對照原frozen，只有這句基本操作權限補充；所選Rust、fixtures及package設定未變，沒有fmt內部或新語法。

actual1280×800、390×844都已實看新截圖，借讀／可再使用規則在第一程式前；DOM亦確認順序。來源diff/hash見round06-ch06a-followup-source.json，URL／時點／HTMLhash／viewport／DOM在round06-ch06a-visual/followup-metadata.json。兩張desktop/mobile-followup-debug-borrow-before-code截圖均實際查看。全域workspace的後文metadata變動另列，不是本修正；沒有Cargo/build、repo編改或重寫初判。
