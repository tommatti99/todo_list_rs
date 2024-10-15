use yew::prelude::*;
use yew_router::prelude::*;
use crate::{routes::Route, Theme};
use yew_router::hooks::use_navigator;
use yew::callback::Callback;
use crate::assets::components::{page_footer::Footer, page_header::Header, btn_genric_small::BtnGenericSmall, btn_generic::BtnGeneric};

#[function_component]
pub fn Login() -> Html {
    let theme = Theme::get_theme();

    return html! {
        <>
        /*
        <div class="shape bool svg-1017707112cf">

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
                    css_extra = {""}
                />
                <BtnGenericSmall 
                    btn_text = {"Esqueci a senha"}
                    route = {Route::RecoverAcc}
                    css_extra = {""}
            />


*/


<div class="frame board-1014866c54e3">
  <div class="shape bool svg-1017707112cf">
    <svg xmlns="http://www.w3.org/2000/svg" width="1174" height="1831.9" fill="none" style="-webkit-print-color-adjust::exact" viewBox="1001 -1828 1174 1831.9"><g data-testid="svg"><g class="fills"><path d="M1227.923,-1748.351L1229.925,-1742.456C1132.290,-1708.922,1030.894,-1595.532,1038.989,-1425.820C1045.520,-1288.894,1106.912,-1200.087,1161.077,-1121.734C1167.139,-1112.965,1173.147,-1104.275,1179.002,-1095.622C1220.577,-1154.201,1280.556,-1211.771,1350.780,-1260.106C1428.774,-1313.790,1510.819,-1350.698,1581.800,-1364.033C1618.319,-1370.892,1682.260,-1370.940,1742.292,-1348.495C1792.536,-1329.710,1856.393,-1288.242,1875.282,-1197.371C1888.836,-1132.163,1879.983,-1072.876,1848.219,-1016.122C1820.573,-966.728,1778.987,-925.464,1742.296,-889.058C1695.719,-842.843,1655.495,-802.931,1649.406,-756.943C1637.444,-666.606,1720.044,-650.758,1824.618,-630.694C1953.349,-605.994,2113.561,-575.254,2144.926,-383.207C2152.979,-333.905,2150.803,-283.216,2138.461,-232.547C2126.716,-184.329,2105.763,-136.042,2076.183,-89.026L2070.964,-92.368C2130.798,-187.473,2154.265,-287.697,2138.830,-382.205C2108.167,-569.951,1950.312,-600.238,1823.472,-624.575C1719.950,-644.438,1630.546,-661.592,1643.282,-757.778C1649.651,-805.879,1690.573,-846.484,1737.958,-893.501C1813.412,-968.369,1898.933,-1053.226,1869.235,-1196.103C1836.648,-1352.875,1662.273,-1372.821,1582.943,-1357.910C1441.409,-1331.322,1271.313,-1216.334,1182.704,-1090.126C1210.335,-1048.878,1234.090,-1008.209,1243.750,-963.086C1259.844,-887.908,1232.755,-841.035,1204.666,-823.739C1190.159,-814.807,1172.769,-812.906,1158.144,-818.657C1146.324,-823.304,1130.362,-835.403,1121.441,-866.647C1112.360,-898.452,1111.637,-932.573,1119.290,-968.062C1126.378,-1000.929,1140.644,-1035.036,1161.695,-1069.436C1165.928,-1076.354,1170.437,-1083.279,1175.202,-1090.190C1168.948,-1099.462,1162.513,-1108.772,1156.010,-1118.179C1101.368,-1197.222,1039.435,-1286.813,1032.818,-1425.531C1028.986,-1505.861,1049.486,-1580.519,1092.102,-1641.431C1127.376,-1691.850,1176.881,-1730.821,1227.923,-1748.351ZZM1180.519,-821.852C1187.696,-822.596,1194.887,-825.022,1201.439,-829.056C1227.803,-845.291,1253.122,-889.793,1237.711,-961.782C1228.433,-1005.118,1205.620,-1044.613,1178.917,-1084.652C1174.731,-1078.512,1170.735,-1072.338,1166.954,-1066.159C1124.109,-996.141,1110.423,-927.743,1127.379,-868.360C1133.885,-845.570,1145.297,-830.390,1160.380,-824.459C1166.684,-821.981,1173.595,-821.133,1180.519,-821.852ZZ" style="fill:#12cf8b"/></g></g></svg>
  </div>
  <div class="frame login-fiel-1016d30dca45">
    <div class="shape rect rectangle-1014cf2cdcf0">
    </div>

    <div class="shape text entrar-1014cf2cdcf1">
    
      <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-1014cf2cdcf1" data-x="575.7874306839186" data-y="-822.3542600896862">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(255, 255, 255, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(255, 255, 255, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#ffffff&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:27px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
                {"Entrar"}
            </span></p>
            <p class="paragraph root-0-paragraph-set-0-paragraph-1" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-1-text-0" style="color:rgba(255, 255, 255, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(255, 255, 255, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#ffffff&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:27px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold"> </span></p>
          </div>
        </div>
      </div>
    </div>


    <div class="shape text esqueci-a-10152056ded2">
      <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-10152056ded2" data-x="501.9242144177449" data-y="-703.517937219731">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(18, 207, 139, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(18, 207, 139, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#12cf8b&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:27px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:400">
                {"Esqueci a senha"}
                </span></p>
          </div>
        </div>
      </div>
    </div>


    <div class="shape rect rectangle-101502487f62">
    </div>

    <div class="shape rect rectangle-101502487f62">
      
    </div>


    <div class="shape text email-1016eb497c5b">
      <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-1016eb497c5b" data-x="493" data-y="-1023">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(18, 207, 139, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:pre;text-rendering:geometricPrecision;caret-color:rgba(18, 207, 139, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#12cf8b&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:35px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:400">
                {"Email"}
                </span></p>
          </div>
        </div>
      </div>
    </div>


    <div class="shape text senha-1017033d94f9">
      <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-1017033d94f9" data-x="493" data-y="-931">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(18, 207, 139, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:pre;text-rendering:geometricPrecision;caret-color:rgba(18, 207, 139, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#12cf8b&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:35px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:400">
                {"Senha"}
                </span></p>
          </div>
        </div>
      </div>
    </div>


    <div class="shape text login-101658ee4821">
      <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-101658ee4821" data-x="502" data-y="-1144">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:55px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:400">
                {"Login"}
                </span></p>
          </div>
        </div>
      </div>
    </div>
  </div>


  <div class="frame home-1014b2ccb85a">
    <div class="shape text home-1014b2ccb85b">
      <div class="text-node-html" id="html-text-node-95c43ca6-14dd-80c8-8005-1014b2ccb85b" data-x="364" data-y="-1332">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(18, 207, 139, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:pre;text-rendering:geometricPrecision;caret-color:rgba(18, 207, 139, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#12cf8b&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:25px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
                {"Home"}
                </span></p>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="shape group svg-10112eacca74">
    <svg xmlns="http://www.w3.org/2000/svg" width="878.1" height="631" fill="none" style="-webkit-print-color-adjust::exact" viewBox="111 -811 878.1 631"><g data-testid="svg" style="fill:#000"><g data-testid="svg-path"><g class="fills"><path d="M676.608,-424.631C609.863,-390.742,635.756,-349.268,664.998,-371.526C680.389,-383.692,686.052,-406.812,676.608,-424.631ZZM881.319,-405.396C913.212,-378.663,936.316,-340.391,941.078,-299.400C942.067,-289.619,942.547,-279.970,939.873,-270.670C937.419,-265.564,931.320,-268.470,931.686,-273.693C931.646,-276.639,932.237,-279.535,932.483,-282.456C933.076,-291.320,932.268,-300.254,930.839,-309.049C925.617,-336.064,912.609,-361.553,894.168,-382.372C878.134,-401.209,857.506,-415.012,835.035,-425.002C786.985,-446.770,729.758,-448.577,682.185,-427.305C693.786,-405.594,685.801,-376.893,665.580,-363.817C658.169,-359.008,648.180,-356.764,639.741,-360.796C630.735,-364.641,627.272,-375.157,628.671,-384.071C629.454,-389.405,631.890,-394.318,634.730,-398.827C644.223,-412.622,658.924,-421.959,673.669,-429.622C663.663,-444.173,646.792,-452.424,630.341,-457.642C578.469,-473.079,526.598,-449.728,474.445,-451.547C440.718,-452.624,403.084,-465.572,385.102,-496.013C370.858,-521.116,376.483,-542.617,382.931,-568.314C388.013,-589.148,384.650,-611.880,373.977,-630.830C337.707,-694.783,257.973,-709.526,192.371,-690.061C177.457,-685.798,163.235,-679.868,149.594,-672.941C144.248,-670.373,138.894,-667.823,133.775,-664.849C132.359,-664.047,130.432,-664.739,129.671,-666.146C128.862,-667.639,129.476,-669.367,130.893,-670.170C152.823,-683.011,176.325,-693.485,201.346,-699.225C241.709,-708.727,286.441,-705.907,324.281,-686.909C348.838,-674.266,369.871,-654.069,382.344,-629.565C391.413,-611.623,394.564,-590.997,390.758,-571.537C388.319,-557.799,383.441,-544.460,383.319,-530.328C382.992,-515.028,389.222,-499.197,399.977,-488.025C422.020,-465.464,454.815,-457.622,485.242,-458.421C517.588,-459.233,549.026,-467.492,581.321,-468.872C603.671,-470.061,626.688,-467.268,647.292,-457.670C659.761,-451.914,671.683,-443.867,679.295,-432.286C743.818,-461.109,825.067,-449.897,881.319,-405.396ZZ" style="fill:#12cf8b"/></g></g></g></svg>
  </div>
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