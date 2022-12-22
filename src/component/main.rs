use crate::component::section::{
    about::*, contact::*, showcase::*, testimonials::*, work::*,
};
use crate::component::blog::articles::*;
use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    return html! {
        <main id="main">
            <About />
            <Showcase />
            <Work />
            <Testimonials />
            <Articles />
            <Contact />
        </main>
    };
}
