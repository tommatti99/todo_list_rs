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
        <div>
            <button class="frame log-out-0eb5c8701c47" onclick={onclick}>
            <div class="shape text log-out-0eb57dcbe410">
              <div class="text-node-html" id="html-text-node-d8bee35b-299f-8000-8005-0eb57dcbe410" data-x="1900" data-y="50">
                <div class="root rich-text root-0" xmlns="http://www.w3.org/1999/xhtml">
                  <div class="paragraph-set root-0-paragraph-set-0">
                    <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style={format!("caret-color:{}; color:{};", theme.main_color,theme.main_color)}>
                    {props.btn_text.clone()}
                    </span></p>
                  </div>
                </div>
              </div>
            </div>
            </button>
        </div>
    }
}
