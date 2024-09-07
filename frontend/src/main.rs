use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::switch::Switch;
use crate::routes::{Route, switch};
use std::sync::{Arc, Mutex, MutexGuard};
use yew::function_component;

pub mod assets {
    pub mod components {
        pub mod box_todo_edit;
        pub mod btn_cog;
        pub mod btn_generic;
        pub mod btn_genric_small;
        pub mod btn_new_todo;
        pub mod menu_hamburger;
        pub mod page_footer;
        pub mod page_header;
    }
    pub mod images {

    }
    pub mod pages {
        pub mod todos;
        pub mod home;
        pub mod authentication;
    }

}
pub mod api;
pub mod routes;

pub struct Theme {
    pub main_color: String,
    pub sec_color: String,
    pub ter_color: String
}
impl Theme {
    pub fn get_theme() -> Self {
        let current_theme: MutexGuard<Theme> = THEME.lock().unwrap();

        return Self {
            main_color: current_theme.main_color.clone(),
            sec_color: current_theme.sec_color.clone(),
            ter_color: current_theme.ter_color.clone()
        }
    }
    pub fn aqua_green_theme() -> Self {
        return Self {
            main_color: "#12cf8b".to_string(), 
            sec_color: "#ffffff".to_string(),
            ter_color: "#000000".to_string()
        }
    }
}

lazy_static::lazy_static! {
    static ref THEME: Arc<Mutex<Theme>> = Arc::new(Mutex::new(Theme::aqua_green_theme()));
}

#[function_component(Main)]
pub fn app() -> Html {
    let theme: Theme = Theme::aqua_green_theme();

    return html! {
        <div style={format!("background-color: {};",theme.sec_color)}>
            <BrowserRouter>
                    <Switch<Route> render={|routes: Route| switch(&routes)} />
            </BrowserRouter>
        </div>
    };
}


pub fn main() {
    yew::Renderer::<Main>::new().render();
}