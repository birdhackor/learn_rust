# 第12章獨立銜接與實頁初判

封存 2026-10-15T10:08:24.890908+00:00；來源始 2026-10-15T09:56:48.477423+00:00。**必要0，可選0。** 依實際1–11已讀前綴順讀12.1→12.4 canonical與全部actualownsource/helpers/tests/fixtures/answers/manifests；未讀作者validation、其他角色12結論、knownaudit/rootdisposition，未用13+作先備。

12.4 current55f1886f0d40f920a3dff640c0b2a3b3f0d0647951e739e805b717a750e68754；root事先告知已前移streams全檔與一句方向文，初判只涵此已修現版，不算舊版獨立重現。

60檔hash始末全穩定；50完整include occurrences各viewport與fullsource匹配，15namedregions另依完整實檔核首用；所有selectedpackage檔全入snapshot。

## 發現

沒有需要修正的銜接或實頁finding。

## 四題與首次必要

逐字引文從本輪frozen抽取；自己的解釋分列，Q2機制／性質與獨立需要／用途分開判。

### workspace共同欄位與共同dependency

首次必要：12.1根Cargo及兩成員manifest內嵌前。

docs/chapters/12-01-lesson.md:3

> 第1.3節已能在workspace選package，第10.1節也已宣告本機與第三方依賴。現在想讓幾個一起維護的package共用版本欄位，並讓使用者選擇是否編入批次功能。這一節要分清：**共同設定由誰採用、這次有哪些介面，以及編譯使用哪組設定？**

docs/chapters/12-01-lesson.md:31

> `[workspace.package]`保存成員可採用的共同package欄位。`version`與`edition`已在前文介紹；本節再用兩項：

docs/chapters/12-01-lesson.md:38

> 成員寫`version.workspace = true`，表示「我的version採用workspace.package中的共同值」。`edition.workspace = true`等欄位也是相同形式，**不是把edition設為布林值**。TOML的點號在這裡指定設定欄位的層級；`true`表示採用。每個成員自己的`name`仍分別填寫。

docs/chapters/12-01-lesson.md:40

> 依賴也有共同表：`[workspace.dependencies]`保存可供成員採用的來源或版本設定。成員還須在自己的`[dependencies]`寫`lesson-subtotal.workspace = true`，才建立這項dependency；只在根設定列出，不會讓所有成員自動取得library。

docs/chapters/12-01-lesson.md:42

> 根表中的`path = "subtotal"`從**workspace根manifest所在資料夾**出發。第10.1節直接在成員宣告的dependency path，則從那份成員manifest出發；同樣要找到subtotal，從app出發會是`../subtotal`。把共同來源移到根表後，起點也跟著改變。

- Q1：共同package欄位可被成員採用；rootdependency表只是可採設定。
- Q2機制／性質（已足夠）：點號是TOML層級true是採用，非editionbool；dependency仍成員明選；共同path起點root。
- Q2獨立需要／用途（已足夠）：一起維護數package需共版本欄、共來源，仍保個別name/依賴責任。
- Q3：rust-version最低需求非安裝選toolchain，publishfalse不發佈；小workspace與repo根各有lock。
- Q4：subtotal只乘法80×2=160，app另一120×1合280，不把設定移動當規則變更。

### profile、最佳化設定與整數溢位

首次必要：12.1完整rootmanifest的profile段之前。

docs/chapters/12-01-lesson.md:46

> **profile**是一組編譯設定。一般`cargo build`／`run`使用`dev`；加`--release`改用`release`。這與Rust edition、要執行哪個package都不同。

docs/chapters/12-01-lesson.md:48

> `opt-level`控制最佳化程度，常用數字為0至3。預設dev使用0，release使用3；較多最佳化工作通常增加編譯時間，成品的實際速度仍要對相同任務量測，不能由這個數字直接推定。

docs/chapters/12-01-lesson.md:50

> 本例還在release設`overflow-checks = true`，要求執行時檢查整數運算是否超出型別範圍；超出就panic。第一章的有限整數範圍仍然成立，這項設定不會替`i32`增加能表示的數字。它讓release也保留這項檢查，避免只因建置模式不同而把超出範圍的運算悄悄放過。

docs/chapters/12-01-lesson.md:52

> profile須寫在**workspace根manifest**，成員manifest的profile設定會被Cargo忽略。以下是本頁小workspace的**完整**`Cargo.toml`；members與resolver回用第1.3節，後面三組是剛介紹的共同設定與profile：

docs/chapters/12-01-lesson.md:198

>     app仍須在自己的dependencies明選lesson-subtotal。release改變編譯設定，本例的原始碼、輸入與未超範圍的整數運算規則相同，因此已知答案不變；這與成品速度是否改變是不同問題。

- Q1：profile是compiler設定組，dev/release不同於edition/package。
- Q2機制／性質（已足夠）：opt-level0/3是更多編譯最佳化工作、需更多編譯時間而非直接速度證明；overflowchecks要求超範圍panic。
- Q2獨立需要／用途（已足夠）：本節問題包括「編譯使用哪組設定」；同原碼/輸入/未超範圍整數結果保持160/280，效能另量。
- Q3：rootmanifest設定、--release選組；不能從profile推跨平台或i32擴大。
- Q4：預設/feature配置/release小計相同，只驗配置可建與答案，未宣稱速度排名；無當下需要的最佳化內部推導。

### 自訂feature與功能組合

首次必要：12.1 app/Cargo.toml完整內嵌前。

docs/chapters/12-01-lesson.md:74

> 第10.1節的`features = ["derive"]`選擇的是dependency提供的功能。現在要替**自己的app package**定義名為`batch`的feature，控制是否提供兩筆小計函式。

docs/chapters/12-01-lesson.md:76

> `[features]`中的`名稱 = [其他功能名稱]`定義功能及它連帶啟用的項目。`batch = []`表示這個開關不連帶啟用其他項目；`default = ["batch"]`表示預設啟用batch。default是一份功能列表，不是另一套計價規則。

docs/chapters/12-01-lesson.md:84

> 建置本package時，`--no-default-features`不啟用default列表，`--features batch`則明確啟用batch；兩者可以一起使用。`--all-features`啟用所選package的全部功能，本例會啟用batch。

docs/chapters/12-01-lesson.md:86

> 這與第10.1節dependency設定中的`default-features = false`分工不同：後者控制那項**依賴**的預設功能，這裡的命令列旗標則選本次建置package的功能。若同一dependency被多個使用者請求不同功能，Cargo通常會合併它們所需的功能，因此feature適合增加能同時存在的能力。本例關掉batch後，單筆介面與160的答案仍保留。

- Q1：自己的feature有名稱與連帶開關列表，default是一份列表。
- Q2機制／性質（已足夠）：batch只新增兩筆介面，關掉仍有單筆；與dependency default-features=false責任不同。
- Q2獨立需要／用途（已足夠）：使用者選是否編入批次功能，不變商品規則；多請求通常合併故適合可並存新增能力。
- Q3：no-default/featuresbatch/all-features三配置，member明dependency。
- Q4：default與明batch各160/280，關default只160；答案數量0+另一120抓兩筆各自資料。

### cfg條件編譯與cfg! bool

首次必要：12.1 app/lib與main及bool負例第一次出現之前。

docs/chapters/12-01-lesson.md:90

> 第2.7節的`#[cfg(test)]`已依測試建置決定是否納入module。這種依建置條件保留或排除程式的做法叫**條件編譯**。

docs/chapters/12-01-lesson.md:92

> `#[cfg(feature = "batch")]`使用相同的屬性形式：啟用batch才保留緊接著的函式；未啟用時，該函式根本不在這次library的公開介面中。字串中的batch須符合manifest所定義的名稱。

docs/chapters/12-01-lesson.md:102

> `cfg!(feature = "batch")`是另一個macro，依建置條件產生true或false，可以保存或顯示。它**不會排除包住自己的程式**。普通`if`即使條件是false，裡面的名稱與型別仍須通過編譯檢查。

docs/chapters/12-01-lesson.md:102

> `cfg!(feature = "batch")`是另一個macro，依建置條件產生true或false，可以保存或顯示。它**不會排除包住自己的程式**。普通`if`即使條件是false，裡面的名稱與型別仍須通過編譯檢查。

docs/chapters/12-01-lesson.md:132

> 沒有batch時，attribute排除了two_orders，但普通if的body仍要檢查，因此得到E0425與`cannot find function two_orders`。正確main使用attribute排除呼叫block，才讓未啟用功能的版本也成立。

