use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routes::Route, Theme};
use yew_router::hooks::use_navigator;
use yew::callback::Callback;

#[derive(Properties, PartialEq, Clone)]
pub struct BtnGenericProps {
    pub btn_text: String,
    pub route: Route,
    pub css_extra: String
}


#[function_component]
pub fn BtnGeneric(props: &BtnGenericProps) -> Html {
    let navigator: Navigator = use_navigator().unwrap();
    let route_clone = props.route.clone();
    let theme = Theme::get_theme();

    let onclick: Callback<MouseEvent> = Callback::from(move |_| navigator.push(&route_clone));

    return html! {
        <div >
            <button style={format!("color: {}; height: 3em; justify-content: center; text-align: center; align-items: center; width: 10em; border-radius:1em; border: none; background-color: {}; cursor: pointer; font-weight: bold; {}", theme.sec_color, theme.main_color, props.css_extra )} 
                onclick={onclick}>{props.btn_text.clone()}</button>
        </div>
    }
}