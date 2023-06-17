use crate::component::section::{
    about::About, showcase::Showcase, testimonials::Testimonials, work::Work,
};
use crate::component::blog::articles::Articles;
use yew::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    let fallback = html!{<div>{"Loading..."}</div>};
    html! {
        <main id="main">
            <About />
            <Showcase />
            <Work />
            <Testimonials />
            <Suspense {fallback}> 
                <Articles />
            </Suspense>
        </main>
    }
}
