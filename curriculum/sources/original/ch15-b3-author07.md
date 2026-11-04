# 15.9 B3 作者來源：有限 task 與完整輸入對應

本輪只寫15.9，15.10／15.11關閉；依clear-tutorial／writing-prompt／Rust術語。A、B1、B2與所有frozen不改。

## 寫前實際前文

- actual `ch15-b2-prerequisite-v9` 七BODY與whole兩packages十一archives全部實讀：原行0-based索引、不先過濾invalid、trim只判空、duplicates、owned Input與typed Check、Empty設定問題、Duration每筆跨度、literal依賴與repo外path、有限guard／cleanup。最初工具合併輸出截掉starter single／checks中段；另完整讀complete source，並逐bytes核兩single相同、starter checks整mod等於完整讀過的complete module，不用缺失部分猜內容。
- actual `ch11-bounded-prerequisite-v6` 九BODY全讀，whole lib/outcomes/main/Cargo/CLI/one-slot答案全source實讀。第一次合併結果截掉source後段，另取完整lib及其餘檔補讀。pending是data-only IntoIter；JoinSet len含完成未收取、回收之後才fill；new需要runtime；tracked/submitted與peak角色、Id登記、故障後補工均已教。
- actual `ch11-tasks-prerequisite-v5` needed002–006完整：Future+Send+static、Output bound、Clone sender／ready、JoinHandle及AbortHandle不同、Id Copy/Eq/Hash與重用界線、JoinError id/is_panic/is_cancelled、join_next_with_id兩層／完成順序、HashMap remove、drop JoinSet不等await清理。
- B1／11.4 timeout與borrowedhandle／cooperative限制沿已實讀；11.8 Fixture whole gate已讀，B3重用真ready／held／hits／release／shutdown。6.7排序角色與按原index整理同樣沿實際前文。

## Fresh EN／ZH 完整對應章

重新完整實讀英語 [Async Book Iteration and Concurrency](https://rust-lang.github.io/async-book/05_streams/02_iteration_and_concurrency.html) body15–52與[中文對應章](https://huangjj27.github.io/async-book/05_streams/02_iteration_and_concurrency.html) body49–92。採「每次只等一份，其他工作尚未開始；允許有限份同時等待」的動機，保留固定上限的目的。不要照搬Stream／Pin／dyn／futures／try_for_each_concurrent：本題控制的是Tokio已提交尚未回收tasks，不是那個stream組合器的future數；逐項錯誤不提早終止，正上限仍在提交前拒絕。

fresh [Tokio Spawning](https://tokio.rs/tokio/tutorial/spawning) body23–322全讀，採ownership／task Output與Send/static必要角色；不搬mini-redis／Bytes／socket或每連線無限制spawn，也不搬64bytes等數字當本工具的效能或記憶體結論。fresh [Tokio Unit Testing](https://tokio.rs/tokio/topics/testing) body23–135全讀，採條件受控／timer與I/O不同；本步用真held事件，不以pausedclock推測網路已完成，不搬tokio-test或AsyncRead/Unpin mock。Async Book multiple futures入口全body15–21也讀供方法角色比較。

本次最初Course concurrent-execution頁與docs.rs固定JoinSet頁不可取，沒有列為已讀；改用公開中文Async Book與fixed官方tag source，保留取來源差異。

## fixed selected API 親查

fresh下載Tokio官方tag1.53.3的join_set.rs與reqwest官方v0.13.5 async_impl/client.rs；各全bytes等於當前compiled registry cache。檔案在 work-only `ch15-b3-author`。

- JoinSet selected50–98、284–327、455–474、589–597：追蹤中Id不重用；len包含仍追蹤者；join_next_with_id可等Id/Output或JoinError.id，未取得結果就取消等待不會移除task；abort_all不移除，不等完成；drop只abort，不能當逐項await。只教這些需要的API，不搬poll_fn／poll／Pin實作或join_all會resume panic的另一種policy。
- Reqwest Client selected73–98：具備Clone，內部共享連線池；clone給owned task而非借scheduler外部client，也不在每筆另builder。必要語意會在第一次clone前局部教，沒有假稱11.7已教Client clone。原Input保留raw String，複本給task，因此沒有Check的panic／abort仍能報原輸入；需要相應字串空間，不把jobs界限說成全部輸入／結果記憶體界限。

來源：https://raw.githubusercontent.com/tokio-rs/tokio/tokio-1.53.3/tokio/src/task/join_set.rs；https://raw.githubusercontent.com/seanmonstar/reqwest/v0.13.5/src/async_impl/client.rs 。

## guarded tests／public角色安排

沿actual11.3 Scheduler，test owner直接持排程與其JoinSet；不spawn外層controller後只abort它，因drop JoinSet沒有替children await。觀察guard失敗時abort_all→逐項join到None→Fixture.shutdown→panic；接到正常Output也另收server。ROOT明確採納此方案。scheduler.next內join完成後只做同步分類／移除／fill／return，test取消尚未取得結果的等待保留owner與集合；所有新helper角色、當下觀察與收尾先教。

一般輸入來自prepare_inputs，原index唯一且由小到大，public API對caller的這個契約正文明說；結果按input_index排序，完成順序可不同。零jobs、Empty是設定拒絕，單筆Check失敗與Task JoinError各保存，不因失敗漏後續。工具中止仍後節，test cleanup不當使用者取消功能。

## 交稿前前文更新與時點

環境恢復後，依ROOT指定完整實讀 `ch11-review-fixed-v15` 的008-001到008-013；第一次合併輸出截斷006–009，這四份另完整補讀。新增 `next_started_within` 把entered觀察包在期限內，`finish_client<T>` 封裝owned handle的abort＋await，並先保存所有client／server結果再斷言。原始B3寫前來源仍是先前的11.8-v13，不將這次更新回寫為當時已讀。新前文也已在需要Client clone時教共用pool；B3原本在首次使用前自教的契約仍成立。

B3沒有改用新helper：本節test owner直接持有Scheduler，以自己先教過的timeout觀察原 `next_started`；失敗時對整個JoinSet要求abort、逐份join到None，再shutdown Fixture。既有介面相容，不為這份前文更新改算法或重跑已通過Rust。當時Cargo依賴的是repo working path；未另保存當時Fixture完整bytes，故不冒稱當時編譯的Fixture必然逐byte等於v13或今日v15。正式source-proof保存本次實讀units與fixed官方API的指紋。

source-proof另核B2來源全仍相同；兩份single core逐byte等於B2-v9。Input／InputError／prepare_inputs拆檔時只省略原B2的 `end:models` snippet結束標記，其餘三份內容不變，不把原始串接含多餘標記說成逐byte相同。
