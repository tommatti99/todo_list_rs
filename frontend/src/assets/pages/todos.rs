use yew::prelude::*;
use yew_router::prelude::*;
use crate::{assets::components::btn_genric_small::BtnGenericSmall, routes::Route};
use yew_router::hooks::use_navigator;
use yew::callback::Callback;

#[function_component]
pub fn Todos() -> Html {

    //hamburger menu 

    // btn_logout

    // btn new todo

    // calendar
    
    return html! {
        <>
            <BtnGenericSmall 
            btn_text = {"LOG OUT"}
            route = {Route::Home}
            css_extra = {"display: flex; margin-right: 4em; margin-left: auto; margin-top: 1em; font-size: 1em;"}
            />
        </>
    }
}