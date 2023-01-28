use log::info;
use wasm_bindgen::JsValue;
use material_yew::*;
use yew::prelude::*;

#[function_component]
pub(crate) fn Menu() -> Html {
    let selected_item = use_state(|| 0);

    let onaction = {
        let selected_item = selected_item.clone();
        move |x| {
            info!("selected index: {:?}", x);
            let value = *selected_item + 1;
            selected_item.set(value);
        }
    };

    html! {
        <section>
            {"This is in the drawer"}
            <MatList onaction={onaction} >
                <MatListItem >{"Item 0"}</MatListItem>
                <MatListItem >{"Item 1"}</MatListItem>
                <MatListItem >{"Item 2"}</MatListItem>
                // <MatListItem selected={item == 3}>{"Item 3"}</MatListItem>
            </MatList>
            <MatButton label="Click me!" icon={AttrValue::from("code")} />
        </section>
    }
}