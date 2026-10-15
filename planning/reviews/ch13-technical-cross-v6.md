# TECH13 CROSS：v6 局部覆核

## 角色與原判保護

本輪沿本人非第13章作者的 TECH13 角色；已有第2章、4.4–4.5、第5章、9.2–9.4作者及12／14等審閱背景。這是同行交叉與最小修後覆核，不是第二次獨立原判，也不稱 blind。ORIGINAL `planning/reviews/ch13-technical-v5.md` 與同 byte initial 保持 SHA `7cdd0eaba110ab4e5f2a0a28832fca4f901a780f2e5a72b30979c2be52a1f76e`，原分級 **必要0／可選0** 不改；既有 TECH12 cross `deb9b7a9…` 等舊 proof 未動。

先全文讀兩份 peer originals，保存 `work/ch13-technical-cross-v6/pre-fixed-pre-root-cross.md`（SHA `992c1ca9e11509e72df6878a877da0ba4bd6d0a39fa27c90c28cb9f7bc246c49`），之後才開唯一修後 `ch13-review-fixed-v6`。完成自己的 diff／受影響正文與實頁判斷並保存後，最後才全文讀 root disposition。未改 originals、學生程式、共享 Cargo／lock／nav，未開第15章。

## 同行全文、差異與通過理由

| 原判 | 本人已完成的全文閱讀 | 原分級與範圍 |
| --- | --- | --- |
| FIRST13 `ch13-first-reading-v5.md` | 430行，1–145／146–290／291–430，SHA `f315175252aeb4111e823b5ef33dee2d5003fd75531f5f2c2f8600221bed3550` | N0/O0。AI輔助 closed 順序讀者，明示 actual1–12及額外14背景；五答案不開、未讀44來源或跑Rust，七SVG是孤立圖而非實頁。 |
| CONT13 `ch13-continuity-v5.md` | 281行，1–100／101–200／201–281，SHA `3c8b9c839a0d53fc5ab2e73e552a8ac0e38bf06fe085f3c09715b005de1b6c34` | N0/O1。全32正文、44來源、必要前文與selected官方；74張實頁保留 capture 與正常向下補查的不同範圍，沒有跑Rust。 |

兩份工具輸出均未截斷，不以摘要代全文。本人原 TECH13 有全32正文／44來源、4項有限probe、22張七圖與兩答案closed/open實頁；不同角色不互借 source、執行或 visual 證據，也不以三人N0代替主要方法核查。

**CONT I1是它的原獨立發現，本人原稿未命中。** 它在390px實頁看到第一欄 `view.get_unchecked_mut()` 拆行、Box::pin參數亦換行。本人原22張視圖沒有專門檢查該表格，FIRST亦無其實頁證據，不能說我們看過同位置卻反對它。接受 **可選呈現改善**：名稱需拼回略增對照負擔，但右欄契約、後方完整呼叫及安全責任原已可理解，不升成必要先備、概念或編譯缺陷。本輪驗表格受同行提示，不能回填為本人原獨立重現。

FIRST保留 Unpin 是否來自prelude、一般projection／stack pinning的未知，屬合理未教範圍：本章無須新寫裸Unpin bound，std::marker來源已給；StartAmount選定Unpin欄位借法在code前說明。本人原待查的數值reference／Option Debug由actual4.3解除，shared Pin Copy／Poll額外traits由原完整source解除。各時點未知／查證不回填首讀，也不升新N。

| 主要方法／原需要 | 三份原判可互核的機制與限制 |
| --- | --- |
| 地址：「內部保存的位置資訊，之後仍對得上同一處資料嗎？」 | lifetime、ownership、位置三分；整Note／Box owner／只換i32三種不同T有有限對照。raw eq只比較位置，不dereference失配saved，不授予provenance或存取資格。 |
| Pin：「收尾也要在該位置完成」 | wrapper／pointee分清，Unpin例外與marker能力先教；constructor／全部safe API／Drop一起維持原位。一次Drop true輸出不是完整soundness證明。 |
| poll：「通知發生後，又由誰真正繼續執行？」 | 最新owned Waker、Context短借、Wake Arc receiver與From bound分開；授權／wake／poll／Output及stop／Drop不同。safe再poll不能UB，本模型panic不是所有Future統一策略。 |
| 外部事件：「事件卻在它登記通知前到達」 | 同guard查結果或登記／事件存值取通知，wake及舊Waker Drop在鎖外；callback重鎖Copy快照不take結果。兩種受控次序不是全部排程證明，普通Mutex仍可能等鎖。 |
| executor：「何時再檢查」 | unit通知不攜80或Future，buffer保early wake；caller-thread的F不額外要求Send/static/Unpin。StartAmount為確定先Pending才供值；Output返回不代外worker join，無任意Future活性或完整Tokio I/O／timer保證。 |

以上各有原需、機制、操作與實際材料範圍，不以用途漂亮代替型別／同步條件。