- Q1：attribute按建置條件保留/排除項目；cfg! macro僅回bool。
- Q2機制／性質（已足夠）：未啟用函式不存在，普通if就算false仍查名稱型別；需整block與use同排。
- Q2獨立需要／用途（已足夠）：可選publicinterface真的缺席時caller也必須合法，單靠runtimebool不行。
- Q3：cfgfeature名稱對manifest，maincfgblock；rustc直接編fixture -o/edition先解釋。
- Q4：獨立負例E0425/cannotfindtwo_orders與完整正main對比；no-default不載未教後文code。

### host/target與Cargo target的分別

首次必要：12.1 main的兩平台cfg與alltargets命令前。

docs/chapters/12-01-lesson.md:104

> 因此呼叫可選函式時，也用`#[cfg(...)]`排除整個呼叫block，連同那個block才需要的use。以下main另外觀察兩個平台條件：`unix`表示目標屬於Unix類系統，例如Linux與macOS；`target_os = "windows"`表示目標作業系統為Windows。

docs/chapters/12-01-lesson.md:106

> 這裡的**編譯目標平台**是程式準備在哪種平台執行；**host**是執行編譯工具的本機。沒有另選平台時，兩者通常相同。crate內的cfg看自己的編譯目標，不是在每次執行時重新偵測；`--release`也不會把Linux程式變成Windows程式。前文的example target則指Cargo選出的某個建置成果，這兩種target用途須依上下文分清。

docs/chapters/12-01-lesson.md:150

> 下面分別驗證預設、關閉預設與全部功能。前兩次features.rs中的測試數應為2與1，全部功能又回到2。`--all-targets`指所選package的library、binary、examples、tests等建置成果，**不是所有作業系統**；本頁的完整來源包含下方練習答案，整包檢查也會編譯它；其中只用本節已教的介面。

docs/chapters/12-01-lesson.md:150

> 下面分別驗證預設、關閉預設與全部功能。前兩次features.rs中的測試數應為2與1，全部功能又回到2。`--all-targets`指所選package的library、binary、examples、tests等建置成果，**不是所有作業系統**；本頁的完整來源包含下方練習答案，整包檢查也會編譯它；其中只用本節已教的介面。

- Q1：host編譯工具本機、target預備執行平台；Cargo exampletarget另是建置成果。
- Q2機制／性質（已足夠）：cfg看crate編譯目標非runtime偵測；release不換OS。
- Q2獨立需要／用途（已足夠）：同工程設定要知道建哪種成果、哪個平台，免把alltargets誤作跨所有OS驗證。
- Q3：unix/target_oswindows，featureintegration各cfg；-p選package再alltargets含lib/bin/example/tests。
- Q4：Linuxunixtrue/windowsfalse限實際target；tests2/1/2，完整答案被編也只當時已教操作。

### repo根與獨立workspace選擇

首次必要：12.1 repo-root-settings節錄前及12.2換workspace7前。

docs/chapters/12-01-lesson.md:165

> 本頁的小workspace與第1.3節的入門workspace各有自己的根。教材repo還有較大的維護workspace，後續章節會明選其中的package。它同樣以members列成員、以`-p`選package，共用自己的Cargo.lock及target資料夾；`exclude = ["examples/intro-workspace"]`則將入門workspace從這份成員管理中排除，沒有把兩份lockfile混成一份。

docs/chapters/12-01-lesson.md:167

> 下面精確節錄**教材repo根Cargo.toml**的三組設定，不是另一份可直接執行的完整manifest，也不要拿它覆蓋repo根檔案：

docs/chapters/12-01-lesson.md:173

> 共同package欄位的語義與小workspace相同；後續成員寫`version.workspace = true`等形式，就是從這裡採用。共同dependency這次叫ch01-basics，path從教材repo根開始，因此是`examples/ch01-basics`；只有明選它的成員才會依賴它。本頁選用的是另一份小workspace的lesson-subtotal，沒有因此載入舊第一章library。

docs/chapters/12-01-lesson.md:175

> 兩處release都使用opt-level 3與整數溢位檢查。改用哪份manifest決定你進入哪個workspace，`-p`再決定該workspace中的package；共用欄位不會把所有成員程式串成一個執行檔。

- Q1：manifest選workspace、-p在其中選package，共同欄位不串全成員。
- Q2機制／性質（已足夠）：exclude入門workspace、各lock分開；舊ch01library只member明採才依賴。
- Q2獨立需要／用途（已足夠）：後續12.2/12.3使用repo主workspace，先建立設定來源，不拿小subtotal偷換成舊全lib。
- Q3：fixture三節錄與實際rootTOML三區段逐值吻合；有明標不可覆蓋整manifest。
- Q4：本頁真依賴小subtotal三行，未讀舊第一章技術body作先備。

### build script與四個時點

首次必要：12.2 manifest16/build.rs47之前。

docs/chapters/12-02-lesson.md:3

> 想讓練習版成品記住自己是demo還是practice，每次啟動都回報同一份建置模式。若主要程式每次啟動都讀環境，模式會隨執行環境改變；這次需要的是**在建置時決定，再把文字固定進成品**。

docs/chapters/12-02-lesson.md:5

> 第12.1節已能選profile、feature與target。現在再加一個步驟：Cargo先執行一支Rust程式，讓它把設定交給主要crate的編譯。這支程式叫**build script**，通常放在package根目錄的`build.rs`；它與成品的`src/main.rs`有各自的執行時點。

docs/chapters/12-02-lesson.md:11

> build script自己也要先編譯成能在host執行的程式。Cargo執行它，成功後才編譯主要crate，最後才有讀者啟動的成品。

docs/chapters/12-02-lesson.md:13

> manifest的`build = "build.rs"`明寫script入口，路徑相對於**這個package根目錄**。Cargo也能自動發現這個慣用檔名，本例明寫是為了看清工作入口。其餘共同欄位採用第12.1節已核對的教材repo根設定：

docs/chapters/12-02-lesson.md:68

> 現在把程式對應回四個階段。圖中向下箭頭只表示先後；第二框交出的TEACHING_MODE，在第三框由env!固定，第四框才由build_mode交給顯示工作：

- Q1：Cargo先編譯hostscript、執行script，再編主要crate，最後使用者啟成品。
- Q2機制／性質（已足夠）：scriptstdout協定把資料交給compile，main執行是另一時間。
- Q2獨立需要／用途（已足夠）：成品須記住建置demo/practice，每次runtime讀環境無法固定那次建置。
- Q3：package根build.rs/manifest入口，四階段圖1→2交TEACHINGMODE→3env固定→4顯示。
- Q4：重新Cargo變practice後直接成品給demo仍practice，區別實際工作與設定名稱。

### env::var與Unicode/target環境

首次必要：12.2 build.rs47之前。

docs/chapters/12-02-lesson.md:21

> **`std::env::var("名稱")`**讀取目前正在執行的行程所收到的環境值，回傳`Result<String, VarError>`。這裡正在執行的是build script。

docs/chapters/12-02-lesson.md:23

> `std::env::VarError`有兩個variants：`NotPresent`表示缺少這個值；`NotUnicode(OsString)`表示值存在，但不能轉成Rust String。第10.5節已區分原生OsString與UTF-8文字，這裡同樣不能把轉換失敗當成「沒有設定」。VarError有Debug，可符合第8.1節的expect契約。

docs/chapters/12-02-lesson.md:25

> 本例替`LESSON_MODE`訂出政策：沒有設定就採demo；有值只接受demo或practice；其他文字或非Unicode值都讓建置失敗。`CARGO_CFG_TARGET_OS`則由Cargo啟動script時提供，內容描述**主要crate的目標作業系統**，例如linux或windows。這份script只由Cargo啟動，因此能以這項保證為依據使用expect。

docs/chapters/12-02-lesson.md:27

> 第12.1節已區分host與target。script內的`cfg!(target_os = "windows")`判斷這支script自己的目標，也就是host；若要知道主要crate的target，應讀Cargo傳入的資料。一般本機建置時兩者碰巧相同，跨平台建置時就可能不同。

- Q1：envvar讀目前行程環境→ResultString/VarError，非編譯macro。
- Q2機制／性質（已足夠）：missing與NotUnicode原OsString不同；scriptcfg目標host，CargoCfgTargetOS才主crate目標。
- Q2獨立需要／用途（已足夠）：驗有限模式政策，不把無法解碼當沒設定；跨平台要保存真正target。
- Q3：NotPresentdefaultdemo、NotUnicode/其他字串error；Cargo提供target才可expect。
- Q4：全文build33行只已教match/String/if/returnunit，bool存文字不暗解析。

### build stdout協定與重跑追蹤

首次必要：12.2 build.rs47之前。

docs/chapters/12-02-lesson.md:31

> 第10.5節的stdout是行程的標準輸出。build script在這個出口印出特定格式，Cargo依每行的**建置協定**處理，而非接收main的回傳值作設定。

