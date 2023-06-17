use yew::prelude::*;

#[function_component(Showcase)]
pub fn show_case() -> Html {
    html! {
        <div class="section-counter paralax-mf bg-image" style="background-image: url(assets/img/bg.webp)">
            <div class="overlay-mf"></div>
            <div class="container">
                <div class="row">
                    <div class="col-sm-3 col-lg-3">
                        <div class="counter-box counter-box pt-4 pt-md-0">
                            <div class="counter-ico">
                                <span class="ico-circle"><i class="ion-hammer"></i></span>
                            </div>
                            <div class="counter-num">
                                <p class="counter">{"10"}</p>
                                <span class="counter-text">{"Java 專案"}</span>
                            </div>
                        </div>
                    </div>
                    <div class="col-sm-3 col-lg-3">
                        <div class="counter-box pt-4 pt-md-0">
                            <div class="counter-ico">
                                <span class="ico-circle"><i class="ion-ios-book"></i></span>
                            </div>
                            <div class="counter-num">
                                <p class="counter">{"6"}</p>
                                <span class="counter-text">{"Rust 專案"}</span>
                            </div>
                        </div>
                    </div>
                    <div class="col-sm-3 col-lg-3">
                        <div class="counter-box pt-4 pt-md-0">
                            <div class="counter-ico">
                                <span class="ico-circle"><i class="ion-iphone"></i></span>
                            </div>
                            <div class="counter-num">
                                <p class="counter">{"40"}</p>
                                <span class="counter-text">{"後端系統搭建"}</span>
                            </div>
                        </div>
                    </div>
                    <div class="col-sm-3 col-lg-3">
                        <div class="counter-box pt-4 pt-md-0">
                            <div class="counter-ico">
                                <span class="ico-circle"><i class="ion-ios-people"></i></span>
                            </div>
                            <div class="counter-num">
                                <p class="counter">{"31"}</p>
                                <span class="counter-text">{"使用完整服務的企業"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
