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




#[function_component]
pub fn Calendar() -> Html {
    let theme = Theme::get_theme();
    
    let dt = CalendarInfo::get_current();
    
    let weekdays: [String; 7] = ["D".to_string(),"S".to_string(),"T".to_string(),"Q".to_string(),"Q".to_string(),"S".to_string(),"S".to_string()];
    let calendar_days: [(u32, bool); 35] = define_calendar_days(dt.dt);

    let calendar = {
        html!{
          <>
          <div style={"display: flex; height: 100vh; width: 100vw; justify-content: center; align-items: center; justify-items: center; "}>
            <style>{format!(".calendar_body {{ margin-top: 13em; width: 80%; height: 70%; background-color:{}; border: none; display:grid; grid-template-columns: repeat(7,  14.5em); grid-template-rows: repeat(8, 7em); justify-content: center; align-content:center;}} 
                             .month_name {{text-align:center; color: {}; align-content:center; font-size:large; text-align:center; overflow: visible; white-space: nowrap; grid-column:4 / span 4;  grid-row: 1;}} 
                             .change_month_btn {{text-align:center; color: {}; height: 20em; width: 4em; align-content:center; margin:4em; margin-bottom:0.5em; font-size:large; background-color: white; border: 0.1em solid {}; cursor: pointer;}} 
                             .weekdays_header {{text-align:center; color: {}; align-content:center; margin:0.1em; margin-bottom:0.5em; font-size:large; font-weight: bold;}} 
                             .month_days {{text-align:center; color: {}; align-content:center; margin:0.1em;font-family:sans-serif; border: none; background-color: white; cursor: pointer;}} 
                             .today {{text-align:center; color: {}; align-content:center; margin:0.1em;font-family:sans-serif; border: 0.1em solid {}; background-color: white; cursor: pointer; font-weight: bold;}} 
                             .no_month_days {{text-align:center;margin:0.1em; color: {}; font-family:sans-serif; align-content:center; border:none; background-color: white; cursor: pointer; font-color: black;}} 
                             .blocos::hover {{border: 0.15em solid {};font-weight: bold; color: {}; font-family:sans-serif;}}", theme.sec_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.main_color, theme.sec_color, theme.main_color, theme.main_color)}
            </style>

            
            <div style={"display: flex; flex-direction: column;"}>
            <div class="month_name">{mes_extenso(dt.dt.month().clone())}</div>   
            
            <div style={"display: flex; flex-direction: row; justify-content: center; align-items: center; justify-items: center;"}>    
            <button class="change_month_btn" onclick={Callback::from(move |_| {
              CalendarInfo::sub_month();
            })}>{"<"}</button>    

            
            <div class="blank_space"></div>  
            
            <div class="calendar_body">

            
              { for weekdays.iter().map(|weekday| {
                html!{
                  <div class="weekdays_header">{weekday.clone()}</div>
                  }
                })
              }

              { for calendar_days.iter().map(|&(day, is_month_day)| {
                
                if is_month_day.clone() {
                  
                  if day == CalendarInfo::get_current().dt.day() {
                    
                    html!{
                      <button class="today" onclick={Callback::from( move |_| {
                        CalendarInfo::change_dt_ref(day.clone());
                      })}>
                    {day}</button>
                      }
                  } else {
                    
                    html!{
                      <button class="month_days" onclick={Callback::from( move |_| {
                        CalendarInfo::change_dt_ref(day.clone());
                      })}>
                      {day}</button>
                      }
                  }
                } else {
                  
                  html!{
                    <button class="no_month_days" onclick={Callback::from( move |_| {
                      CalendarInfo::change_dt_ref(day.clone());
                    })}>
                    {day}</button>
                    }
                  }
                })
              }  
              </div>
            <button class="change_month_btn" onclick={Callback::from(move |_| {
                CalendarInfo::add_month();
              })}> {">"}</button>    
            </div>
            </div>
            </div>
            </>


            
          }
    };

    return calendar;
}

