use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    return html! {
        <header id="header" class="fixed-top">
            <div class="container d-flex align-items-center justify-content-between">
                <nav class="navbar navbar-b navbar-trans navbar-expand-md fixed-top d-none d-sm-block" id="mainNav">
                    <div class="container">
                        <div class="navbar-collapse collapse justify-content-center" id="navbarDefault">
                            <ul class="navbar-nav">
                                <li class="nav-item">
                                    <a class="nav-link js-scroll active" href="#page-top">{"主頁"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link js-scroll" href="#about">{"關於我"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link js-scroll" href="#work">{"工作"}</a>
                                </li>
                                <li class="nav-item">
                                    <a class="nav-link js-scroll" href="#articles">{"文章、雜談 & 筆記"}</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </nav>
            </div>
        </header>
    };
}
