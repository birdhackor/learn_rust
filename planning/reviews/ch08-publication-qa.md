# 第8章發布候選檢查

2026-10-14。原稿ch08-complete-prerequisite-v8為10頁／66個BODY單位／150份來源。FIRST、非作者TECH與非作者CONT三份初判先各自保存，再取得同行與root處置；原SHA、分級與未知範圍保持。Root完整讀取三份原稿與後續覆核，不把先備核對當作獨立首次閱讀。

三份原必要項均為0。採用五項可選改善：Sized／Box標準名稱與prelude來源、mixed-Vec fixture的證據歸因、dyn圖標籤與箭線分離、回指7.6已教的bound組合，以及binary輸出的library定義主詞。處置逐項核對理解收益與最小修改，保留原可選性質；where／imports／Box Debug候選的否證路徑亦保留。詳見ch08-root-disposition.md。

修後ch08-review-fixed-v9只改一份Rust檔的println與一張SVG的兩個標籤座標，其餘148份source不變。Root自行建立byte差異證據：六份BODY有實質影響，另七份只換同byte圖檔的凍結目錄前綴。各角色也獨立重建此範圍；沒有把13個raw hash差異全算教材改動。Root、TECH及CONT各自實跑修後normal target，三行stdout與正文相符，沒有因文句與圖座標重跑所有Rust。

TECH與CONT交叉／修後局部紀錄已封存，分別為0fbdd5cf…及508909ec…；各自保存原發現與受同行提示補查的不同時點。兩者都重讀六段與相鄰過渡，核實名稱來源、正確fixture支持範圍及新主詞；沒有新增必要問題。FIRST的六段逐段修後覆核另封存16809f05…，先保存四題／Q2兩面與引文才開下一段；原O1／O2在受查位置釐清，不回填原初判。Root完整讀取三份followup。

Root原先已看八張SVG桌面／手機，原dyn圖穿字由FIRST與CONT獨立提出，root此前漏看不能改稱原已發現。修後root、TECH、CONT與FIRST均親看更新dyn原位圖文：兩份同型handle各自找到資料與method表、結果80／120可定位，標籤不再穿線。FIRST手機截图未包含圖後正文，其局部限制保留；root以實際BODY及自己的頁面核對後續Shipping過渡。其餘七張未改SVG沿用原實際畫面範圍，沒有聲稱本輪全站重新看過或真人讀者測試。

三類原初判、交叉與受影響局部閱讀已關閉。下一步在獨立Git index候選只開第1–8章，核對pages/source與修後凍結byte相同，跑AGENTS要求的完整scripts/check.sh。尚未取得候選執行與遠端部署結果，本紀錄不先宣稱發布成功。

獨立release8-candidate已完成完整scripts/check.sh，退出0：strict文件建置、fmt、all-targets Clippy、workspace全targets tests、doctests及90個故意失敗契約全通過。Log含237個test target結果，123個個別tests／doctests通過、零失敗；合計包含前章，不當成本章單獨數量。Root自行核對10頁與150來源共160個路徑，候選byte全部精確匹配v9。workspace／Cargo.lock僅增加本章Std-only package，沒有帶入後章活躍作者的新依賴；預覽導覽已恢复。證據為release8-staged-byte-proof.json、release8-check.log及release8-check-counts.json。三類及修後受查範圍均關閉，候選可發布；遠端結果另行追加。

提交7e53e831e395fa1ca5f6e6b9b3baad7230bdc1e5的[Actions 38624401465](https://github.com/birdhackor/learn_rust/actions/runs/38624401465)已成功完成check／deploy。Root實際請求首頁及第1–8章59節，60個網址全部200且H1相符；另核八份第8章SVG遠端bytes匹配修後凍結SHA，未發布9.1仍404。證據為release8-live-http.json。初次HTTP檢查把產生的¶標題permalink也算進H1而誤報不符；修正檢查器後重請求核實，原失敗保存parser-v1，不是網站內容問題。本段部署證據下次內容發布帶入，不只為紀錄再觸發CI。
