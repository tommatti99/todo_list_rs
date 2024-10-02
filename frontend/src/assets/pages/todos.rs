use yew::prelude::*;
use yew_router::prelude::*;
use crate::{assets::components::{btn_genric_small::BtnGenericSmall, menu_hamburger::HamburgerMenu}, routes::Route};
use yew_router::hooks::use_navigator;
use yew::callback::Callback;
use chrono::NaiveDate;
use crate::assets::components::calendar::Calendar;
use crate::assets::components::btn_new_task::New_Task;

#[derive(PartialEq, Properties)]
pub struct TodoItem {
    pub user_id: i32,
    pub todo_id: i32,
    pub creation_dt: NaiveDate,
    pub active_status: bool,
    pub title: String,
    pub description: String,
    pub set_dt: NaiveDate,
    pub color: String
}


#[function_component]
pub fn Todos() -> Html {

    let todo_mock = 
            TodoItem {
                user_id: 1,
                todo_id: 1,
                creation_dt: NaiveDate::from_ymd(2024, 08, 01),
                active_status: true,
                title: "a".to_string(),
                description: "a".to_string(),
                set_dt: NaiveDate::from_ymd(2024, 08, 01),
                color: "color".to_string()
            };

    return html! {
        <>
            <BtnGenericSmall 
                btn_text = {"LOG OUT"}
                route = {Route::Home}
                css_extra = {"display: flex; margin-right: 4em; margin-left: auto; margin-top: 1em; font-size: 1em;"}
            />

            <HamburgerMenu 
                todos = {vec![todo_mock]}/>


            <New_Task />
            <Calendar />
        </>
    }
}
