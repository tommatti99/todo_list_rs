use yew::prelude::*;
use yew_router::{navigator, prelude::*};
use crate::{routes::Route, Theme};
use yew_router::hooks::use_navigator;
use yew::callback::Callback;
use crate::assets::components::{page_footer::Footer, page_header::Header, btn_genric_small::BtnGenericSmall, btn_generic::BtnGeneric};

#[function_component]
pub fn Login() -> Html {
    let theme = Theme::get_theme();

    return html! {
        <>
            <div style={"position: absolute; margin-left: 12em; margin-top: 13em;"}>
                <h2>{"Login"}</h2>
                <div style={"flex-direction: column;"}>
                <input placeholder="Email" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)}
                />
                <input placeholder="Senha" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)}
                />
                </div>
                <BtnGeneric
                    btn_text = {"Entrar"}
                    route = {Route::Todos}
                    css_extra = {"margin-top: 0.5em;"}
                />
                <BtnGenericSmall 
                    btn_text = {"Esqueci a senha"}
                    route = {Route::RecoverAcc}
                    css_extra = {"margin-top: 0.8em; font-size: 1em;"}
            />
            </div>
        </>
    }
}

fn login_success() -> bool {
    return true;
}

#[function_component]
pub fn RecoverAcc() -> Html {
    let theme = Theme::get_theme(); 
    let insert_recover_code_state = use_state(|| false);
    let navigator: Navigator = use_navigator().unwrap();

    let insert_recover_code_toggle: Callback<MouseEvent> = {
        let insert_recover_code_state = insert_recover_code_state.clone();
        Callback::from( move |_| insert_recover_code_state.set(!*insert_recover_code_state))
    };

    let send_recover_code_field = {
        html!{
            <>
                <div style={"position: absolute; margin-left: 12em; margin-top: 13em;"}>
                    <h2>{"Recuperar Senha"}</h2>
                    <input placeholder="Email" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)}
                    />
                    <button style={format!("color: {}; height: 3em; justify-content: center; text-align: center; align-items: center; width: 10em; border-radius:1em; border: none; background-color: {}; cursor: pointer; font-weight: bold; margin-top: 0.5em;", theme.sec_color, theme.main_color)} 
                    onclick={insert_recover_code_toggle}>{"Enviar"}</button>                   
                </div>
            </>
        }
    };

    let recover_result = {
        if login_success() {
            let navigator = navigator.clone();
            Callback::from(move |_| navigator.push(&Route::Todos))
        } else {
            Callback::from(move |_| navigator.push(&Route::RecoverAcc))
        }
    };

    let insert_recover_code_field = {
        html!{
            <>
                <div style={"position: absolute; margin-left: 12em; margin-top: 13em;"}>
                    <h2>{"Insira o código recebido por email"}
                    </h2>
                        <div style={"display: flex; gap: 0.2em;"}>
                            <input style={format!("text-align: center; display: flex; margin-bottom: 0.2em; border-radius:1.1em; border: 0.1em solid {}; box-shadow: none; width: 2.5em; height: 2.5em;", theme.main_color)}/>
                            <input style={format!("text-align: center; display: flex; margin-bottom: 0.2em; border-radius:1.1em; border: 0.1em solid {}; box-shadow: none; width: 2.5em; height: 2.5em;", theme.main_color)}/>
                            <input style={format!("text-align: center; display: flex; margin-bottom: 0.2em; border-radius:1.1em; border: 0.1em solid {}; box-shadow: none; width: 2.5em; height: 2.5em;", theme.main_color)}/>
                            <input style={format!("text-align: center; display: flex; margin-bottom: 0.2em; border-radius:1.1em; border: 0.1em solid {}; box-shadow: none; width: 2.5em; height: 2.5em;", theme.main_color)}/>
                            <input style={format!("text-align: center; display: flex; margin-bottom: 0.2em; border-radius:1.1em; border: 0.1em solid {}; box-shadow: none; width: 2.5em; height: 2.5em;", theme.main_color)}/>
                            <input style={format!("text-align: center; display: flex; margin-bottom: 0.2em; border-radius:1.1em; border: 0.1em solid {}; box-shadow: none; width: 2.5em; height: 2.5em;", theme.main_color)}/>
                </div>
                    <button style={format!("color: {}; height: 3em; justify-content: center; text-align: center; align-items: center; width: 10em; border-radius:1em; border: none; background-color: {}; cursor: pointer; font-weight: bold; margin-top: 0.5em;", theme.sec_color, theme.main_color)} 
                    onclick={recover_result}>{"Enviar"}</button>                   
                </div>
            </>
        }
    };

    let recover_acc_field = {
        if *insert_recover_code_state {
            insert_recover_code_field
        } else {
            send_recover_code_field
        }};

    return recover_acc_field;
}


#[function_component]
pub fn CreateAcc() -> Html {
    let theme: Theme = Theme::get_theme(); 
    let navigator: Navigator = use_navigator().unwrap();

    let onclick: Callback<MouseEvent> = {
        let navigator: Navigator = navigator.clone();    
        Callback::from(move |_| navigator.push(&Route::Login))
    };
    return html! {
        <>
            <Header />
            <BtnGenericSmall 
                btn_text = {"Home"}
                route = {Route::Home}
                css_extra = {"display: flex; margin-right: auto; margin-left: 7em; margin-top: 1em; font-size: 1em;"}
           />

           <div style={"position: absolute; margin-left: 12em; margin-top: 13em;"}>
                <h2>{"Criar novo usuário"}
                </h2>
                    <input placeholder="Nome" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)} /> 
                    <input placeholder="Email" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)} />
                    <input placeholder="Senha" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)} />
                    <input placeholder="Confirmação de senha" style={format!("display: flex; margin-bottom: 0.2em; border-radius:1.5em; border: 0.2em solid {}; box-shadow: none; width: 24em; height: 3em;", theme.main_color)} />
                    <button style={format!("color: {}; height: 3em; justify-content: center; text-align: center; align-items: center; width: 10em; border-radius:1em; border: none; background-color: {}; cursor: pointer; font-weight: bold; margin-top: 0.5em;", theme.sec_color, theme.main_color)} 
                    onclick={onclick}>{"Enviar"}</button>   
            </div>
            
            <Footer/>
        </>
    }
}