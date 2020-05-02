mod time_tracking;
mod model;
use crate::model::*;

fn main() {
    let data = TimeTrackingData {
        month_data: MonthData {
            holidays_count:0,
            worked_hours: 9,
        },
        config: WorkConfig{
            project_name: String::from("work"),
            base: 200,
        },
    };
    println!("{}", time_tracking::hours_report(data));
}
