# 第13章 QA 彙整與發布待辦

三份 original、兩份 cross 與 FIRST 單單位 LOCAL 已封存；**候選完整檢查與公開部署確認也已完成**，證據見文末。下文保留各角色封存當時的狀態，不回填原判。本文件只做協調彙整，不是新的獨立 QA。我不是第13章作者，曾參與第14章作者工作；以下實驗與親看畫面均明列原執行角色，不算本人的新證據。

## 已封存報告

協調者已真正全文讀下列六份報告，並重算實際 bytes 的 SHA；沒有拿 Root 摘要代替角色原文。三原判依同一 `ch13-complete-prerequisite-v5`，原 manifest SHA 為 `b4a997a6f10c76039c1a1a0e60a6ad60a395b1c4641b789756ffe4a46704f772`。

| 角色／範圍 | 報告 path | SHA256 |
| --- | --- | --- |
| TECH original，五頁32 BODY／44 archives | `planning/reviews/ch13-technical-v5.md` | `7cdd0eaba110ab4e5f2a0a28832fca4f901a780f2e5a72b30979c2be52a1f76e` |
| FIRST original，五頁32 BODY | `planning/reviews/ch13-first-reading-v5.md` | `f315175252aeb4111e823b5ef33dee2d5003fd75531f5f2c2f8600221bed3550` |
| CONT original，五頁32 BODY／44 archives | `planning/reviews/ch13-continuity-v5.md` | `3c8b9c839a0d53fc5ab2e73e552a8ac0e38bf06fe085f3c09715b005de1b6c34` |
| FIRST LOCAL v6，僅002-004 | `planning/reviews/ch13-first-reading-local-v6.md` | `cfc85c362f610e4071f5591ed310016ecd3b5e6ccc888fa8d9bf6ecc5080db54` |
| CONT cross v6 | `planning/reviews/ch13-continuity-cross-v6.md` | `4767a222201e0c0b9b4ca67d53ea77ecec44f8dff7566e73ecf82b48a57e3977` |
| TECH cross v6 | `planning/reviews/ch13-technical-cross-v6.md` | `1a257933d00b469887032f85964d446bc3b888c869f8898a6cff1d7866c3cc69` |

Root 已先全文讀三 original 的紀錄為 `work/tutorial-implementation/root13-originals-full-read.json`，包括 FIRST 全430行及最後單位的恢復核對；並另確認已全文讀兩 cross 與 FIRST LOCAL。三份原判與各角色早期未知、原分級保持原 bytes。cross／LOCAL 是後見複查，不補算新 original。

## 分級、角色與證據界線

| 角色 | 原判／有限證據與未驗範圍 |
| --- | --- |
| FIRST：必要0／可選0 | AI輔助順序讀者模擬，已有 actual1–12及14背景，不是真人 Python 新手或全無提示的讀者。每單位先閉合選讀、分開記機制與原需；七SVG只在原位讀來源／孤立390px看，五份完整答案均閉合。未讀44 archives、未執行Rust、未看CSS實頁。Unpin是否prelude仍未知，但本章不要求新寫裸Unpin bound且已給canonical來源；一般projection／stack pinning不承擔本例已明教的Unpin借法，不後見補成已會。 |
| TECH：必要0／可選0 | 非13作者，已有2／4／5／9作者與多章審閱背景。全文32 BODY／44 archives含所有答案、helpers、tests、負例與配置，查固定Std1.99 selected契約。四項有25秒上限的 own probes只核callback重鎖1 test、early-notice example、owner-swap example及get_mut跨crate E0277；未全跑18 tests／16 examples／五負例、Clippy或Miri，未執行UB／無界Pending／死鎖。22張實頁為1280×1200／390×1200的七圖與兩份答案closed/open，不包含專門API表格的版面驗收。 |
| CONT：必要0／可選1（I1） | 非13作者，有第10章作者及9／11 QA背景。全文32 BODY／44 archives與selected官方契約；未跑Rust。原稿74張親看為1280×800／390×844的五頁入口／末段、七圖、五份答案、API表與兩張正常閱讀補查。原navigation.top自動跳位曾覆code的觀察保留；正常下讀補查沒有覆必要行，不刪原不利畫面、不把全站按鈕問題升成必要教材洞。 |

三角色都能從實際材料接起：地址記錄失配的原問題→Pin保pointee到原位收尾→通知／poll／Output不同→同guard檢查登記、離鎖回呼→unit通知安排同一份pinned工作。通過項各有原需、機制與有限材料，不能外推為任意unsafe soundness、所有排程／平台／Future活性、runtime效能或真人學習成果的證明。

