use component::{blog::content::*, footer::*, main::*, navbar::*};
use yew::prelude::*;
use yew_router::prelude::*;
mod component;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
}

#[function_component(App)]
fn app() -> Html {
    return html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    };
}

fn switch(routes: Route) -> Html {
    return match routes {
        Route::Home => {
            html! {
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
                </>
            }
        }
        Route::Blog => {
            html! {
                <>
                <BlogNavBar />
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
                            <Content />
                        </div>
                    </section>
                </main>
                <Footer />
                </>
            }
        }
    };
}

fn main() {
    yew::Renderer::<App>::new().render();
}
