# 第13章前半獨立後續查漏

新增必要項 CL-13-ARRAY-SLICE：13.2 的 thread_join/main.rs:10 使用 sum(&[1, 2, 3])，但array reference→slice coercion直到15.3才明教（該頁還寫「第一次」）。3.7實教的是&prices[..]及range slice；11.4的Deref、11.6的dyn轉換不能代替此規則。

最小改法：先建立local_values array，再sum(&local_values[..])，沿用3.7，不必新增概念。收益是能以已教語法追蹤傳給sum的slice來源。

本項由獨立讀15.3時自發回查發現，非root／作者提示；保留13prefix早期無問題初判，不倒寫。詳見independent-13prefix-addendum-array.json。
