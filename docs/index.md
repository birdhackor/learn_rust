# Rust 教學

這份教學以正體中文解說 Rust，面向具備基本 Python 經驗的讀者。

目前可閱讀第 1–14 章。從環境安裝、Cargo、crate 與測試開始，再逐步學 ownership、reference、泛型、trait、lifetime、錯誤處理與 Iterator，組成可用的文字搜尋 CLI。接著學 smart pointer、檔案內容處理、thread、同步與 atomic，再以 async、有限排程與取消完成 HTTP 實作和受控本機測試，接著理解 GAT、variance、raw pointer 與 unsafe，將 C 契約包成安全的 FFI 介面。從資料位置串起 Pin、Future、Waker 與小型 executor 後，再學 macro、API 相容性、property testing、Loom、Miri、benchmark 與 profiling。

最後一章將完成兩個引導型專案。

教學範例會依序使用已經介紹的概念，讓你能邊讀、邊理解、邊動手寫。

[從 1.1 開始閱讀](chapters/01-01-start.md)

[查看 GitHub 專案](https://github.com/birdhackor/learn_rust)