## 本人差異與 bytes 核查

原manifest SHA `b4a997a6f10c76039c1a1a0e60a6ad60a395b1c4641b789756ffe4a46704f772`；修後manifest SHA `b2414211655a3dfd7246e50b7aff4bf781f89568d99d825c8f0a1d0ff82e65c1`。本人逐份讀 byte／重算SHA：**44 archives（含七SVG）全同**，集合與各manifest source指紋一致，不只引用root的範圍說明。

32 BODY的raw hash有七份改變：001-002、001-004、002-004、003-006、004-002、004-004、005-004。將snapshot自身asset目錄前綴作同一標記後，**僅002-004有實質差異**，其餘六份只換圖URL。對照與unified diff保存在 `own-diff-source-proof.json`、`002-004.md.diff`。

本人全文讀修後002-004，核002-003末尾Unpin／move限定及002-005完整練習／Future轉場。改動僅導句加「完整呼叫見下方實作」，左欄改為完整API名 `Box::pin`／`as_ref`／`as_mut`／`get_ref`／`get_unchecked_mut` 並保持名內不換行。五項右欄契約逐字相同：owned/shared/mutable、回傳型別、!Unpin限定與unsafe／Drop責任未刪減。後方首次完整模型仍有owner／view實際呼叫與typed self說明；無新Rust API、語法、數值、unsafe能力、helper、答案或測試需求。這份先於root處置的核查保於 `fixed-semantic-pre-root-proof.json`。

## 六張本人親看的修後表格

自己的v6 snapshot preview以32 BODY重組五頁，只去重複H1與改靜態圖URL；重組不稱本輪重新全文閱讀32段。公開CSS明列extra_css且實際HTTP載入同byte SHA `fbf2cd306daad6f9cc1d05d59f713fca56dcf767d773c70d3ae8712e6d5a491b`，並非只寫config便報已載入。沿原theme，**移除navigation.top以配合當時working config**；沒有改root config或把global配置變動算成13章Rust修法。preview config SHA `b8b85a2e1a545bc061d82a3857947c0578e48a9dae8860e542fbb1a0f3c2f1a0`，本人zensical build exit0（0.32秒）。

本人實際Chromium／Playwright在桌面1280×800、手機390×844，各截表格head context、tail context與element補充，共 **六張均view_image親看**。五名稱完整一行，get_unchecked_mut不需跨行拼回；導句與後方reborrow／typed self轉場可讀。手機表格343px在有效375px頁寬內，沒有因nowrap造成整頁額外橫溢；右欄較窄而多換行，可正常上下滾動讀完整輸入／結果與限制。答案仍closed，不靠展開答案補含義。

保留capture限制：手機element screenshot自動scroll把表頂移到sticky header後，表頭被遮。沒有將它宣稱全表無遮或刪除；本人的head context表頭完整，tail含最後unsafe契約與後文，所以判斷依兩張context，element僅補充。六圖及served CSS／標籤／bbox／頁寬／SHA保於 `visual-metadata.json`、`visual-viewed-proof.json`，不以bbox或root畫面代本人實看。

自己的圖文判斷先存 `fixed-visual-pre-root-judgment.md`，SHA `d84215fd0d967f3bbde71bb832a6ce8bc1644d7cc9bc2bad97e7149be1fc807b`，之後才讀root disposition。結論：**CONT原I1的可選收益達成，沒有新增必要問題**。未重新看七張未改SVG／其他四頁完整實頁，未測所有scroll／TOC／menu／深色／viewport或全站；原22張證據僅保原scope，不假稱本輪重驗。

## 最後 root 對照、執行限制與恢復

最後已全文讀 `planning/reviews/ch13-root-disposition-v6.md`：保留FIRST/TECH N0/O0、CONT N0/O1，採短名稱＋nowrap＋導句，原契約與全部Rust／SVG不變；FIRST未知不回填，各角色probe與visual限制各自保存。與本人先存交叉及修後判斷一致。root處置是協調裁定，不是第四份original，其待完成正式check／部署不冒稱本人完成。

本輪 **零Rust執行**：未跑cargo test／check／clippy／負例／Miri／UB／無界等待／任何Rust suite。44來源同byte，文字標記修法不需重做原四probe；只完成自己的preview建置與瀏覽器檢視，原probe不算本輪新跑。

環境中斷發生在最後report／end-proof寫入的process建立階段。恢復後先核：report與end-proof皆未落檔，兩份先存判斷SHA吻合、6張已親看metadata仍在、original與其initial仍同byte／SHA7cdd0ea。只補本報告、end-proof及samebyte seal，**沒有重讀peer、重看captures、重跑Rust或preview**，未把中斷時未完成的finalwrite說成已sealed。完整peer、原來源fingerprints、差異proof、親看圖文與先存判斷隨新報告封存；完成即停止，等待另授權，不開其他章或15。
