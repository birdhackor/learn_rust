# 第17.8節獨立技術初判

封存：2026-10-15T09:46:07.689Z。必要問題0；額外可選問題0。

按current8ff6ed...全文重讀、17snippets、16owned檔與完整performance path dependency、5條命令已靜態核對。30份repo與1份dhat0.3.3來源hash、原JSON／frames／stdout／stderr／reproduction比較見JSON。早先讀過舊正文但未量當時hash；root僅版本通知後按current形成初判，未看17.8diff或他評作提示。

static／global allocator／unit struct、Profiler builder與get／Drop、debug=1與release的角色都在首用前說明。repeat實際完成100次同工作，追蹤前材料仍存在，snapshot只是owned數字；Drop寫檔失敗與顯示正確不是同一成功條件。

primary三份profile SHA與歷史record一致；6份profiling Rust/manifest SHA全同，performance載入來源與17.7已核版本同。clone的25600blocks／326400bytes／peak15／end0、borrowed0PP、solution6400blocks／81600bytes／peak15／1600hits均對raw JSON與原輸出。callchain確有main:18→repeat:12→count_cloned:21→String Clone；兩圖來源與範圍正確。

數字只對本次歷史有限工作與追蹤範圍，不等CPU占比／全程零heap／通用配置次數。沒有本波Cargo或新profile，沒有開viewer；只讀JSON、source與SVG XML，不把record既有viewer主張當自己的操作。