docs/chapters/12-02-lesson.md:37

> | `cargo::rerun-if-changed=build.rs` | 縮小檔案變動追蹤範圍。script本身需要重編時，Cargo也會處理並重新執行。 |

docs/chapters/12-02-lesson.md:38

> | `cargo::rerun-if-env-changed=LESSON_MODE` | 追蹤啟動Cargo時這個外部環境值的變動，包括設定與取消設定，必要時重跑script。 |

docs/chapters/12-02-lesson.md:39

> | `cargo::rustc-env=名稱=值` | 提供接下來package crate編譯時可讀取的環境值。 |

docs/chapters/12-02-lesson.md:40

> | `cargo::error=訊息` | script結束後顯示錯誤，讓這次建置失敗；即使main正常回傳也不算建置成功。 |

docs/chapters/12-02-lesson.md:44

> 現在看完整`build.rs`。它先驗證有限的模式名稱，才印rustc-env，所以任意外部文字也不會直接成為多行協定。錯誤分支中的`return;`結束這個回傳unit的main，Cargo接著處理剛印出的error：

- Q1：Cargo按stdout每行協定接工作，不用Rustmain返回值當設定。
- Q2機制／性質（已足夠）：限定檔案與外環境變動重跑，rustcenv給compile，error即使mainunit正常也令buildfail。
- Q2獨立需要／用途（已足夠）：模式改變須更新固定結果；無需每次無變化全部重做，錯配置須在建置阻止。
- Q3：先有限mode驗證才交rustcenv，外部任意文字不成多行協定；-vv查log。
- Q4：demo/practice追蹤變化、wrong檢查失敗；Cargo自提供TARGET另追蹤不偽用rerunenv。

### env!、option_env!與Bash一次命令環境

首次必要：12.2 lib59/兩shell77與optionalexample118之前。

docs/chapters/12-02-lesson.md:54

> **`env!("名稱")`**是編譯時讀取指定環境值的macro，交出`&'static str`。第11.5節已說明這種文字reference可在程式剩餘的執行期間使用；這裡的內容來自編譯時嵌入的文字。

docs/chapters/12-02-lesson.md:56

> 編譯時缺值或不能表示成Unicode，env!會讓編譯失敗。它不在每次函式呼叫時讀執行環境，所以library可以用三個普通函式借出建置結果。完整`src/lib.rs`：

docs/chapters/12-02-lesson.md:74

> 以下採Bash。`NAME=value command`只把該環境值交給這次命令及它啟動的子行程，不是永久修改shell設定：

docs/chapters/12-02-lesson.md:89

> 它仍顯示`建置時選的模式=practice`。這次沒有啟動Cargo或重新編譯，runtime環境中的demo不能改寫已固定的文字。若需求改成每次啟動讀新模式，才在主要程式呼叫env::var並處理Result。

docs/chapters/12-02-lesson.md:113

> 若額外標記可以省略，就用**`option_env!("名稱")`**：它在編譯時交出`Option<&'static str>`，有Unicode值時是Some，缺值則是None；有值但不是Unicode仍是編譯錯誤。這與runtime的env::var不同，也沒有把主要模式的驗證規則改成可有可無。

docs/chapters/12-02-lesson.md:128

> 第二次顯示`編譯時的額外標記=lab`。Cargo與編譯器也會追蹤source中的env!／option_env!所讀值的變更，所以這項標記不需先讓build script讀一次。它仍是建置時值；直接啟動已編譯的example，會保留那一次編譯的標記。

- Q1：envmacro compile讀值回staticstr，optionmacro compile回Optionstaticstr。
- Q2機制／性質（已足夠）：envmissing/nonUnicode編譯錯；optionmissingNone但nonUnicode仍錯；Cargo追sourceenv變動。
- Q2獨立需要／用途（已足夠）：固定模式與可省標記責任不同；runtime環境不能改嵌入文字。
- Q3：NAME=value只本次及child；直接target/debug程式不經Cargo，三library函式借已固定結果。
- Q4：practice後成品runtime給demo仍practice；缺envfixture無main可執行，lab標記可Some完整source。

### OUT_DIR/build-dependencies與連結預告

首次必要：12.2:132–143角色預告，沒有當前生成或連結操作。

docs/chapters/12-02-lesson.md:132

> Cargo執行script時還提供`OUT_DIR`，指向這次建置使用的輸出目錄。生成供編譯使用的檔案應寫到這裡，保持原始碼目錄作為輸入。OUT_DIR可能留有前次檔案，script須自行管理需要重建的內容。本節只交付文字設定，因此沒有生成檔案或加入讀生成檔案的macro。

docs/chapters/12-02-lesson.md:134

> 若script自己需要第三方Rust library，放在package的`[build-dependencies]`，供host上的script編譯使用；`[dependencies]`則供主要crate。兩組須各自明確宣告。本例只用標準函式庫，所以完整manifest沒有這兩個區段，也沒有隱藏的第三方helper。

docs/chapters/12-02-lesson.md:136

> 後面與原生library合作時，還會用到**linker（連結器）**：它將編譯結果與所需library組成最後成品。build script要先找到或建好target需要的library，再交付搜尋位置及名稱：

docs/chapters/12-02-lesson.md:141

> | `cargo::rustc-link-lib=static=名稱` | 要求連結名為該名稱的靜態library；靜態連結會將所需程式納入成品。 |

docs/chapters/12-02-lesson.md:143

> 這些指令負責連結設定；library的取得、目標平台相容性，以及Rust端外部函式宣告與ABI契約，仍須由實際整合工作完成。第15章會在學過那些必要介面後實作。本頁script沒有印這兩條指令，也不需要C工具鏈。

- Q1：Outdir供建置生成檔；builddeps供hostscript，dependencies主要crate；linker組最後成品。
- Q2機制／性質（已足夠）：outdir可能留舊檔需管理；linksearch/linklib只交位置與名稱不取得library/完成ABI。
- Q2獨立需要／用途（已足夠）：當前只交文字無生成/第三方helper，將來原生合作才用連結工作，現在不要求C工具鏈。
- Q3：兩協定只給基本角色，script實際不印；生成macro/ABI完整教學合理待後。
- Q4：最小manifest無兩dependency區段可查，非把未教ABI當本節必要已懂。

### build結果測試與完整識別答案

首次必要：12.2 embedded150/identity174及最後alltargets186前。

docs/chapters/12-02-lesson.md:147

> 完整`tests/embedded.rs`用公開library確認模式屬於允許集合，並在Linux／Windows測試對應target文字。條件編譯與integration test沿用上一節：

docs/chapters/12-02-lesson.md:167

> Cargo編譯package的crate時，會自動提供`CARGO_PKG_NAME`，內容是manifest的package名稱；用env!取得，不必為它另寫script協定。模式與target則由本頁完整library提供，再用第8.3節的format!保存整行。

docs/chapters/12-02-lesson.md:183

> 讀完完整答案後，CARGO_PKG_NAME與兩份example的介面都已說明，也可把它們一起編譯檢查：

- Q1：integrationtest按公開library確認建置模式/目標文字；CargoPkgName是編譯環境。
- Q2機制／性質（已足夠）：cfgplatform測對應文字，source環境值改變需重建，非runtime查。
- Q2獨立需要／用途（已足夠）：需讓成品能回報那次package/mode/target，不以runtime新mode代替。
- Q3：format行String，完整答案後才編alltargets含兩example。
- Q4：已讀所有root/manifest/build/lib/examples/tests，沒有縮成只顯示mode或把未讀答案藏library。

### symbolic link 與掃描政策

首次必要：12.3 policy表39及validateRoot112之前。

docs/chapters/12-03-lesson.md:3

> 第10.5節已能把一個原生路徑交給讀檔工作。現在把輸入改為一個目錄：要列出它與子目錄中的一般檔案，包括隱藏目錄內的檔案，保存原生路徑、byte 長度及修改時間，供後面的內容工作使用。

docs/chapters/12-03-lesson.md:5

> 這次先不讀內容，也不判重複。成功清單旁邊還要保存失敗項目；否則遇到不可處理的檔案時，「沒有列出」可能被誤認為「不存在」。**根設定不成立則整個工作無法開始；個別項目失敗則保留原因，繼續其他項目。**

docs/chapters/12-03-lesson.md:35

> **symbolic link（符號連結）** 是檔案系統中的一種項目，保存指向另一個位置的資訊。它可以指向檔案或目錄，也可能指向不存在的位置。追蹤 link 表示改去處理指向的目標；若目錄 link 又指回上層，追蹤還可能遇到循環。

docs/chapters/12-03-lesson.md:41

> | root 是 symbolic link | 根設定錯誤，停止 | 不把使用者給的 link 默默換成另一份目錄樹 |

