#[path = "time_utils.rs"] mod time_utils;
#[path = "model.rs"] mod model {}

use crate::model::TimeTrackingData;
use crate::time_tracking::time_utils::{workable_days, business_days, current_month, current_year};

pub fn hours_report(data:TimeTrackingData) -> String {
    let today = time_utils::today().to_string();

    let mut report:String = format!("======================================\n\
       TIME_TRACKING.rs by Diego Pacheco\n\
--------------------------------------
Project  : {}
Goal     : {} hours
Days Off : {} days
======================================\n",data.config.project_name,data.config.base,data.month_data.holidays_count);
    report = format!("{} {} {}/{}/{}\n",report,"Today Is      : ",today, current_month(),current_year());
    report = format!("{} {} {}\n",report,"Business Days : ",time_utils::business_days());
    report = format!("{} {} {}\n",report,"Worked  Days  : ",time_utils::worked_days());
    report = format!("{} {} {}\n",report,"Remain  Days  : ",time_utils::workable_days() - data.month_data.holidays_count);
    report = format!("{} {} {}\n",report,"Worked  Hours : ",data.month_data.worked_hours);
    report = format!("{}{}",report, calculate_need_to_work(data));
    report = format!("{}{}",report,"-------------------------------------");
    report = format!("{} {}",report,get_avg_hours_predictions());
    report = format!("{}{}",report,"=====================================");
    return report;
}

fn calculate_need_to_work(data:TimeTrackingData) -> String {
    let mut report:String = format!("");
    let total_hours = data.config.base as f32 - data.month_data.worked_hours as f32;
    report = format!("{} {} {}{}\n",report,"Need to Work  : ",total_hours," hours total <<< ");

    let hours_to_work = data.config.base as f32 - data.month_data.worked_hours as f32;
    let available_days = (workable_days() - data.month_data.holidays_count) as f32;
    let avg_working_hours_yet = hours_to_work / available_days;
    report = format!("{} {} {:.2}{}\n",report,"Need to Work  : ",avg_working_hours_yet," avg hours yet ");
    return report;
}

fn get_avg_hours_predictions() -> String {
    let mut report:String = String::from("\n Hours Predictions \n");
    report = format!("{} {} {} {}\n", report, "7h per day    : ", business_days()*7, "h");
    report = format!("{} {} {} {}\n", report, "8h per day    : ", business_days()*8, "h");
    report = format!("{} {} {} {}\n", report, "9h per day    : ", business_days()*9, "h");
    report = format!("{} {} {} {}\n", report, "10h per day   : ", business_days()*10, "h");
    return report;
}