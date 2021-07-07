# ca-tech-dojo-rust

Techbowl 社が運営するサービス TechTrain のミッションの一つである「[CA Tech Dojo サーバサイド (Go)編](https://techbowl.co.jp/techtrain/missions/12)」を Rust で実装したものです

## 仕様

本来は Go, MySQL で実装することになっていますが、今回は
- Rust(Rocket) https://github.com/SergioBenitez/Rocket
- diesel https://github.com/diesel-rs/diesel
- MySQL  

で実装しています

API の仕様は概ね課題に沿ったものになります　https://github.com/CyberAgentHack/techtrain-mission/blob/master/api-document.yaml

## 使い方

Docker 化してあるので、 `docker-compose up -d` でコンテナを起動し、 `localhost:8000` にアクセスすればすぐ使うことができます。使用できる機能については TechTrain の課題ページを参照してください