docs/chapters/12-03-lesson.md:47

> 「不是目錄」不能當成「一般檔案」；link 與其他特殊項目也可能不是目錄。因此後面會明確選 `is_file()`。

- Q1：symboliclink存另一位置資訊，可file/dir/missing/循環；追link改處理目標。
- Q2機制／性質（已足夠）：rootlink拒、rootfile/missing拒；內link/特殊略過不算issue，hidden子dir仍走。
- Q2獨立需要／用途（已足夠）：只處理選定目錄樹的一般檔，且不把失敗當不存在。
- Q3：明isfile而非非isdir；metadata選查link本身實現root政策。
- Q4：三完整fixtureRustLF5/RustLF5/ABCSELF6，遍歷只觀察不讀內容；linkAPItests後教。

### metadata、u64與SystemTime

首次必要：12.3 records78/observe90之前。

docs/chapters/12-03-lesson.md:49

> ## metadata 是觀察到的檔案資料

docs/chapters/12-03-lesson.md:53

> 標準函式庫 `std::fs` 提供兩種查詢：`metadata(path)` 會追蹤最後的 symbolic link，`symlink_metadata(path)` 則查最後這個項目本身。兩者都接受原生路徑，回傳 `io::Result<Metadata>`，可能因不存在或權限等原因失敗。要拒絕 root link，必須用後者。

docs/chapters/12-03-lesson.md:63

> | `modified()` | `io::Result<SystemTime>` | 保存修改時間；有些平台或檔案系統無法提供，失敗也要保留 |

docs/chapters/12-03-lesson.md:65

> `u64` 是無號 64-bit 整數，從零到它能表示的最大值；沿用 1.4 的有限範圍概念。`usize` 用於目前程式的索引與清單長度，metadata 選擇的 `u64` 則用於檔案 byte 長度；兩者不會自動互換。

docs/chapters/12-03-lesson.md:67

> `std::time::SystemTime` 表示一個由系統回報的時間點。本節只保存它，之後用相等比較觀察修改時間是否不同，不換算經過秒數。這個型別有 `Debug`、`Clone`、`Copy`、`PartialEq`，所以能顯示、保存副本及比較。同樣 byte 長度不代表內容相同，時間相同也不是內容不變的證明。

- Q1：metadata是類別/長度/修改時間觀察，不是內容或鎖。
- Q2機制／性質（已足夠）：metadata跟最後link，symlinkmetadata查本身；modified也可能Err；timeCopyEq可保存比變化。
- Q2獨立需要／用途（已足夠）：先保存byte長度與時間供後續工作，不能推同內容或一致快照。
- Q3：u64byte長度不同usize索引不自轉；isdir/isfile/filetype各角色表。
- Q4：Snapshot derive能力由SystemTime契約先給，觀察路徑可能類別變而NotRegular，原生PathBuf不固定diskfile。

### 成功record、item issue與rooterror

首次必要：12.3 records78/observe90與root112前。

docs/chapters/12-03-lesson.md:71

> library 的 `scan` module 用幾個型別分開保存結果。`Snapshot` 保存這次觀察的 `len`／`modified`，名字不承諾一致的檔案快照。`FileRecord` 再持有原生 `PathBuf`。第10.5節的 owner／reference 關係仍然成立：這份記錄擁有路徑文字，沒有接管或固定磁碟上的檔案。

docs/chapters/12-03-lesson.md:73

> `ScanCause` 保存具體原因：I/O 錯誤、walker 錯誤、不能轉成 Unicode、或觀察時已不是一般檔案。walker 選用的 `walkdir` crate 以具體 `walkdir::Error` 保存走訪失敗，這個型別已有 `Debug`／`Display`，可以檢查和顯示原始原因。`ScanIssue.path` 用 `Option<PathBuf>`，因為 walker 失敗不保證能提供路徑；`action` 是程式中固定的 `&'static str`，沿用第11.5節的 literal lifetime。Display 沿用第8.3節，保留兩種原始錯誤的顯示方式。

docs/chapters/12-03-lesson.md:75

> `RootError` 描述整個工作開始前的失敗。它和 `ScanIssue` 不能混成一份成功空清單，否則「root 不能用」會被誤當成「root 裡沒有檔案」。

docs/chapters/12-03-lesson.md:87

> 把觀察一般檔案的工作先寫成 `observe`。它重新查詢不追 link 的 metadata；即使上一個步驟看過一般檔案，目前位置也可能已改變。查詢、類別與修改時間取得都是可能失敗的步驟：

- Q1：成功FileRecordownedPath+Snapshot；ScanIssue可無path、固定action、具體cause；RootError整體前置失敗。
- Q2機制／性質（已足夠）：Io/WalkErrorDebugDisplay保原原因，observemetadata各步可失敗，不混完整空成功。
- Q2獨立需要／用途（已足夠）：根不能用就無法開始，個別item失敗留cause繼續並顯示完整度。
- Q3：ResultRootError vsScanReportfiles/issues；DataDisplay沿8.3，lifetime沿11.5。
- Q4：完整library只scan，mainpartialmovefiles仍能讀issues有4.5精確回指；不是完整CLIexit政策。

### walkdir逐項Result與兩個link開關

首次必要：12.3 walker101解約及scan145之前。

docs/chapters/12-03-lesson.md:95

> 自己管理每層目錄的待辦也能完成走訪。這裡選 **`walkdir`** crate，把逐層進入目錄與保留走訪進度交給它；我們專注於項目政策和輸出資料。依第10.1節在 manifest 宣告依賴，測試則回收第10.8節的 `tempfile`：

docs/chapters/12-03-lesson.md:101

> `walkdir::WalkDir::new(root)` 接受原生路徑，交回一份走訪設定；它有 `IntoIterator`，所以第9.4節的 `for` 可以接手這份工作。逐筆的 **Item 是 `Result<walkdir::DirEntry, walkdir::Error>`**。`walkdir::Error` 已有 Debug／Display，供前面 `ScanCause` 保存和顯示。遇到錯誤會交出 Err，仍可繼續下一項；若某個目錄打不開，就不能取得那個目錄內的內容。

docs/chapters/12-03-lesson.md:103

> 走訪順序沒有供報表使用的固定保證。本例收集完成後，才用第9.7節的 `sort_by` 指定順序，不從「這次剛好先看到哪個檔案」推定下次相同。

docs/chapters/12-03-lesson.md:107

> `follow_links(false)` 接手設定、回傳更新後的設定，控制**裡面遇到的** link；預設就是 false。另一個 `follow_root_links(false)` 控制**root 自己**的 link，其預設是 true。因此只寫前者不能完成我們的 root 政策。

docs/chapters/12-03-lesson.md:115

> walker 再明寫兩個 false。驗證與走訪之間外部程式仍可能改動檔案系統；這個設定避免 walker 接著主動追 root link，沒有把前面的檢查升級成抵抗惡意同時替換的安全保證。這節適用於穩定目錄上的一般檔案整理。

- Q1：walkdir設定IntoIterator產每項ResultDirEntry/Error，不保排序。
- Q2機制／性質（已足夠）：打不開dir不能取內內容但下一項可續；root/innerlink兩開關預設不同。
- Q2獨立需要／用途（已足夠）：把逐層進入/進度交library，自己的工作仍是項目政策/輸出；保錯不能filter掉。
- Q3：驗root後兩false，collectgenericIntoIterator，Err存issuecontinue，sortbyPathcmp。
- Q4：Ok/Err/Ok受控證後項仍處理，穩定目錄適用不假裝能抗惡意同時替換。

### DirEntry ownership與Option map

首次必要：12.3 collect_entries145之前。

docs/chapters/12-03-lesson.md:122

> | `entry.path()` | `&Path`，借用 entry 保存的原生路徑 | 判 Unicode、交給 observe |

docs/chapters/12-03-lesson.md:123

> | `entry.into_path()` | 接手 entry，交回 owned `PathBuf` | 把位置保存到結果，之後不用原 entry |

docs/chapters/12-03-lesson.md:124

> | `error.path()` | `Option<&Path>`，借用錯誤中的位置 | 有位置才保存 owned 副本；None 如實留下 |

docs/chapters/12-03-lesson.md:128

> 這裡另外用 **`Option::is_none()`**：shared 借用 Option、回傳 bool，只有 None 才是 true。它檢查的是能否表示路徑，不會自動改造資料。

docs/chapters/12-03-lesson.md:130

> 為了保存錯誤裡可能有的路徑，再選 **`Option::map(callback)`**：接手 Option，Some 時將內容交給 `FnOnce` callback，再把回傳值包成 Some；None 時不執行 callback，仍交回 None。因此把 `Option<&Path>` 轉成 `Option<PathBuf>` 可以只寫 `map(|path: &Path| path.to_path_buf())`。這與第9.5節逐筆 iterator map 的數量不同，只處理零或一份資料；沒有丟棄原始 walkdir 錯誤。

