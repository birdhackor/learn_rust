# 第15.1–15.2節獨立技術初判

封存：2026-10-15T09:33:19.014Z。必要問題0；可選問題0。

全文、30snippets、5個run與1個example test（3函式）、5組完整compile-fail／expected、2SVG source已靜態核對。35份SHA-256、6份snapshot bytes比較及逐target閉包見JSON；未讀作者驗證或其他角色紀錄。

GAT以本次receiver lifetime選View，必要Self:'a的角色正確；slice與Ref guard保留各自借用／Drop要求，generic helper沒有假設未知View必有Deref。test所用is_err先教，完整可變視圖答案修改原Vec後再借，總額260吻合。

variance分清reference借用lifetime與被借T；&mut的前者covariant、後者invariant，Cell寫能力造成不變，function pointer輸入逆變／輸出協變均對actual helper。另建Cell的答案沒有把原容器改成協變。全部負例仍由type/lifetime檢查拒絕，沒有執行失效存取。

no-lib各target只載自己的明示helper；早期快照不提前載tests、callbacks或unsafe。沒有本波Cargo/rustc或visual實驗，SVG僅讀XML/source。
