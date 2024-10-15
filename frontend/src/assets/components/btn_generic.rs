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
        <>
        <button class="frame create-acc-1013bd0321c1" style={format!("{}", props.css_extra )}>
          <div class="shape rect rectangle-1013bd0321c2">
          </div>
          <div class="shape text crie-sua-c-1013bd0321c3">
            <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-1013bd0321c3" data-x="847.5" data-y="-2152.083333333333">
              <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
                <div class="paragraph-set root-0-paragraph-set-0">
                  <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(255, 255, 255, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(255, 255, 255, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#ffffff&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:27px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold" 
                    onclick={onclick}>{props.btn_text.clone()}
                  </span></p>
                </div>
              </div>
            </div>
          </div>
        </button>
        </>
    }
}