- Q1：entrypath借、into_path接owner；ErrorpathOptionborrow可能None。
- Q2機制／性質（已足夠）：Optionmap接零或一內容交FnOnce回包Some，None不執行，與iteratormap數量不同。
- Q2獨立需要／用途（已足夠）：需保存可能的錯誤路徑owned副本而保原錯誤，不虛構位置。
- Q3：map(path.to_path_buf)，to_str.is_none檢可表示性非改資料。
- Q4：入口match錯誤路徑取完再move原error，正entry失敗則into_path，只用當下已教API。

### 原生Path身分、Borrow查詢與排序

首次必要：12.3 collect排序145/完整main159/答案200前。

docs/chapters/12-03-lesson.md:134

> 第10.5節已教 `Path::display()` 可能無法忠實表示非 Unicode 路徑。兩份不同原生資料可能顯示成相同文字，所以不能把 `format!("{}", path.display())` 當成核心身分或再拿去開檔。

docs/chapters/12-03-lesson.md:136

> 本例核心一直保存 `PathBuf`；報告的最小政策則只接受 `to_str()` 成功的檔案路徑。失敗者保留原生位置、動作與 `NonUnicode` 原因，跳過該檔案，其他項目繼續。錯誤顯示以 `Debug` 檢查原生資料，不悄悄改成另一個檔名。

docs/chapters/12-03-lesson.md:138

> 第6.5節建立了map鍵的`Eq`／`Hash`契約，第9.7節則建立`Ord`排序契約；`Path`／`PathBuf`也提供這些能力。它們依原生路徑組成比較、分組與排序，沒有查磁碟證明是同一個實體檔案。`PathBuf` 以 `Borrow<Path>` 提供對應 borrowed key，因此 `HashMap<PathBuf, u64>` 可以用 `&Path` 查詢。這和第6.5節用 `&str` 查 `String` 鍵是同一份身分一致性的契約。

docs/chapters/12-03-lesson.md:156

> main 顯示成功資料與項目錯誤，最後以第10.4節已教的 slice `is_empty()` 判斷有沒有項目失敗。這裡 `for` 接手 `report.files` 欄位，但仍可借用互不重疊的 `report.issues`。這回用第4.5節的partial move：只移走部分欄位，其餘欄位仍可使用，已不完整的整份 `report` 則不能再當完整值交出去。本例只示範元件結果，沒有把它包裝成完整 CLI 的結束狀態政策：

docs/chapters/12-03-lesson.md:193

> 把 `scan` 的成功記錄放到 `HashMap<PathBuf, u64>`，再用 `&Path` 查教材的 `nested/second.txt`，應得到 `Some(&5)`。

- Q1：PathBufown原生位置非display字串，PathEqHashOrd只按組成不證實體同file。
- Q2機制／性質（已足夠）：BorrowPath保持同鍵契約，可HashMapPathBuf用&Path查；nonUnicode按本報政策issue保原資料。
- Q2獨立需要／用途（已足夠）：顯示可能不同原生資料同文字，核心身分/開檔需保原名。
- Q3：sort平台內固定；recordpathmove而snapshotlenCopy，reportfilespartialmove不妨issues。
- Q4：nested查Some&5仍報issueslen，不能由一項查到推整掃成功；fullanswer同頁。

### Unix fixture API與受控失敗順序

首次必要：12.3 完整tests184之前。

docs/chapters/12-03-lesson.md:172

> - `fs::create_dir(path) -> io::Result<()>` 建立最後一層目錄；父目錄必須已存在，不能當成一次建立所有上層。

docs/chapters/12-03-lesson.md:173

> - 在 `#[cfg(unix)]` 的測試中，`std::os::unix::fs::symlink(original, link) -> io::Result<()>` 建立指向 original 的 link；`std::os::unix::ffi::OsStringExt` 為 OsString 提供 `from_vec(Vec<u8>) -> OsString`，接手 Unix 原生檔名字節。`vec![255]` 不是合法 UTF-8，可在 Unix 作可控制的非 Unicode 材料。其他平台不編入這些測試。

docs/chapters/12-03-lesson.md:175

> 測試另外直接建立三筆 iterator 材料：一般檔案的 Ok、不存在路徑的 Err、另一份一般檔案的 Ok，再把這個 Vec 交給 `collect_entries`。`WalkDir::new` 也能以一般檔案作 root，會產生一筆；不存在的 root 則交出 Err。因此可以驗證中間失敗有保留，且後面的成功項目仍有處理，沒有靠目前登入者的權限判定「一定不可讀」。這是收集元件的受控材料，不是把正式 `scan` 的 root 驗證關掉。

docs/chapters/12-03-lesson.md:179

> 這是實際使用的完整`tests/scan.rs`，含上述材料建立與八項檢查；Unix專用三項使用第12.1節的cfg與剛介紹的API，其他平台不編入：

- Q1：create_dir單層；symlink與OsStringExtfromvec是Unix專用資料建立。
- Q2機制／性質（已足夠）：vec255可原生非UTF8，cfg排其他平台；WalkDirfile一筆/missingErr可直接組Vec項目。
- Q2獨立需要／用途（已足夠）：驗政策與中間失敗繼續，不靠目前權限碰巧失敗；正式scanroot驗證未被關掉。
- Q3：TempDiro保到掃描完，完整8tests含3Unix。
- Q4：hidden空file、空dir、file/missingroot、observeNotRegular、rootlink/innerlinks/nonUnicode各完整同頁。

### 大小→摘要→內容的工作分配

首次必要：12.4 policy7、size86之前。

docs/chapters/12-04-lesson.md:3

> 第12.3節已保存一般檔案的原生路徑與觀察資料。現在要回答：**哪些不同路徑的檔案，讀出的 bytes 完全相同？** 檔名相同、長度相同、修改時間相同都不足以回答這個問題。

docs/chapters/12-04-lesson.md:9

> | 依 byte 長度分組 | 長度不同，完整 bytes 必定不同 | 同長檔案可能是 `AAA` 與 `BBB` |

docs/chapters/12-04-lesson.md:13

> 先分大小，是利用已經取得的 metadata 減少讀取工作：若 6-byte 檔案只有一份，就沒有另一份相同長度可配對。對留下的檔案再算摘要，讓直接內容比較集中在更小的候選桶，而不是一開始就讓所有檔案兩兩重讀。這是工作分配的理由；本節沒有測量或承諾特定倍數的加速。

docs/chapters/12-04-lesson.md:15

> 本節使用`examples/ch09-content/lesson-workspace/`的獨立workspace，以下命令都從**教材repo根目錄**執行。依工作選四個階段，避免只看大小時就載入後面的內容分群：

docs/chapters/12-04-lesson.md:83

> 教材的三份檔案長度為 5、5、6，應只留下 5-byte 的兩筆候選。先收集，再將 `(長度, 候選清單)` 排序；map 自己的走訪順序仍無保證：

- Q1：長度不同排除，同摘要仍候選，真正讀完整bytes相等才判相同。
- Q2機制／性質（已足夠）：先利用既有metadata，再摘要縮小直接比較桶，不能由相同長度/時間推內容。
- Q2獨立需要／用途（已足夠）：原需要不同路徑哪些bytes真正相同；避免所有file兩兩重讀，未承諾特定加速倍數。
- Q3：sizeHashMapu64→Vec借FileRecord、收rows排序，5/5/6只留5桶。
- Q4：四獨立package避免size早載refine；6singleton排除由無同長對象直接推，不靠演算法名推需要。

### hash/digest/collision與BLAKE3 API

首次必要：12.4 hash_reader119與streamsfullsource176前。

docs/chapters/12-04-lesson.md:97

> **hash** 在這裡指把任意長度 bytes 算成一份固定長度摘要的工作，摘要叫 **digest**。相同 bytes 必須得到相同摘要；不同 bytes 也可能得到相同摘要，這種情況叫 **collision**（碰撞）。因此摘要是縮小候選的工具，不能替代本次「真正相同內容」的判準。

docs/chapters/12-04-lesson.md:99

> 我們選用 **BLAKE3** 演算法，由 `blake3` crate 提供。只使用它的普通、依序更新 API，不另開平行讀取或記憶體映射功能。依第10.1節的依賴設定，先在streams階段宣告：

docs/chapters/12-04-lesson.md:108

> | `hasher.update(bytes)` | mutable 借用 hasher、接收 `&[u8]`，追加這次有效 bytes；回傳同一 hasher 的 `&mut Hasher`，本例不使用回傳值 |

docs/chapters/12-04-lesson.md:112

