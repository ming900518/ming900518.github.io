use yew::prelude::*;

#[function_component(Articles)]
pub fn articles() -> Html {
    return html! {
        <section id="articles" class="portfolio-mf sect-pt4 route">
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
                <div class="row">
                    <div class="col-md-4">
                        <div class="work-box">
                            <a href="">
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"Title Placeholder"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"文章"}</span>{" / "}<span class="w-date">{"Date Placeholder"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="work-box">
                            <a href="">
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"Title Placeholder"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"雜談"}</span>{" / "}<span class="w-date">{"Date Placeholder"}</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="work-box">
                            <a href="https://mingchang.tw/NTPU-Introduction_to_Criminal_Procedure_Law/">
                                <div class="work-content">
                                    <div class="row">
                                        <div class="col-sm-12">
                                            <h2 class="w-title">{"北大刑事訴訟法 上課筆記"}</h2>
                                            <div class="w-more">
                                                <span class="w-ctegory">{"筆記"}</span>{" / "}<span class="w-date">{"Dec 12, 2022"}</span>
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
