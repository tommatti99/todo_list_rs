use yew::prelude::*;
use crate::assets::pages::todos::TodoItem;

#[function_component]
pub fn Box_Todo_Edit(todo: &TodoItem) -> Html {
    let edit_todo_toggle = use_state(|| false);
    
    return html! {
        <> 
            {"editing"}
        </>
    };
}