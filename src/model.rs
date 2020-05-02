pub struct WorkConfig{
    pub base:i32,
    pub project_name:String,
}
pub struct MonthData{
    pub holidays_count:i32,
    pub worked_hours:i32,
}
pub struct TimeTrackingData{
    pub month_data:MonthData,
    pub config:WorkConfig,
}