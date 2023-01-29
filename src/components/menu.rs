use log::info;
use wasm_bindgen::JsValue;
use material_yew::*;
use material_yew::list::ListIndex;
use material_yew::list::ListIndex::Single;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MenuProps {
    pub set_page: Callback<usize>,
}

#[function_component]
pub(crate) fn Menu(props: &MenuProps) -> Html {

    let list_changed = {
        let set_page = props.set_page.clone();
        move |x: ListIndex| {
            info!("selected index: {:?}", x);
            match x {
                Single(x) => set_page.emit(x.unwrap()),
                _ => (),
            }
        }
    };

    html! {
        <section>
            {"This is in the drawer"}
            <MatList activatable=true onaction={list_changed} >
                <MatListItem >{"Item 0"}</MatListItem>
                <MatListItem >{"Item 1"}</MatListItem>
                <MatListItem >{"Item 2"}</MatListItem>
                <MatListItem >{"Item 3"}</MatListItem>
            </MatList>
            <MatButton label="Click me!" icon={AttrValue::from("code")} />
        </section>
    }
}