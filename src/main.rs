#![allow(non_camel_case_types)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use axum::{extract::Path, response::IntoResponse, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use component::{blog::content::Content, footer::Footer, main::Main, navbar::NavBar};
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use yew::{prelude::*, ServerRenderer};
mod component;

#[function_component(MainApp)]
fn main_app() -> Html {
    html! {
        <>
        <NavBar />
        <div id="home" class="intro route bg-image" style="background-image: url(/assets/img/bg.webp)">
            <div class="overlay-itro"></div>
            <div class="intro-content display-table">
                <div class="table-cell">
                    <div class="container">
                        <p class="intro-subtitle"><span class="text-slider-items">{"Backend & MIS Engineer,Rustaceans,Neovim lover"}</span><strong class="text-slider"></strong></p>
                        <h1 class="intro-title mb-4">{"Ming Chang"}</h1>
                    </div>
                </div>
            </div>
        </div>
        <Main />
        <Footer />
        <MainScript />
        </>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub article_filename: String,
}

#[function_component(BlogApp)]
fn blog_app(props: &Props) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    let article_filename = props.article_filename.clone();
    html! {
        <>
            <Suspense { fallback }>
                <Content { article_filename }/>
                <Footer />
                <MainScript />
            </Suspense>
        </>
    }
}

#[function_component(MainScript)]
fn main_script() -> Html {
    html! {
        <>
        <script src="/assets/js/main.js"></script>
        </>
    }
}

async fn main_page() -> impl IntoResponse {
    page_assembler(ServerRenderer::<MainApp>::new().render().await).await
}

async fn blog_page(Path(article_filename): Path<String>) -> impl IntoResponse {
    page_assembler_blog(
        ServerRenderer::<BlogApp>::with_props(|| Props { article_filename })
            .render()
            .await,
    )
    .await
}

async fn page_assembler(content: String) -> axum::response::Html<String> {
    let index_html = tokio::fs::read_to_string("index.html")
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) =
        index_html.split_once("<body id=\"page-top\">").unwrap();
    let html = format!("{index_html_before}{content}{index_html_after}");
    axum::response::Html::from(html)
}

async fn page_assembler_blog(content: String) -> axum::response::Html<String> {
    let index_html = tokio::fs::read_to_string("index.html")
        .await
        .expect("failed to read index.html");

    let content_cloned = content.clone();

    let (_, with_title) = content_cloned
        .split_once("<h2 class=\"intro-title mb-4\">")
        .unwrap();
    let (title, _) = with_title.split_once("</h2>").unwrap();

    let (index_html_before_title, index_html_after_title) =
        index_html.split_once("<title>").unwrap();
    let (index_html_before_content, index_html_after_content) = index_html_after_title
        .split_once("<body id=\"page-top\">")
        .unwrap();

    let html = format!("{index_html_before_title}<title>{title} - {index_html_before_content}{content}{index_html_after_content}");
    axum::response::Html::from(html)
}

#[tokio::main]
async fn main() {
    let tracing_filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::DEBUG)
        .with_target("tower_http::trace::on_request", Level::DEBUG)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_filter)
        .init();
    let assets = ServeDir::new("assets");
    let router = axum::Router::new()
        .route("/", get(main_page))
        .route("/blog/:article_filename", get(blog_page))
        .nest_service("/assets", assets)
        .layer(TraceLayer::new_for_http())
        .into_make_service();

    if let Ok(config) = RustlsConfig::from_pem_file("ssl/ssl.pem", "ssl/ssl.key").await {
        let addr = SocketAddr::from(([0, 0, 0, 0], 443));
        println!("SSL enabled. Listening on {addr}");
        axum_server::bind_rustls(addr, config)
            .serve(router)
            .await
            .expect("Server startup failed.");
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        println!("SSL disabled. Listening on {addr}");
        axum_server::bind(addr)
            .serve(router)
            .await
            .expect("Server startup failed.");
    }
}
