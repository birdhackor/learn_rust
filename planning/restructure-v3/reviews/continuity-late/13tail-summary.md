# 第13章後半獨立銜接初判

13.5–13.8 全文、所有 snippet 完整來源、實際載入 modules、fixture／expected、manifest 與同頁答案已讀。完全相同 helper／expected 用逐檔 SHA256 比對保留內容去重證據。初判與實源 hash 見 independent-13tail-initial.json；未讀作者 ledger 或其他13tail finding。

必要問題：無。可選問題：無。

channel 以端點關閉、待收訊息與 join 分別處理資料／收尾，bounded channel 的等待循環與容量0停止有足夠機制說明。Arc／Mutex 分開共同存活與獨占存取，Cell bound、guard Drop、部分業務更新與 poison 修復均能依已教契約判斷。RwLock 先教可並存 reader，再用窗口與 downgrade 分開重取／轉換權限；poison、平台優先權與取得順序沒有過度保證。atomic 分開不漏單次更新、工作完成與跨位置發布；P1→P2→R1→R2、SeqCst 矛盾循環及 stronger ordering 仍不合併讀改寫都有具體推理。

自有 frozen preview 的五張原位圖均實看桌面1280×800及手機390×844；必要標示與相鄰說明可對應，手機可讀。另實開13.8手機長答案；同頁完整來源已比對 rendered code。未跑Rust，未稱全書首讀盲測。
