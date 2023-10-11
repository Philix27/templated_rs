use serde::{Deserialize, Serialize};
use yew::prelude::*;

mod comps;

use comps::title::AppTitle;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    language: String,
}

#[function_component]
pub fn App() -> Html {
    let names_of_babes = vec!["Cynthia", "Chidera", "Chinwe"];

    html! {
        <div>
            <AppTitle title={"Hey Ngozi"} />
            <hr />
            <p class={"body"}>{"if we had it all."}</p>
            <hr />
            <div>{names_of_babes.iter().map(|n| html!(<li>{n}</li>)).collect::<Html>()}</div>
        </div>
    }
}
