use std::vec;

use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    language: String,
}

#[function_component]
pub fn Lesson() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let name = "Joh Robinson";
    let my_obj = MyObject {
        username: name.to_string(),
        language: "French".to_string(),
    };
    log!(serde_json::to_string(&my_obj).unwrap());
    log!(name);
    let card_value = "cards";
    let tasks_with_tags = vec![
        html!(<li>{"Sandra"} </li>),
        html!(<li>{"Chioma"} </li>),
        html!(<li>{"Adaeze"} </li>),
        html!(<li>{"Ngozi"} </li>),
    ];

    let names_of_babes = vec!["Cynthia", "Chidera", "Chinwe"];

    html! {
        <div>
            <h1>{name}  {"- Sirdalaud"}</h1>
            <hr />
            <p class={"body"}>{"if we had it all."}</p>
           if card_value == "card" {
             <p >{"I am a card node."}</p>
           } else {
            <p >{"Card no concern me."}</p>
           }
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }  {" value"}</p>
            <div>{tasks_with_tags} </div>
            <hr />
            <div>{names_of_babes.iter().map(|n| html!(<li>{n}</li>)).collect::<Html>()}</div>
        </div>
    }
}
