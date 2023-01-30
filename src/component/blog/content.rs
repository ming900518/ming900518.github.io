use comrak::plugins::syntect::SyntectAdapter;
use comrak::{
    markdown_to_html_with_plugins, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};
use gloo_net::http::Request;
use web_sys::{Document, window};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: String
}

#[derive(Clone, PartialEq)]
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

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    let blog_article_content = use_state(|| BlogArticleContent::default());
    {
        let blog_article_content = blog_article_content.clone();
        let id = props.id.clone();
        use_effect_with_deps(
            move |_| {
                let blog_article_content = blog_article_content.clone();
                let id = id.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::get(&*format!("https://raw.githubusercontent.com/ming900518/articles/main/{}", id))
                            .send()
                            .await
                            .unwrap();
                    let fetched_data = response.text()
                        .await
                        .unwrap();
                    if response.status() == 200 {
                        let collected_data = fetched_data.as_str().lines().collect::<Vec<&str>>();
                        let split_data = collected_data.split_first().unwrap_or((&"載入失敗", &["請回上一頁"]));
                        let title = &split_data.0[2..];
                        let document = window()
                            .expect("Window not found.")
                            .document()
                            .expect("Document not found.");
                        Document::set_title(&document, &[title, Document::title(&document).as_str()].join(" - "));
                        blog_article_content.set(BlogArticleContent {
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
                                    },
                                },
                            ),
                        })
                    } else {
                        blog_article_content.set(BlogArticleContent {
                            title: String::from(format!("錯誤代碼：{}", response.status())),
                            content: String::from(format!("<p>請確認網址是否正確，網路環境是否暢通<br>如有疑問請<a href=\"mailto:mail@mingchang.tw\">與我聯繫</a></p><p>{}</p>", fetched_data)),
                        })
                    }
                });
                || ()
            },
            (),
        );
    }

    return html! {
        <>
        <main id="main" class="bg-image" style="background-image: url(/assets/img/bg.webp);">
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
                                    <div class="article-content">
                                    <div class="title-box" id="article-content">
                                        {Html::from_html_unchecked(blog_article_content.content.clone().into())}
                                    </div>
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
    };
}
