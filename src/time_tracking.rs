#[path = "time_utils.rs"] mod time_utils;
#[path = "model.rs"] mod model {}

use crate::model::TimeTrackingData;
use crate::time_tracking::time_utils::{workable_days, working_days, current_month, current_year};

pub fn hours_report(data:TimeTrackingData) -> String {
    let today = time_utils::today().to_string();
    let mut report:String = format!("======================================\n\
       TIME_TRACKING.rs by Diego Pacheco\n\
--------------------------------------
Project: {}
Goal   : {} working hours
======================================\n",data.config.project_name,data.config.base);
    report = format!("{} {} {}/{}/{}\n",report,"Today Is      : ",today, current_month(),current_year());
    report = format!("{} {} {}\n",report,"Working Days  : ",time_utils::working_days());
    report = format!("{} {} {}\n",report,"Worked  Days  : ",time_utils::worked_days());
    report = format!("{} {} {}\n",report,"Remain  Days  : ",time_utils::workable_days());

    let total_hours = data.config.base as f32 - data.month_data.worked_hours as f32;
    report = format!("{} {} {}{}\n",report,"Need to Work  : ",total_hours," hours total <<< ");

    let avg_working_hours_yet = (data.config.base as f32 - data.month_data.worked_hours as f32) / workable_days() as f32;
    report = format!("{} {} {:.2}{}\n",report,"Need to Work  : ",avg_working_hours_yet," avg hours yet ");

    let avg =diff_hours(data.config.base as f32,data.month_data.worked_hours) / (workable_days() - data.month_data.holidays_count) as f32;
    report = format!("{} {:.2} {}",report,"AVG Work/Hours: ",avg);
    report = format!("{} {}",report,get_avg_hours_predictions());
    report = format!("{}{}",report,"=====================================");
    return String::from(report);
}

fn get_avg_hours_predictions() -> String {
    let mut report:String = String::from("\n Hours Predictions \n");
    report = format!("{} {} {}\n",report,"7h   : ",working_days()*7);
    report = format!("{} {} {}\n",report,"8h   : ",working_days()*8);
    report = format!("{} {} {}\n",report,"9h   : ",working_days()*9);
    report = format!("{} {} {}\n",report,"10h  : ",working_days()*10);
    return format!("{}",report);
}

fn diff_hours(base:f32,worked:f32) -> f32{
    return base - worked;
}