# 本地驗證紀錄

`check-all-rust-final.sh` 實際執行當時 `scripts/check.sh` 的完整Rust／C／fixture尾段，涵蓋入門workspace、兩個第12章lesson workspaces及根workspace；只加工作目錄、共用環境與節省磁碟的Cargo建置設定。stdout／stderr及當時2031份程式輸入指紋一併保存，程序退出0。網站建置與最後結構驗收另外記錄，不假稱這一支腳本已驗網站。

後續僅對 `scripts/check.sh` 加入結構驗收入口，Rust／C／fixture尾段逐字相同；2030份其他凍結輸入未變。本地嚴格網站建置及結構自測另已通過；全部獨立複核與修訂後複讀完成後，最終結構檢查實際通過：0 error、0 pending；結果見final-structure.stdout。Miri使用固定nightly-2026-10-01，正常純Rust路徑與指定Miri負例另行實跑；不在host執行UB。

visual-final-*只記真實瀏覽器抽查與已實看旗標，原PNG保留在工作區。1280／390兩種尺寸抽查新版導覽、舊入口、基礎圖、專案排程及停止圖與長收合答案。程式橫移的正確元素是code：390 viewport內client311、scroll803、scrollLeft實際由0到492；之前對pre的嘗試沒有橫移，原紀錄也保留。高圖可能超過單張viewport，不能將這些抽查說成root逐頁全畫面驗收；各章的獨立實頁報告另存reviews。

作者／獨立角色／AI逐段試讀與本目錄的實際程式執行不可互相代替。本地結果不自動表示GitHub Actions或Pages已完成；公開發布以實際workflow與線上頁面為準。

C18B whitespace-sensitive材料修正另以真正current HTML中的JSON code文字解碼，四組viewport/page都逐byte等於42-byte原檔，非空白行索引1／3／4。此為DOM文字驗證，不冒稱已自動操作OS剪貼簿。全canonical TXT／CSV引用另外盤點24處，只有這兩份有檔首LF與單空白行；其精確表示／逐行預覽已分清。原稿程式與fixture未改。
