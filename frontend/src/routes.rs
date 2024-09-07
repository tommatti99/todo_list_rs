use yew_router::prelude::*;
use yew::prelude::*;
use crate::assets::pages::{home::Home, todos::Todos, authentication::{Login, CreateAcc, RecoverAcc}};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/authentication/login")]
    Login,

    #[at("/authentication/new_acc")]
    CreateAcc,

    #[at("/authentication/recover")]
    RecoverAcc,

    #[at("/logged/todos")]
    Todos,

}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::CreateAcc => html! { <CreateAcc /> },
        Route::RecoverAcc => html! { <RecoverAcc /> },
        Route::Todos => html! { <Todos /> }
    }
}
