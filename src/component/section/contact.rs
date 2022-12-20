use gloo_console::error;
use web_sys::{window, Location};
use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    fn link_click(link: &str) {
        let document = window()
            .expect("Window not found.")
            .document()
            .expect("Document not found.");
        Location::set_href(&document.location().unwrap(), link)
            .unwrap_or_else(|_| error!("Can't set_href."));
    }

    return html! {
        <section class="paralax-mf footer-paralax bg-image sect-mt4 route"
            style="background-image: url(assets/img/bg.webp)">
            <div class="overlay-mf"></div>
            <div class="container">
                <div class="row">
                    <div class="col-sm-12">
                        <div class="contact-mf">
                            <div id="contact" class="box-shadow-full">
                                <div class="row">
                                    <div class="col-sm-12">
                                        <div class="title-box text-center">
                                            <h3 class="title-a">
                                                {"聯繫我"}
                                            </h3>
                                            <div class="line-mf"></div>
                                        </div>
                                    </div>
                                </div>
                                <div class="row">
                                    <div class="col-md-12 text-center">
                                        <button type="submit" class="button button-a button-big button-rouded"
                                            onclick={ move |_| { link_click("https://twitter.com/mingchang137");}}>
                                            <span class="ion-social-twitter">{" Twitter"}</span>
                                        </button>
                                        <button type="submit" class="button button-a button-big button-rouded"
                                            onclick={ move |_| { link_click("https://www.instagram.com/mingchang900518/");}}>
                                            <span class="ion-social-instagram">{" Instagram"}</span>
                                        </button>
                                        <button type="submit" class="button button-a button-big button-rouded"
                                            onclick={ move |_| { link_click("https://www.reddit.com/user/MingChang137");}}>
                                            <span class="ion-social-reddit">{" Reddit"}</span>
                                        </button>
                                        <button type="submit" class="button button-a button-big button-rouded"
                                            onclick={ move |_| { link_click("tel:+886919488769");}}>
                                            <span class="ion-ios-telephone">{" Phone"}</span>
                                        </button>
                                        <button type="submit" class="button button-a button-big button-rouded"
                                            onclick={ move |_| { link_click("mailto:mail@mingchang.tw");}}>
                                            <span class="ion-email">{" Email"}</span>
                                        </button>
                                        <button type="submit" class="button button-a button-big button-rouded"
                                            onclick={ move |_| { link_click("https://github.com/ming900518");}}>
                                            <span class="ion-social-github">{" GitHub"}</span>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    };
}