CONT I1只針對390px手機表格把 `view.get_unchecked_mut()` 等名稱拆行，增加對照code的負擔；第二欄安全責任與完整實作原本已足，故始終**可選**。FIRST無CSS實頁、TECH無該表專門畫面；其可選0不是反證，也不回填成漏掉必要概念。

## 最小修正、LOCAL／cross與Root證據

Root採I1：只把13.2 `002-004` selected API表第一欄縮成五個完整API名並nowrap，補「完整呼叫見下方實作」；輸入／輸出、借用、unsafe／Drop責任、隨後Rust全部保留。修後 `frozen/ch13-review-fixed-v6` 為五頁32 BODY、44 sources、七SVG，manifest SHA `b2414211655a3dfd7246e50b7aff4bf781f89568d99d825c8f0a1d0ff82e65c1`。圖的封存絕對目錄URI正規化後，material BODY僅002-004 changed；44 sources與七SVG精確原 byte，不能因raw URI換目錄而稱另外六單位內容改寫。

- FIRST LOCAL只核002-004一單位的五API／Pin責任仍可理解，沒有新增必要／可選項；只見原SVG來源／孤立圖，沒有實頁表格或Rust證據，亦不是新盲讀。
- CONT cross先全文互看兩原判並保存stage A，再自行diff、重讀受影響209行、親看五張表格局部，最後才對照Root裁定。原I1收益明確、未增加必要項；same CSS實載200／exact SHA、noTop、名稱完整，輸入結果與unsafe責任未省略。只延續未變來源與原74畫面，不能宣稱v6整章所有圖／fold重新驗收；未重跑Rust。
- TECH cross先保存同行交叉與自己的修後判斷，最後才讀Root裁定；原未命中I1的背景保留，現在接受可選收益不回算獨立發現。重讀受影響單位及前後轉場、逐份核44 sources，親看1280×800／390×844各三張表格，共六張。手機element捕捉有sticky header遮表頂，保留此限制，依完整head／tail context作可讀判斷；same CSS實載／noTop、無頁溢出，不冒稱每張全表無遮或七SVG重驗。本輪零Rust執行；環境恢復只補最後報告及seal，先存判斷／畫面未重做。
- Root自己的四張表格context／element，1280×800與390×844，紀錄於 `root13-minimal-fix-views-proof.json`：Root親看、CSS HTTP200且SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`、`.md-top`為0、無整頁溢出。這是Root的局部證據，我未新看這四張或冒稱自己執行。

Root處置及範圍依 `planning/reviews/ch13-root-disposition-v6.md`、`ch13-minimal-fix-scope.json`。處置末段「cross／LOCAL／實頁待完成」是當時狀態，原文不覆寫；本彙整追加目前review／cross／LOCAL已閉環、候選與發布仍pending的時點。不因純表格標記重跑Rust，也不以其他章PASS替本章實證。

## 閉環與後續待辦

- 三original＋兩cross＋FIRST LOCAL均已封存、由協調者全文讀及核SHA；`release13-signoffs.json`的`closed: true` **只代表review／cross／LOCAL閉環**。
- Root仍須全文核協調文件，建立符合v6學生bytes的發布候選、執行完整check，並另記提交／Actions／公開HTTP。現在candidate、fullcheck與publication均pending，不能寫已發布或全套PASS。

本任務不修改student、原判／cross、Cargo／lock／nav／workflow／scripts或Git index，不讀第15章同行或B內容。自己的協調讀取與SHA checkpoint保存在 `work/tutorial-implementation/coordination-ch13-author04/`；JSON位於 `work/tutorial-implementation/release13-signoffs.json`。

## Root候選完整驗證

Root已全文讀三original、兩cross、FIRST LOCAL與本彙整。release13-candidate只納入已審1–13章90小節＋首頁；本章49學生檔working／INDEX／candidate均精確符合v6，六封存report SHA保持。Cargo僅新增Std-only ch13-futures，候選獨立lock與導覽不含14／15；Miri workflow／script留14章。完整bash scripts/check.sh退出0：strict build、fmt、all-target Clippy、all-target及doc tests、139指定negative、3正常C11與1指定C compile-fail皆通過。399個test target結果，231項tests通過、零失敗；來源與lock獨立，僅重用前章生成快取。公開部署仍待完成，候選PASS不替代Actions／公開HTTP證據。

## 公開部署確認

已提交並push 67160f7de0f77b644d97309cb763b1419ba57036。[Actions38669335058](https://github.com/birdhackor/learn_rust/actions/runs/38669335058) completed success。Root實際核對首頁＋90小節共91頁HTTP200／H1精確，七SVG逐SHA符合v6，未釋出的14.1為404；逐頁proof在release13-live-http.json。公開確認與本地候選分開，沒有回填角色原始執行／視覺範圍。
