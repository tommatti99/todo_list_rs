
use yew::prelude::*;
use crate::{routes::Route, Theme};
use crate::assets::components::{page_footer::Footer, page_header::Header, btn_genric_small::BtnGenericSmall, btn_generic::BtnGeneric};


#[function_component]
pub fn Home() -> Html {
    #[allow(unused_variables)]
    let theme = Theme::get_theme();


    //let btn_login;
 // texto titulo
    //texto 

    //let btn_create_acc;

    return html! {
        <div style={format!("background-color: {}; color: {}", theme.sec_color, theme.main_color)}>
            <Header/>
            <BtnGenericSmall 
                btn_text = {"LOG IN"}
                route = {Route::Login}
                css_extra = {"display: flex; margin-right: 4em; margin-left: auto; margin-top: 1em; font-size: 1em;"}
            />

            <div style={"margin-left: 30em; margin-top:20em; position: absolute;"}>
                <h1 style={format!("color:{}", theme.main_color)}>
                    {"Todo Planner"}
                </h1>
                <h3 style={format!("color:{}", theme.ter_color)}>
                    {"O melhor planejador de tarefas, gratuito!"}
                </h3>
            </div>

            <BtnGeneric 
            btn_text = {"Crie sua conta"}
            route = {Route::CreateAcc}
            css_extra = {"display: flex; margin-right: auto; margin-left: 40em; margin-top: 30em; font-size: 1em;"}
            />

            <Footer/>
        </div>    
    }
}