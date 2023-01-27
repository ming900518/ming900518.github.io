use comrak::plugins::syntect::SyntectAdapter;
use comrak::{
    markdown_to_html_with_plugins, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
// use web_sys::{Document, window};
use yew::prelude::*;
use crate::Props;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct BlogArticleContent {
    title: String,
    content: String,
}

impl Default for BlogArticleContent {
    fn default() -> Self {
        BlogArticleContent {
            title: String::from("文章載入中，請稍候......"),
            content: String::from("<p>無法載入？請檢查網路環境，或<a href=\"mailto:mail@mingchang,tw\">與我聯繫</a></p>"),
        }
    }
}

async fn get_article_html(article_filename: String) -> BlogArticleContent {
    match reqwest::get(format!("https://raw.githubusercontent.com/ming900518/articles/main/{}", article_filename)).await {
        Ok(resp) => {
            let resp_text = resp.text().await.unwrap_or("載入失敗\n請回上一頁".to_string());
            let collected_data = resp_text.lines().collect::<Vec<&str>>();
            let split_data = collected_data.split_first().unwrap_or((&"載入失敗", &["請回上一頁"]));
            let title = &split_data.0[2..];
            BlogArticleContent {
                title: title.parse().unwrap(),
                content: markdown_to_html_with_plugins(
                    split_data.1.join("\n").trim(),
                    &ComrakOptions {
                        extension: ComrakExtensionOptions {
                            strikethrough: true,
                            table: true,
                            tasklist: true,
                            superscript: true,
                            ..ComrakExtensionOptions::default()
                        },
                        parse: ComrakParseOptions {
                            smart: true,
                            ..ComrakParseOptions::default()
                        },
                        render: ComrakRenderOptions {
                            github_pre_lang: true,
                            unsafe_: true,
                            ..ComrakRenderOptions::default()
                        },
                    },
                    &ComrakPlugins {
                        render: ComrakRenderPlugins {
                            codefence_syntax_highlighter: Some(&SyntectAdapter::new(
                                "base16-ocean.dark",
                            )),
                            heading_adapter: None,
                        },
                    },
                ),
            }
        },
        Err(err) => {
            BlogArticleContent {
                title: String::from(format!("錯誤代碼：{}", err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))),
                content: String::from("<p>請確認網址是否正確，網路環境是否暢通<br>如有疑問請<a href=\"mailto:mail@mingchang.tw\">與我聯繫</a></p><p>{}</p>"),
            }
        }
    }
}

#[function_component(Content)]
pub fn content(props: &Props) -> HtmlResult {
    let article_filename = props.clone().article_filename.clone();
    let blog_article_content = use_prepared_state!(async move |_| -> BlogArticleContent { get_article_html(article_filename).await }, ())?.unwrap();

    // NOT WORKING, NEED TO FIND A WORKAROUND.
    //use_state(|| {
    //    let document = window()
    //            .expect("Window not found.")
    //            .document()
    //            .expect("Document not found.");
    //    Document::set_title(&document, &[&blog_article_content.title, Document::title(&document).as_str()].join(" - "));
    //});

    return Ok(html! {
        <>
        <main id="main" class=" bg-image" style="background-image: url(/assets/img/bg.webp);">
        <div class="intro intro-single route">
            <div class="overlay-mf"></div>
            <div class="intro-content display-table">
                <div class="table-cell">
                    <div class="container">
                        <h2 class="intro-title mb-4">{&blog_article_content.title}</h2>
                        <ol class="breadcrumb d-flex justify-content-center">
                            <li class="breadcrumb-item">
                                <a class="nav-link js-scroll" href="../#articles">{"回到主頁"}</a>
                            </li>
                        </ol>
                    </div>
                </div>
            </div>
        </div>
        <section class="paralax-mf footer-paralax route" style="padding-top: 0;">
            <div class="overlay-mf"></div>
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div id="contact" class="box-shadow-full">
                            <div class="row">
                                <div class="col-sm-12">
                                    <div class="title-box" id="articleContent">
                                        {Html::from_html_unchecked(blog_article_content.content.clone().into())}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        </main>
        </>
    });
}
