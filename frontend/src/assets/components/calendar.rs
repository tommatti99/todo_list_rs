use yew::prelude::*;
use yew::callback::Callback;
use chrono::{Datelike, NaiveDate, Utc, Days};
use crate::Theme;
use std::sync::{Arc, Mutex, MutexGuard};


lazy_static::lazy_static! {
    pub static ref CURRENT_DT_REF: Arc<Mutex<CalendarInfo>> = Arc::new(Mutex::new(CalendarInfo::default()));
  }

  
#[derive(PartialEq, Properties)]
pub struct CalendarInfo {
  pub dt: NaiveDate,
  pub calendar_days: [(u32, bool); 35]
}
impl CalendarInfo {
  fn default() -> Self {
    let dt = Utc::now().date_naive();

    return CalendarInfo {
      dt: dt,
      calendar_days: define_calendar_days(dt)
    }
  }

  pub fn get_current() -> Self {
    let dt_ref: MutexGuard<CalendarInfo> = CURRENT_DT_REF.lock().unwrap();

    let dt: NaiveDate = dt_ref.dt;

    return CalendarInfo{dt: dt,
                        calendar_days: define_calendar_days(dt)};
  }

  fn change_dt_ref(day: u32) {

    let mut dt_ref: MutexGuard<CalendarInfo> = CURRENT_DT_REF.lock().unwrap();

    let new_dt_ref: NaiveDate = NaiveDate::from_ymd_opt(dt_ref.dt.year(), dt_ref.dt.month(), day).unwrap();

    *dt_ref = CalendarInfo{dt: new_dt_ref,
                           calendar_days: define_calendar_days(new_dt_ref)};
  }

    fn sub_month() {
        let mut dt_ref: MutexGuard<CalendarInfo> = CURRENT_DT_REF.lock().unwrap();

        let new_month: u32 = dt_ref.dt.month() - 1;

        match NaiveDate::from_ymd_opt(dt_ref.dt.year(), new_month, dt_ref.dt.day()) {
        Some(new_dt_ref) => {
            *dt_ref =  CalendarInfo{dt: new_dt_ref,
                                    calendar_days: define_calendar_days(new_dt_ref)}
        },
        None => {
            if new_month <= 0 {
            let dt: NaiveDate = NaiveDate::from_ymd_opt(dt_ref.dt.year() - 1, new_month, dt_ref.dt.day()).unwrap();
            *dt_ref =  CalendarInfo{dt: dt,
                                    calendar_days: define_calendar_days(dt)};

            } else {
            let dt: NaiveDate = NaiveDate::from_ymd_opt(dt_ref.dt.year(), new_month, 1).unwrap();
            *dt_ref =  CalendarInfo{dt: dt,
                                    calendar_days: define_calendar_days(dt)}
        }
        }
    }
  }

    fn add_month() {
        let mut dt_ref: MutexGuard<CalendarInfo> = CURRENT_DT_REF.lock().unwrap();

        let new_month: u32 = dt_ref.dt.month() + 1;

        match NaiveDate::from_ymd_opt(dt_ref.dt.year(), new_month, dt_ref.dt.day()) {
        Some(new_dt_ref) => {
            *dt_ref =  CalendarInfo{dt: new_dt_ref,
                                    calendar_days: define_calendar_days(new_dt_ref)};
        },
        None => {
            if new_month > 12 {
            let dt = NaiveDate::from_ymd_opt(dt_ref.dt.year() + 1, new_month, dt_ref.dt.day()).unwrap();
            *dt_ref =  CalendarInfo{dt: dt,
                                    calendar_days: define_calendar_days(dt)};

            } else {
            let dt = NaiveDate::from_ymd_opt(dt_ref.dt.year(), new_month, 1).unwrap();
            *dt_ref =  CalendarInfo{dt: dt,
                                    calendar_days: define_calendar_days(dt)};
            }
        }
        }
  }
}


