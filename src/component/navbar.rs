use crate::{Link, Route};
use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    return html! {
        <nav class="navbar navbar-b navbar-trans navbar-expand-md fixed-top" id="mainNav">
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
                        <a class="nav-link js-scroll" href="#project">{"作品"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link js-scroll" href="#like">{"愛好"}</a>
                    </li>
                    <li class="nav-item">
                        <a class="nav-link js-scroll" href="#contact">{"聯繫我"}</a>
                    </li>
                </ul>
            </div>
        </div>
    </nav>
    };
}

#[function_component(BlogNavBar)]
pub fn blog_nav_bar() -> Html {
    return html! {
        <nav class="navbar navbar-b navbar-trans navbar-expand-md fixed-top" id="mainNav">
        <div class="container">
            <div class="navbar-collapse collapse justify-content-center" id="navbarDefault">
                <ul class="navbar-nav">
                    <li class="nav-item">
                        <a class="nav-link js-scroll" href="../">{"回到主頁"}</a>
                    </li>
            </div>
        </div>
    </nav>
    };
}
