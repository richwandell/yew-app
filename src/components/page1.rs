use log::info;
use yew::prelude::*;
use material_yew::*;
use wasm_bindgen::JsValue;
use super::block::Block;

#[function_component]
pub(crate) fn Page1() -> Html {
    let blocks = use_state(|| 10);

    let onclick = {
        let blocks = blocks.clone();
        let object = JsValue::from("world");
        info!("Hello {}", object.as_string().unwrap());
        move |_| {
            let value = *blocks + 1;
            blocks.set(value);
        }
    };

    let mut rows = vec![];

    for _ in 0..*blocks {
        let mut row = vec![];
        for _ in 0..*blocks {
            row.push(html! {
                <Block />
            })
        }
        rows.push(html! {
            <div class="row">{row.into_iter().collect::<Html>()}</div>
        });
    }

    html! {
        <>
            <MatButton label="Click me!" />
            <button {onclick}>{"+1"}</button>
            <p>{*blocks}</p>
            { rows.into_iter().collect::<Html>()}
        </>
    }
}