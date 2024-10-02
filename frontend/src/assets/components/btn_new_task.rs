use yew::prelude::*;

#[function_component]
fn New_Task_Svg() -> Html {
    html!{
        <>
            <svg width="45.259" xmlns="http://www.w3.org/2000/svg" height="45.659" id="screenshot-00937caf-e511-8022-8004-f12216475b0b" viewBox="439.333 101.008 45.259 45.659" style="-webkit-print-color-adjust: exact" fill="none" version="1.1">
                <g id="shape-00937caf-e511-8022-8004-f12216475b0b" data-testid="svg-path">
                <g class="fills" id="fills-00937caf-e511-8022-8004-f12216475b0b">
                    <path d="M458.667,106.333L449.800,106.333C447.186,106.333,445.880,106.333,444.881,106.842C444.003,107.289,443.289,108.003,442.842,108.881C442.333,109.880,442.333,111.186,442.333,113.800L442.333,136.200C442.333,138.814,442.333,140.120,442.842,141.119C443.289,141.997,444.003,142.711,444.881,143.158C445.880,143.667,447.186,143.667,449.800,143.667L472.200,143.667C474.814,143.667,476.120,143.667,477.119,143.158C477.997,142.711,478.711,141.997,479.158,141.119C479.667,140.120,479.667,138.814,479.667,136.200L479.667,126.167M469.167,109.833L475.766,116.433M458.113,120.887L473.626,105.374C475.448,103.552,478.403,103.552,480.225,105.374C482.048,107.197,482.048,110.152,480.225,111.974L464.214,127.985C462.437,129.763,461.548,130.651,460.536,131.358C459.638,131.985,458.669,132.504,457.649,132.906C456.501,133.358,455.269,133.607,452.805,134.104L451.667,134.333L451.777,133.558C452.169,130.816,452.365,129.445,452.811,128.164C453.206,127.028,453.746,125.948,454.418,124.950C455.175,123.825,456.154,122.846,458.113,120.887ZZ" fill="none" stroke-linejoin="round" style="fill:none">
                    </path>
                </g>
                <g fill="none" stroke-linejoin="round" id="strokes-00937caf-e511-8022-8004-f12216475b0b" class="strokes">
                    <g class="stroke-shape">
                    <path d="M458.667,106.333L449.800,106.333C447.186,106.333,445.880,106.333,444.881,106.842C444.003,107.289,443.289,108.003,442.842,108.881C442.333,109.880,442.333,111.186,442.333,113.800L442.333,136.200C442.333,138.814,442.333,140.120,442.842,141.119C443.289,141.997,444.003,142.711,444.881,143.158C445.880,143.667,447.186,143.667,449.800,143.667L472.200,143.667C474.814,143.667,476.120,143.667,477.119,143.158C477.997,142.711,478.711,141.997,479.158,141.119C479.667,140.120,479.667,138.814,479.667,136.200L479.667,126.167M469.167,109.833L475.766,116.433M458.113,120.887L473.626,105.374C475.448,103.552,478.403,103.552,480.225,105.374C482.048,107.197,482.048,110.152,480.225,111.974L464.214,127.985C462.437,129.763,461.548,130.651,460.536,131.358C459.638,131.985,458.669,132.504,457.649,132.906C456.501,133.358,455.269,133.607,452.805,134.104L451.667,134.333L451.777,133.558C452.169,130.816,452.365,129.445,452.811,128.164C453.206,127.028,453.746,125.948,454.418,124.950C455.175,123.825,456.154,122.846,458.113,120.887ZZ" style="fill:none;stroke-width:2;stroke:#000000;stroke-opacity:1">
                    </path>
                    </g>
                </g>
                </g>
            </svg>      
        </>
    }
}

#[function_component]
pub fn New_Task() -> Html {
    return html! {
        <>

            <button class="frame new-task-0d99bbdf5dde">
            <div class="shape rect rectangle-f11a99c4b282">
            </div>
            <div class="shape text nova-taref-f1218e768127">
            <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f1218e768127" data-x="506" data-y="102">
                <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
                <div class="paragraph-set root-0-paragraph-set-0">
                    <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:pre;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:38px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:400">{"Nova Tarefa"}</span></p>
                </div>
                </div>
            </div>
            </div>
            <div class="shape group pensquare-f1221645c65a">
                <New_Task_Svg />
            </div>
        </button>
        </>
    };
}