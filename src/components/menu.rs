use material_yew::*;
use yew::prelude::*;

#[function_component]
pub(crate) fn Menu() -> Html {
    html! {
        <section>
            {"This is in the drawer"}
            <MatList>
                <MatListItem>{"Item 0"}</MatListItem>
                <MatListItem>{"Item 1"}</MatListItem>
                <MatListItem>{"Item 2"}</MatListItem>
                <MatListItem>{"Item 3"}</MatListItem>
            </MatList>
            <MatButton label="Click me!" icon={AttrValue::from("code")} />
        </section>
    }
}