use gloo_net::http::Request;

use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct BlogArticleProps {
    blog_article_content: UseStateHandle<String>,
}

#[derive(Clone, PartialEq, Default)]
struct BlogArticleContent {
    content: String
}

// #[derive(Clone, PartialEq, Deserialize)]
// struct BlogArticleInfo {
//     name: String,
//     path: String,
//     sha: String,
//     size: i32,
//     url: String,
//     html_url: String,
//     git_url: String,
//     download_url: String,
//     #[serde(rename = "type")]
//     blog_article_type: String,
//     #[serde(rename = "_links")]
//     links: BlogArticleLinks,
// }
//
// #[derive(Clone, PartialEq, Deserialize)]
// struct BlogArticleLinks {
//     #[serde(rename = "self")]
//     links_self: String,
//     git: String,
//     html: String,
// }

// #[function_component(BlogArticleList)]
// fn blog_article_list(
//     BlogArticleProps {
//         blog_article_info_vec,
//     }: &BlogArticleProps,
// ) -> Html {
//     blog_article_info_vec
//         .iter()
//         .map(|blog_article_info| {
//             html! {
//                 <p>{format!("{}: {}", blog_article_info.name, blog_article_info.download_url)}</p>
//             }
//         })
//         .collect()
// }

#[function_component(Content)]
pub fn content() -> Html {
    let blog_article_content = use_state(|| BlogArticleContent::default());
    {
        let blog_article_content = blog_article_content.clone();
        use_effect_with_deps(
            move |_| {
                let blog_article_content = blog_article_content.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data: String =
                        Request::get("https://raw.githubusercontent.com/ming900518/articles/main/MariaDB-Store-Array.md")
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                    blog_article_content.set(BlogArticleContent {
                        content: fetched_data
                    })
                });
                || ()
            },
            (),
        );
    }

    return html! {
        <>
        <div class="intro intro-single route bg-image" style="background-image: url(assets/img/bg.webp)">
            <div class="overlay-mf"></div>
            <div class="intro-content display-table">
                <div class="table-cell">
                    <div class="container">
                        <h2 class="intro-title mb-4">{"文章標題"}</h2>
                        <ol class="breadcrumb d-flex justify-content-center">
                            <li class="breadcrumb-item">
                                <a href="index.html">{"首頁"}</a>
                            </li>
                            <li class="breadcrumb-item active">{"文章標題"}</li>
                        </ol>
                    </div>
                </div>
            </div>
        </div>
        <main id="main">
            <section class="portfolio-details">
                <div class="container">
                    <p>
                        {&blog_article_content.content}
                    </p>
                </div>
            </section>
        </main>
        </>
    };
}
