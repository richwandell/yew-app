mod components;

use yew::prelude::*;
use material_yew::*;
use log::info;
use wasm_bindgen::JsValue;
use material_yew::drawer::MatDrawerAppContent;
use components::block::Block;
use components::menu::Menu;
use components::page1::Page1;


#[function_component]
fn App() -> Html {

    let page = use_state(|| 1);

    let set_page = {
        let page = page.clone();
        move |x| {
            info!("selected index: {:?}", x);
            page.set(x);
        }
    };


    html! {
        <div>
            <MatDrawer>
                <Menu {set_page} />
                <MatDrawerAppContent>
                    {match *page {
                        0 => html! {<Block />},
                        1 => html! {<Page1 />},
                        2 => html! {<Block />},
                        3 => html! {<Block />},
                        _ => html! {<Block />},
                    }}
                </MatDrawerAppContent>
            </MatDrawer>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
