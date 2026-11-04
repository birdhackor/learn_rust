# 第8章來源實讀與取捨

本輪為第8章另行親讀錯誤處理對應教材，沒有把先前第3～5章40份來源視為本章已讀。Book錯誤處理導論、panic、Result與panic政策英／繁中共8頁均讀完整抽出main；另有型別別名與13份官方API的選定範圍，共22份成功實讀。HTML、main文字、URL、抓取／讀取時間與兩種hash在`/workspace/work/tutorial-implementation-20261015/ch08-preparation/external-sources/sources.json`。下列range是一基底且包含兩端的保存TXT行號，不表示外部網站圖文版面已驗或範例已跑。

| 親讀原文 | 保存文字行 | 原始HTML SHA-256 |
| --- | --- | --- |
| [book-intro-en](https://doc.rust-lang.org/book/ch09-00-error-handling.html) | 1–21 | `42510591bb00aad665f7c65ce5629032b317455e4e928c70c5ba03712228201e` |
| [book-intro-tw](https://rust-lang.tw/book-tw/ch09-00-error-handling.html) | 1–4 | `378949de2e52931f9a60eec9178f4e4ab2eaad163edcc8ba91dcfe3bbff1c98b` |
| [book-panic-en](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html) | 1–131 | `1cb24b35caf15eff61fe48bd8da7f5498d36903d808812472b37ea64f12d46cf` |
| [book-panic-tw](https://rust-lang.tw/book-tw/ch09-01-unrecoverable-errors-with-panic.html) | 1–63 | `7584f94605fd7741fb4bbdaa93b3c90f596a6f6dec45d0aff18d71bd94b0e402` |
| [book-result-en](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) | 1–473 | `3ee5ea6a5f947cf8c8da81c54a9945db85cc0f37f6fd97f274b0fcbbbfb946e2` |
| [book-result-tw](https://rust-lang.tw/book-tw/ch09-02-recoverable-errors-with-result.html) | 1–231 | `3fe21297511f624e151c13d32844a2d4668a7f7289165d6e4bdf5116d47bd56d` |
| [book-policy-en](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html) | 1–235 | `e6d236ff3bac5ebc9f4d869bfef0618a1042387f1904d079f685375b5b692450` |
| [book-policy-tw](https://rust-lang.tw/book-tw/ch09-03-to-panic-or-not-to-panic.html) | 1–88 | `2222661cb8da788d78c9792ae35b31b58346f37df13e39cb007adc0d6ff67463` |
| [book-alias-en](https://doc.rust-lang.org/book/ch20-03-advanced-types.html) | 1–170 | `2a4a66368a0edfbafb6e3d63f09964364990af0a9af63b0cd1ae124351bc351b` |
| [std-display](https://doc.rust-lang.org/std/fmt/trait.Display.html) | 1–155、328–341 | `0d4075c997d24adc4ab62981d35aa2a93b74749b76b51ea4660ced377fb0f887` |
| [std-from](https://doc.rust-lang.org/std/convert/trait.From.html) | 1–190、1245–1256 | `2dafdb7a3479374dacb5193b7ef1d27cea6f1aec41e0b41cd1ca7dd0a30cfecd` |
| [std-into](https://doc.rust-lang.org/std/convert/trait.Into.html) | 1–63 | `d4ff5838a18121036fdb8928ca02433070a8b3e3dc8704f8f873eb3514d2f55c` |
| [std-tryfrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) | 1–135 | `2ccf1d6f22e7a6402e2493d059b046d7cbd838da0b3a44bd5851af83bbb9bcb7` |
| [std-fromstr](https://doc.rust-lang.org/std/str/trait.FromStr.html) | 1–150、175–210 | `d4331475ca3b75f7457dd044a379b72f52dedd2e19d6923e6d46143d3fc2c8f4` |
| [std-asref](https://doc.rust-lang.org/std/convert/trait.AsRef.html) | 1–145 | `896245a1fa5dc66162c8f7ed0424e71d876c8ddc7ce8d13aecc4a4f7a6b1a562` |
| [std-toowned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) | 1–75 | `9b298b4907f52aa63b0d727f04be5c165c712ecc1527b4f531251e95bb29d48c` |
| [std-tostring](https://doc.rust-lang.org/std/string/trait.ToString.html) | 1–33 | `9642d4df615bccaf8d7c55b128def7513314f6b8c1cbd0704cf4550ad77ddf09` |
| [std-fmt-result](https://doc.rust-lang.org/std/fmt/type.Result.html) | 1–33 | `355877aa23c2aaef6bc5544ecbf2ad9f423b8f3ab4114d7b9de6d149ceee0b5b` |
| [std-formatter](https://doc.rust-lang.org/std/fmt/struct.Formatter.html) | 1–80 | `465d7a015b582e39c3c17ccaea98b14b6d9553cf0a5bba35e91c3c21b092003a` |
| [std-parseinterror](https://doc.rust-lang.org/std/num/struct.ParseIntError.html) | 1–135 | `06f7de5302317a6583621b02ba85ea7a5e4d88fd82158d497c7695e9b2a82fd2` |
| [std-result-methods](https://doc.rust-lang.org/std/result/enum.Result.html) | 1–85、246–300、579–591、722–739 | `24488a7f890997c7b4bf71fb06d42b406d0127d304852b2cdab0fdc470a91138` |
| [std-option-methods](https://doc.rust-lang.org/std/option/enum.Option.html) | 124–172 | `beb418aa8bde3d404ae9bf01f54c0115f6a4e90dc4875ccc72be454af7f20d26` |

## 8.1／8.2已採用

Book9.2先把Result說成成功值／失敗資訊，再明說其範例是呼叫者「選擇panic」；9.3把可能失敗時回Result作為合適預設，讓呼叫者決定。本教材用CountError、逐筆match與最後完成訊息呈現同一件事，不帶入File／I/O／ErrorKind。

Book9.3對examples／prototypes／tests與固定成功保證分別交代。8.1只在固定3／0件及已建兩項的索引1展示unwrap／expect；遇到Err或None的刻意失敗另放fixture，正常輸入仍明確處理。std Result文件核對receiver為self、E:Debug、結果可能panic，及expect訊息應說「為什麼預期應成功」；不把Book9.2的expect優於unwrap語句放大成業務處理一律expect。Result Copy／Debug／PartialEq各自對T/E的條件在首次必要處先說，再用來源或assertion。

Book advanced-types的alias說明採「另一個名稱、不是新型別」及減少重複動機。8.2使用已知Result與i32／tuple；不照搬Box<dyn Fn() + Send + 'static>、Write、never或DST。CountNumber仍可放−1，驗證工作不因名稱而發生。可選的繁中advanced-types兩個URL為404，metadata明記未讀；不冒稱已有這一頁的中英配對。要求的繁中錯誤處理四頁皆成功實讀。

## 8.3～8.6的採用界線

Display核對給使用者看的格式、不能derive、對reference的實作、與ToString的自動關係；fmt只有formatter回Err時才回Err，業務錯誤值本身不是fmt::Error。Display不保證可反向parse；Formatter內部借用與placeholder須承接實際7章，文件簽名不能替前綴補教。

From採infallible／lossless／value-preserving／obvious；Into來自From，反向不保證。TryFrom／FromStr關聯Error／Err承接實際6.4；FromStr::Err沒有必須實作std::error::Error的bound，本章保留具體錯誤。AsRef提供便宜不失敗的借用view，與Borrow鍵身分承諾分開；ToOwned具體教str→String／slice→Vec，不泛稱deep copy或移除所有內含參照。

Book9.2手動match→?的順序保留；?早退目前helper並按From轉換，呼叫者仍可繼續。Option?不自行把None換任意Err。File、Box<dyn Error>、Result-returning main、closure／Iterator串接，以及其他全文trait header中的?Sized／dyn／內部Deref／unstable API不因此進入教材。

逐機制對應cache行與具體採用／不採理由在`ch08-preparation/external-reading-notes.md`，本頁給repo內交接用。正常程式、失敗fixtures與測試的實測另記author-ch08；親讀來源本身不是實作或讀者驗收證據。
