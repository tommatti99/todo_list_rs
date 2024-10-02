use yew::prelude::*;
use crate::assets::pages::todos::TodoItem;
use crate::Theme;
use yew_router::prelude::Navigator;
use yew_router::hooks::use_navigator;
use yew::callback::Callback;
use crate::assets::components::box_todo_edit::Box_Todo_Edit;

#[derive(Properties, PartialEq)]
pub struct HamburgerMenuProps {
    pub todos: Vec<TodoItem>
}


#[function_component]
pub fn HamburgerMenu(props: &HamburgerMenuProps) -> Html {
    let theme: Theme = Theme::get_theme();
   
    let navigator: Navigator = use_navigator().unwrap();
    let hamburger_state = use_state(|| false);

    let hamburger_toggle: Callback<MouseEvent> = {
        let hamburger_state = hamburger_state.clone();
        Callback::from(move |_| hamburger_state.set(!*hamburger_state))
    };

    let hamburger_close = {
        html! {

            <button onclick={hamburger_toggle.clone()} class="frame hamb-menu-0d99a3a5378f" style={format!("background-color: {};",theme.sec_color)}>
                <div class="shape rect rectangle-f1225598b64a" style={format!("background-color: {};", theme.main_color)}></div>
                <div class="shape rect rectangle-f122bc99d204" style={format!("background-color: {};", theme.main_color)}></div>
                <div class="shape rect rectangle-f122bcda537b" style={format!("background-color: {};", theme.main_color)}></div>
          </button>
        }
    };

    let hamburger_open = {
        html! {
            <>
                <div class="hamburger_menu" style={format!("width: 25em; height: 100vh; background-color: {}; transition: height 0.3s ease; border-top-right-radius: 2em; border-bottom-right-radius: 2em; z-index: 101;", theme.sec_color)}>
                    <button onclick={hamburger_toggle.clone()} class="btn_hamburger_menu_opened" style={format!("margin-right: auto; margin-left: 24em; background-color: {}; margin-top: 0.55em; border: none; cursor: pointer; width: 3.5em; height: 2.2em; display: flex; flex-direction: column; justify-content: space-between;",  theme.sec_color)}>
                        <div class="btn_hamburger_menu_comp" style={format!("background-color: {}; width: 100%; height: 0.4em; border-radius: 1em; margin: 0em; transition: all 0.3s ease;",theme.main_color)}></div>
                        <div class="btn_hamburger_menu_comp" style={format!("background-color: {}; width: 100%; height: 0.4em; border-radius: 1em; margin: 0em; transition: all 0.3s ease;",theme.main_color)}></div>
                        <div class="btn_hamburger_menu_comp" style={format!("background-color: {}; width: 100%; height: 0.4em; border-radius: 1em; margin: 0em; transition: all 0.3s ease;",theme.main_color)}></div>
                    </button>


                    <div class="hamburger_menu_groups_parent">
                        <div class="hamburger_menu_groups" style={format!("font-family: sans-serif; font-weight: bold; color: {}; margin: 2em; font-size: x-large;",theme.main_color)}>
                        { for props.todos.iter().enumerate().map(|todo| {
                            html! { 
                                    <>
                                        <button class="hamburger_menu_groups_items" style={format!("margin-top: 1em; font-family: sans-serif; font-weight: normal; color: {}; cursor: pointer; margin-left: 0em; font-size: large; margin-bottom: 4em; background-color: {}; border: none; width: 12em;",theme.main_color, theme.sec_color)}>
                                        {todo.1.title.clone()}
                                        </button>                                       
                                    </>
                                }
                        })}
                        </div>
                    </div>
                </div>
            <div class="overlay"></div>
        </>
        }
    };

    let hamburger = {
        if *hamburger_state {
            hamburger_open
        }else {
            hamburger_close
        }
    };
    return hamburger;
}