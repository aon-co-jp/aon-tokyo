# 開発方針＆開発環境ルール(aon-tokyo-server)

作業ドライブは`F:\open-runo`。この節は[`open-raid-z`](https://github.com/aon-co-jp/open-raid-z)の`CLAUDE.md`を正本とし、各プロジェクトへコピーして同期する方針に準じる。

## このリポジトリの役割(2026-07-17新設)

`aon.tokyo` / `aon.co.jp`(同一内容・同一バイナリで両ドメインを配信)のTOPページ。
テーマはAI・IT・WEB開発と本格オーディオ(JBL・B&W等の大型スピーカー)。
[`aruaru-tokyo-server`](https://github.com/aon-co-jp/aruaru-tokyo-server)(aruaru.tokyo)
と全く同じ技術スタック・実装方針(Rust+Poem、DB非依存、1バイナリ完結、
テンプレートエンジン不使用のサーバーサイド文字列組み立てHTML)を踏襲した姉妹サイト。
相互リンクで連携する(統合対象ではなく、あくまで相互リンク)。

## 「クリックで検索」リンクの方針(ユーザー指示、2026-07-17)

検索結果の長いURL(トラッキングパラメータだらけのGoogle/YouTube検索結果URL)を
そのままページに貼らない。代わりに検索エンジン自身の`?q=`/`?search_query=`形式の
短いURLを組み立て、クリックした瞬間にその都度検索・表示させる
(`google_search_link`/`youtube_search_link`、`src/main.rs`)。
percent-encodingは外部crateに依存せず手書き(このエコシステムの「外部パース
クレート非依存」方針をURLエンコードにも適用)。

## ページ構成

- `/` — TOPページ(AI・IT・WEB / オーディオの2セクション)
- `/links` — リンク集(コミケアニソン海外検索・建築士再雇用検索・
  外壁材検索・レアアース検索・GitHubバックアップ検索等 + KAZUMA多言語
  コミュニケーション動画への直接リンク、fabeee紹介リンク)
- `/municipal` — あきる野市・青梅市・奥多摩町・昭島市等への
  企業誘致・工場/倉庫誘致PR提案ページ(ドローン空撮・生ごみ発酵肥料化・
  廃プラスチック燃料化・陸上養殖等のYouTube/Google検索リンク集)
- `/healthz` — ヘルスチェック

## 意図的に含めなかったもの(2026-07-17、ユーザー確認済み)

`/municipal`ページの元の依頼には「テレワーク中の親子の写真をGoogle画像検索で
集めて掲載」という項目があったが、**実在する第三者(特定可能な個人・家族)の
写真を本人の許諾なく収集し別サイトへ転載する行為はプライバシー・著作権上の
問題があるため実装していない**。ユーザーにもその旨を明示して確認済み。

## デプロイ(aruaru-tokyo-serverと同一パターン)

VPS上で`cargo build --release`、systemdサービス化、`127.0.0.1:4200`にバインド
(aruaru-tokyo-serverの4100と衝突しないポート)。nginxで`aon.tokyo`・
`aon.co.jp`両方の443番vhostからこのポートへリバースプロキシする
(同一ポートへの2ドメイン割り当て=同一内容配信)。

## 関連プロジェクト

- [aruaru-tokyo-server](https://github.com/aon-co-jp/aruaru-tokyo-server) — 技術スタック・実装方針の出典元、姉妹サイト
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本

## HANDOFF

- **2026-07-17 aon.co.jp本番HTTPS化完了、aon.tokyoはDNS修正待ち**:
  当初LOLIPOP(`157.7.107.37`)を向いていたDNSをConoHa VPS
  (`160.251.237.162`)へ切替中。移行過程で2つのバグを発見・修正:
  (1) VPS側nginx vhost(`/etc/nginx/conf.d/aon.tokyo.conf`)に
  `/.well-known/acme-challenge/`のlocationが無く、全リクエストが
  Rustバックエンドへプロキシされ証明書発行が404で失敗していた——
  webroot用locationを追加して解消。(2) `aon.tokyo`/`aon.co.jp`とも
  443番(HTTPS)のvhostブロックが無く、SNI不一致時にnginxが
  `aruaru.tokyo`の443ブロックへフォールバックし、HTTPSアクセス時に
  aruaru.tokyoが表示されてしまうバグがユーザーから報告された——
  証明書取得後、正式な443ブロックを追加して解消。
  `aon.co.jp`は証明書取得・443ブロック追加まで完了し、実際に
  `https://aon.co.jp/`が200でRust版サイトを返すことを確認済み。
  `aon.tokyo`本体は、ユーザーがAレコードを一度誤入力(`160.251.252.72`、
  ConoHa内の別サーバーのIP)しており、修正後もパブリックDNS
  (1.1.1.1/8.8.8.8/9.9.9.9)への反映待ち。次にすべきこと:
  `aon.tokyo`のDNS反映確認後、同様に証明書取得・443ブロック追加。
- **2026-07-17 新規作成**: aruaru-tokyo-serverと同じ構成でゼロから新設。
  ローカル(WSL Ubuntu、rustc/cargo)で`cargo build`成功、実バイナリを起動し
  `curl`で`/`・`/links`・`/municipal`・`/healthz`すべて200・期待通りの
  HTML(クリックで検索リンクがGoogle/YouTubeの正しいURL形式で組み立てられて
  いること)を確認済み。
  次にすべきこと: (1) GitHubへの新規リポジトリ作成・初回push、(2) VPSへの
  デプロイ(systemdユニット追加、nginx vhost追加、aon.tokyo/aon.co.jpの
  DNS設定確認)、(3) aruaru.tokyo側TOPページに本サイトへの相互リンクを追加。