fn mes_extenso(mes: u32) -> String {
  match mes {
    1 => "Janeiro".to_string(),
    2 => "Fevereiro".to_string(),
    3 => "Março".to_string(),
    4 => "Abril".to_string(),
    5 => "Maio".to_string(),
    6 => "Junho".to_string(),
    7 => "Julho".to_string(),
    8 => "Agosto".to_string(),
    9 => "Setembro".to_string(),
    10 => "Outubro".to_string(),
    11 => "Novembro".to_string(),
    12 => "Dezembro".to_string(),
    _ => "ERRO".to_string()
  
  }
}
fn define_calendar_days(date_calendar: NaiveDate) -> [(u32, bool); 35] {
    let mut calendar_days: [(u32, bool); 35] = [(0, true); 35];
    
    let month_first_day: NaiveDate = NaiveDate::from_ymd_opt(date_calendar.year(),date_calendar.month(), 1).unwrap();
    let month_first_day_weekday: u32 = month_first_day.weekday().num_days_from_sunday();

    let mut first_calendar_day: NaiveDate = month_first_day.checked_sub_days(Days::new(month_first_day_weekday as u64)).unwrap();

    for day in 0..35 {
        calendar_days[day] = (first_calendar_day.day(), (date_calendar.month() == first_calendar_day.month()));
        first_calendar_day = first_calendar_day.checked_add_days(Days::new(1)).unwrap();
    }
    return calendar_days;
}

#[derive(Properties, PartialEq, Clone)]
pub struct CalendarMonthNameProps {
    pub dt_calendar_month_name: NaiveDate
}
#[function_component]
fn Calendar_Month_Name(props: &CalendarMonthNameProps) -> Html {
  html! {
    <div class="frame month-name-0eac3e4a3c6b">
    <div class="shape text month-f12134b7d82d">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f12134b7d82d" data-x="912" data-y="125">
        <div class="root rich-text root-0" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0">
               {mes_extenso(props.dt_calendar_month_name.month().clone())}
            </span></p>
          </div>
        </div>
      </div>
    </div>
  </div>
  }
}
#[function_component]
fn Calendar_Sub_Month() -> Html {
  html!{
    <>
    <button class="frame last-month-0d9a1c4d45e5" onclick={Callback::from(move |_| {
      CalendarInfo::sub_month();
    })}>
      <div class="shape rect rectangle-f11a6fde1df1">
      </div>
      <div class="shape group arrowprev-f12f7fd000d2">
        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" fill="none" style="-webkit-print-color-adjust::exact" viewBox="191 621 50 50"><g data-testid="arrow-prev" style="fill:#000"><path d="M224.707 627.81a2.082 2.082 0 0 0-2.946 0l-13.843 13.843a6.25 6.25 0 0 0-.001 8.837l13.688 13.698a2.082 2.082 0 1 0 2.946-2.946l-13.692-13.692a2.084 2.084 0 0 1 0-2.946l13.848-13.848a2.082 2.082 0 0 0 0-2.946Z" class="fills" data-testid="svg-path" style="fill:#0f0f0f"/>
          </g>
        </svg>
      </div>
    </button>
    </>
  }
}


#[function_component]
fn Calendar_Add_Month() -> Html {
  html!{
    <>
      <button class="frame next-month-0d9a2d96ad8d" onclick={Callback::from(move |_| {
        CalendarInfo::add_month();
      })}>
        <div class="shape rect rectangle-f120e370f4c7">
      </div>
      <div class="shape group arrownext-f12f9a6d2d6c">
        <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" fill="none" style="-webkit-print-color-adjust:exact" viewBox="1995 621 50 50"><g data-testid="arrow-next" style="fill:#000"><path d="M2011.293 664.19a2.082 2.082 0 0 0 2.946 0l13.843-13.842a6.25 6.25 0 0 0 .001-8.838l-13.688-13.698a2.082 2.082 0 1 0-2.946 2.946l13.692 13.692a2.084 2.084 0 0 1 0 2.946l-13.848 13.848a2.082 2.082 0 0 0 0 2.946Z" class="fills" data-testid="svg-path" style="fill:#0f0f0f"/>
          </g>
        </svg>
      </div>
    </button>
    </>
  }
}

