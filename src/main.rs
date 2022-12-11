use yew::prelude::*;
use component::{
    navbar::*,
    main::*,
    footer::*
};
mod component;

#[function_component(App)]
fn app() -> Html {
    return html! {
        <>
            <NavBar />
            <div id="home" class="intro route bg-image" style="background-image: url(assets/img/bg.webp)">
                <div class="overlay-itro"></div>
                <div class="intro-content display-table">
                    <div class="table-cell">
                        <div class="container">
                            <p class="intro-subtitle"><span class="text-slider-items">{"全端工程師,MIS工程師,iOS程式開發者"}</span><strong class="text-slider"></strong></p>
                            <h1 class="intro-title mb-4">{"Ming Chang"}</h1>
                        </div>
                    </div>
                </div>
            </div>
            <Main />
            <Footer />
            <a href="#" class="back-to-top"><i class="fa fa-chevron-up"></i></a>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
