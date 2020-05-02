mod time_tracking;
mod model;
use crate::model::*;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprint!("Error! You need pass 2 arguments: Worked_hours and holidays_count.\nI.g: ./time-tracking 10 0");
        exit(1);
    }
    let data = parse_configs(args);
    println!("{}", time_tracking::hours_report(data));
}

fn parse_configs(args:Vec<String>) -> TimeTrackingData{
    let worked_hours = args[1].parse::<f32>().unwrap_or(1.0);
    let holidays = args[2].parse::<i32>().unwrap_or(0);
    let project_name = args.get(3).unwrap_or(&String::from("Work")).to_string();
    let data = TimeTrackingData {
        month_data: MonthData {
            holidays_count: holidays,
            worked_hours: worked_hours,
        },
        config: WorkConfig{
            project_name: project_name,
            base: 200,
        },
    };
    return data;
}