> 最後一列的 `blake3::Hash` 是**保存摘要的型別**；第6.5節的 `std::hash::Hash` 則是「能提供 map 鍵雜湊」的 **trait**。它們同名但角色不同。摘要型別實作這個 trait，所以能當 HashMap key；map 的鍵契約不因此替我們證明兩個原檔案內容相同。

docs/chapters/12-04-lesson.md:114

> 按順序 `update("ABC".as_bytes())` 再 `update("DE".as_bytes())`，和一次送入 `ABCDE` 是同一串輸入。第10.4節的 Read 可能短讀取，因此每次只送 `buffer[..count]`，不能把舊尾段一起算入，也不能因第一次沒填滿就停止。

- Q1：hash任意bytes→固定digest，collision異bytes可同；BLAKE3具體library。
- Q2機制／性質（已足夠）：orderedupdates與一次同序bytes一致；update借mut，finalize借shared不清，Hash32byte具鍵能力。
- Q2獨立需要／用途（已足夠）：縮小候選而非取代內容判準；Hash摘要型別不同stdHash trait，mapEq不能證來源內容同。
- Q3：只buffer有效前段/Interrupted重試/其他Err返，empty或非UTF8也合法摘要。
- Q4：ABC+DE同ABCDE，9001不同正上限、empty/nonUTF8tests；不必先教BLAKE3內部才能用契約。

### fill_chunk與短讀邊界正規化

首次必要：12.4 圖128/fill135/equal143之前。

docs/chapters/12-04-lesson.md:126

> 同一串 `ABCDE`，左來源可以先交 `ABC` 再交 `DE`，右來源可以每次只交一個 byte。若只呼叫一次 read，看到左長 3、右長 1 就回報不同，判斷會是錯的：切分是讀取行為，完整序列才是內容。

docs/chapters/12-04-lesson.md:130

> 圖中直線分隔**原始 read 的批次**，不是內容裡的 byte。向下箭頭把批次重新接成相同位置的比較段。圖為了看清楚用四格，實作使用 4096 格；容量改變不改判準。在這張四格圖中，左邊第一次收 ABC 後只剩一格，下一次 read 只能交 D；填滿第一段後，下一段才收 E。因此左邊實際批次是 ABC、D、E，仍和右邊逐 byte 的內容一致。

docs/chapters/12-04-lesson.md:132

> 先做 `fill_chunk`：對每邊分別重複 read，直到這份 buffer 滿，或非空剩餘 buffer 得到 `Ok(0)`，表示這輪來源結束。`filled` 是這份 buffer 目前已保存幾格；下一次只借剩餘後段，不覆蓋前面新收到的 bytes：

docs/chapters/12-04-lesson.md:138

> 接著比較兩份有效 slice。`[u8]` 的 PartialEq 要求**長度相同且每個位置 byte 相同**；不同長度也會比較為不同。這時兩邊都已填到相同容量或結尾，才能把段長不同當成完整內容長度的差異，而不是不同短讀取的差異。

docs/chapters/12-04-lesson.md:140

> `CompareReadError::Left(io::Error)`／`Right(io::Error)` 保留失敗來源。I/O 失敗不是 `Ok(false)`；否則後面會把「不能確認」誤當「已確認不同」。兩份 slice 都空且相等時，兩邊都已結束，才交回 `Ok(true)`：

- Q1：原read切分是來源交付批次，非內容不同；填到相同容量或EOF再比段。
- Q2機制／性質（已足夠）：filled保存已填量，後續只借剩餘；sliceeq長度+位置每byte；都空才true，讀失敗區分Left/Right。
- Q2獨立需要／用途（已足夠）：同ABCDE left3/right1不能因一次read長度不同誤判內容，需保真正序列。
- Q3：loopfill兩邊4096，先比較有效段false/emptytrue；圖4格而code4096限定清楚。
- Q4：圖ABC/D/E vs逐byte→ABCD=ABCD再E=E；true完整、false早遇差可停，不誤拿Err當false。

### ShortReader/事件reader驗收與完整streams

首次必要：12.4 short151/failure159/main166/第一run185前。

docs/chapters/12-04-lesson.md:148

> 回收第10.4節的ShortReader：保存 owned bytes、目前位置與每次上限。`short_reader(data, maximum)` 複製 byte slice 成 Vec，建立每次至多交出 maximum 格的來源；maximum 必須為正數，零會 panic，因為那不是這份測試來源的有效設定。

docs/chapters/12-04-lesson.md:154

> 回用第2.7節的 **`assert!(condition)`** 核對 bool：true 時繼續，false 時 panic；`assert!(!condition)` 則要求原條件為 false。它和第2.7節的 `assert_eq!` 一樣用於檢查已知答案，不是處理使用者配置錯誤的介面。

docs/chapters/12-04-lesson.md:156

> 另一份 `failing_reader()` 沿用第10.4節的事件來源：第一次 Interrupted、第二次交出 R、第三次 PermissionDenied。hash／比較都要重試第一次，再保留第三次失敗；比較還要指出是左還是右來源。完整 source 只有這三種已教事件：

docs/chapters/12-04-lesson.md:163

> streams的main分別以3與1的上限讀相同資料，先比較完整內容，再對照分批 hash 和一次 update。這讓「切分不同但內容相同」有實際執行的材料：

docs/chapters/12-04-lesson.md:171

> 前面已列完整manifest與main。以下兩檔是main實際載入的完整library與readers module，包含所有imports和剛介紹的兩種來源：

- Q1：短來源ownedbytes/position/max，失敗來源step事件；assertbool檢已知答案。
- Q2機制／性質（已足夠）：max0不是有效source；Interrupted→R→PermissionDenied，retry後仍不得回局部成功。
- Q2獨立需要／用途（已足夠）：需可控展示切分無關與左右失敗歸屬，不依真正File剛好短讀。
- Q3：所有library/readers imports/helper全檔在首次run前；斷言用在固定材料。
- Q4：六tests9001跨4096、最後byte/短尾、empty兩種、非UTF8、兩側error；本輪只查當前修後版。

### checked File觀察、原生失敗者與限制

首次必要：12.4 issues205/observe211/hash217/equal223之前。

docs/chapters/12-04-lesson.md:192

> Read 的流程正確，仍不足以把檔案當成固定內容。其他程式，或本程式另一項 `fs::write`，都能在 File 打開期間修改磁碟資料；shared reference 限制的是這份 Rust 借用，沒有替檔案系統所有操作上鎖。

docs/chapters/12-04-lesson.md:194

> 沿用第12.3節的 `Snapshot`，本節作**盡力而為的觀察**：開啟前、開啟後的讀取前、hash／內容比較後，都觀察類別、len、modified。除了按原生路徑呼叫 `observe`，再用新 method **`File::metadata() -> io::Result<Metadata>`** 查已打開的 File handle 所指檔案。兩者指向資料的方式不同；每次成功觀察都和掃描記錄比較。

docs/chapters/12-04-lesson.md:202

> `FileIssue` 保存確切失敗者的原生 `PathBuf`、當時動作，以及 I/O／metadata／可觀察變動的具體原因。比較左邊失敗就指左路徑，右邊失敗就指右路徑；不是一律指新候選。Display 用 Debug 路徑供人辨認，仍不拿顯示字串當 key：

docs/chapters/12-04-lesson.md:208

> `observe_handle` 查 File，`verify_unchanged` 查路徑及 handle，兩份都要和原記錄相等。`open_checked` 在打開前先查路徑，再打開並驗證。後續 wrapper 必須在讀取後再次驗證，不能只呼叫 open_checked 就宣稱整個讀取過程沒有變動：

docs/chapters/12-04-lesson.md:220

> `equal_files` 開兩份檔案，按剛才的左右錯誤保留確切失敗路徑；就算 bytes 比較得到 false，也完成兩邊讀取後的觀察，再交回這個正常比較結果：

docs/chapters/12-04-lesson.md:226

> 這些檢查能揭露**可觀察到的差異或讀取失敗**。沒有看見差異，不代表取得一致快照：例如內容可能改過又恢復成相同長度／修改時間，時間記錄也有其精度。前後觀察不是原子操作，路徑替換也可能發生在兩次查詢之間。本節適用於穩定目錄的一般整理，不承諾抵抗惡意同時替換或保證檔案往後不變。

- Q1：Filemetadata查已開handle，observe查路徑；snapshot僅類別/len/time觀察。
- Q2機制／性質（已足夠）：open前與開後讀前、成功read後path/handle都比原snapshot；False也驗後觀察，左右錯路徑精確。
- Q2獨立需要／用途（已足夠）：外部或同程式fswrite可改disk，Rustsharedborrow不鎖FS；需揭露可觀察變動與無法確認。
- Q3：hash/compareResultFileIssue，Error不中途當正常false或部分成功。
- Q4：File開期間讀完再write短檔/verify測可見差，明非每read同步操控；相同len/time恢復與時間精度仍可能漏，不保原子一致。