#[function_component]
fn Calendar_WeekDays() -> Html {
  html!{
    <>
    <div class="frame week-days-0d99e4eb21c5">
    <div class="shape text saturday-f12821dce182">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f12821dce182" data-x="1046" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"QUI"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
    <div class="shape text friday-f12978a210c5">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f12978a210c5" data-x="428" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"SEG"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
    <div class="shape text thursday-f1298527f195">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f1298527f195" data-x="634" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"TER"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
    <div class="shape text wednesday-f12989ccace7">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f12989ccace7" data-x="840" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"QUA"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
    <div class="shape text tuesday-f12993d15dcd">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f12993d15dcd" data-x="1252" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"SEX"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
    <div class="shape text monday-f1299682a13d">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f1299682a13d" data-x="1458" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"SAB"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
    <div class="shape text sunday-f12999f45716">
      <div class="text-node-html" id="html-text-node-00937caf-e511-8022-8004-f12999f45716" data-x="1664" data-y="305">
        <div class="root rich-text root-0" style="display:flex;white-space:break-spaces;align-items:flex-start" xmlns="http://www.w3.org/1999/xhtml">
          <div class="paragraph-set root-0-paragraph-set-0">
            <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="auto"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0" style="color:rgba(0, 0, 0, 1);text-transform:none;line-break:auto;overflow-wrap:initial;white-space:break-spaces;text-rendering:geometricPrecision;caret-color:rgba(0, 0, 0, 1);text-decoration:none;--font-id:sourcesanspro;--fills:[[&quot;^ &quot;,&quot;~:fill-color&quot;,&quot;#000000&quot;,&quot;~:fill-opacity&quot;,1]];letter-spacing:0px;font-size:45px;font-family:&quot;sourcesanspro&quot;;font-style:normal;font-weight:bold">
              {"DOM"}
            </span></p>
          </div>
        </div>
      </div>
    </div>
  </div>
    </>
  }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CalendarDaysProps {
    pub calendar_days: [(u32, bool); 35],
    pub theme: Theme
}

#[function_component]
fn Calendar_Days(props: &CalendarDaysProps) -> Html {
  html!{
    <>
      <div class="frame month-days-0d9a02295970">

      { for props.calendar_days.iter().map(|&(day, is_month_day)| {
        
        if is_month_day.clone() {
          
          if day == CalendarInfo::get_current().dt.day() {
            
            html!{
              <button class="today" style={format!("color: {};  background-color: {}; border: 0.1em solid {};   ",  props.theme.main_color, props.theme.sec_color,  props.theme.main_color)}
                  onclick={Callback::from( move |_| {
                CalendarInfo::change_dt_ref(day.clone());
              })}>
            {day}</button>
              }
          } else {
            
            html!{
              <button class="month_days" style={format!("color: {}; background-color: {};", props.theme.ter_color, props.theme.sec_color)}
                onclick={Callback::from( move |_| {
                CalendarInfo::change_dt_ref(day.clone());
              })}>
              {day}</button>
              }
          }
        } else {
          
          html!{
            <button class="no_month_days" style={format!("color: gray;  background-color: {};  ", props.theme.sec_color)}
              onclick={Callback::from( move |_| {
              CalendarInfo::change_dt_ref(day.clone());
            })}>
            {day}</button>
            }
          }
        })
      }
    </div>
    </>
  }
}

#[function_component]
pub fn Calendar() -> Html {
  
    let theme = Theme::get_theme();
    
    let dt = CalendarInfo::get_current();
    
    let weekdays: [String; 7] = ["D".to_string(),"S".to_string(),"T".to_string(),"Q".to_string(),"Q".to_string(),"S".to_string(),"S".to_string()];
    let calendar_days: [(u32, bool); 35] = define_calendar_days(dt.dt);

    let calendar = {
        html!{
          <>
            <Calendar_Month_Name 
              dt_calendar_month_name = {dt.dt}
            />

            <Calendar_Sub_Month/>
            <Calendar_Add_Month/>

            <div class="calendar-body"> 
              <Calendar_WeekDays/>
              <Calendar_Days
              calendar_days = {calendar_days}
              theme = {theme}
              />
            </div>
          </>
        }
      /*
        html!{
          <>
          <div style={"display: flex; height: 100vh; width: 100vw; justify-content: center; align-items: center; justify-items: center; "}>
            <style>{format!(".calendar_body {{ }} 
                             .month_name {{text-align:center; color: {}; align-content:center; font-size:large; text-align:center; overflow: visible; white-space: nowrap; grid-column:4 / span 4;  grid-row: 1;}} 
                             .change_month_btn {{text-align:center; color: {}; height: 20em; width: 4em; align-content:center; margin:4em; margin-bottom:0.5em; font-size:large; background-color: white; border: 0.1em solid {}; cursor: pointer;}} 
                             .weekdays_header {{text-align:center; color: {}; align-content:center; margin:0.1em; margin-bottom:0.5em; font-size:large; font-weight: bold;}} 
                             .blocos::hover {{border: 0.15em solid {};font-weight: bold; color: {}; font-family:sans-serif;}}", theme.sec_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.sec_color, theme.main_color, theme.main_color)}
            </style>

           
            <div style={"display: flex; flex-direction: row; justify-content: center; align-items: center; justify-items: center;"}>    

            
            <div class="blank_space"></div>  
            
            <div class="calendar_body">

            
                
              </div>  
            </div>
            </div>
            </div>
            </>

          }
            */
    };
  
    return calendar;
}

