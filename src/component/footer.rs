use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="copyright-box">
                            <p class="copyright">{"Copyright © 2023 "}<strong>{"Ming Chang"}</strong>{". All Rights Reserved."}</p>
                            <div class="credits">
                                {"Designed by "}<a href="https://bootstrapmade.com/">{"BootstrapMade"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}