### tuple候選key與階段閉包

首次必要：12.4 candidates255/root264前。

docs/chapters/12-04-lesson.md:250

> 這次以 `(u64, blake3::Hash)` 作 key：第一欄是長度，第二欄是摘要。標準 tuple 在兩欄都有 Eq／Hash 時也提供相應能力，只有**兩欄都相等**才是同一鍵。因此不同長度不會進同桶，同長度同摘要則仍等待內容確認。

docs/chapters/12-04-lesson.md:252

> `DigestBuckets` 是第8.2節已教的type alias，只替這份 `HashMap<(u64, Hash), Vec<FileRecord>>` 取較短名稱。`hash_candidates` 接手 owned 記錄，成功才交回候選 map；第一次 FileIssue 就返回，沒有把之前的局部 map 當完整成功：

docs/chapters/12-04-lesson.md:252

> `DigestBuckets` 是第8.2節已教的type alias，只替這份 `HashMap<(u64, Hash), Vec<FileRecord>>` 取較短名稱。`hash_candidates` 接手 owned 記錄，成功才交回候選 map；第一次 FileIssue 就返回，沒有把之前的局部 map 當完整成功：

docs/chapters/12-04-lesson.md:260

> 下面只把先前已知會留下的兩份 5-byte 檔案交給這個元件。filter 借用 owned Item 判斷，保留後的 FileRecord 才 move 進 collect，沿用第9.5節：

- Q1：tuple兩欄EqHash才mapkey；alias只縮短HashMap型別。
- Q2機制／性質（已足夠）：length與digest皆同才桶同，ownedrecord成功移桶，首Err不回部分map。
- Q2獨立需要／用途（已足夠）：把大小與摘要保為候選鍵，不能把查到同digest當真內容結論。
- Q3：testmodule屬example，library無candidates，filter借FileRecord保留者movecollect。
- Q4：realkey3桶/AAA2、missing中途Err精確路徑兩test完整；不提前依賴groupsmetrics。

### 各真正群組代表、borrowed group與singleton驗證

首次必要：12.4 圖284/refine297之前。

docs/chapters/12-04-lesson.md:280

> 假設故意把三份 3-byte 記錄放進同一 digest 桶，依序為 a 的 `AAA`、b2 的 `BBB`、b1 的 `BBB`。這是**受控同摘要的測試安排**，不是找到 BLAKE3 真實碰撞的實驗。

docs/chapters/12-04-lesson.md:282

> a 先建立第一組；b2 和 a 不同，建立第二組；b1 和 a 不同之後，還要和第二組的 b2 比，才能加入 B 組。若只和桶第一份 a 比，就會漏掉 b1／b2：

docs/chapters/12-04-lesson.md:288

> `ContentGroup<'a>` 保存 `Vec<&'a FileRecord>`，借用來源記錄，沒有複製檔案。`refine_bucket` 的回傳 `ContentGroup<'_>` 沿用第7.1／7.2節的單一reference來源：回傳借用連到輸入 slice，來源記錄必須在使用群組時仍有效。

docs/chapters/12-04-lesson.md:290

> 元件先以 `HashMap<PathBuf, ()>` 記已處理過的原生路徑。unit 值只是「看過這個 key」的標記；同一路徑重複輸入會略過，不當成兩份檔案。每個新候選再依序和各組第一位代表比較，相同就加入；全部不同才建立新組。

docs/chapters/12-04-lesson.md:292

> 新組只有一位時還沒有另一份資料能比較，所以仍以 `hash_file` 完整讀一次，驗證可讀性與前後 metadata；不用它的摘要當內容判準。這會增加一次讀取，本例選擇把獨立元件收到的候選也完整驗證，而不是假設 caller 剛才一定已讀完。

docs/chapters/12-04-lesson.md:294

> 最後只改變 reference 的排列：各組先按原生路徑排序，再依各組最小路徑排序。`sort_by` callback 的元素是 `&FileRecord`，因此參數是 `&&FileRecord`，和第9.5節的 reference Item 多一層借用同樣道理。元件建立的每一組都有至少一位，取第一位的索引才有效；空輸入則回傳零組：

- Q1：ContentGroup借來源FileRecord；候選桶可含多真正內容群。
- Q2機制／性質（已足夠）：每新候選依次對各組代表，首次相同加入，全部異新組；dedupsamepath、singleton也hash讀驗。
- Q2獨立需要／用途（已足夠）：A/BBB/BBB若只比首A會漏BB；獨立元件不能假設caller剛讀，仍驗可讀與前後觀察。
- Q3：groupslifetimereturn連輸入slice，sortrefcallback&&FileRecord，組非空索引有效，空輸入零組。
- Q4：手工forcedkey非真BLAKEcollision，a,b2,b1實測排序a／b1b2；完整code先教全部需要後才metrics合併。

### 首錯回傳與確切失敗者重試

首次必要：12.4 refineResult297語義收束302／完整測試367之前。

docs/chapters/12-04-lesson.md:302

> 遇到第一次 FileIssue，元件交回 Err，不交出尚未完成的群組。caller 要保存那筆問題，以原生路徑移除**確切失敗者**，才可對剩餘候選重試；一次報出的錯誤不能證明其他項目都沒問題。每次確實移除一份已在候選中的失敗路徑，才有有限次進展。完整工具的繼續／報告政策留到練功專案；這裡不把 Err 當不同內容，也不默默丟棄有效候選。

docs/chapters/12-04-lesson.md:302

> 遇到第一次 FileIssue，元件交回 Err，不交出尚未完成的群組。caller 要保存那筆問題，以原生路徑移除**確切失敗者**，才可對剩餘候選重試；一次報出的錯誤不能證明其他項目都沒問題。每次確實移除一份已在候選中的失敗路徑，才有有限次進展。完整工具的繼續／報告政策留到練功專案；這裡不把 Err 當不同內容，也不默默丟棄有效候選。

docs/chapters/12-04-lesson.md:302

> 遇到第一次 FileIssue，元件交回 Err，不交出尚未完成的群組。caller 要保存那筆問題，以原生路徑移除**確切失敗者**，才可對剩餘候選重試；一次報出的錯誤不能證明其他項目都沒問題。每次確實移除一份已在候選中的失敗路徑，才有有限次進展。完整工具的繼續／報告政策留到練功專案；這裡不把 Err 當不同內容，也不默默丟棄有效候選。

- Q1：Err是本輪無完整群組，不是已確認不同或無其他問題。
- Q2機制／性質（已足夠）：保存issue按原生path移確切offender再重建，每次去一已在候選者才有限進展。
- Q2獨立需要／用途（已足夠）：要保剩餘有效候選而不吞錯，完整工具繼續報告政策18待整合。
- Q3：測試Vecintoiterfilter與9.5一致，沒有memtake/retain等新API藏test。
- Q4：missing夾AAA/BBB/BBB，Errmissing→filter精確者後兩組保BB；單次成功不替其他項目驗證。

### 路徑重複量、checked整數與hard link

首次必要：12.4 metrics317、完整tests367之前。

docs/chapters/12-04-lesson.md:306

> 一組相同內容若有 `path_count` 個不同路徑，每份長 `len` bytes，留下其中一路徑的內容後，其餘路徑的重複內容量是 `(path_count - 1) × len`。三份各 3 bytes，就是 6 bytes；兩份各 3 bytes，就是 3 bytes。空內容的重複量是零，零／一路徑也回報零。

docs/chapters/12-04-lesson.md:308

> 這是**按路徑**的指標。**hard link**（硬連結） 是同一個實際檔案的另一個名字；兩個 hard-link 路徑仍在這個指標中各算一路徑。因此數字不是實體磁碟可回收量，也不是刪除建議。

docs/chapters/12-04-lesson.md:310

> 長度是 u64、清單數量是 usize。沿用第8.4節的TryFrom契約，這裡選 `u64::try_from` 轉換額外路徑數。這份標準整數轉換的錯誤型別是 **`std::num::TryFromIntError`**，表示來源數字無法由目的整數型別表示；用 `use std::num::TryFromIntError;` 引入。它已有 Debug，所以 `MetricError` 保存它之後也能 derive Debug。這是本節首次選用的標準錯誤型別，不是第8.4節的自訂Count轉換錯誤。

docs/chapters/12-04-lesson.md:312

> 再選本節新 API **`u64::checked_mul`**／**`checked_add`**。兩者按值接收數字，成功回傳 `Some(result)`，超出 u64 範圍回傳 None，不繞回較小數字。

