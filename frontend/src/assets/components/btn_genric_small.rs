use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routes::Route, Theme};
use yew_router::hooks::use_navigator;
use yew::callback::Callback;

#[derive(Properties, PartialEq, Clone)]
pub struct BtnGenericSmallProps {
    pub btn_text: String,
    pub route: Route,
    pub css_extra: String
}

#[function_component]
pub fn BtnGenericSmall(props: &BtnGenericSmallProps) -> Html {
    let navigator: Navigator = use_navigator().unwrap();
    let route_clone = props.route.clone();
    let theme = Theme::get_theme();

    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&route_clone));
    return html! {
        <div >
            <button style={format!("color: {}; border: none; background-color: {}; cursor: pointer; font-weight: bold; {}", theme.main_color, theme.sec_color, props.css_extra )} 
                onclick={onclick}>{props.btn_text.clone()}</button>
        </div>
    }
}