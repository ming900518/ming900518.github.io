use crate::component::section::{
    about::*, showcase::*, testimonials::*, work::*,
};
use crate::component::blog::articles::*;
use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    let fallback = html!{<div>{"Loading..."}</div>};
    return html! {
        <main id="main">
            <About />
            <Showcase />
            <Work />
            <Testimonials />
            <Suspense {fallback}> 
                <Articles />
            </Suspense>
        </main>
    };
}
