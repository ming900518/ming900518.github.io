use crate::component::section::{
    about::*, contact::*, like::*, project::*, showcase::*, testimonials::*, work::*,
};
use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    return html! {
        <main id="main">
            <About />
            <Work />
            <Showcase />
            <Project />
            <Testimonials />
            <Like />
            <Contact />
        </main>
    };
}