docs/chapters/12-04-lesson.md:353

> 讀取後觀察測試先讀完，再以 `fs::write` 改成不同長度，最後呼叫與 wrapper 相同的 `verify_unchanged`；它驗證這項觀察能揭露變動，不佯稱在測試中同步修改每次 File read。溢位材料用 **`u64::MAX`**，這個 associated constant 是 u64 最大可表示值，無需建立巨大檔案。

docs/chapters/12-04-lesson.md:355

> Unix 的 hard-link 測試另用 **`fs::hard_link(original, new_path) -> io::Result<()>`** 建立指向同一實際檔案的另一名字，驗收兩個路徑仍算 3 bytes；這項 fixture 不編入其他平台，沒有把平台未跑的結果算已驗證。

- Q1：duplicatebytes是按不同路徑去一份後的重複內容量，非實體回收空間或刪除建議。
- Q2機制／性質（已足夠）：usize→u64TryFrom保錯，checkedmul/addSome或NoneOverflow；hardlink同實檔兩名仍兩path。
- Q2獨立需要／用途（已足夠）：報告量不能因integerwrap虛假變小，也不能把候選三份直接算6而漏內容分群。
- Q3：0/1path/emptycontent零，3bytes×(2−1)=3；u64MAX作溢位材料不造巨大檔。
- Q4：forcedA/BB總3，兩hardlinkpath仍3（Unixcfg）；convert自訂Count錯不同本次標準TryFromIntError明教。

## 閉包、完整材料與累積負荷

54直接refs/15regions snapshot外，另讀存兩adjacent.expected（E0425、missing_env完整訊息）、repo Cargo.toml/lock、engineeringlesson與contentlesson各lock，合60檔。完整盤點selectedpackages除target產物外無漏檔，沒有未揭露library/build/.cargo或helper；第12.2的build.rs就是當節新教且完整展示的本體。repo根三設定區段與repo-root-settings.toml節錄逐值一致；只看共同package/dependency/profile的使用責任，不把root列出的未讀章節members當Rust先備。walkdir2.5.0、blake31.8.7、tempfile3.27.0來自當前selectedlocks，未執行Cargo。

12.1在獨立lesson-workspace，root/subtotal/app三manifest及subtotal三行library、app完整lib/main、features integrationtests、失敗fixture與完整featureanswer全部讀。條件編譯與cfgbool先解才進程式，alltargets明說也編下方已教操作答案。商品160與280規則獨立於setting，release不靠profile數字推速度；最佳化只在本節需要的編譯設定角色/代價層，沒有使用未教的具體最佳化機制推論。根設定節錄明標非完整manifest，未讓讀者覆蓋repo根。

12.2切回repo workspace明說。完整manifest/build33行/library三函式/main9行/兩example與embeddedtests均讀。VarErrorNotUnicode持原OsString、有限mode先驗後協定、unitreturn+Cargoerror令buildfail、targethost與三時間點都在首次必要前；env!雖10.8已用過compiletime CARGOBIN契約，本節再完整教其static文字/缺值及option_env的差別。先給外部配置固定需要，才build流程，先改build再改runtime與圖/程式一致。OUTDIR/builddeps/linker只基本角色預告且明說本體不用，ABI完整說明待15合理，不拿未教C工具鏈為當前操作。

12.3ScanSnapshot/Record/Cause/Issue/RootError在型別region前已給用途，SystemTime及u64能力亦先教。observe查當前notlink一般file與modified，scan先validateroot後walker，collectEntry逐Err保存continue。完整scan.rs160行與tests154行全部讀，沒有filter_map(Resultok)吞錯。Optionmap/isnone與PathBorrow新用途在scanregion前已解。3固定fixture完整bytes分別RustLF5、RustLF5、ABCSELF6，與metadata輸出一致；測試動態fixture完整literal/bytearray：hidden空檔、空目錄、rootfile/missing、rootlink/innerlinks、nonUnicode原生255；Unix建立API在tests前說，非借OS權限碰巧fail。八test與pathidentity答案完全內嵌，partialmove明回4.5精確角色，而不是先讓reader從helper猜所有權。

12.4四獨立成員按首用load：size只有scan；streams兩Read函式加fill與兩受控reader；checked-files僅觀察/hash/equal，不帶candidate/metrics；candidate是example ownmodule、兩test全檔在本步；groups最後才refine/metrics，main為TempDir受控答案，tempfile放一般dependency有真使用理由。streams讀取helper在第一run前完整，故不因Cargo只選run就偷載未知source；本輪不讀更早版、不說獨立發現root已修時序。

實際完整source核對：hashreader與fill都只validprefix/剩餘slice，Interrupted重試；equalreaders先正規化兩段再比，Left/RightIOfail不同正常false；true才證全序列讀到底。checkedFile路徑/handle觀察與左右錯誤歸屬吻合正文，正常false仍做讀後觀察。新組singleton也hash讀驗是獨立元件政策，沒有從同digest推內容；seenPathBufunit去重與每組代表完整搜尋，使forced A/B/B分1/2，排序b2到b1/b2有真不同輸入次序。指標先獨立給(pathcount−1)×len，再u64TryFrom/checkedmul/add/Count/Overflow，MAX/hardlinkAPI在最後tests前。六streams、兩candidate、十groups（含Unixhardlink）測試全讀；測試中真正檔案讀完再改長度證verify觀察，不假裝每read同步競爭；缺路徑失敗者filter精確移後再驗其餘group，沒有未教retain/memtake藏test。

同頁完整來源/答案不是外部延伸才能補：scan、streams與checkedFile完整imports/helper放可展開「完整檔」區，但當下主要新規則、必要region/材料/圖都先在正文教；完整tests/answers區只由已教操作組合。必要概念不留到延伸Cookbook補，兩處還明說那些短例可能略錯、這裡要留錯。最後workspacealltargets命令到所有四成員/answer都說完才出現，未把filter當不編其他模組。12.4完整CLI與部分失敗報告留18合理：當前finitecomponent公開首Err及重試所需確切路徑/進展，而未承諾整個工具已完成。

## 實頁與必要圖

使用root另建8102preview；四canonical copy先與本輪snapshot逐檔hash相同，非拿8097舊HTML充新。四頁各1280×800與390×844，時點10:00:39–10:01:13 UTC/servedHTMLhash詳visualmetadata。35pair圖內70原截圖全部親看，精確清單JSON；全9表實看無橫向溢出（手機table343、wrapperclient=scroll375），型別/指令部分換行仍保同行對應，沒有先前四欄表需左右切換問題。

3圖每張皆桌面/手機原位看；同340px寬，字24px可讀，servedhash吻合snapshot，箭頭/標記/位置逐核：

- 12.2 build-phases340×570：1編script供host→2執行讀LESSONMODE交TEACHINGMODE→3compile主要crate/env固定→4執成品不重讀。向下箭頭是先後，段落68明連第二到三到四，不需看輸出倒猜同一份mode在哪固定。
- 12.4 short-read-comparison340×455：原始read左ABC|D|E與右A|B|C|D|E，藍框批次→黃框各填4或EOF→綠框第1ABCD相等/第2E相等。分隔線不是資料byte，正文130先對4格圖說左剩1格僅D、下一段E，實作4096差異也交代；同字串/同位置比較關係可追，沒有把一read長度當內容長度。
- 12.4 candidate-content-groups340×480：上手工同digest aAAA/b2BBB/b1BBB皆3bytes→中b1先a不同再b2同、要各組代表→下真群1a／群2b1b2。圖/286對a.txt/b2.txt/b1.txt明確，候選桶與真正群組不同層；箭頭為分群工作，無真BLAKE碰撞實測暗示。兩個主要新設計（多代表比較、按路徑指標）仍各有文字需要與例子，未拿一圖代答全部。

實看全部10folds closed/open：12.1 featureanswer；12.2 identityanswer；12.3完整scan、全8tests、pathidentityanswer；12.4 repeatedscan、完整streams/readers、checkedfile全library、groups完整main答案、streams+groups全部tests。另capture buildscript完整尾、candidate全module/tests尾、groups完整metrics尾。50fullinclude每viewport exactDOM matching，15regions人工依actual完整檔首用核；無必要步驟只給外部檔案名字或片段後直接run。長碼手機橫捲仍是現行呈現方式，不稱全碼一屏可見。

## 限制

本角色不是blindfirstreader，12.4已知版本背景如首段保留；沒有讀其他角色12初判/validation/knownaudit，也沒有用13+補。頁nav露13.1標題不算已教，未讀其body。沒有Cargo/build/tests執行、BLAKE3內部技術實跑或跨平台驗證。partialpreview不是finalstrictsite/全書導航驗收，結論限定實際60檔版本與兩尺寸。
