# aon-tokyo-server

`aon.tokyo` / `aon.co.jp`(同一内容配信)のTOPページ。Rust + Poem製、DB非依存の1バイナリ完結サーバー。
テーマ: AI・IT・WEB開発、本格オーディオ(JBL・B&W等の大型スピーカー)。
姉妹サイト [aruaru.tokyo](https://aruaru.tokyo/) と相互リンク。

## ページ

- `/` — TOP(AI・IT・WEB / オーディオ)
- `/links` — リンク集(クリックで検索、KAZUMA動画リンク等)
- `/municipal` — あきる野市等への企業・工場誘致提案ページ
- `/healthz` — ヘルスチェック

## ビルド・起動

```bash
cargo build --release
./target/release/aon-tokyo-server   # 127.0.0.1:4200
```

## ライセンス

Apache-2.0 OR MIT
