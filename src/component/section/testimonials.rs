use yew::prelude::*;

#[function_component(Testimonials)]
pub fn testimonials() -> Html {
    return html! {
        <div class="testimonials paralax-mf bg-image" style="background-image: url(assets/img/bg.webp)">
            <div class="overlay-mf"></div>
            <div class="container">
                <div class="row">
                    <div class="col-md-12">
                        <div id="testimonial-mf" class="owl-carousel owl-theme">
                            <div class="testimonial-box">
                                <div class="author-test">
                                    <span class="author">{"Jony Ive"}</span>
                                </div>
                                <div class="content-test">
                                    <p class="description lead">
                                        {"There is beauty when something works and it works intuitively."}
                                    </p>
                                    <span class="comit"><i class="fa fa-quote-right"></i></span>
                                </div>
                            </div>
                            <div class="testimonial-box">
                                <div class="author-test">
                                    <span class="author">{"Steve Jobs"}</span>
                                </div>
                                <div class="content-test">
                                    <p class="description lead">
                                        {"Have the courage to follow your heart and intuition. They somehow know what you truly want to become."}
                                    </p>
                                    <span class="comit"><i class="fa fa-quote-right"></i></span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    };
}
