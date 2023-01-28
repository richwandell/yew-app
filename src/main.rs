mod components;

use yew::prelude::*;
use log::info;
use wasm_bindgen::JsValue;
use material_yew::*;
use material_yew::drawer::MatDrawerAppContent;
use components::block::Block;
use components::menu::Menu;


#[function_component]
fn App() -> Html {

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
        <div>
            <MatDrawer>
                <Menu />
                <MatDrawerAppContent>
                    <MatButton label="Click me!" />
                    <button {onclick}>{"+1"}</button>
                    <p>{*blocks}</p>
                    { rows.into_iter().collect::<Html>()}
                </MatDrawerAppContent>
            </MatDrawer>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
