use yew::prelude::*;

#[function_component(Work)]
pub fn work() -> Html {
    html! {
        <section id="work" class="services-mf pt-5 route">
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="title-box text-center">
                            <h3 class="title-a">
                                {"工作"}
                            </h3>
                            <div class="line-mf"></div>
                        </div>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-4">
                        <div class="service-box">
                            <div class="service-ico">
                                <span class="ico-circle"><i class="ion-monitor"></i></span>
                            </div>
                            <div class="service-content">
                                <h2 class="s-title">{"系統維護"}</h2>
                                <p class="s-description text-center">
                                    <br/>{"熟悉 Linux/macOS/Windows 指令"}<br/>{"於公司負責所有政府單位及企業之維護案"}
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="service-box">
                            <div class="service-ico">
                                <span class="ico-circle"><i class="ion-android-phone-portrait"></i></span>
                            </div>
                            <div class="service-content">
                                <h2 class="s-title">{"前端設計"}</h2>
                                <p class="s-description text-center">
                                    <br/>{"任職的公司為全端網頁設計公司"}<br/>{"為企業及政府單位設計美觀的動態網站"}<br/>
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-4">
                        <div class="service-box">
                            <div class="service-ico">
                                <span class="ico-circle"><i class="ion-code-working"></i></span>
                            </div>
                            <div class="service-content">
                                <h2 class="s-title">{"後端搭建"}</h2>
                                <p class="s-description text-center">
                                    <br/>{"研究並使用 Spring / Axum 開發穩定的後端"}<br/>{"為用戶帶來穩定快速的網路服務"}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
