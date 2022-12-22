use comrak::plugins::syntect::SyntectAdapter;
use comrak::{
    markdown_to_html_with_plugins, ComrakExtensionOptions, ComrakOptions, ComrakParseOptions,
    ComrakPlugins, ComrakRenderOptions, ComrakRenderPlugins,
};
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
struct BlogArticleContent {
    title: String,
    content: String,
}

#[function_component(Content)]
pub fn content() -> Html {
    let blog_article_content = use_state(|| BlogArticleContent::default());
    {
        let blog_article_content = blog_article_content.clone();
        use_effect_with_deps(
            move |_| {
                let blog_article_content = blog_article_content.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data =
                        Request::get("https://raw.githubusercontent.com/ming900518/articles/main/Spring-Boot-Fetching-Data-Response.md")
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                    let collected_data = fetched_data.as_str().lines().collect::<Vec<&str>>();
                    let splited_data = collected_data.split_first().unwrap_or((&"", &[""]));
                    blog_article_content.set(BlogArticleContent {
                        title: splited_data.0[2..].parse().unwrap(),
                        content: markdown_to_html_with_plugins(
                            splited_data.1.join("\n").trim(),
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
                });
                || ()
            },
            (),
        );
    }

    return html! {
        <>
        <div class="intro intro-single route bg-image" style="background-image: url(/assets/img/bg.webp)">
            <div class="overlay-mf"></div>
            <div class="intro-content display-table">
                <div class="table-cell">
                    <div class="container">
                        <h2 class="intro-title mb-4">{&blog_article_content.title}</h2>
                    </div>
                </div>
            </div>
        </div>
        <main id="main">
            <section class="portfolio-details">
                <div class="container">
                    {Html::from_html_unchecked(blog_article_content.content.clone().into())}
                </div>
            </section>
        </main>
        </>
    };
}
