use yew::prelude::*;

#[function_component(Project)]
pub fn project() -> Html {
    return html! {
        <section id="project" class="portfolio-mf sect-pt4 route">
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="title-box text-center">
                            <h3 class="title-a">
                                {"作品"}
                            </h3>
                            <div class="line-mf"></div>
                        </div>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-4">
                        <div class="work-box">
                            <a href="work.html">
                                <div class="work-img" style="text-align: center;">
                                    <img src="./assets/img/logokc2cht.webp" alt="" class="img-fluid"
                                        style="height: 300px;"/>
                                </div>
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"iKanColleCommand Tweaked Version"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"iOS程式"}</span>{" / "}<span class="w-date">{"Jul 7, 2019"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="work-box">
                            <a href="https://mingchang.tw/articles/">
                                <div class="work-img" style="text-align: center;">
                                    <img src="./assets/img/article.webp" alt="" class="img-fluid"
                                        style="height: 300px;"/>
                                </div>
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"程式開發 / 系統架設記錄文章集"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"文章集（持續更新）"}</span>{" / "}<span class="w-date">{"Mar 10, 2021"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="work-box">
                            <a href="https://mingchang.tw/CompTIA-Security-Plus-Study-Notes/">
                                <div class="work-img" style="text-align: center;">
                                    <img src="./assets/img/logosecurityplus.svg" alt="" class="img-fluid"
                                        style="height: 300px;"/>
                                </div>
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"CompTIA Security+ 讀書筆記"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"筆記（未完稿）"}</span>{" / "}<span class="w-date">{"Jan 20, 2021"}</span>
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
    };
}
