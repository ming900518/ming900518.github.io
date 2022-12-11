use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    return html! {
        <footer>
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="copyright-box">
                            <p class="copyright">{"Copyright &copy; 2021 "}<strong>{"Ming Chang"}</strong>{". All Rights Reserved."}</p>
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
