use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct ArticleData {
    name: String,
    #[serde(with = "time::serde::iso8601")]
    date: OffsetDateTime,
    url: String,
}

async fn fetch_article_data() -> Vec<ArticleData> {
    let resp =
        reqwest::get("https://raw.githubusercontent.com/ming900518/articles/main/article.json")
            .await
            .unwrap();
    let mut fetched_data = resp.json::<Vec<ArticleData>>().await.unwrap();
    fetched_data.sort_by_key(|x| x.date);
    fetched_data.reverse();
    fetched_data
}

#[function_component(Articles)]
pub fn articles() -> HtmlResult {
    let article_data = use_prepared_state!(
        async move |_| -> Vec<ArticleData> { fetch_article_data().await },
        ()
    )?
    .unwrap();

    let blocks = article_data.iter().map(|data| {
        let url = data.url.as_str().to_string();
        html! {
            <div class="col-md-12">
                <div class="work-box">
                    <a href={format!("/blog/{url}")}>
                        <div class="work-content">
                            <div class="row">
                                <div class="col-sm-12">
                                    <h2 class="w-title">{&data.name}</h2>
                                    <div class="w-more">
                                        <span class="w-ctegory">{"文章"}</span>{" / "}<span class="w-date">{&data.date.date()}</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </a>
                </div>
            </div>
        }
    }).collect::<Html>();

    Ok(html! {
        <section id="articles" class="portfolio-mf sect-pt4 route footer-paralax" style="padding-bottom: 4em;">
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="title-box text-center">
                            <h3 class="title-a">
                                {"文章、雜談 & 筆記"}
                            </h3>
                            <div class="line-mf"></div>
                        </div>
                    </div>
                </div>
                <div class="row" style="display: flex; flex-wrap: wrap;">
                    { blocks.clone() }
                    <div class="col-md-12">
                        <div class="work-box">
                            <a href="https://mingchang.tw/NTPU-Introduction_to_Criminal_Procedure_Law/">
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"北大刑事訴訟法 上課筆記"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"筆記"}</span>{" / "}<span class="w-date">{"2022-12-12"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="col-md-12">
                        <div class="work-box">
                            <a href="https://mingchang.tw/CompTIA-Security-Plus-Study-Notes/">
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"CompTIA Security+ 讀書筆記"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"筆記（未完稿）"}</span>{" / "}<span class="w-date">{"2021-01-20"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    })
}
