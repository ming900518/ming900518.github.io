use axum::{response::IntoResponse, routing::get, extract::Path};
use axum_extra::routing::SpaRouter;
use std::net::SocketAddr;
use component::{blog::content::*, footer::*, main::*, navbar::*};
use yew::{prelude::*, ServerRenderer};
mod component;

#[function_component(MainApp)]
fn main_app() -> Html {
    return html! {
                <>
                <NavBar />
                <div id="home" class="intro route bg-image" style="background-image: url(/assets/img/bg.webp)">
                    <div class="overlay-itro"></div>
                    <div class="intro-content display-table">
                        <div class="table-cell">
                            <div class="container">
                                <p class="intro-subtitle"><span class="text-slider-items">{"全端工程師,MIS工程師,iOS程式開發者"}</span><strong class="text-slider"></strong></p>
                                <h1 class="intro-title mb-4">{"Ming Chang"}</h1>
                            </div>
                        </div>
                    </div>
                </div>
                <Main />
                <Footer />
                <MainScript />
                </>
            };
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub article_filename: String,
}

#[function_component(BlogApp)]
fn blog_app(props: &Props) -> Html {
    let fallback = html!{<div>{"Loading..."}</div>};
    let article_filename = props.article_filename.clone();
    return html! {
        <>
            <Suspense { fallback }>
                <Content { article_filename }/>
                <Footer />
                <MainScript />
            </Suspense>
        </>
    };
}

#[function_component(MainScript)]
fn main_script() -> Html {
    return html! {
        <>
        <script src="/assets/js/main.js"></script>
        </>
    };
}

async fn main_page() -> impl IntoResponse {
    page_assembler(ServerRenderer::<MainApp>::new().render().await).await
}

async fn blog_page(Path(article_filename): Path<String>) -> impl IntoResponse {
    page_assembler(ServerRenderer::<BlogApp>::with_props(|| Props{article_filename}).render().await).await
}

async fn page_assembler(content: String) -> axum::response::Html<String> {
    let index_html = tokio::fs::read_to_string("index.html")
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html.split_once("<body id=\"page-top\">").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str(content.as_str());
    index_html_before.push_str(index_html_after);
    axum::response::Html::from(index_html_before)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let spa = SpaRouter::new("/assets", "assets");

    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(axum::Router::new()
            .route("/", get(main_page))
            .route("/blog/:article_filename", get(blog_page))
            .merge(spa)
            .into_make_service()
        )
        .await
        .expect("Server startup failed.");
}
