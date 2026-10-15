# 15.6／18.8 root提示後窄複核

三處均已解決，沒有新增finding。這是root在本人各scope初判封存後提供的回指修訂；不算本人獨立初判抓到，原initial／hash保留。

- 15.6:74「前節」改「先前」正確：前節15.5是C，mutable設定與method能力已由更早的3.6等正文建立。
- 15.6:453 str→獨立String的to_owned改回8.5正確：8.5:123–130明示該標準契約與獨立ownership；10.4:73談byte slice→Vec，現有Greeting借讀後複製再drop的例子應指8.5。
- 18.8:53把14.5回指限於Duration時間跨度正確：14.5:9定義跨度，沒有在那裡教Copy。新句在當場交代標準Duration的Copy，再讓每次B1工作取得相同跨度；各自建立期限的語義不變。

15.6 current `9d61d616276c916246f3d83967d34ead056dc35a282e61dfadc8da5c78b2e9ba`；18.8 current `f71c74937f99791d0f4c1160a168c7f0993c3e10b0003beddca14bfe4adfba34`。僅逆轉15.6兩處詞句即重得本人原hash；18.8 root保存before與本人原hash一致，僅施加這一句改寫即得current全文。兩波共67個相關owned Rust／C／header／manifest檔與本人initial一致。

沒有改教材或source，也沒有執行Cargo、Rust、C、HTTP或瀏覽器。逐項hash與證據見JSON；按root指示完成後停止本輪。
