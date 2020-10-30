# lettre で日本語メールを送る

## 手順
[src/lib.rs](src/lib.rs) のテスト部を改修して、  
`$ cargo test`

問題なければ宛先アドレスにメールが届く  
[届いたメール](届いたメール.eml)  

## ポイント
`header::ContentTransferEncoding`  
`header::ContentType`  
を適切に設定してからビルドしてあげれば自動的にエンコードされるようです

参考：https://github.com/lettre/lettre/blob/master/src/message/mod.rs
