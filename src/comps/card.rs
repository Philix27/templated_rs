use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(Card)]
pub fn card() -> Html {
    // let Card Props {} = props;
    let style_sheet = style!("");
    html! {
        <div>{"Card comps"}</div>
    }
}
