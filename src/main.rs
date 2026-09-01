//! aon.tokyo / aon.co.jp — Rust + Poem 版TOPページ。
//! aruaru-tokyo-server (aruaru.tokyo) と同じ技術スタック・実装方針を踏襲する:
//! DB非依存・1バイナリ完結。全ページの本文は完全に決定的(リクエスト毎に
//! 変化するデータが無い)ため、`static/`配下の静的HTML/CSSファイルとして
//! 事前生成し、各ルートはディスクから読み込んで配信するだけの薄い
//! ハンドラとしている(PDF/XLSX配信〈`serve_known_asset`〉と同じパターン)。
//!
//! テーマ: AI・IT・WEB・オーディオ(JBL・B&W等の大型スピーカー含む)。
//! **2026-07-28追記**: aruaru.tokyo・icpo.tokyo・fbi.tokyoへのリンクは
//! ユーザー指示により削除済み(相互リンク関係を解消)。

use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::{get, handler, IntoResponse, Response, Route, Server};

#[handler]
fn healthz() -> &'static str {
    "ok"
}

/// カレントディレクトリ直下の`static/`に置かれた静的ファイル(HTML/CSS/
/// PDF/XLSX)を配信する。任意のファイル名を受け付ける汎用静的配信ではなく、
/// 既知のファイル名のみを個別ルートとして明示登録する(ディレクトリ
/// トラバーサル対策・意図しないファイルの公開を避けるため)。
async fn serve_known_asset(filename: &'static str, content_type: &'static str) -> Response {
    match tokio::fs::read(filename).await {
        Ok(bytes) => Response::builder()
            .header("Content-Type", content_type)
            .body(bytes),
        Err(e) => {
            tracing::warn!(filename, error = %e, "failed to read static asset");
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

#[handler]
async fn serve_top() -> Response {
    serve_known_asset("static/index.html", "text/html; charset=utf-8").await
}

#[handler]
async fn serve_links() -> Response {
    serve_known_asset("static/links.html", "text/html; charset=utf-8").await
}

#[handler]
async fn serve_municipal() -> Response {
    serve_known_asset("static/municipal.html", "text/html; charset=utf-8").await
}

#[handler]
async fn serve_cancer() -> Response {
    serve_known_asset("static/cancer.html", "text/html; charset=utf-8").await
}

#[handler]
async fn serve_style_css() -> Response {
    serve_known_asset("static/style.css", "text/css; charset=utf-8").await
}

#[handler]
async fn serve_r_pdf() -> Response {
    serve_known_asset("r.pdf", "application/pdf").await
}

#[handler]
async fn serve_r_xlsx() -> Response {
    serve_known_asset(
        "r.xlsx",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    )
    .await
}

#[handler]
async fn serve_s_pdf() -> Response {
    serve_known_asset("s.pdf", "application/pdf").await
}

#[handler]
async fn serve_s_xlsx() -> Response {
    serve_known_asset(
        "s.xlsx",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    )
    .await
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/", get(serve_top))
        .at("/healthz", get(healthz))
        .at("/links", get(serve_links))
        .at("/municipal", get(serve_municipal))
        .at("/cancer", get(serve_cancer))
        .at("/style.css", get(serve_style_css))
        .at("/r.pdf", get(serve_r_pdf))
        .at("/r.xlsx", get(serve_r_xlsx))
        .at("/s.pdf", get(serve_s_pdf))
        .at("/s.xlsx", get(serve_s_xlsx));

    tracing::info!("aon-tokyo-server listening on 127.0.0.1:4200");
    Server::new(TcpListener::bind("127.0.0.1:4200")).run(app).await
}
