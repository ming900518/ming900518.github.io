use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="about-mf sect-pt4 route">
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="box-shadow-full">
                            <div class="row">
                                <div class="col-md-6">
                                    <div class="row">
                                        <div class="col-sm-6 col-md-5">
                                            <div class="about-img">
                                                <img src="assets/img/profile-pic.webp"
                                                    class="img-fluid rounded b-shadow-a lazyload" alt="" />
                                            </div>
                                        </div>
                                        <div class="col-sm-6 col-md-7">
                                            <div class="about-info">
                                                <p><span class="title-s">{"姓名: "}</span> <span>{"張仲閔 Ming Chang"}</span></p>
                                                <p><span class="title-s">{"Email: "}</span> <span><a
                                                            href="mailto:mail@mingchang.tw">
                                                            {"mail@mingchang.tw"}</a></span></p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="skill-mf">
                                        <p class="title-s">{"技能"}</p>
                                        <span>{"Rust"}</span> <span class="pull-right">{"90%"}</span>
                                        <div class="progress">
                                            <div class="progress-bar" role="progressbar" style="width: 90%;"
                                                aria-valuenow="90" aria-valuemin="0" aria-valuemax="100"></div>
                                        </div>
                                        <span>{"JavaScript / TypeScript"}</span> <span class="pull-right">{"70%"}</span>
                                        <div class="progress">
                                            <div class="progress-bar" role="progressbar" style="width: 70%"
                                                aria-valuenow="70" aria-valuemin="0" aria-valuemax="100"></div>
                                        </div>
                                        <span>{"Java"}</span> <span class="pull-right">{"95%"}</span>
                                        <div class="progress">
                                            <div class="progress-bar" role="progressbar" style="width: 95%"
                                                aria-valuenow="95" aria-valuemin="0" aria-valuemax="100"></div>
                                        </div>
                                        <span>{"SQL"}</span> <span class="pull-right">{"100%"}</span>
                                        <div class="progress">
                                            <div class="progress-bar" role="progressbar" style="width: 100%"
                                                aria-valuenow="100" aria-valuemin="0" aria-valuemax="100"></div>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="about-me pt-4 pt-md-0">
                                        <div class="title-box-2">
                                            <h5 class="title-left">
                                                {"關於我"}
                                            </h5>
                                        </div>
                                        <p class="lead">
                                            {"全端工程師兼任 MIS 工程師，在公司中除了進行維護和開發外，也負責著公司專案測試及軟硬體環境搭建的工作。"}
                                        </p>
                                        <p class="lead">
                                            {"雖然身為全端工程師，但其實比較喜歡後端 😂 ，也有自己開發過原生程式和智能合約"}
                                        </p>
                                        <p class="lead">
                                            {"Java 專注於 Spring 框架，除了傳統的 Spring MVC ，已將研究方向轉向 Spring WebFlux ，並樂於引入 Spring Native 等新技術。"}
                                        </p>
                                        <p class="lead">
                                            {"日前則專注於 Rust 開發，正在運用 Tokio/Axum 框架製作過微服務模組，也同時使用 NEAR 鏈開發智能合約。"}
                                        </p>
                                        <p class="lead">
                                            <strong>{"本頁採用 Rust 與 Yew 框架編譯 WebAssembly 實作功能"}</strong>
                                        </p>
                                    </div>
                                </div>
                                <div class="col-md-12 text-center" style="padding-top: 2em;">
                                        <a rel="me" href="https://mstdn.social/@mingchang">
                                            <button type="submit" class="button button-a button-big button-rouded">
                                                <span>{"Mastodon"}</span>
                                            </button>
                                        </a>
                                        <a href="https://twitter.com/mingchang137">
                                            <button type="submit" class="button button-a button-big button-rouded">
                                                <span>{"Twitter"}</span>
                                            </button>
                                        </a>
                                        <a href="https://www.instagram.com/mingchang900518/">
                                            <button type="submit" class="button button-a button-big button-rouded">
                                                <span>{"Instagram"}</span>
                                            </button>
                                        </a>
                                        <a href="https://www.reddit.com/user/MingChang137">
                                            <button type="submit" class="button button-a button-big button-rouded">
                                                <span>{"Reddit"}</span>
                                            </button>
                                        </a>
                                        <a href="https://github.com/ming900518">
                                            <button type="submit" class="button button-a button-big button-rouded">
                                                <span>{"GitHub"}</span>
                                            </button>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
            </div>
        </section>
    }
}
