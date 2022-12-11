use yew::prelude::*;
use crate::component::section::{
    about::*,
    work::*,
    showcase::*,
    project::*,
    testimonials::*,
    like::*,
    contact::*
};

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
    }
}
