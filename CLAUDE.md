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

## HANDOFF追記(2026-07-24) — `/cancer`ページ新設(がん治療研究報道の紹介)

- 新規ルート`GET /cancer`(`cancer_page`ハンドラ)を追加。ユーザー提供の
  実際の報道見出し・リンク2件(大阪公立大 肝臓がん新治療法の報道・
  Facebookリンク、九州大学の"からだ自身ががん治療"報道・YouTubeリンク)を
  日英併記で紹介するのみに留め、独自の医療的な効能・安全性の主張や推奨は
  一切追加していない。ナビゲーションにも「がん治療研究」リンクを追加。
- **検証**: `cargo build`成功。実バイナリ起動+`curl`で`/cancer`が
  HTTP 200・日英両見出し文言・両リンク先URLを含むことを確認済み。
- 姉妹リポジトリ`aon.co.jp`にも同一の変更を反映済み(同一内容配信方針)。

## 関連プロジェクト

- [aruaru-tokyo-server](https://github.com/aon-co-jp/aruaru-tokyo-server) — 技術スタック・実装方針の出典元、姉妹サイト
- [open-raid-z](https://github.com/aon-co-jp/open-raid-z) — 開発ルールの正本

## HANDOFF

- **2026-07-17 aon.tokyo本番HTTPS化完了(証明書・443ブロック追加)**:
  DNSがパブリックDNS(8.8.8.8/9.9.9.9)側でまだ古いIP(`160.251.252.72`)を
  キャッシュしている段階でも、Let's Encryptの検証システムは正しいIPを
  見ていたため証明書取得に成功。443番vhostブロックを追加し、HTTPは
  他サイトと同様301でHTTPSへリダイレクトするよう統一。VPS自身から
  `https://aon.tokyo/`が200・正しいタイトルを返すことを確認済み
  (外部パブリックDNSのキャッシュ反映は別途待ち)。
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

## API 方針: REST 全廃・gRPC / Connect へ (2026-11-04)

### 日本語
- **REST は全廃する。新しい API は REST で作らない。** ブラウザや `curl` から呼ぶ API は **Connect の単純呼び出し**(`POST /<パッケージ>.<Service>/<Method>`、`content-type: application/json`、本文は JSON)、サービス間は **gRPC / gRPC-web**(RPoem の管理API `open_runo.admin.v1.Admin` と同じ流儀)。ストリーミングは、必要になってから。
- **既存の REST は、触るときに順に移す。** 移したら旧 REST の経路は消す。移行が終わるまで残るが、**新規には追加しない**。
- **JSON は RS-JSON に統一**する(読み込みは `from_str_secure` / `parse_secure` でサイズ・深さ・重複キーの上限つき、書き出しは `rust_json::fast`)。ブラウザの WASM に `prost` は入れない(サイズが増えるため)。
- **例外(自分たちの API ではなく、相手の規格が決めているもの)**: ACME、OAuth/OIDC のコールバック、第三者(GitHub など)からの Webhook、ヘルスチェック、静的ファイル配信、OTLP など。これらは規格どおりの HTTP を受ける。
- 参考実装: `RPoem`(`open_runo.admin.v1.Admin`)、`open-easy-web`(`server/src/connect.rs`、`proto/open_easy_web/admin/v1/admin.proto`)。
- ※ このリポジトリのコードが既に方針どおりかどうかは、各リポジトリの現状による。この節は方針の記録であり、コードの変更を意味しない。

### English
- **REST is being abolished. New APIs are not built as REST.** APIs called from browsers or `curl` use **Connect unary calls** (`POST /<package>.<Service>/<Method>`, `content-type: application/json`, JSON body); service-to-service calls use **gRPC / gRPC-web** (the same style as RPoem's admin API `open_runo.admin.v1.Admin`). Streaming only when it is actually needed.
- **Existing REST endpoints are migrated one by one, when they are touched.** The old REST route is removed once migrated. Until migration is finished they remain, but **no new ones are added**.
- **JSON is unified on RS-JSON**: reading with `from_str_secure` / `parse_secure` (size, depth and duplicate-key limits), writing with `rust_json::fast`. `prost` is not put into browser WASM (it would grow the binary).
- **Exceptions (shapes dictated by someone else's standard, not our own API)**: ACME, OAuth/OIDC callbacks, webhooks from third parties (e.g. GitHub), health checks, static file serving, OTLP, and so on. These accept plain HTTP as the standard requires.
- Reference implementations: `RPoem` (`open_runo.admin.v1.Admin`) and `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Note: whether the code in a given repository already follows this policy depends on that repository. This section records the policy; it does not mean the code has been changed.

### Deutsch (KI-Übersetzung, redaktionell überarbeitet; noch nicht von Muttersprachlern geprüft)
- **REST wird vollständig abgeschafft; neue APIs entstehen nicht mehr als REST.** APIs, die aus dem Browser oder per `curl` aufgerufen werden, verwenden **unäre Connect-Aufrufe** (`POST /<Paket>.<Service>/<Methode>`, `content-type: application/json`, JSON-Body). Zwischen Diensten kommen **gRPC bzw. gRPC-web** zum Einsatz, im selben Stil wie die Admin-API von RPoem (`open_runo.admin.v1.Admin`). Streaming wird erst ergänzt, wenn es tatsächlich gebraucht wird.
- **Bestehende REST-Endpunkte werden schrittweise umgestellt, und zwar immer dann, wenn man ohnehin an ihnen arbeitet.** Sobald ein Endpunkt umgestellt ist, wird die alte REST-Route entfernt. Bis zum Abschluss der Migration bleiben die übrigen bestehen, es werden aber **keine neuen mehr hinzugefügt**.
- **JSON wird einheitlich über RS-JSON verarbeitet**: Gelesen wird mit `from_str_secure` bzw. `parse_secure` (mit Obergrenzen für Größe, Verschachtelungstiefe und doppelte Schlüssel), geschrieben mit `rust_json::fast`. `prost` wird nicht in das Browser-WASM aufgenommen, da es die Datei vergrößern würde.
- **Ausnahmen (Schnittstellen, deren Form ein fremder Standard vorgibt und die nicht zu unserer eigenen API gehören)**: ACME, OAuth/OIDC-Callbacks, Webhooks von Drittanbietern (z. B. GitHub), Health-Checks, die Auslieferung statischer Dateien, OTLP und Ähnliches. Diese nehmen, wie vom jeweiligen Standard verlangt, gewöhnliches HTTP entgegen.
- Referenzimplementierungen: `RPoem` (`open_runo.admin.v1.Admin`) und `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Hinweis: Ob der Code eines einzelnen Repositorys diese Richtlinie bereits erfüllt, hängt vom jeweiligen Repository ab. Dieser Abschnitt dokumentiert lediglich die Richtlinie; er besagt nicht, dass der Code schon angepasst wurde.

### Русский (перевод выполнен с помощью ИИ и отредактирован; носителями языка пока не проверен)
- **От REST мы полностью отказываемся; новые API больше не создаются в стиле REST.** API, к которым обращаются из браузера или через `curl`, используют **унарные вызовы Connect** (`POST /<пакет>.<Service>/<Method>`, `content-type: application/json`, тело запроса — JSON). Для взаимодействия между сервисами применяются **gRPC и gRPC-web** — в том же стиле, что и административный API RPoem (`open_runo.admin.v1.Admin`). Потоковая передача данных добавляется лишь тогда, когда в ней действительно возникает необходимость.
- **Существующие REST-эндпоинты переводятся поэтапно — по мере того, как с ними приходится работать.** После перевода старый REST-маршрут удаляется. Пока миграция не завершена, оставшиеся эндпоинты сохраняются, но **новые не добавляются**.
- **Работа с JSON унифицирована на основе RS-JSON**: чтение — через `from_str_secure` или `parse_secure` (с ограничениями на размер, глубину вложенности и повторяющиеся ключи), запись — через `rust_json::fast`. `prost` в браузерный WASM не включается, поскольку это увеличило бы размер файла.
- **Исключения (интерфейсы, форму которых определяет чужой стандарт, а не наш собственный API)**: ACME, обратные вызовы OAuth/OIDC, вебхуки сторонних сервисов (например, GitHub), проверки работоспособности, раздача статических файлов, OTLP и тому подобное. Такие интерфейсы принимают обычный HTTP, как того требует соответствующий стандарт.
- Эталонные реализации: `RPoem` (`open_runo.admin.v1.Admin`) и `open-easy-web` (`server/src/connect.rs`, `proto/open_easy_web/admin/v1/admin.proto`).
- Примечание: соответствует ли код конкретного репозитория этой политике, зависит от самого репозитория. Данный раздел лишь фиксирует политику и не означает, что код уже изменён.

## API 方針の補足: 用語の整理と使い分け (2026-11-04)

### 日本語
**1つに統一するものではなく、別の層の話です。** 「REST の代わり」になるのは Connect・gRPC・GraphQL だけで、WASM は画面を作る技術です。

| 用語 | 何か | REST の代わりか |
|---|---|---|
| REST | URL と HTTP メソッドで操作を表す API の作り方 | 全廃する側 |
| **Connect** | `POST /サービス名/メソッド名` に JSON を送る形式。`curl` でもブラウザの `fetch` でも呼べる | はい(**採用**) |
| **gRPC** | 同じ「サービス/メソッド」の考え方を、バイナリ(protobuf)と HTTP/2 で行う形式。速く、ストリーミングも得意 | はい(**サービス同士に採用**) |
| gRPC-web | gRPC のブラウザ向けの変形版 | はい(必要なら) |
| GraphQL | 欲しい項目を問い合わせる形式 | 一部だけ(**問い合わせ(読み取り)専用**) |
| WASM | Rust をブラウザの中で動かす仕組み | いいえ(画面を作る技術) |

- **ブラウザの画面や `curl` から呼ぶ API は Connect**(POST、JSON を RS-JSON で読み書き)。ブラウザの WASM は素の gRPC を話せず、Connect なら `fetch` のままで、`prost` を足さずに済む(画面のサイズも増えない)。
- **サービス同士の通信(サーバー間、重い処理、ストリーミング)は gRPC。** 多くは Connect で足りるので、ストリーミングが必要になった所だけ gRPC に上げる。
- **GraphQL は、まとめて問い合わせたいときの読み取り専用。** 書き込み(操作)には使わない。
- **WASM は画面を作る技術として、そのまま使う。** 画面から呼ぶ API が Connect になっただけ。
- 外部の規格が形を決めるもの(ACME、OAuth のコールバック、第三者の Webhook、ヘルスチェック、静的ファイル、OTLP)は、規格どおりの HTTP のまま。
- **迷ったら Connect。**

### English
**These are not alternatives to unify; they sit at different layers.** Only Connect, gRPC and GraphQL can replace REST; WASM is a way to build the UI.

| Term | What it is | Replaces REST? |
|---|---|---|
| REST | API style that expresses operations through URLs and HTTP methods | The thing being abolished |
| **Connect** | `POST /Service/Method` with a JSON body; callable from `curl` and from a browser `fetch` | Yes (**adopted**) |
| **gRPC** | The same service/method idea over binary (protobuf) and HTTP/2; fast, good at streaming | Yes (**for service-to-service**) |
| gRPC-web | A browser-friendly variant of gRPC | Yes (if needed) |
| GraphQL | A query language: ask for exactly the fields you want | Partly (**read-only queries**) |
| WASM | Runs Rust inside the browser | No (a UI technology) |

- **APIs called from browser screens or `curl` use Connect** (POST, JSON read and written with RS-JSON). Browser WASM cannot speak plain gRPC; with Connect it stays a `fetch`, and `prost` is not needed (the binary does not grow).
- **Service-to-service calls (server to server, heavy work, streaming) use gRPC.** Connect is enough in most places; move a call to gRPC only where streaming is actually needed.
- **GraphQL is read-only**, for queries that gather several pieces of data. It is not used for writes (operations).
- **WASM stays as the way to build the UI.** The API it calls simply became Connect.
- Interfaces whose shape an outside standard dictates (ACME, OAuth callbacks, third-party webhooks, health checks, static files, OTLP) stay plain HTTP.
- **When in doubt, use Connect.**

### Deutsch (KI-Übersetzung, sprachlich überarbeitet; noch nicht von Muttersprachlern geprüft)
**Es geht nicht darum, eine einzige Technik auszuwählen – die Begriffe liegen auf unterschiedlichen Ebenen.** Als Ersatz für REST kommen nur Connect, gRPC und GraphQL in Frage; WASM dagegen ist eine Technik, mit der die Benutzeroberfläche gebaut wird.

| Begriff | Was ist das? | Ersatz für REST? |
|---|---|---|
| REST | Ein API-Stil, bei dem Operationen über URLs und HTTP-Methoden ausgedrückt werden | wird abgeschafft |
| **Connect** | `POST /Dienst/Methode` mit JSON im Body; aufrufbar mit `curl` ebenso wie mit `fetch` im Browser | ja (**gewählt**) |
| **gRPC** | Dasselbe Prinzip aus Dienst und Methode, aber über binäres protobuf und HTTP/2; schnell und gut für Streaming geeignet | ja (**zwischen Diensten**) |
| gRPC-web | Eine für den Browser angepasste Variante von gRPC | ja (bei Bedarf) |
| GraphQL | Abfragesprache, mit der man genau die benötigten Felder anfordert | teilweise (**nur für lesende Abfragen**) |
| WASM | Führt Rust-Code direkt im Browser aus | nein (Technik für die Oberfläche) |

- **APIs, die von Browseroberflächen oder per `curl` aufgerufen werden, verwenden Connect** (POST; JSON wird mit RS-JSON gelesen und geschrieben). WASM im Browser kann kein reines gRPC sprechen. Mit Connect bleibt es bei einem gewöhnlichen `fetch`, und `prost` ist nicht nötig – die WASM-Datei wird dadurch nicht größer.
- **Für die Kommunikation zwischen Diensten (Server zu Server, rechenintensive Aufgaben, Streaming) wird gRPC eingesetzt.** In den meisten Fällen reicht Connect aus; auf gRPC wird nur dort umgestellt, wo Streaming tatsächlich gebraucht wird.
- **GraphQL dient ausschließlich lesenden Abfragen**, wenn mehrere Informationen gebündelt abgefragt werden sollen. Für Schreibzugriffe (Operationen) wird es nicht verwendet.
- **WASM bleibt die Technik für die Oberfläche.** Geändert hat sich lediglich, dass die aufgerufene API nun auf Connect beruht.
- Schnittstellen, deren Form durch einen externen Standard festgelegt ist (ACME, OAuth-Callbacks, Webhooks von Drittanbietern, Health-Checks, statische Dateien, OTLP), bleiben gewöhnliches HTTP.
- **Im Zweifel: Connect.**

### Русский (перевод выполнен с помощью ИИ и литературно отредактирован; носителями языка пока не проверен)
**Это не варианты, из которых нужно выбрать один, а технологии разных уровней.** Заменить REST могут только Connect, gRPC и GraphQL; WASM же — это способ создания пользовательского интерфейса.

| Термин | Что это | Заменяет REST? |
|---|---|---|
| REST | Стиль построения API, при котором операции выражаются через URL и HTTP-методы | отменяется |
| **Connect** | `POST /Сервис/Метод` с телом в формате JSON; вызывается как из `curl`, так и из браузерного `fetch` | да (**выбран**) |
| **gRPC** | Тот же принцип «сервис — метод», но поверх двоичного формата protobuf и HTTP/2; работает быстро и хорошо подходит для потоковой передачи | да (**для связи между сервисами**) |
| gRPC-web | Адаптированный для браузера вариант gRPC | да (при необходимости) |
| GraphQL | Язык запросов: клиент запрашивает ровно те поля, которые ему нужны | частично (**только запросы на чтение**) |
| WASM | Позволяет выполнять код на Rust прямо в браузере | нет (технология для интерфейса) |

- **API, которые вызываются из браузерных интерфейсов или через `curl`, строятся на Connect** (POST; JSON читается и записывается через RS-JSON). Браузерный WASM не умеет работать с «чистым» gRPC. С Connect достаточно обычного `fetch`, а `prost` не требуется — размер WASM-файла не увеличивается.
- **Для обмена между сервисами (сервер — сервер, ресурсоёмкие задачи, потоковая передача) применяется gRPC.** В большинстве случаев достаточно Connect; на gRPC переходят только там, где потоковая передача действительно необходима.
- **GraphQL используется только для чтения**, когда нужно получить сразу несколько связанных данных. Для записи (операций) он не применяется.
- **WASM по-прежнему остаётся технологией для интерфейса.** Изменилось лишь то, что вызываемый им API теперь построен на Connect.
- Интерфейсы, форма которых задана внешним стандартом (ACME, обратные вызовы OAuth, вебхуки сторонних сервисов, проверки работоспособности, статические файлы, OTLP), остаются обычным HTTP.
- **Если есть сомнения — выбирайте Connect.**

## REST全廃の確認(2026-11-07)
全13ルートを調査した結果、**自前のAPIは0本**。`/healthz`(ヘルスチェック)と、静的ページ・CSS・動画・PDF・xlsxの配信(`/links` `/cancer` `/p` `/k` ほか)だけで、
いずれも方針の**例外(ヘルスチェック・静的ファイル配信)**。移す対象は無く、コードは変更していない。今後APIを足す場合は Connect(`POST /<パッケージ>.<Service>/<Method>`、JSONはRS-JSON)で作る。
