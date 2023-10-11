use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct AppTitleProps {
    pub title: String,
}

#[function_component]
pub fn AppTitle(props: &AppTitleProps) -> Html {
    let AppTitleProps { title } = props;
    // html! {
    //     <div></div>
    // }
    html!(<h1>{title} </h1>)
}
