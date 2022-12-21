use gloo_net::http::Request;
use yew::prelude::*;

#[function_component(Content)]
pub fn content() -> Html {
    let result = use_state(|| Some(String::default()));

    let on_video_select = {
        let result = result.clone();
        Callback::from(move |data: String| result.set(Some(data)))
    };

    let data = use_state(|| String::default());
    {
        let result = result.clone();
        use_effect_with_deps(
            move |_| {
                let result = result.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched =
                        Request::get("https://api.github.com/repos/ming900518/articles/contents/")
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                    result.set(Some(fetched));
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            <button on_click={on_video_select.clone()}>{ "GO" }</button>
            <p>
                { *result }
            </p>
        </div>
    }
}
