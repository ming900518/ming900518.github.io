use component::{blog::content::*, footer::*, main::*, navbar::*};
use yew::{prelude::*, ServerRenderer};
use yew_router::prelude::*;
mod component;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/blog/:id")]
    BlogArticle { id: String },
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
                <div id="home" class="intro route bg-image" style="background-image: url(/assets/img/bg.webp)">
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
                <MainScript />
                </>
            }
        }
        Route::BlogArticle { id } => {
            html! {
                <>
                <Content { id }/>
                <Footer />
                <MainScript />
                </>
            }
        }
    };
}

#[function_component(MainScript)]
fn main_script() -> Html {
    return html! {
        <>
        <script src="/assets/js/main.js"></script>
        </>
    };
}

fn main() {
    ServerRenderer::<App>::new().render().await;
}
